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

/// Static and constant values

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use chrono::offset::local::Local;


/// Application state
///
/// The window map is a HashMap that stores window IDs using
/// Receipt ID as key
pub struct State {
    pub db_path: String,
    pub window_map: HashMap<i32, u32>,
}

/// Date regular expression
pub const RE_DATE: &'static str = r"^(\d{2})/(\d{2})/(\d{4})$";

/// Currencies
pub const CURRENCIES: [&'static str; 3] = [
    "EUR",
    "GBP",
    "USD"
];

/// Payment types
pub const PAYMENTS: [&'static str; 3] = [
    "card",
    "cash",
    "points"
];
