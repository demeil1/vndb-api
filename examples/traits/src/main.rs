use dotenvy::dotenv;
use std::env;

use vndb_api::client::VndbApiClient;
use vndb_api::request::query::{QueryBuilder, TraitQuery, TraitField, TraitFieldChoices, SortField};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let api_client = VndbApiClient::new(&String::from(api_key));

    // Query trait by name, id, etc.
    // For information on the struct fields see src/format/trait.rs
    //      or see the documentation on tag here:
    //      https://api.vndb.org/kana#post-trait
    // Trait queries are similar to visual novel queries 
    // see examples/vn/main.rs for detailed documentation
    let query = QueryBuilder::<TraitQuery>::new()
        .filters(&r#"["id", "=", "i5"]"#.to_string())
        .fields(TraitFieldChoices::all())
        .enable_compact_filters()
        .enable_normalized_filters()
        .build();
    if let Ok(response) = api_client.trait_search(&query).await {
        println!("{:#?}", response);
    }

    // this does the same as the query above except it searches by trait 
    // name instead of id, the "blond" = trait id "i5" in the above query
    let query = QueryBuilder::<TraitQuery>::new()
        .filters(&r#"["search", "=", "blond"]"#.to_string())
        .fields(TraitFieldChoices::all())
        .enable_compact_filters()
        .enable_normalized_filters()
        .build();
    if let Ok(response) = api_client.trait_search(&query).await {
        println!("{:#?}", response);
    }
}



