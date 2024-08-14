import { Injectable } from '@angular/core';
import L from 'leaflet';
import { HttpClient } from '@angular/common/http';
import { Coordinate } from './model/coordinate';
import { FileList } from './model/files';
import { Observable } from 'rxjs';
import { HeatmapCoordinate } from './model/heatmap';

@Injectable({
  providedIn: 'root'
})
export class HeatmapService {
  backendServer: string = 'http://localhost:3000';
  heatmapPath: string = 'heatmap';

  constructor(private httpClient: HttpClient) { }


  getHeatmapInsideSquare(northEastCoordinate: L.LatLng, southWestCoordinate: L.LatLng): Observable<HeatmapCoordinate[]> {
    const params = `northWestLatitude=${southWestCoordinate.lat}&northWestLongitude=${northEastCoordinate.lng}&` +
      `southEastLatitude=${northEastCoordinate.lat}&southEastLongitude=${southWestCoordinate.lng}`;

    const url = `${this.backendServer}/${this.heatmapPath}?${params}`;
    return this.httpClient.get<HeatmapCoordinate[]>(url);
  }

}
