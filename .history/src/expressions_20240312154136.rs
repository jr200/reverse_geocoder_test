use polars_arrow::array::MutablePlString;
use polars_core::utils::align_chunks_binary;
use reverse_geocoder::ReverseGeocoder;

#[polars_expr(output_type=String)]
fn reverse_geocode(inputs: &[Series]) -> PolarsResult<Series> {
    let lat = inputs[0].f64();
}