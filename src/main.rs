extern crate actix_web;
extern crate r2d2;
extern crate r2d2_sqlite;

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use rusqlite::{Connection, Result, NO_PARAMS};
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::{PooledConnection};
use std::env;

#[derive(Debug)]
struct Link {
    id: String,
    url: String
}

#[derive(Deserialize)]
struct GetInfo {
    id: String,
}

fn open_db() -> r2d2::Pool<r2d2_sqlite::SqliteConnectionManager> {
    let filepath: String;

    match env::var("NNUS_DB") {
        Ok(val) => { 
            filepath = val
        }
        Err(_e) => filepath = String::from("./db.db3"),
    }

    println!("Using {}", filepath);

    let manager = SqliteConnectionManager::file(filepath);
    r2d2::Pool::new(manager).unwrap()
}

fn check_schema(conn: PooledConnection<r2d2_sqlite::SqliteConnectionManager>) -> Result<()> {
    let create_table_res = conn.execute(
        "CREATE TABLE links (
                  id              TEXT PRIMARY KEY,
                  url            TEXT NOT NULL
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

fn handle_get(info: web::Path<GetInfo>, data: web::Data<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>) -> HttpResponse {
    match get_link(&data.get().unwrap(), &info.id) {
        Ok(link) => {
            HttpResponse::PermanentRedirect().header("Location", link).finish()
        },
        Err(_e) => HttpResponse::NotFound().body("Link not found")
    }
}

fn handle_head(info: web::Path<GetInfo>, data: web::Data<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>) -> HttpResponse {
    match get_link(&data.get().unwrap(), &info.id) {
        Ok(link) => {
            HttpResponse::Ok().header("Location", link).finish()
        },
        Err(_e) => HttpResponse::NotFound().finish()
    }
}


fn main() {
    let master_pool = open_db();
    check_schema(master_pool.get().unwrap()).unwrap();
    let bind_to = env::var("NNUS_BIND").unwrap_or(String::from("127.0.0.1:6767"));
    println!("Binding to http://{}", bind_to);
    HttpServer::new(move || {
        App::new().data(master_pool.clone())
        .route("/{id}", web::head().to(handle_head))
        .route("/{id}", web::get().to(handle_get))
    }).bind(bind_to).unwrap().run().unwrap();
}
