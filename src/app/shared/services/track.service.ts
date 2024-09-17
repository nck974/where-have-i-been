import { Injectable } from '@angular/core';
import L from 'leaflet';
import { HttpClient } from '@angular/common/http';
import { Coordinate } from '../../model/coordinate';
import { FileList } from '../../model/files';
import { Observable } from 'rxjs';
import { environment } from '../../environment/environment';



@Injectable({
  providedIn: 'root'
})
export class TrackService {

  private readonly backendUrl = environment.backendUrl;
  tracksPath: string = 'tracks';
  coordinatesPath: string = 'tracks/coordinates';
  filteredTracksPath: string = 'tracks/filtered-tracks';

  constructor(private httpClient: HttpClient) { }


  getTrack(filename: string): Observable<Coordinate[]> {
    let file = `${this.backendUrl}/${this.coordinatesPath}/${filename}`;
    return this.httpClient.get<Coordinate[]>(file);
  }

  getTracksInsideSquare(northEastCoordinate: L.LatLng, southWestCoordinate: L.LatLng): Observable<FileList> {
    const params = `northWestLatitude=${northEastCoordinate.lat}&northWestLongitude=${southWestCoordinate.lng}&` +
      `southEastLatitude=${southWestCoordinate.lat}&southEastLongitude=${northEastCoordinate.lng}`;

    const url = `${this.backendUrl}/${this.filteredTracksPath}?${params}`;
    return this.httpClient.get<FileList>(url);
  }

}
