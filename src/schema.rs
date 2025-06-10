diesel::table! {
    clients (client_id) {
        client_id -> Text,
        name -> Text,
        first_name -> Text,
        birth_date -> Text,
        address -> Text,
        postal_code -> Text,
        city -> Text,
    }
}