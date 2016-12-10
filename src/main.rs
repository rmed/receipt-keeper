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

extern crate chrono;
extern crate gio;
extern crate gtk;
extern crate ini;
extern crate regex;
extern crate rusqlite;

mod config;
mod common;
mod db;
mod migrations;
mod gui;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::Application;

use common::State;
use gui::main_window;


/// Activation signal handler
fn do_activate(app: &Application) {
    // Application state
    let db_path = config::read_config_file();

    let state: Rc<RefCell<State>> = Rc::new(RefCell::new(State {
        db_path: db_path,
        window_map: HashMap::new()
    }));

    // Record ID -> Window ID
    // let mut window_map: HashMap<i32, u32> = HashMap::new();
    // migrations::migrate("/tmp/test.db", 0);

    // Add main window
    let window = main_window::create_window(&app, &state);
    window.show_all();

    app.add_window(&window);
}

fn main() {
    if gtk::init().is_err() {
		println!("Failed to initialize GTK.");
        return;
    }

    let app = Application::new(
        Some("com.rmedgar.ReceiptKeeper"),
        gio::APPLICATION_FLAGS_NONE
    ).unwrap();

    app.connect_activate(|ref app| {
        do_activate(&app,)
    });

    // Number of params and params array
    app.run(0, &[]);
}
