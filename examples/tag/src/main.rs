use dotenvy::dotenv;
use std::env;

use vndb_api::client::VndbApiClient;
use vndb_api::request::query::{QueryBuilder, TagQuery, TagField, TagFieldChoices, SortField};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let api_client = VndbApiClient::new(&String::from(api_key));

    // Query tag by name, id, etc.
    // For information on the struct fields see src/format/tag.rs
    //      or see the documentation on tag here:
    //      https://api.vndb.org/kana#post-tag
    // Tag queries are similar to visual novel queries 
    // see examples/vn/main.rs for detailed documentation
    let query = QueryBuilder::<TagQuery>::new()
        .filters(&r#"["id", "=", "g2"]"#.to_string())
        .fields(TagFieldChoices::all())
        .enable_compact_filters()
        .enable_normalized_filters()
        .build();
    if let Ok(response) = api_client.tag_search(&query).await {
        println!("{:#?}", response);
    }
    
    // this does the same as the query above except it searches by tag 
    // name instead of id, the "fantasy" = trait id "g2" in the above query
    let query = QueryBuilder::<TagQuery>::new()
        .filters(&r#"["search", "=", "fantasy"]"#.to_string())
        .fields(TagFieldChoices::all())
        .results(3)
        .enable_compact_filters()
        .enable_normalized_filters()
        .build();
    if let Ok(response) = api_client.tag_search(&query).await {
        println!("{:#?}", response);
    }
}



