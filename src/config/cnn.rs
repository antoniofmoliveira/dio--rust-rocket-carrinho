extern crate dotenv;
extern crate rusqlite;

use dotenv::dotenv;
use rusqlite::Connection;
use std::env;

/// Establish a connection to the database.
///
/// The connection is established using the `DATABASE_PATH` environment variable.
/// If the variable is not set, a panic will occur.
///
/// The `dotenv` crate is used to load the environment variables from the `.env`
/// file. If the file does not exist, a panic will not occur, and the
/// `DATABASE_PATH` variable is expected to be set in another way.
///
/// # Panics
///
/// This function will panic if the `DATABASE_PATH` environment variable is not
/// set.
pub fn establish_connection() -> Connection {
    dotenv().ok();

    let database_path = env::var("DATABASE_PATH").expect("DATABASE_PATH must be set");

    Connection::open(database_path).expect("Error connecting to the database")
}
