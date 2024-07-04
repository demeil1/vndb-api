use dotenvy::dotenv;
use std::env;

use vndb_api::client::VndbApiClient;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let api_client = VndbApiClient::new(&String::from(api_key));

    // Validates and returns information about the given API token
    // For information on the struct fields see src/format/auth.rs
    //      or see the documentation on authentication here:
    //      https://api.vndb.org/kana#get-authinfo
    if let Ok(auth_info) = api_client.get_auth_info().await {
        println!("{:#?}", auth_info);
    }
}
