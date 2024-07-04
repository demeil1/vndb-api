use dotenvy::dotenv;
use std::env;

use vndb_api::client::VndbApiClient;
use vndb_api::format::rlist::RListPatch;
use vndb_api::format::ulist::UListStatus;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let api_client = VndbApiClient::new(&String::from(api_key));

    // Aadd to or remove from a release list by id
    // For information on the struct fields see src/format/rlist.rs
    //      or see the documentation on user lists here:
    //      1. https://api.vndb.org/kana#patch-rlistid
    //      2. https://api.vndb.org/kana#delete-rlistid

    
    // 1. add or update a release in the user’s list. 
    // requires the listwrite permission. 
    // all visual novels linked to the release are also added 
    // to the user’s visual novel list if they aren’t in the list yet.
    let patch = RListPatch::new().status(UListStatus::Obtained);
    if let Ok(_) = api_client.rlist_patch(&String::from("r12"), &patch).await {
        println!("Successfully patched release list!");
    }

    // 2. remove a release from the user’s list. 
    // returns success even if the release is not on the user’s list. 
    // removing a release does not remove the associated visual novels from the user’s visual novel list
    // see examples/ulist/main.rs for that
    if let Ok(_) = api_client.rlist_remove(&String::from("r12")).await {
        println!("Successfully removed release!");
    }
}