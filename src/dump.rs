use crate::Cli;
use std::error::Error;
use std::process::Command;

pub fn pg_dump(option: &Cli) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new("pg_dump");

    // cheat pg_dump for not promt password :*
    cmd.env("PGPASSWORD", option.password.clone());

    cmd.arg(format!("--dbname={}", option.dbname));
    cmd.arg(format!("--username={}", option.username));
    cmd.arg(format!("--host={}", option.host));
    cmd.arg(format!("--port={}", option.port));
    cmd.arg(format!("--table={}", option.table_name));

    if option.file.is_some() {
        let file = option.file.as_ref().unwrap();
        cmd.arg(format!("--file={}", file));
    }

    if option.encoding.is_some() {
        let encoding = option.encoding.as_ref().unwrap();
        cmd.arg(format!("--encoding={}", encoding));
    }

    if option.data_only {
        cmd.arg("--data-only");
    }

    if option.no_owner {
        cmd.arg("--no-owner");
    }

    if option.verbose {
        cmd.arg("--verbose");
    }

    if option.column_insert {
        cmd.arg("--column-insert");
    }

    if option.inserts {
        cmd.arg("--insert");
    }

    if option.if_exists {
        cmd.arg("--if-exists");
    }

    cmd.spawn()?;

    Ok(())
}

pub fn pg_dump_selective(_option: &Cli) -> Result<(), Box<dyn Error>> {
    Ok(())
}
