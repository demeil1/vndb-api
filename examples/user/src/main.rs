use dotenvy::dotenv;
use std::env;

use vndb_api::client::VndbApiClient;
use vndb_api::format::user::{UserDataField, UserSearchFields};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let api_client = VndbApiClient::new(&String::from(api_key));

    // Lookup users by id or username
    // For information on the struct fields see src/format/user.rs
    //      or see the documentation on users here:
    //      https://api.vndb.org/kana#get-user

    // Single user with default data fields
    let name = vec!["AYO".to_string()];
    let fields = UserSearchFields::new();
    if let Ok(auth_info) = api_client.get_user(&name, &fields).await {
        println!("{:#?}", auth_info);
    }

    // All data fields
    let name = vec!["AYO".to_string()];
    let fields = UserSearchFields::all();
    if let Ok(auth_info) = api_client.get_user(&name, &fields).await {
        println!("{:#?}", auth_info);
    }

    // Custom data fields
    let name = vec!["AYO".to_string()];
    let fields = UserSearchFields::from(vec![UserDataField::Lengthvotes]);
    if let Ok(auth_info) = api_client.get_user(&name, &fields).await {
        println!("{:#?}", auth_info);
    }

    // Multiple user search with both usernames and ids
    let names = vec![
        "AYO".to_string(),
        "NoUserWithThisNameExists".to_string(),
        "u3".to_string(),
    ];
    let fields = UserSearchFields::all();
    if let Ok(auth_info) = api_client.get_user(&names, &fields).await {
        println!("{:#?}", auth_info);
    }
}
