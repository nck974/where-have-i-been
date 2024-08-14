import { AfterViewInit, Component, signal, WritableSignal } from '@angular/core';
import * as L from 'leaflet';
import 'leaflet.heat';
import { TrackService } from '../track.service';
import { Coordinate } from '../model/coordinate';
import { FileList } from '../model/files';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatTooltip } from '@angular/material/tooltip';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatChipsModule } from '@angular/material/chips';
import { HeatmapService } from '../heatmap.service';
import { HeatmapCoordinate } from '../model/heatmap';

@Component({
  selector: 'app-map',
  standalone: true,
  imports: [MatButtonModule, MatIconModule, MatTooltip, MatProgressSpinnerModule, MatChipsModule],
  templateUrl: './map.component.html',
  styleUrl: './map.component.scss'
})
export class MapComponent implements AfterViewInit {

  private map!: L.Map;
  private static defaultLocation: L.LatLng = new L.LatLng(49.4521, 11.0767);
  displayedTracks: L.Polyline[] = []
  displayedHeatmap: L.HeatLayer[] = []
  tracksToDownload: WritableSignal<number> = signal(0);
  downloadedTracks: WritableSignal<number> = signal(0);
  isLoadingTracks = false;

  constructor(private trackService: TrackService, private heatmapService: HeatmapService) { }

  ngAfterViewInit(): void {
    this.initializeMap();
  }

  /// This adds one layer to the map
  private addDefaultMap(): void {
    const tiles = L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      maxZoom: 18,
      minZoom: 3,
      attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
    });

    tiles.addTo(this.map);
  }


  /// Create an empty map without any layer and then add the default map
  private initializeMap(): void {
    this.map = L.map('map', {
      center: MapComponent.defaultLocation,
      zoom: 3
    });
    this.addDefaultMap();
  }

  private addTrackToMap(coordinates: L.LatLng[]) {
    let polyline = L.polyline(coordinates, { color: 'blue', opacity: 0.75, smoothFactor: 3 });
    polyline.addTo(this.map);
    this.displayedTracks.push(polyline);
  }

  private addHeatmapToMap(heatmapData: L.HeatLatLngTuple[]) {
    var heatmap = L.heatLayer(heatmapData, { radius: 10, max: 200, gradient: { 0.1: 'yellow', 0.4: 'orange', 0.6: 'red', 0.8: 'white' }, minOpacity: 0.8 });
    heatmap.addTo(this.map);
    this.displayedHeatmap.push(heatmap);

  }

  private displayTrack(filename: string): void {
    this.trackService.getTrack(filename).subscribe((rawCoordinates: Coordinate[]) => {
      const coordinates = rawCoordinates.map<L.LatLng>(coordinate => new L.LatLng(coordinate.a, coordinate.o));
      this.addTrackToMap(coordinates);
      this.downloadedTracks.update(currentValue => currentValue + 1);
      if (this.downloadedTracks() == this.tracksToDownload()) {
        this.isLoadingTracks = false;
        this.downloadedTracks.set(0);
        this.tracksToDownload.set(0);
      }
    });
  }


  /// Get the current map location and download only the tracks that have a point within the given
  /// position
  showTracksForCurrentScreen(): void {
    const bounds = this.map.getBounds();
    const northEast = bounds.getNorthEast();
    const southWest = bounds.getSouthWest();

    console.log("northEast");
    console.log(northEast);
    console.log("southWest");
    console.log(southWest);

    this.tracksToDownload.set(0);
    this.isLoadingTracks = true;
    this.trackService.getTracksInsideSquare(northEast, southWest).subscribe((file: FileList) => {

      const numberFilesFound = file.fileList.length;
      if (numberFilesFound > 0) {
        this.tracksToDownload.set(numberFilesFound);
        this.downloadedTracks.set(0);
        file.fileList.forEach(filename => this.displayTrack(filename));
      } else {
        this.isLoadingTracks = false;
      }
    });

  }


  /// Get the current map location and download only the tracks that have a point within the given
  /// position
  showHeatmapForCurrentScreen(): void {
    const bounds = this.map.getBounds();
    const northEast = bounds.getNorthEast();
    const southWest = bounds.getSouthWest();

    console.log("northEast");
    console.log(northEast);
    console.log("southWest");
    console.log(southWest);

    this.tracksToDownload.set(0);
    this.isLoadingTracks = true;
    this.heatmapService.getHeatmapInsideSquare(northEast, southWest).subscribe((rawHeatmap: HeatmapCoordinate[]) => {

      const heatmapData = rawHeatmap.map<L.HeatLatLngTuple>(coordinate => [coordinate.a, coordinate.o, coordinate.f]);
      this.addHeatmapToMap(heatmapData);
      this.isLoadingTracks = false;

    });

  }

  /// Remove all tracks displayed. This is useful when you want to check a new zone and what is
  /// already displayed would take too much memory
  clearMap(): void {
    // Tracks
    this.displayedTracks.forEach((track) => {
      this.map.removeLayer(track);
    })
    this.displayedTracks.splice(0, this.displayedTracks.length);
    // Heatmap
    this.displayedHeatmap.forEach((heatMap) => {
      this.map.removeLayer(heatMap);
    })
    this.displayedHeatmap.splice(0, this.displayedHeatmap.length);
  }

}
