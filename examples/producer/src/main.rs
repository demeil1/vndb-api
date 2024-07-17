use dotenvy::dotenv;
use std::env;

use vndb_api::client::VndbApiClient;
use vndb_api::request::query::{QueryBuilder, ProducerQuery, ProducerField, ProducerFieldChoices, SortField};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let api_client = VndbApiClient::new(&String::from(api_key));

    // Query producer by name, id, etc.
    // For information on the struct fields see src/format/producer.rs
    //      or see the documentation on producers here:
    //      https://api.vndb.org/kana#post-producer
    // Producer queries are similar to visual novel queries 
    // see examples/vn/main.rs for detailed documentation
    let query = QueryBuilder::<ProducerQuery>::new()
        .filters(&r#"["id", "=", "p1"]"#.to_string())
        .fields(ProducerFieldChoices::all())
        .enable_compact_filters()
        .enable_normalized_filters()
        .build();
    if let Ok(response) = api_client.producer_search(&query).await {
        println!("{:#?}", response);
    }

    // searching for the same producer that appeared in the query above
    // but by name this time
    let query = QueryBuilder::<ProducerQuery>::new()
        .filters(&r#"["search", "=", "Yamikumo-Communications"]"#.to_string())
        .fields(ProducerFieldChoices::all())
        .enable_compact_filters()
        .enable_normalized_filters()
        .build();
    if let Ok(response) = api_client.producer_search(&query).await {
        println!("{:#?}", response);
    }
}



