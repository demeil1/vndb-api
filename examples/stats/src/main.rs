use dotenvy::dotenv;
use std::env;

use vndb_api::client::VndbApiClient;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let api_client = VndbApiClient::new(&String::from(api_key));

    // Returns a few overall database statistics
    // For information on the struct fields see src/format/stats.rs
    //      or see the documentation on statistics here:
    //      https://api.vndb.org/kana#get-stats
    if let Ok(stats) = api_client.get_stats().await {
        println!("{:#?}", stats);
    }
}
