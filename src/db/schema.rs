table! {
    articles (id) {
        id -> Text,
        published -> Timestamp,
        modified -> Timestamp,
        modified_on_disk -> Timestamp,
        local_path -> Text,
        server_path -> Text,
        title -> Text,
        html -> Text,
    }
}

table! {
    images (id) {
        id -> Text,
        modified_on_disk -> Timestamp,
        width -> Integer,
        height -> Integer,
        local_path -> Text,
        server_path -> Text,
    }
}

table! {
    pages (id) {
        id -> Text,
        published -> Timestamp,
        modified -> Timestamp,
        modified_on_disk -> Timestamp,
        local_path -> Text,
        server_path -> Text,
        title -> Text,
        html -> Text,
    }
}

table! {
    urls (id) {
        id -> Text,
        url -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    articles,
    images,
    pages,
    urls,
);
