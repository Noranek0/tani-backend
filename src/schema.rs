// @generated automatically by Diesel CLI.

diesel::table! {
    inventory (id) {
        id -> Int4,
        #[max_length = 250]
        item_name -> Varchar,
        current_stock -> Int4,
    }
}
