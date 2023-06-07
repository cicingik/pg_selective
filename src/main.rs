use crate::dump::{pg_dump, pg_dump_selective};
use crate::options::Cli;
use clap::Parser;
use std::error::Error;

mod dump;
mod options;
mod selective;
mod sql_string;

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn Error>> {
    let options = Cli::parse();

    if options.condition.is_some() {
        println!(
            "pg_selective still in development with condition: {:?}",
            options.condition.as_ref().unwrap()
        );
        pg_dump_selective(&options)?;
    } else {
        // failing to pq_dump cause condition is not exist
        pg_dump(&options)?;
    }

    Ok(())
}
