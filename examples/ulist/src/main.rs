use dotenvy::dotenv;
use std::env;

use vndb_api::client::VndbApiClient;
use vndb_api::request::query::{QueryBuilder, UListQuery, UListField, UListFieldChoices, SortField};
use vndb_api::format::ulist::{UListLabelsFieldChoices, UListPatchBuilder, Date, LabelId};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let api_client = VndbApiClient::new(&String::from(api_key));

    // Fetch, add to, or remove from a user list by id
    // For information on the struct fields see src/format/ulist.rs
    //      or see the documentation on user lists here:
    //      1. https://api.vndb.org/kana#post-ulist
    //      2. https://api.vndb.org/kana#get-ulist_labels
    //      3. https://api.vndb.org/kana#patch-ulistid
    //      4. https://api.vndb.org/kana#delete-ulistid

    // 1. get user list by id
    let query = QueryBuilder::<UListQuery>::new()
        // also accepts all the same QueryBuilder functions as vn except .enable_count()
        // and all other structures that rely on the QueryBuilder 
        // such as Producer, Release, etc.
        // see examples/vn/main.rs for more documentation
        // except UListQuery utilizes the .user() function  
        .user(&String::from("u2"))
        .results(1)
        .filters(vec!["id".to_string(), "=".to_string(), "v1".to_string()])
        .fields(UListFieldChoices::all())
        .sort(SortField::Id)
        .page(1)
        .reverse()
        .enable_compact_filters()
        .enable_normalized_filters()
        .build();
    if let Ok(response) = api_client.ulist(&query).await {
        println!("{:#?}", response);
    }

    // 2. fetch the list labels for a certain user by id
    if let Ok(response) = api_client.get_ulist_labels(&String::from("u2"), &UListLabelsFieldChoices::all()).await {
        println!("{:#?}", response);
    }

    // 3. Add or update a visual novel in the user’s list
    let patch = UListPatchBuilder::new()
        // rating for the visual novel from 10 - 100 automatically adjust if out of bounds
        .vote(97)
        // leave a notes for that specific visual novel
        .notes(String::from("This is my favorite"))
        // start date
        .started(Date{ year: 2024, month: 7, day: 3 })
        // finish date
        .started(Date{ year: 2024, month: 7, day: 4 })
        // setting this will overwrite any existing labels assigned to the VN with the given array
        .labels(vec![LabelId::Finished])
        // array of label ids to add to the VN any already existing labels will be unaffected
        .labels_set(vec![LabelId::WishList])
        // array of label ids to remove from the VN
        .labels_unset(vec![LabelId::BlackList])
        // build the patcher
        .build();
    if let Ok(_) = api_client.ulist_patch(&String::from("v101"), &patch).await {
        println!("Success: added v101");
    }

    // 4. remove a visual novel from the user’s list. 
    // returns success even if the VN is not on the user’s list. 
    // removing a VN also removes any associated releases from the user’s list.
    if let Ok(_) = api_client.ulist_remove(&String::from("v101")).await {
        println!("Success: removed v101");
    }
}
