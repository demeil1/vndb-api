use dotenvy::dotenv;
use std::env;

use vndb_api::client::VndbApiClient;
use vndb_api::request::query::{QueryBuilder, VnQuery, VnField, VnFieldChoices, SortField};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let api_client = VndbApiClient::new(&String::from(api_key));

    // Query visual novel entries by name, id, etc.
    // For information on the struct fields see src/format/vn.rs
    //      or see the documentation on visual novels here:
    //      https://api.vndb.org/kana#post-vn
    // Client functions that use the QueryBuilder are wrapped in a response object
    //      see src/request/response.rs for more information
    let query = QueryBuilder::<VnQuery>::new()
        // allows for filtering through searches there are many other ways to 
        // filter through searches such as filtering based on languages and producers
        // this field can be blank in the form of an empty vector
        // currently this code filters by name to find a specific visual novel
        // more ways to filter can be found in the official documentation
        .filters(vec!["search".to_string(), "=".to_string(), "Saya no Uta".to_string()])
        // allows for selection of desired data fields can be done by hand using
        // VnFieldChoices::from() by passing in a vector of the VnField type
        // which can be found in src/request/query.rs
        .fields(VnFieldChoices::all())
        // field to sort on, defaults to id if not used or an invalid sort field is entered
        // as is, this lists visual novels from lowest to highest rating; however this can be
        // flipped with the .reverse() function
        // more use cases for the sort field can be found in the official documentation
        .sort(SortField::Rating)
        // limits results recieved to a number between 0 - 100 and automatically
        // adjust numbers outside of that range to fit within it
        .results(3)
        // VNDB API's cheaper form of pagination
        // more information can be found in the official documentation here:
        //      https://api.vndb.org/kana#api-structure
        //      https://api.vndb.org/kana#pagination
        .page(1)
        // reverses the entirety of the results making the last result come first and 
        // the first result come last 
        .reverse()
        // if enabled the response provides the number of results in addition to the results
        .enable_count()
        // if enabled the response provides the filters as a string representation
        // futher explained in the official documentation here:
        //      https://api.vndb.org/kana#api-structure
        .enable_compact_filters()
        // if enabled the response provides the filters in their original structure
        // futher explained in the official documentation here:
        //      https://api.vndb.org/kana#api-structure
        .enable_normalized_filters()
        // builds the query for compatibility with this crates client
        .build();
    // run the query
    if let Ok(response) = api_client.vn_search(&query).await {
        println!("{:#?}", response);
    }
    
    // gets autocomplete options
    let query = QueryBuilder::<VnQuery>::new()
        .filters(vec!["search".to_string(), "=".to_string(), "DDLC".to_string()])
        .fields(VnFieldChoices::from(vec![VnField::Title]))
        .results(3)
        .page(1)
        .build();
    if let Ok(response) = api_client.vn_search(&query).await {
        response.results.iter()
            .filter(|vn| vn.title.is_some())
            .for_each(|vn| {
                println!("{}", vn.title.as_ref().unwrap());
            });
    }

    // prints the name and rating for the top 3 visual novels on the site
    let query = QueryBuilder::<VnQuery>::new()
        .fields(VnFieldChoices::from(vec![VnField::Id, VnField::Title, VnField::Rating]))
        .sort(SortField::Rating)
        .results(3)
        .page(1)
        .reverse()
        .build();
    if let Ok(response) = api_client.vn_search(&query).await {
        response.results.iter()
            .filter(|vn| vn.title.is_some() && vn.rating.is_some())
            .for_each(|vn| {
                println!("{}: {}", vn.title.as_ref().unwrap(), vn.rating.unwrap());
            });
    }

}


