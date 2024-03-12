import polars as pl
import minimal_plugin as mp

df = pl.DataFrame({
    'lat': [37.7749, 51.01, 52.5],
    'lon': [-122.4194, -3.9, -.91]
})
print(df.with_columns(city=mp.reverse_geocode('lat', 'lon')))
