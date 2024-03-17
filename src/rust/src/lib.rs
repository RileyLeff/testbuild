use extendr_api::prelude::*;
use polars::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

/// Verify that the build system worked.
/// @export
#[extendr]
fn verify_install() -> &'static str {
    "Package build was successful"
}

/// Do stuff with polars
/// @export
#[extendr]
fn do_polars_thing(cool_number: f64) -> &'static str {
    let df = df! [
        "names" => ["a", "b", "c"],
        "values" => [1f64, 2f64, 3f64],
        "values_nulls" => [Some(1), None, Some(3)]
    ].expect("fucked up the polars thing");

    let vals = df.column("values").expect("broken vals");
    let mask = vals.gt(cool_number).expect("broken mask");

    let filtered_vals = vals.filter(&mask).expect("broken filter");

    let fvals_sum:f64 = filtered_vals.sum().expect("broken sum");

    if fvals_sum > cool_number {
        "polars thing was successful and cool_num was tiny"
    } else {
        "polars thing was successful and cool_num was huge"
    }
    
    
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod testbuild;
    fn hello_world;
    fn verify_install;
    fn do_polars_thing;
}
