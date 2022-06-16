#[macro_use] extern crate magnus;

use magnus::Error;

mod errors;
mod stylesheet;
mod style_attribute;
mod shared;

mod providers;
mod visitors;

#[magnus::init]
fn initialize() -> Result<(), Error> {
    stylesheet::initialize()?;
    style_attribute::initialize()
}
