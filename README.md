# VNDB

This crate allows for data collection from the [VNDB site](https://vndb.org). VNDB (Visual Novel Database)
stores a plethora of information about Visual Novels, Characters, Producers, Tags, and so on.

# Documentation

The entirety of this crate is based on the new [HTTPS-based API Documentation](https://api.vndb.org/kana#post-character) and aims to simplify the way you collect data from the site. Further documentation 
and use cases for this crate can be found in the "examples" directory or in the respective modules of
the parts of the crate you use.

# Requirments
This crate only requires a VNDB API key to use. To get an API key follow the instructions below, or click on this [link](https://vndb.org/u/tokens) (only if you already have an account). [API Key Documentation](https://api.vndb.org/kana#user-authentication).

Obtaining an API Key:
1. Register for a VNDB account (or Login if you already have one)
2. Open the "My Profile" tab and navigate to the "Applications" section
3. Create a new token and use as instructed (see "examples" directory)

# Current Limitations of This Crate

1. **Filters**: This section offers a variety of [ways to filter through searches](https://api.vndb.org/kana#filters) 
ranging from a simple predicate consisting of a single array of strings to complex predicates with an array containing 
both boolean logic and arrays. As of now, this crate only supports simple predicates; however, this is subject to 
change in the future.
2. **Fields**: This section allows you to select the pieces of information you want to pull from the site about a
particular subject (Visual Novels, Producers, etc.). Unfortunately, due to the recursive nature of the [Visual Novel](https://api.vndb.org/kana#post-vn), [Release](https://api.vndb.org/kana#post-release), and [Character](https://api.vndb.org/kana#post-character) queries, recursive field choices have been limited. And, while the structs in this crate **DO** allow for said recursive field queries, recursive selection of fields is quite impractical and causes VNDB to deny providing a response due to the large size. These limitations only apply to the Visual Novel, Release, and Character queries. You **ARE** still be able to fill all struct fields with the aforementioned queries and recursive sections (such as related Visual Novels in a Visual Novel query) in the structure **WILL** still have their name and id such that they can be searched later on for extra information.
