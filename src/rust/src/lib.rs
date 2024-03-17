use extendr_api::prelude::*;
use plomo::{models::sperry, Model};

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

/// Passes a config to a model, then runs it. 
/// @export
#[extendr]
fn model_runner(
    path_to_config: String,
    path_to_data: String, 
    path_to_output: String
) -> &'static str {
    let m = sperry::SperryModel::try_new_from_paths(path_to_config, path_to_data)
        .expect("bad model, fail");

    let o = m.execute(path_to_output);
    
    // return o;

    let s: &'static str = "This is a static string";

    return s;
}

/// saves config to a path 
/// @export
#[extendr]
fn write_default_config(
    path_to_write: String
) -> &'static str {
    let _x = sperry::SperryConfig::serialize_default_to_path(path_to_write);
    //return String::from("ok")
    let s: &'static str = "This is a static string";

    return s;
}




// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod testbuild;
    fn hello_world;
    fn verify_install;
    fn model_runner;
    fn write_default_config;
}
