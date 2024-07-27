import { AfterViewInit, Component } from '@angular/core';
import * as L from 'leaflet';
import { TrackService } from '../track.service';

@Component({
  selector: 'app-map',
  standalone: true,
  imports: [],
  templateUrl: './map.component.html',
  styleUrl: './map.component.scss'
})
export class MapComponent implements AfterViewInit {

  private map!: L.Map;

  constructor(private trackService: TrackService) { }

  ngAfterViewInit(): void {
    this.initializeMap();
    this.trackService.createAllTracks(this.map);
  }

  /// This adds one layer to the map
  private addMapTile(): void {
    const tiles = L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      maxZoom: 18,
      minZoom: 3,
      attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
    });

    tiles.addTo(this.map);
  }


  /// Create an empty map without any layer and then add one
  private initializeMap(): void {
    this.map = L.map('map', {
      center: [49.4521, 11.0767],
      zoom: 3
    });
    this.addMapTile();
  }

}
