#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, Request};
use nickel::status::StatusCode;
use hyper::header::Location;
use rusqlite::{Connection, Result, NO_PARAMS};
use time::Timespec;
use std::env;

#[derive(Debug)]
struct Link {
    id: String,
    url: String,
    time_created: Timespec,
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

    Ok(conn)
}

fn get_link(conn: &Connection, id: String) -> Result<String> {
    conn.query_row(
        "SELECT * FROM links WHERE id = :id",
        &[id],
        | row | row.get(1),
    )
}

fn main() {
    let mut server = Nickel::new();
    server.get("/:id", middleware!( | req, mut res | {
        // TODO: Don't open a connection each time.
        let connection = open_db().unwrap();
        let link_result = match req.param("id") {
            Some(id) => {
                match get_link(&connection, String::from(id)) {
                    Ok(link) => {
                        link
                    },
                    Err(_e) => String::from("not found")
                }
            },
            None => String::from("not found")
        };
        if link_result != "not found" {
            res.set(StatusCode::PermanentRedirect).set(Location(link_result));
        }
        "not found"
    }));
    server.listen("127.0.0.1:6767").unwrap();
}
