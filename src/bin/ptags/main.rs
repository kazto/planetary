extern crate clap;
use clap::{Arg, Command};
use sea_query::*;
use sqlx::sqlite::*;
use futures::future;
use std::fs;
use std::fs::{File, OpenOptions};

#[derive(Iden)]
enum Db {
    Table,
    Id,
    Key,
    Dat,
    Extra,
}

enum Gfiles {
    Gpath(String),
    Gtags(String),
    Grtags(String),
}

#[tokio::main]
async fn main() {
    /*
    let options = Command::new("Planetary: GNU Global like tagging system")
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .takes_value(true)
                .value_name("FILE")
                .help("debug dump input file")
        )
        .get_matches();
    */
    let gpath = Gfiles::Gpath(String::from("GPATH"));
    let gtags = Gfiles::Gtags(String::from("GTAGS"));
    let grtags = Gfiles::Grtags(String::from("GRTAGS"));

    create_file(&gpath).await;
    create_file(&gtags).await;
    create_file(&grtags).await;

    println!("end main.");
}


async fn create_file(gfile: &Gfiles) {
    let mut file_name = String::from("");
    match gfile {
        Gfiles::Gpath(f) => file_name = String::from(f),
        Gfiles::Gtags(f) => file_name = String::from(f),
        Gfiles::Grtags(f) => file_name = String::from(f),
    }
    let open_result = OpenOptions::new().create(true).write(true).open(&file_name);
    match open_result {
        Ok(_) => (),
        Err(_) => (),
    }
    let result = SqlitePoolOptions::new()
        .max_connections(20)
        .connect(&file_name)
        .await;
    
    match result {
        Ok(mut db) => {
            init_db(&mut db).await;
            db.close().await;
        },
        Err(v) => println!("Err: {:?}", (v as sqlx::Error)),
    }

    println!("end create_file.");
}

fn create_table_sql() -> String {
    return Table::create()
        .table(Db::Table)
        .col(
            ColumnDef::new(Db::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Db::Key).string())
        .col(ColumnDef::new(Db::Dat).string())
        .col(ColumnDef::new(Db::Extra).string())
        .build(SqliteQueryBuilder);
}

async fn init_db(db: &mut SqlitePool) {
    let create_sql = create_table_sql();
    
    println!("{}", create_sql);

    if let Ok(mut v) = db.acquire().await {
        let result = sqlx::query(&create_sql).execute(&mut v).await;
        match result {
            Ok(v) => println!("{:?}", (v as SqliteQueryResult)),
            Err(v) => println!("Err: {:?}", (v as sqlx::Error)),
        }
    }

    println!("end init_db.");
}

