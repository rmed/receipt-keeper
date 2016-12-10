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

/// Management of the configuration file

use ini::Ini;
use std::env;
use std::path::{Path, PathBuf};

use common::State;

/// Attempt to read the config file
///
/// If it does not exist, an empty file will be created
pub fn read_config_file() -> String {
    let mut conf_path = env::home_dir().unwrap();
    conf_path.push(".receipt-keeper");

    if !conf_path.exists() {
        // Create empty file
        create_empty_config(&conf_path);
    }

    // Read config file
    let conf = Ini::load_from_file(conf_path.to_str().unwrap()).unwrap();

    let db_sec = conf.section(Some("DB".to_owned())).unwrap();
    let db_path = db_sec.get("path").unwrap();

    // return State{ db_path: db_path.to_string() };
    db_path.to_string()
}

/// Creates an empty config file
fn create_empty_config(conf_path: &PathBuf) {
    let mut conf = Ini::new();

    conf.with_section(Some("DB".to_owned()))
        .set("path", "");

    conf.write_to_file(conf_path.to_str().unwrap()).unwrap();
}
