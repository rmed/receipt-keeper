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

/// Edit/View dialog definition

use std::cell::RefCell;
use std::os::raw::{c_int, c_double};
use std::rc::Rc;

use chrono::NaiveDate;
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
use db;
use db::Receipt;

/// Fill combobox store
macro_rules! fill_store {
    ($list:ident, $values:ident) => {
        for (index, val) in $values.into_iter().enumerate() {
            $list.insert_with_values(Some((index+1) as u32), &[0], &[val]);
        }
    }
}

/// Set combobox active element based on the value
macro_rules! set_active_combo {
    ($combo:ident, $list:ident, $value:ident) => {
        let check: &str = $value.as_str();

        for (index, val) in $list.into_iter().enumerate() {
            if check == val.to_string() {
                $combo.set_active(index as i32);
                break;
            }
        }
    }
}

/// Creates a view/edit dialog
pub fn create_window(app: &Application, state: &Rc<RefCell<State>>,
                     receipt_id: i32) -> ApplicationWindow {

    let mut title = "";
    let mut is_modal = false;
    let receipt: Receipt;

    if receipt_id >= 0 {
        title = "Edit receipt";
        receipt = db::get_receipt(state.borrow().db_path.as_str(), receipt_id).unwrap();

    } else {
        title = "New receipt";
        is_modal = true;
        receipt = Receipt::new();
    }

    let window = ApplicationWindow::new(&app);
    window.set_title(title);
    // window.set_border_width(10);
    window.set_default_size(600, 400);
    window.set_modal(is_modal);
    window.set_position(WindowPosition::Center);

    let builder = Builder::new();
    builder.add_from_string(include_str!("edit_window.ui"));

    // Header bar
    let header_bar: HeaderBar = builder.get_object("header_bar").unwrap();
    header_bar.set_title(Some(&title));
    window.set_titlebar(Some(&header_bar));

    // Container
    let main_box: Box = builder.get_object("main_box").unwrap();
    window.add(&main_box);

    // Fill stores
    let store_currency: ListStore = builder.get_object("store_currency").unwrap();
    fill_store!(store_currency, CURRENCIES);

    let store_type: ListStore = builder.get_object("store_type").unwrap();
    fill_store!(store_type, PAYMENTS);

    // Load data
    if receipt_id >= 0 {
        let entry_shop: Entry = builder.get_object("entry_shop").unwrap();
        entry_shop.set_text(receipt.shop.as_str());

        let entry_desc: TextView = builder.get_object("entry_desc").unwrap();
        let desc_buffer = entry_desc.get_buffer().unwrap();
        desc_buffer.set_text(receipt.description.as_str());

        let spin_cost: SpinButton = builder.get_object("spin_cost").unwrap();
        spin_cost.set_value(receipt.amount);

        let combo_type: ComboBox = builder.get_object("combo_type").unwrap();
        let payment_type = receipt.payment_type.clone();
        set_active_combo!(combo_type, PAYMENTS, payment_type);

        let combo_currency: ComboBox = builder.get_object("combo_currency").unwrap();
        let currency = receipt.currency.clone();
        set_active_combo!(combo_currency, CURRENCIES, currency);

        let entry_date: Entry = builder.get_object("entry_date").unwrap();
        entry_date.set_text(receipt.date_paid.format("%d/%m/%Y").to_string().as_str());
    }

    // Events

    // Hide the information bar
    {
        let builder = builder.clone();
        let info_bar: InfoBar = builder.get_object("info_bar").unwrap();

        info_bar.connect_response(|bar, _| {
            bar.hide();
        });
    }

    // Toogle date chooser (to)
    {
        let builder = builder.clone();
        let btn_date: Button = builder.get_object("btn_date").unwrap();

        let popover_date: Popover = builder.get_object("popover_date").unwrap();
        let entry_date: Entry = builder.get_object("entry_date").unwrap();

        btn_date.connect_clicked(move |_| {
            popover_date.set_relative_to(Some(&entry_date));
            popover_date.show_all();
        });
    }

    // Calendar chooser
    // NOTE: Months start in 0
    {
        let builder = builder.clone();
        let calendar: Calendar = builder.get_object("calendar").unwrap();

        let entry_date: Entry = builder.get_object("entry_date").unwrap();

        calendar.connect_day_selected(move |calendar| {
            let (year, month, day) = calendar.get_date();

            entry_date.set_text(format!("{}/{}/{}", day, month+1, year).as_str());
        });
    }

    // Save receipt
    {
        let builder = builder.clone();
        let btn_save: Button = builder.get_object("btn_save").unwrap();
        let state = state.clone();

        let info_bar: InfoBar = builder.get_object("info_bar").unwrap();
        let lbl_info: Label = builder.get_object("lbl_info").unwrap();
        let entry_shop: Entry = builder.get_object("entry_shop").unwrap();
        let entry_desc: TextView = builder.get_object("entry_desc").unwrap();
        let spin_cost: SpinButton = builder.get_object("spin_cost").unwrap();
        let combo_type: ComboBox = builder.get_object("combo_type").unwrap();
        let combo_currency: ComboBox = builder.get_object("combo_currency").unwrap();
        let entry_date: Entry = builder.get_object("entry_date").unwrap();
        let receipt_id = receipt_id.clone();

        btn_save.connect_clicked(move |_| {
            // Check shop, date and comboboxes
            let mut error_check = Vec::new();

            let value_shop = entry_shop.get_text().unwrap();
            if value_shop.is_empty() {
                error_check.push("shop");
            }

            if combo_type.get_active() < 0 {
                error_check.push("type");
            }

            if combo_currency.get_active() < 0 {
                error_check.push("currency");
            }

            let value_date = entry_date.get_text().unwrap();
            let re_date = Regex::new(RE_DATE).unwrap();
            if value_date.is_empty() || !re_date.is_match(value_date.as_str()) {
                error_check.push("date");
            }

            if error_check.len() > 0 {
                // Show information and abort saving
                lbl_info.set_text(error_check.join(", ").as_str());
                //FIXME
                info_bar.show();

                return;
            }

            // Create receipt
            let mut receipt = receipt.clone();

            let buffer = entry_desc.get_buffer().unwrap();
            receipt.description = buffer.get_text(
                &buffer.get_start_iter(),
                &buffer.get_end_iter(),
                true
            ).unwrap();

            receipt.shop = value_shop;
            receipt.amount = spin_cost.get_value();

            let iter_type = combo_type.get_active_iter().unwrap();
            let model_type = combo_type.get_model().unwrap();
            receipt.payment_type = model_type.get_value(&iter_type, 0).get::<String>().unwrap();

            let iter_currency = combo_currency.get_active_iter().unwrap();
            let model_currency = combo_currency.get_model().unwrap();
            receipt.currency = model_currency.get_value(&iter_currency, 0).get::<String>().unwrap();

            receipt.date_paid = NaiveDate::parse_from_str(value_date.as_str(), "%d/%m/%Y").unwrap();

            // Save receipt
            let status;
            if receipt_id < 0 {
                // Creating
                status = db::insert_receipt(&state.borrow().db_path, &receipt);

            } else {
                // Updating
                // TODO
            }

            //TODO check status and show infobar if error
        });
    }

    // Window closed, remove from map if applicable
    {
        let window = window.clone();
        let state = state.clone();
        let receipt_id = receipt_id.clone();

        window.connect_delete_event(move |_, _| {
            let mut borrowed = state.borrow_mut();

            if receipt_id >= 0 && borrowed.window_map.contains_key(&receipt_id) {
                // Remove window
                borrowed.window_map.remove(&receipt_id);
            }

            Inhibit(false)
        });
    }

    window
}
