/// This module is a literal translation from js to TS
/// of https://github.com/Leaflet/Leaflet.heat/blob/gh-pages/src/HeatLayer.js

import * as L from 'leaflet';
import simpleheat from 'simpleheat';
// Copy the interface defined in the types of @types/leaflet.heat": "^0.2.4",
export type HeatLatLngTuple = [number, number, number]

export interface ColorGradientConfig {
    [key: number]: string;
}

export interface HeatMapOptions extends L.LayerOptions {
    minOpacity?: number | undefined;
    maxZoom?: number | undefined;
    max?: number | undefined;
    radius?: number | undefined;
    blur?: number | undefined;
    gradient?: ColorGradientConfig | undefined;
}

export interface HeatLayerInterface extends L.Layer {
    setOptions(options: HeatMapOptions): HeatLayer;
    addLatLng(latlng: L.LatLng | HeatLatLngTuple): HeatLayer;
    setLatLngs(latlngs: Array<L.LatLng | HeatLatLngTuple>): HeatLayer;
}


// Make a translation to TS of the heatmap file of "leaflet.heat": "^0.2.0",
export class HeatLayer extends L.Layer implements HeatLayerInterface {
    private _latlngs: Array<L.LatLng | HeatLatLngTuple> = [];
    private _heat: any;
    private _frame: any;
    private _canvas: HTMLCanvasElement | undefined;
    override options: HeatMapOptions

    constructor(latlngs: Array<L.LatLng | HeatLatLngTuple>, options: HeatMapOptions) {
        super();
        this._latlngs = latlngs;
        this.options = options;
        this.setOptions(options);
    }

    setLatLngs(latlngs: Array<L.LatLng | HeatLatLngTuple>): HeatLayer {
        this._latlngs = latlngs;
        return this.redraw();
    }

    addLatLng(latlng: L.LatLng | HeatLatLngTuple): HeatLayer {
        this._latlngs.push(latlng);
        return this.redraw();
    }

    setOptions(options: HeatMapOptions): HeatLayer {
        L.setOptions(this, options);
        if (this._heat) {
            this._updateOptions();
        }
        return this.redraw();
    }

    getBounds(): L.LatLngBounds {
        return L.latLngBounds(this._latlngs as L.LatLng[]);
    }

    redraw(): HeatLayer {
        if (this._heat && !this._frame && this._map) {
            // if (this._heat && !this._frame && this._map && !this._map._animating) {
            this._frame = L.Util.requestAnimFrame(this._redraw.bind(this), this);
        }
        return this;
    }

    override onAdd(map: L.Map): this {
        this._map = map;

        if (!this._canvas) {
            this._initCanvas();
        }

        const pane = this.options.pane ? this.getPane() : map.getPane('overlayPane');
        if (pane && this._canvas) {
            pane.appendChild(this._canvas);
        }

        map.on('moveend', this._reset, this);

        if (map.options.zoomAnimation && L.Browser.any3d) {
            map.on('zoomanim', this._animateZoom, this);
        }

        this._reset();
        return this;
    }

    override onRemove(map: L.Map): this {
        const pane = this.options.pane ? this.getPane() : map.getPane('overlayPane');
        if (pane && this._canvas) {
            pane.removeChild(this._canvas);
        }

        map.off('moveend', this._reset, this);

        if (map.options.zoomAnimation) {
            map.off('zoomanim', this._animateZoom, this);
        }

        return this;
    }

    override addTo(map: L.Map): this {
        map.addLayer(this);
        return this;
    }

    private _initCanvas(): void {
        const canvas = this._canvas = L.DomUtil.create('canvas', 'leaflet-heatmap-layer leaflet-layer') as HTMLCanvasElement;
        const originProp = L.DomUtil.testProp(['transformOrigin', 'WebkitTransformOrigin', 'msTransformOrigin']);

        if (originProp) {
            (canvas.style as any)[originProp] = '50% 50%';
        }

        const size = this._map!.getSize();
        canvas.width = size.x;
        canvas.height = size.y;

        const animated = this._map!.options.zoomAnimation && L.Browser.any3d;
        L.DomUtil.addClass(canvas, 'leaflet-zoom-' + (animated ? 'animated' : 'hide'));

        this._heat = simpleheat(canvas);
        this._updateOptions();
    }

    private _updateOptions(): void {
        this._heat.radius(this.options.radius || this._heat.defaultRadius, this.options.blur);

        if (this.options.gradient) {
            this._heat.gradient(this.options.gradient);
        }
        if (this.options.max) {
            this._heat.max(this.options.max);
        }
    }

    private _reset(): void {
        const topLeft = this._map!.containerPointToLayerPoint([0, 0]);
        L.DomUtil.setPosition(this._canvas!, topLeft);

        const size = this._map!.getSize();
        if (this._heat._width !== size.x) {
            this._canvas!.width = this._heat._width = size.x;
        }
        if (this._heat._height !== size.y) {
            this._canvas!.height = this._heat._height = size.y;
        }

        this._redraw();
    }

    private _redraw(): void {
        if (!this._map) return;

        const data: [number, number, number][] = [];
        const r = this._heat._r;
        const size = this._map.getSize();
        const bounds = new L.Bounds(L.point([-r, -r]), size.add([r, r]));

        const max = this.options.max === undefined ? 1 : this.options.max;
        const maxZoom = this.options.maxZoom === undefined ? this._map.getMaxZoom() : this.options.maxZoom;
        const v = 1 / Math.pow(2, Math.max(0, Math.min(maxZoom - this._map.getZoom(), 12)));
        const cellSize = r / 2;
        const grid: Array<Array<any>> = [];
        const panePos = this._map.getPixelOrigin();
        const offsetX = panePos.x % cellSize;
        const offsetY = panePos.y % cellSize;

        for (let i = 0, len = this._latlngs.length; i < len; i++) {
            const p = this._map.latLngToContainerPoint(this._latlngs[i] as L.LatLng);
            if (bounds.contains(p)) {
                const x = Math.floor((p.x - offsetX) / cellSize) + 2;
                const y = Math.floor((p.y - offsetY) / cellSize) + 2;
                // const alt = (this._latlngs[i] as any).alt !== undefined
                //     ? (this._latlngs[i] as any).alt
                //     : this._latlngs[i][2] !== undefined ? +this._latlngs[i][2] : 1;
                const latlng = this._latlngs[i];

                // Narrow the type using a type guard
                let alt: number;
                if (latlng instanceof L.LatLng) {
                    // It's an L.LatLng object
                    alt = latlng.alt !== undefined ? latlng.alt : 1;
                } else if (Array.isArray(latlng)) {
                    // It's a HeatLatLngTuple (an array)
                    alt = latlng[2] !== undefined ? +latlng[2] : 1;
                } else {
                    alt = 1; // Fallback, in case there's an unexpected case
                }
                const k = alt * v;

                grid[y] = grid[y] || [];
                const cell = grid[y][x];

                if (!cell) {
                    grid[y][x] = [p.x, p.y, k];
                } else {
                    cell[0] = (cell[0] * cell[2] + p.x * k) / (cell[2] + k);
                    cell[1] = (cell[1] * cell[2] + p.y * k) / (cell[2] + k);
                    cell[2] += k;
                }
            }
        }

        for (let i = 0; i < grid.length; i++) {
            if (grid[i]) {
                for (let j = 0; j < grid[i].length; j++) {
                    const cell = grid[i][j];
                    if (cell) {
                        data.push([Math.round(cell[0]), Math.round(cell[1]), Math.min(cell[2], max)]);
                    }
                }
            }
        }

        this._heat.data(data).draw(this.options.minOpacity);
        this._frame = null;
    }

    private _animateZoom(e: L.ZoomAnimEvent): void {
        const scale = this._map.getZoomScale(e.zoom);

        // Calculate the offset manually
        const mapSize = this._map.getSize(); // Get the map's size
        const mapCenter = this._map.latLngToLayerPoint(e.center); // Get the pixel coordinates of the event's center (e.center)
        const containerCenter = L.point(mapSize.x / 2, mapSize.y / 2); // Find the center of the container

        // Offset is the difference between map center and container center, adjusted for scale
        const offset = mapCenter.subtract(containerCenter).multiplyBy(-scale).subtract(this._map.getPixelOrigin());

        if (L.DomUtil.setTransform) {
            L.DomUtil.setTransform(this._canvas!, offset, scale);
        } else {
            // this._canvas!.style[L.DomUtil.TRANSFORM] = L.DomUtil.getTranslateString(offset) + ' scale(' + scale + ')';
            this._canvas!.style.transform = `translate(${offset.x}px, ${offset.y}px) scale(${scale})`;
        }
    }
}

// Factory function to create a heat layer
export function heatLayer(latlngs: Array<HeatLatLngTuple>, options: HeatMapOptions): HeatLayer {
    return new HeatLayer(latlngs, options);
}