#[macro_use] extern crate magnus;

use magnus::Error;

mod errors;
mod stylesheet;
mod declarations;

mod visitors;

#[magnus::init]
fn initialize() -> Result<(), Error> {
    stylesheet::initialize()?;
    declarations::initialize()
}
