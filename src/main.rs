use crate::dump::{pg_dump, pg_dump_selective};
use crate::options::Cli;
// use crate::utils::StringWriter;
use clap::Parser;
use std::error::Error;
// use std::io::{self, Write};
use std::path::PathBuf;

mod dump;
mod options;
mod selective;
mod sql_string;
mod utils;

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn Error>> {
    let options = Cli::parse();

    if options.condition.is_some() {
        pg_dump_selective(&options)?;

        let file_path = PathBuf::from(options.file);

        options.get_output(file_path)?;
    } else {
        // failing to pq_dump cause condition is not exist
        pg_dump(&options)?;
    }

    Ok(())
}
