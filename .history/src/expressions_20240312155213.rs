use pyo3_polars::derive::polars_expr;
use pyo3_polars::export::polars_core::prelude::*;


use polars_arrow::array::MutablePlString;
use polars_core::utils::align_chunks_binary;
use reverse_geocoder::ReverseGeocoder;

#[polars_expr(output_type=String)]
fn reverse_geocode(inputs: &[Series]) -> PolarsResult<Series> {
    let lat = inputs[0].f64()?;
    let lon = inputs[1].f64()?;
    let geocoder = ReverseGeocoder::new();

    let (lhs, rhs) = align_chunks_binary(lat, lon);
    let chunks = lhs
        .downcast_iter()
        .zip(rhs.downcast_iter())
        .map(|(lat_arr, lon_arr)| {
            let mut mutarr = MutablePlString::with_capacity(lat_arr.len());

            for(lat_opt_val, lon_opt_val) in lat_arr.iter().zip(lon_arr.iter()) {
                match (lat_opt_val, lon_opt_val) {
                    (Some(lat_val), Some(lon_val)) => {
                        let res = &geocoder.search((*lat_val, *lon_val)).record.name;
                        mutarr.push(Some(res))
                    }
                    _ => mutarr.push_null(),
                }
            }

            mutarr.freeze().boxed()
        })
        .collect();


}