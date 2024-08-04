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
  filteredTracksPath: string = 'tracks/filtered-tracks';

  constructor(private httpClient: HttpClient) { }

  private _createPolylineTrack(map: L.Map, coordinates: L.LatLng[]) {
    L.polyline(coordinates, { color: 'blue', opacity: 0.75, smoothFactor: 3 }).addTo(map);
  }

  createSingleTrack(map: L.Map, filename: string): void {
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

  createTracksInsideSquare(map: L.Map, northEastCoordinate: L.LatLng, southWestCoordinate: L.LatLng): void {
    const params = `northWestLatitude=${northEastCoordinate.lat}&northWestLongitude=${southWestCoordinate.lng}&` +
      `southEastLatitude=${southWestCoordinate.lat}&southEastLongitude=${northEastCoordinate.lng}`;

    const url = `${this.backendServer}/${this.filteredTracksPath}?${params}`;
    this.httpClient.get<FileList>(url).subscribe((file: FileList) => {
      console.log(file)
      file.fileList.forEach((filename, index) => {
        // if (index == 1) {
          this.createSingleTrack(map, filename);
        // }
      });
    });
  }

}
