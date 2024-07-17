use dotenvy::dotenv;
use std::env;

use vndb_api::client::VndbApiClient;
use vndb_api::request::query::{QueryBuilder, StaffQuery, StaffField, StaffFieldChoices, SortField};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let api_client = VndbApiClient::new(&String::from(api_key));

    // Query staff by name, id, etc.
    // For information on the struct fields see src/format/staff.rs
    //      or see the documentation on staff here:
    //      https://api.vndb.org/kana#post-staff
    // Staff queries are similar to visual novel queries 
    // see examples/vn/main.rs for detailed documentation
    let query = QueryBuilder::<StaffQuery>::new()
        .filters(&r#"["id", "=", "s1"]"#.to_string())
        .fields(StaffFieldChoices::all())
        .enable_compact_filters()
        .enable_normalized_filters()
        .build();
    if let Ok(response) = api_client.staff_search(&query).await {
        println!("{:#?}", response);
    }


    // searching for a vn staff member by name
    let query = QueryBuilder::<StaffQuery>::new()
        .filters(&r#"["search", "=", "Yukari Tamura"]"#.to_string())
        .fields(StaffFieldChoices::all())
        .enable_compact_filters()
        .enable_normalized_filters()
        .build();
    if let Ok(response) = api_client.staff_search(&query).await {
        println!("{:#?}", response);
    }
}



