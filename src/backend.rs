use std::error::Error;
use crate::{schemas::IADBSeries, utils::{VPD, CSVF, Param, call_api_endpoint}};


pub struct IADB;

impl IADB {

    /// Makes an API request to the IADB and deserializes the response into a time series.
    /// 
    /// # Input
    /// - `series_code`: Code of the time series in the IADB.
    /// - `date_from`: Date from which the data will be extracted (Note: Date format is `%d/%b/%Y`)
    /// - `date_from`: Date up to which the data will be extracted (Note: Date format is `%d/%b/%Y`)
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use iadb_api::{schemas::IADBSeries, backend::IADB};
    /// 
    /// #[tokio::main]
    /// async fn main() -> () {
    /// 
    ///     // Parameters
    ///     let series_code: String = SeriesCode::IUDSOIA.to_string();
    ///     let date_from: String = String::from("01/Jan/2000");
    ///     let date_to: String = String::from("01/Oct/2018");
    /// 
    ///     // Data collection
    ///     let data: IADBSeries = IADB::get_data(&series_code, &date_from, &date_to).await.unwrap();
    /// 
    ///     println!("{}", data);
    /// 
    /// }
    /// ```
    pub async fn get_data(series_code: &String, date_from: &String, date_to: &String) -> Result<IADBSeries, Box<dyn Error>> {
        // Parameters
        let using_codes: String = String::from("Y");
        let vfd: String = String::from("N");
        // Request
        let params: Vec<Param> = vec![
            Param::DateFrom { v: &date_from }, Param::DateTo { v: &date_to }, Param::CSVF { v: &CSVF::TN }, Param::UsingCodes { v: &using_codes },
            Param::VPD { v: &VPD::Y }, Param::VFD { v: &vfd },
        ];
        call_api_endpoint(&series_code, params, None).await
    }
}


#[cfg(test)]
mod tests {
    use tokio;

    #[tokio::test]
    async fn unit_test_get_data() -> () {
        use crate::{SeriesCode, schemas::IADBSeries, backend::IADB};
        // Parameters
        let series_code: String = SeriesCode::IUDSOIA.to_string();
        let date_from: String = String::from("01/Jan/2000");
        let date_to: String = String::from("01/Oct/2018");
        // Data collection
        let data: IADBSeries = IADB::get_data(&series_code, &date_from, &date_to).await.unwrap();
        println!("{}", data);
    }
}