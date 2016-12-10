// MIT License
//
// Copyright (c) 2016 Rafael Medina García <rafamedgar@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use rusqlite::Connection;
use db::open_connection;

/// Execute SQL queries to create/update tables of the database on-the-go.
///
/// Only migrations with a higher version number than the one in the database
/// (default is -1) will be executed.
pub fn migrate(db_path: &str, version: i32) {
    let conn = open_connection(&db_path).unwrap();

    if version < 1 {
        run_migration_ver1(&conn);
    }
}

/// Base migration that creates the database tables.
fn run_migration_ver1(conn: &Connection) {
    // Create receipts table
    conn.execute("
        CREATE TABLE receipts (
            id INTEGER PRIMARY KEY,
            description TEXT,
            shop TEXT NOT NULL,
            amount REAL NOT NULL DEFAULT 0.0,
            currency TEXT NOT NULL,
            payment_type TEXT NOT NULL,
            date_paid TEXT NOT NULL
        )", &[]).unwrap();

    // Create revision table
    conn.execute("
        CREATE TABLE __revision (
            version INTEGER
        )", &[]).unwrap();

    // Insert revision number
    conn.execute("
        INSERT INTO __revision (version)
        VALUES (1)",
        &[]).unwrap();
}
