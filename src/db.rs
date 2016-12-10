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

/// Operations in the database

use std::os::raw::{c_int, c_double};
use std::path::Path;

use chrono::*;
use rusqlite::Connection;
use rusqlite::Result;

/// Receipt model
#[derive(Clone)]
pub struct Receipt {
    pub id: i32,
    pub description: String,
    pub shop: String,
    pub amount: c_double,
    pub currency: String,
    pub payment_type: String,
    pub date_paid: NaiveDate
}

impl Receipt {
    /// Create an empty receipt
    pub fn new() -> Receipt {
        Receipt {
            id: -1,
            description: "".to_string(),
            shop: "".to_string(),
            amount: 0.0,
            currency: "".to_string(),
            payment_type: "".to_string(),
            date_paid: NaiveDate::from_ymd(1970, 1, 1)
        }
    }
}

/// Obtain a connection to the database
pub fn open_connection(db_path: &str) -> Result<Connection> {
    let path = Path::new(db_path);

    Connection::open(path)
}

/// Delete a receipt from the database
pub fn delete_receipt(db_path: &str, id: i32) -> Result<c_int> {
    let conn = try!(open_connection(&db_path));

    conn.execute("
        DELETE FROM receipts
        WHERE id=$1",
        &[&id])
}

/// Fetch a single receipt from database
pub fn get_receipt(db_path: &str, id: i32) -> Result<Receipt> {
    let conn = try!(open_connection(&db_path));

    conn.query_row("
        SELECT * FROM receipts
        WHERE id=$1",
        &[&id], |row| {
            Receipt {
                id: row.get(0),
                description: row.get(1),
                shop: row.get(2),
                amount: row.get(3),
                currency: row.get(4),
                payment_type: row.get(5),
                date_paid: row.get(6)
            }
        })
}

/// Obtain all receipts from the database
pub fn get_all_receipts(db_path: &str) -> Result<Vec<Receipt>> {
    let conn = try!(open_connection(&db_path));

    let mut query = conn.prepare("SELECT * from receipts").unwrap();

    let mut rows = query.query_map(&[], |row| {
        Receipt {
            id: row.get(0),
            description: row.get(1),
            shop: row.get(2),
            amount: row.get(3),
            currency: row.get(4),
            payment_type: row.get(5),
            date_paid: row.get(6)
        }
    }).unwrap();

    let mut receipts = Vec::new();

    for receipt in rows {
        receipts.push(receipt.unwrap());
    }

    Ok(receipts)
}

/// Insert a new receipt in the database
pub fn insert_receipt(db_path: &str, receipt: &Receipt) -> Result<c_int> {
    let conn = try!(open_connection(&db_path));

    conn.execute("
        INSERT INTO receipts (description, shop, amount, currency, payment_type, date_paid)
        VALUES ($1, $2, $3, $4, $5, $6)",
        &[
            &receipt.description,
            &receipt.shop,
            &receipt.amount,
            &receipt.currency,
            &receipt.payment_type,
            &receipt.date_paid
        ])
}

/// Update the details of a receipt in the database
pub fn update_receipt(db_path: &str, receipt: &Receipt) -> Result<c_int> {
    let conn = try!(open_connection(&db_path));

    conn.execute("
        UPDATE receipts
        SET description=$1,shop=$2,amount=$3,currency=$4,payment_type=$5,date_paid=$6
        WHERE id=$7",
        &[
            &receipt.description,
            &receipt.shop,
            &receipt.amount,
            &receipt.currency,
            &receipt.payment_type,
            &receipt.date_paid,
            &receipt.id
        ])
}
