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

/// Settings definition

use std::cell::RefCell;
use std::os::raw::{c_int, c_double};
use std::rc::Rc;

use chrono::NaiveDate;
use glib;
use gtk;
use gtk::prelude::*;
use gtk::{
    Application,
    ApplicationWindow,
    Builder,
    Box,
    ButtonsType,
    Dialog,
    DialogFlags,
    MessageType,
    Revealer,
    Window,
    WindowPosition
};
use gtk::{
    Button,
    CellRendererText,
    Calendar,
    ComboBox,
    Entry,
    HeaderBar,
    InfoBar,
    Label,
    ListStore,
    MessageDialog,
    Popover,
    ScrolledWindow,
    Separator,
    SpinButton,
    TextView,
    ToggleButton,
    TreeIter,
    TreeView,
    TreeViewColumn
};
use regex::Regex;

use common::{State, RE_DATE, CURRENCIES, PAYMENTS};
use config;
use db;
use db::Receipt;
use gui::main_window;


/// Creates the settings dialog
pub fn create_window(app: &Application, state: &Rc<RefCell<State>>)
                     -> ApplicationWindow {

    let window = ApplicationWindow::new(&app);
    window.set_title("Settings");
    // window.set_border_width(10);
    window.set_default_size(250, 150);
    window.set_modal(true);
    window.set_position(WindowPosition::Center);

    let builder = Builder::new();
    builder.add_from_string(include_str!("settings_window.ui"));

    // Header bar
    let header_bar: HeaderBar = builder.get_object("header_bar").unwrap();
    window.set_titlebar(Some(&header_bar));

    // Container
    let main_box: Box = builder.get_object("main_box").unwrap();
    window.add(&main_box);

    // Load settings
    {
        let state = state.clone();

        let entry_db: Entry = builder.get_object("entry_db").unwrap();
        entry_db.set_text(state.borrow().db_path.as_str());
    }


    // Events

    // Hide the information bar
    {
        let builder = builder.clone();
        let info_bar: InfoBar = builder.get_object("info_bar").unwrap();

        let revealer: Revealer = builder.get_object("revealer").unwrap();

        info_bar.connect_response(move |_, _| {
            revealer.set_reveal_child(false);
        });
    }


    // Save settings
    {
        let builder = builder.clone();
        let btn_save: Button = builder.get_object("btn_save").unwrap();
        let state = state.clone();

        let entry_db: Entry = builder.get_object("entry_db").unwrap();
        let revealer: Revealer = builder.get_object("revealer").unwrap();

        btn_save.connect_clicked(move |_| {
            let mut borrowed = state.borrow_mut();

            borrowed.db_path = entry_db.get_text().unwrap();

            config::write_config_file(&borrowed);

            revealer.set_reveal_child(true);
        });
    }

    window
}
