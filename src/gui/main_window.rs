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

/// Main window definition

use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;
use std::thread;

use glib;
use gtk;
use gtk::prelude::*;
use gtk::{
    AboutDialog,
    Application,
    ApplicationWindow,
    Box,
    Builder,
    HeaderBar,
    Orientation,
    Revealer,
    RevealerTransitionType,
    Type,
    Window,
    WindowPosition,
    WindowType
};
use gtk::{
    Button,
    CellRendererText,
    Calendar,
    ComboBox,
    Entry,
    Label,
    ListStore,
    Popover,
    ScrolledWindow,
    Separator,
    SpinButton,
    ToggleButton,
    TreeIter,
    TreeSelection,
    TreeView,
    TreeViewColumn
};
use rusqlite::Connection;

use common::{State, CURRENCIES, PAYMENTS};
use migrations;
use db;
use db::Receipt;
use gui::edit_window;


/// Creates the main window
pub fn create_window(app: &Application,
                     state: &Rc<RefCell<State>>) -> ApplicationWindow {

    let window = ApplicationWindow::new(&app);

    window.set_title("Receipt Keeper");
    window.set_border_width(10);
    window.set_default_size(800, 600);
    window.set_position(WindowPosition::Center);

    let builder = Builder::new();
    builder.add_from_string(include_str!("main_window.ui"));

    // Header bar
    let header_bar: HeaderBar = builder.get_object("header_bar").unwrap();
    window.set_titlebar(Some(&header_bar));

    // Window container
    let main_box: Box = builder.get_object("main_box").unwrap();
    window.add(&main_box);

    // Fill stores
    let store_currency: ListStore = builder.get_object("store_currency").unwrap();
    fill_store!(combo => store_currency, CURRENCIES);

    let store_type: ListStore = builder.get_object("store_type").unwrap();
    fill_store!(combo => store_type, PAYMENTS);

    let store_table: ListStore = builder.get_object("store_table").unwrap();
    let receipts = db::get_all_receipts(&state.borrow().db_path).unwrap();
    fill_store!(table => store_table, receipts);

    // Events

    // Create a new record
    {
        let builder = builder.clone();
        let app = window.get_application().unwrap();
        let state = state.clone();
        let btn_new: Button = builder.get_object("btn_new").unwrap();

        btn_new.connect_clicked(move |_| {
            let dialog = edit_window::create_window(&app, &state, -1);
            dialog.show();
        });
    }

    // Toggle search box
    {
        let builder = builder.clone();
        let toggle_search: ToggleButton = builder.get_object("toggle_search").unwrap();

        let revealer: Revealer = builder.get_object("revealer").unwrap();

        toggle_search.connect_toggled(move |toggle| {
            if toggle.get_active() {
                revealer.set_reveal_child(true);

            } else {
                revealer.set_reveal_child(false);
            }
        });
    }

    // Toogle date chooser (from)
    {
        let builder = builder.clone();
        let btn_date_from: Button = builder.get_object("btn_date_from").unwrap();

        let popover_date: Popover = builder.get_object("popover_date").unwrap();
        let entry_date_from: Entry = builder.get_object("entry_date_from").unwrap();

        btn_date_from.connect_clicked(move |_| {
            popover_date.set_relative_to(Some(&entry_date_from));
            popover_date.show_all();
        });
    }

    // Toogle date chooser (to)
    {
        let builder = builder.clone();
        let btn_date_to: Button = builder.get_object("btn_date_to").unwrap();

        let popover_date: Popover = builder.get_object("popover_date").unwrap();
        let entry_date_to: Entry = builder.get_object("entry_date_to").unwrap();

        btn_date_to.connect_clicked(move |_| {
            popover_date.set_relative_to(Some(&entry_date_to));
            popover_date.show_all();
        });
    }

    // Calendar chooser
    // NOTE: Months start in 0
    {
        let builder = builder.clone();
        let calendar: Calendar = builder.get_object("calendar").unwrap();

        let popover_date: Popover = builder.get_object("popover_date").unwrap();
        let entry_date_from: Entry = builder.get_object("entry_date_from").unwrap();
        let entry_date_to: Entry = builder.get_object("entry_date_to").unwrap();


        calendar.connect_day_selected(move |calendar| {
            let (year, month, day) = calendar.get_date();

            if popover_date.get_relative_to().unwrap().eq(&entry_date_from) {
                entry_date_from.set_text(
                    format!("{:02}/{:02}/{}", day, month+1, year).as_str()
                );

            } else {
                entry_date_to.set_text(
                    format!("{:02}/{:02}/{}", day, month+1, year).as_str()
                );
            }
        });
    }

    // Clear search
    {
        let builder = builder.clone();
        let btn_clear: Button = builder.get_object("btn_clear_search").unwrap();

        let popover_menu: Popover = builder.get_object("popover_menu").unwrap();
        let entry_shop: Entry = builder.get_object("entry_shop").unwrap();
        let spin_cost_from: SpinButton = builder.get_object("spin_cost_from").unwrap();
        let spin_cost_to: SpinButton = builder.get_object("spin_cost_to").unwrap();
        let entry_date_from: Entry = builder.get_object("entry_date_from").unwrap();
        let entry_date_to: Entry = builder.get_object("entry_date_to").unwrap();
        let combo_type: ComboBox = builder.get_object("combo_type").unwrap();
        let combo_currency: ComboBox = builder.get_object("combo_currency").unwrap();

        btn_clear.connect_clicked(move |_| {
            entry_shop.set_text("");
            spin_cost_from.set_value(0.0);
            spin_cost_to.set_value(0.0);
            entry_date_from.set_text("");
            entry_date_to.set_text("");
            combo_type.set_active(-1);
            combo_currency.set_active(-1);

            // Hide menu
            popover_menu.hide();

            // TODO filter results
        });
    }

    // Show about dialog
    {
        let builder = builder.clone();
        let btn_about: Button = builder.get_object("btn_about").unwrap();

        let popover_menu: Popover = builder.get_object("popover_menu").unwrap();
        let dialog: AboutDialog = builder.get_object("about_dialog").unwrap();

        btn_about.connect_clicked(move |_| {
            popover_menu.hide();
            dialog.run();
        });
    }

    // Toggle remove and edit buttons
    {
        let builder = builder.clone();
        let table_selection: TreeSelection = builder.get_object("table_selection").unwrap();

        let btn_remove: Button = builder.get_object("btn_remove").unwrap();
        let btn_edit: Button = builder.get_object("btn_edit").unwrap();

        table_selection.connect_changed(move |selection| {
            if selection.count_selected_rows() == 1 {
                btn_remove.set_sensitive(true);
                btn_edit.set_sensitive(true);

            } else {
                btn_remove.set_sensitive(false);
                btn_edit.set_sensitive(false);
            }
        });
    }

    // Show window to view/edit a receipt
    {
        let builder = builder.clone();
        let btn_edit: Button = builder.get_object("btn_edit").unwrap();

        let state = state.clone();
        let app = window.get_application().unwrap();
        let table_selection: TreeSelection = builder.get_object("table_selection").unwrap();

        btn_edit.connect_clicked(move |_| {
            let (model, iter) = table_selection.get_selected().unwrap();
            let id = model.get_value(&iter, 0).get::<i32>().unwrap();

            let dialog: ApplicationWindow;
            let mut exists = false;
            let mut stored: Window = Window::new(WindowType::Toplevel); // Dummy

            {
                // Check if window is already running
                let borrowed = state.borrow();

                if borrowed.window_map.contains_key(&id) {
                    // Load stored window
                    stored = app.get_window_by_id(
                        *borrowed.window_map.get(&id).unwrap()
                    ).unwrap();

                    exists = true;
                }
            }

            if exists {
                // Bring window to front
                unsafe {
                    dialog = mem::transmute::<Window, ApplicationWindow>(stored);
                };

                dialog.present();

            } else {
                // Create new window and add it to the map
                dialog = edit_window::create_window(&app, &state, id);

                state.borrow_mut().window_map.insert(id, dialog.get_id());

                dialog.show();
            }
        });
    }

    // Store builder and database path in thread local storage
    {
        let builder = builder.clone();
        let db_path = state.borrow().db_path.clone();

        REFRESH.with(move |r| {
            *r.borrow_mut() = Some((builder, db_path));
        });
    }

    window
}

/// Refresh the receipt table
///
/// This is usually done when adding or editing a receipt
pub fn refresh_table() -> glib::Continue {
    REFRESH.with(move |r| {
        if let Some((ref builder, ref db_path)) = *r.borrow() {
            let store_table: ListStore = builder.get_object("store_table").unwrap();

            let receipts = db::get_all_receipts(&db_path).unwrap();
            fill_store!(table => store_table, receipts);
        }
    });

    glib::Continue(false)
}

// Keep builder in thread local storage to update the table
thread_local!(
    static REFRESH: RefCell<Option<(Builder, String)>> = RefCell::new(None)
);
