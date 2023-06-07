use postgres::Client;
use std::error::Error;

use crate::sql_string::SqlString;
use crate::Cli;

#[derive(Debug, Clone)]
#[allow(unused_variables, dead_code)]
pub struct Table {
    pub schema: String,
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Column {
    pub name: String,
    pub data_type: String,
    pub character_maximum_length: Option<i32>,
    pub is_nullable: String,
    pub column_default: Option<String>,
}

#[allow(dead_code)]
pub fn pg_client(options: &Cli) -> Result<Client, Box<dyn Error>> {
    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(rustls::RootCertStore::empty())
        .with_no_client_auth();
    let tls = tokio_postgres_rustls::MakeRustlsConnect::new(config);
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        options.username, options.password, options.host, options.port, options.dbname
    );

    Ok(Client::connect(&database_url, tls)?)
}

impl Table {
    #[allow(dead_code)]
    fn sql_identifier(&self) -> String {
        format!(
            "{}.{}",
            self.schema.sql_identifier(),
            self.name.sql_identifier()
        )
    }
}

#[allow(dead_code)]
pub fn get_table(client: &mut Client, options: &Cli) -> Result<Table, Box<dyn Error>> {
    let query = format!(
        r#"
        select column_name,
            data_type,
            character_maximum_length,
            is_nullable,
            column_default,
            pg_get_serial_sequence('{schema}.{table_name}', column_name) as sequence
            from information_schema.columns
        where table_name = '{table_name}'
        "#,
        schema = options.schema,
        table_name = options.table_name,
    );

    let columns: Vec<Column> = client
        .query(&query, &[])?
        .into_iter()
        .map(|row| Column {
            name: row.get("column_name"),
            data_type: row.get("data_type"),
            character_maximum_length: row.get("character_maximum_length"),
            is_nullable: row.get("is_nullable"),
            column_default: row.get("column_default"),
        })
        .collect();

    let table = Table {
        name: options.table_name.clone(),
        columns,
        schema: options.schema.clone(),
    };

    Ok(table)
}
