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
        .filters(vec!["id".to_string(), "=".to_string(), "i5".to_string()])
        .fields(TraitFieldChoices::all())
        .sort(SortField::Id)
        .results(3)
        .page(1)
        .reverse()
        .enable_count()
        .enable_compact_filters()
        .enable_normalized_filters()
        .build();
    if let Ok(response) = api_client.trait_search(&query).await {
        println!("{:#?}", response);
    }
}



