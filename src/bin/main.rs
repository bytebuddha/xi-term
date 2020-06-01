extern crate xi_term;

use xi_term::Error;

fn main() -> Result<(), Error> {
    Ok(xi_term::run()?)
}
