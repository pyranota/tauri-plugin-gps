<script>
  import { MapLibre, DefaultMarker } from "svelte-maplibre";
  let marker = {
    lon: 13.413244,
    lat: 52.521992,
  };
</script>

<MapLibre
  center={[13.413244, 52.521992]}
  zoom={14}
  class="map"
  style="https://basemaps.cartocdn.com/gl/positron-gl-style/style.json"
  interactive = {false}
  bearing={190}
>
  <!-- Unlike the custom marker example, default markers do not have mouse events,
    and popups only support the default openOn="click" behavior -->
  <DefaultMarker lngLat={marker} draggable>
  </DefaultMarker>
</MapLibre>

<style>

  :global(.map) {
    height: 100vh;
  }
  :global(*){
    margin: 0px;
  }
</style>
