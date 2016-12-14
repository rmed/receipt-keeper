/// Fill combobox and table stores
macro_rules! fill_store {
    (combo => $list:ident, $values:ident) => {
        for (index, val) in $values.into_iter().enumerate() {
            $list.insert_with_values(Some((index+1) as u32), &[0], &[val]);
        }
    };

    (table => $list:ident, $values:ident) => {
        $list.clear();

        for (index, val) in $values.iter().enumerate() {
            $list.insert_with_values(
                None,
                &[0, 1, 2, 3, 4, 5],
                &[
                    &val.id,
                    &val.shop,
                    &val.amount,
                    &val.currency,
                    &val.payment_type,
                    &val.date_paid.to_string()
                ]);
        }
    };
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

pub mod main_window;
pub mod edit_window;
