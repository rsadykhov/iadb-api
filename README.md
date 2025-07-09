# IADB API Wrapper

`iadb-api` is a wrapper for the Bank of England's Statistical Interactive Database (IADB). The data can be accessed using specific series codes,
most of which are defined in the variants of the `SeriesCode` enum.

**Disclaimer:** This crate is an unofficial IADB wrapper, the maintainers of the crate are independent developers.
The developers of the crate do not accept any responsibility or liability for the accuracy, security, or completeness of the code,
or the information provided within the crate.

# Example

```rust
use iadb_api::{schemas::IADBSeries, backend::IADB};

#[tokio::main]
async fn main() -> () {

    // Parameters
    let series_code: String = SeriesCode::IUDSOIA.to_string();
    let date_from: String = String::from("01/Jan/2000");
    let date_to: String = String::from("01/Oct/2018");

    // Data collection
    let data: IADBSeries = IADB::get_data(&series_code, &date_from, &date_to).await.unwrap();

    println!("{}", data);

}
```

# General information
If you would like to add a commit or an issue, please do so using the GitHub link to the project:
- <https://github.com/rsadykhov/iadb-api>