import { Injectable } from '@angular/core';
import L from 'leaflet';
import 'leaflet-gpx';
import { HttpClient } from '@angular/common/http';

export interface FileList {
  fileList: string[]
}

@Injectable({
  providedIn: 'root'
})
export class TrackService {

  backendServer: string = 'http://localhost:3000'; 
  tracksPath: string = 'tracks';

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

  createAllTracks(map: L.Map): void {
    this.httpClient.get<FileList>(`${this.backendServer}/${this.tracksPath}`).subscribe((file: FileList) => {
      file.fileList.forEach(fileName => {
        this._createTrack(map, `${this.backendServer}/${this.tracksPath}/${fileName}`)
      });
    });
  }
}
