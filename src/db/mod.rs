pub mod models;
pub mod repositories;
pub mod schema;

#[macro_use]
pub mod uuid_macro;

generate_uuid_field! {
    ArticleId
}
