use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Deserialize, Serialize)]
#[command(name = "pg_selective")]
#[command(author = "Dany S. <danysatyanegara@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Just `pg_dump` table with condition", long_about = None)]
pub struct Cli {
    /// Database server host or socket dirctory
    #[arg(short = 'H', long)]
    pub host: String,

    /// Database server port number
    #[arg(short = 'P', long)]
    pub port: String,

    /// Connect as specified database user
    #[arg(short = 'U', long)]
    pub username: String,

    /// Force password prompt (should happen automaticaly)
    #[arg(short = 'W', long)]
    pub password: String,

    /// Database to dump
    #[arg(short, long)]
    pub dbname: String,

    /// Database schema
    #[arg(short, long)]
    pub schema: String,

    /// table name
    #[arg(short, long)]
    pub table_name: String,

    /// Dump the data in encoding
    #[arg(short, long)]
    pub encoding: Option<String>,

    /// Condition query
    #[arg(short = 'q', long)]
    pub condition: Option<String>,

    /// Output file name
    #[arg(short, long)]
    pub file: Option<String>,

    /// Dump only data, not the schema
    #[arg(short = 'a', long)]
    pub data_only: bool,

    /// Skip restoration of object ownership in plain-text format
    #[arg(short = 'O', long)]
    pub no_owner: bool,

    /// Verbose mode
    #[arg(short, long)]
    pub verbose: bool,

    /// Dump data as INSERT command with column name
    #[arg(long)]
    pub column_insert: bool,

    /// Dump data as INSERT command, rather than COPY
    #[arg(long)]
    pub inserts: bool,

    /// use IF EXISTS when dropping objects
    #[arg(long)]
    pub if_exists: bool,
}
