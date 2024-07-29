import { Injectable } from '@angular/core';
import L from 'leaflet';
import 'leaflet-gpx';
import { HttpClient } from '@angular/common/http';
import { Coordinate } from './model/coordinate';
import { FileList } from './model/files';



@Injectable({
  providedIn: 'root'
})
export class TrackService {

  backendServer: string = 'http://localhost:3000';
  tracksPath: string = 'tracks';
  coordinatesPath: string = 'tracks/coordinates';

  constructor(private httpClient: HttpClient) { }

  private _createTrack(map: L.Map, url: string): void {
    new L.GPX(url, {
      async: true,
      marker_options: {
        startIcon: undefined,
        endIcon: undefined,
        shadowUrl: undefined,
        endIconUrl: undefined,
        startIconUrl: undefined
      },
      polyline_options: {
        color: 'blue',
        opacity: 0.75,
        smoothFactor: 3
      }
    }).on('loaded', function (e: any) {
      // Consider if focusing makes sense
      // map.fitBounds(e.target.getBounds());
    }).addTo(map);
  }

  private _createPolylineTrack(map: L.Map, coordinates: L.LatLng[]) {

    L.polyline(coordinates, { color: 'blue', opacity: 0.75, smoothFactor: 3 }).addTo(map);
  }


  createSingleTrack(map: L.Map, filename: string): void {
    // let file = `${this.backendServer}/${this.tracksPath}/1389563275.gpx`;
    // this._createTrack(map, file);
    // let filename = '1389563275.gpx';
    let file = `${this.backendServer}/${this.coordinatesPath}/${filename}`;
    this.httpClient.get<Coordinate[]>(file).subscribe((rawCoordinates: Coordinate[]) => {
      const coordinates = rawCoordinates.map<L.LatLng>(coordinate => new L.LatLng(coordinate.a, coordinate.o));
      this._createPolylineTrack(map, coordinates);
    });
  }

  createAllTracks(map: L.Map): void {
    this.httpClient.get<FileList>(`${this.backendServer}/${this.tracksPath}`).subscribe((file: FileList) => {
      file.fileList.forEach((filename, index) => {
        if (index == 1) {
          this.createSingleTrack(map, filename);
        }
      });
    });
  }

}
