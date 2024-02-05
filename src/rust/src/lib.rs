use extendr_api::prelude::*;

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

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod testbuild;
    fn hello_world;
    fn verify_install;
}
