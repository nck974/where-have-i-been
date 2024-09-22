import { Component, inject, model } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import {
  MAT_DIALOG_DATA,
  MatDialogActions,
  MatDialogClose,
  MatDialogContent,
  MatDialogRef,
  MatDialogTitle,
} from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { TrackFilter } from '../../../model/track-filter';
import { MatOptionModule } from '@angular/material/core';
import { MatSelectModule } from '@angular/material/select';
import { TrackService } from '../../../shared/services/track.service';
import { finalize } from 'rxjs';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { CapitalizeFirstPipe } from "../../../shared/pipes/capitalize-first.pipe";


@Component({
  selector: 'app-filter-dialog',
  standalone: true,
  imports: [
    MatFormFieldModule,
    MatInputModule,
    FormsModule,
    MatButtonModule,
    MatDialogTitle,
    MatDialogContent,
    MatDialogActions,
    MatDialogClose,
    MatSelectModule,
    MatOptionModule,
    MatProgressSpinnerModule,
    CapitalizeFirstPipe
],
  templateUrl: './filter-dialog.component.html',
  styleUrl: './filter-dialog.component.scss'
})
export class FilterDialogComponent {
  readonly dialogRef = inject(MatDialogRef<FilterDialogComponent>);
  readonly data = inject<TrackFilter>(MAT_DIALOG_DATA);
  readonly activity_type = model(this.data?.activity_type);

  allActivityTypes?: string[]
  isLoading = true;

  constructor(private trackService: TrackService) { }

  ngOnInit(): void {
    this.trackService.getAllActivityTypes().pipe(
      finalize(() => this.isLoading = false)
    ).subscribe(res => {
      this.allActivityTypes = res.activityTypes;
    })
  }

  onReset(): void {
    let filters: TrackFilter = {
      activity_type: undefined
    };
    this.dialogRef.close(filters);
  }

  onSaveFilters(): void {
    let filters: TrackFilter = {
      activity_type: this.activity_type()
    }

    this.dialogRef.close(filters);
  }
}
