use extendr_api::prelude::*;
use project_with_bindings::use_fancy_now;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

#[extendr]
fn use_fancy() -> String {
    stringify!(use_fancy_now()).into()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod projectRbindings;
    fn hello_world;

    fn use_fancy;
}
