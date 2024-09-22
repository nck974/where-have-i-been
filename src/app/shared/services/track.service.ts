import { Injectable } from '@angular/core';
import L from 'leaflet';
import { HttpClient } from '@angular/common/http';
import { Coordinate } from '../../model/coordinate';
import { FileList } from '../../model/files';
import { Observable } from 'rxjs';
import { environment } from '../../environment/environment';
import { TrackFilter } from '../../model/track-filter';
import { ActivityTypes } from '../../model/activity-types';



@Injectable({
  providedIn: 'root'
})
export class TrackService {

  private readonly backendUrl = environment.backendUrl;
  private coordinatesPath: string = 'tracks/coordinates';
  private filteredTracksPath: string = 'tracks/filtered-tracks';
  private allActivityTypes: string = 'tracks/activity-types';

  constructor(private httpClient: HttpClient) { }


  getTrack(filename: string): Observable<Coordinate[]> {
    let url = `${this.backendUrl}/${this.coordinatesPath}/${filename}`;
    return this.httpClient.get<Coordinate[]>(url);
  }

  getAllActivityTypes(): Observable<ActivityTypes> {
    let url = `${this.backendUrl}/${this.allActivityTypes}`;
    return this.httpClient.get<ActivityTypes>(url);
  }

  getTracksInsideSquare(northEastCoordinate: L.LatLng, southWestCoordinate: L.LatLng, filters?: TrackFilter): Observable<FileList> {
    let params = `northWestLatitude=${northEastCoordinate.lat}&northWestLongitude=${southWestCoordinate.lng}&` +
      `southEastLatitude=${southWestCoordinate.lat}&southEastLongitude=${northEastCoordinate.lng}`;

    if (filters?.activity_type) {
      params += `&activityType=${filters.activity_type}`;
    }

    const url = `${this.backendUrl}/${this.filteredTracksPath}?${params}`;
    return this.httpClient.get<FileList>(url);
  }

}
