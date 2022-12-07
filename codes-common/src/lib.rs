/*!
One-line description.

More detailed description, with

# Example

YYYYY

# Features

*/

#![warn(
    unknown_lints,
    // ---------- Stylistic
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    nonstandard_style, /* group */
    noop_method_call,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Future
    future_incompatible, /* group */
    rust_2021_compatibility, /* group */
    // ---------- Public
    missing_debug_implementations,
    // missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    // ---------- Unused
    unused, /* group */
)]
#![deny(
    // ---------- Public
    exported_private_dependencies,
    private_in_public,
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]

use std::{env, fs::File, path::Path};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const DEFAULT_DATA_DIR: &str = "data";

pub const DEFAULT_TEMPLATE_DIR: &str = "templates";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn process<T, I, P, F, R>(
    init: I,
    process_data: P,
    finalize: F,
    render: R,
) -> Result<(), Box<dyn std::error::Error>>
where
    I: FnOnce() -> Result<T, Box<dyn std::error::Error>>,
    P: FnOnce(T) -> Result<T, Box<dyn std::error::Error>>,
    F: FnOnce(T) -> Result<tera::Context, Box<dyn std::error::Error>>,
    R: FnOnce(tera::Context) -> Result<tera::Context, Box<dyn std::error::Error>>,
{
    init()
        .and_then(process_data)
        .and_then(finalize)
        .and_then(render)?;
    Ok(())
}

pub fn default_init<T>() -> Result<T, Box<dyn std::error::Error>>
where
    T: Default,
{
    Ok(Default::default())
}

pub fn default_finalize<T>(data: T) -> Result<tera::Context, Box<dyn std::error::Error>>
where
    T: Into<tera::Context>,
{
    Ok(data.into())
}

pub fn make_default_renderer<S1, S2>(
    template_name: S1,
    generated_file_name: S2,
) -> impl Fn(tera::Context) -> Result<tera::Context, Box<dyn std::error::Error>>
where
    S1: Into<String>,
    S2: Into<String>,
{
    let template_name = template_name.into();
    let generated_file_name = generated_file_name.into();
    move |ctx: tera::Context| -> Result<tera::Context, Box<dyn std::error::Error>> {
        let output_dir: String = env::var("OUT_DIR").unwrap();
        let file_name = Path::new(&output_dir).join(&generated_file_name);

        println!(
            "cargo:rerun-if-changed={}/{}",
            DEFAULT_TEMPLATE_DIR, template_name
        );

        let tera = tera::Tera::new(&format!("{}/*._rs", DEFAULT_TEMPLATE_DIR))?;

        let file = File::create(&file_name)?;
        tera.render_to(&template_name, &ctx, file)?;

        Ok(ctx)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
mod error;
pub use error::{invalid_format, invalid_length, unknown_value, CodeParseError};
