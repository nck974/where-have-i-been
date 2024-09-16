import { Injectable } from '@angular/core';
import L from 'leaflet';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { HeatmapCoordinate } from './model/heatmap';
import { environment } from './environment/environment';

@Injectable({
  providedIn: 'root'
})
export class HeatmapService {
  private readonly backendUrl = environment.backendUrl;
  heatmapPath: string = 'heatmap';

  constructor(private httpClient: HttpClient) { }


  getHeatmapInsideSquare(northEastCoordinate: L.LatLng, southWestCoordinate: L.LatLng): Observable<HeatmapCoordinate[]> {
    const params = `northWestLatitude=${northEastCoordinate.lat}&northWestLongitude=${southWestCoordinate.lng}&` +
      `southEastLatitude=${southWestCoordinate.lat}&southEastLongitude=${northEastCoordinate.lng}`;

    const url = `${this.backendUrl}/${this.heatmapPath}?${params}`;
    return this.httpClient.get<HeatmapCoordinate[]>(url);
  }

}
