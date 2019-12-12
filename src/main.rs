extern crate actix_web;

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use rusqlite::{Connection, Result, NO_PARAMS};
use time::Timespec;
use std::env;

#[derive(Debug)]
struct Link {
    id: String,
    url: String,
    time_created: Timespec,
}

#[derive(Deserialize)]
struct GetInfo {
    id: String,
}

fn open_db() -> Result<(Connection)> {
    let filepath: String;

    match env::var("NNUS_DB") {
        Ok(val) => { 
            filepath = val
        }
        Err(_e) => filepath = String::from("./db.db3"),
    }

    println!("Using {}", filepath);

    let conn = Connection::open(filepath)?;
    Ok(conn)
}

fn check_schema() -> Result<()> {
    let conn = open_db()?;
    let create_table_res = conn.execute(
        "CREATE TABLE links (
                  id              TEXT PRIMARY KEY,
                  url            TEXT NOT NULL,
                  time_created    TEXT NOT NULL
                  )",
        NO_PARAMS,
    );

    match create_table_res {
        Ok(_v) => println!("New links table created"),
        Err(_e) => println!("Table ready"),
    }

    Ok(())
}

fn get_link(conn: &Connection, id: &String) -> Result<String> {
    conn.query_row(
        "SELECT * FROM links WHERE id = :id",
        &[id],
        | row | row.get(1),
    )
}

fn handle_get(info: web::Path<GetInfo>) -> HttpResponse {
    // TODO: Don't open a connection each time.
    let connection = open_db().unwrap();
    match get_link(&connection, &info.id) {
        Ok(link) => {
            HttpResponse::PermanentRedirect().header("Location", link).finish()
        },
        Err(_e) => HttpResponse::NotFound().body("Link not found")
    }
}

fn main() {
    check_schema().unwrap();
    HttpServer::new(|| {
        App::new()
            .route("/{id}", web::get().to(handle_get))
    }).bind("127.0.0.1:6767").unwrap().run().unwrap();
}
