# Example
```rust
use dotenvy::dotenv;
use std::env;

use vndb_api::client::VndbApiClient;
use vndb_api::request::query::{QueryBuilder, VnQuery, VnField, VnFieldChoices, SortField};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let api_client = VndbApiClient::new(&String::from(api_key));

    // For more examples see the ["examples" directory]
    //      (https://github.com/demeil1/vndb-api/tree/main/examples) 
    // in the github repo

    // search for visual novel by name with autocomplete options
    let query = QueryBuilder::<VnQuery>::new()
        .filters(vec!["search".to_string(), "=".to_string(), "DDLC".to_string()])
        .fields(VnFieldChoices::all()))
        .results(3)
        .page(1)
        .build();
    match api_client.vn_search(&query).await {
        Ok(response) => {
            response.results.iter()
                .for_each(|vn| {
                    println!("{}", vn.title.as_ref().unwrap());
                });
        }
        Err(error) => {
            eprintln!("{:#?}", error);
        }
    }

    // prints the name and rating for the top 3 visual novels on the site
    let query = QueryBuilder::<VnQuery>::new()
        .filters(&r#"["search", "=", "DDLC"]"#.to_string())
        .fields(VnFieldChoices::from(vec![VnField::Title]))
        .results(3)
        .build();
    match api_client.vn_search(&query).await {
        Ok(response) => {
            response.results.iter()
                .for_each(|vn| {
                    println!("{}", vn.title.as_ref().unwrap());
                });
        }
        Err(error) => {
            eprintln!("{:#?}", error);
        }
    }

    // using complex filters
    let filters = r#"
        [ "and"
            , [ "or"
                , [ "olang", "!=", "en" ]
                , [ "olang", "!=", "ja" ]
            ]
            , [ "released", ">=", "2020-01-01" ]
        ]
    "#.to_string();
    let query = QueryBuilder::<VnQuery>::new()
        .filters(&filters)
        .results(10)
        .page(1)
        .build();
    match api_client.vn_search(&query).await {
        Ok(response) => {
            println!("{:#?}", response);
        }
        Err(error) => { eprintln!("{:#?}", error); }
    }
}
```

# VNDB

This crate allows for data collection from the [VNDB site](https://vndb.org). VNDB (Visual Novel Database)
stores a plethora of information about Visual Novels, Characters, Producers, Tags, and so on.

# Documentation

The entirety of this crate is based on the new [HTTPS-based API Documentation](https://api.vndb.org/kana#post-character) and aims to simplify the way you collect data from the site. Further documentation 
and use cases for this crate can be found in the ["examples" directory on github](https://github.com/demeil1/vndb-api/tree/main/examples) or in the respective modules of
the parts of the crate you use.

# Requirments
This crate only requires a VNDB API key to use. To get an API key follow the instructions below, or click on this [link](https://vndb.org/u/tokens) (only if you already have an account). [API Key Documentation](https://api.vndb.org/kana#user-authentication).

Obtaining an API Key:
1. Register for a VNDB account (or Login if you already have one)
2. Open the "My Profile" tab and navigate to the "Applications" section
3. Create a new token and use as instructed (see ["examples" directory](https://github.com/demeil1/vndb-api/tree/main/examples))

# Reqursive Queries

**Fields**: This section allows you to select the pieces of information you want to pull from the site about a
particular subject (Visual Novels, Producers, etc.). Unfortunately, due to the recursive nature of the [Visual Novel](https://api.vndb.org/kana#post-vn), [Release](https://api.vndb.org/kana#post-release), and [Character](https://api.vndb.org/kana#post-character) queries, recursive field choices have been limited. And, while the structs in this crate **DO** allow for said recursive field queries, recursive selection of fields is quite impractical and causes VNDB to deny providing a response due to the large size. These limitations only apply to the Visual Novel, Release, and Character queries. You **ARE** still be able to fill all struct fields with the aforementioned queries. Recursive sections (such as related Visual Novels in a Visual Novel query) in the structure **WILL** still have their name and id such that they can be searched later on for extra information.