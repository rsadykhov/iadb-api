use std::{error::Error, fmt};
use reqwest::{Client, Response};
use csv::{Reader, ReaderBuilder, StringRecord};
use crate::{BASE_URL, schemas::{IADBSeries, IADBDataPoint}};


pub enum CSVF {
    TT,
    TN,
    CT,
    CN,
}

impl fmt::Display for CSVF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CSVF::TT => write!(f, "TT"),
            CSVF::TN => write!(f, "TN"),
            CSVF::CT => write!(f, "CT"),
            CSVF::CN => write!(f, "CN"),
        }
    }
}


pub enum VPD {
    Y,
    N,
}

impl fmt::Display for VPD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VPD::Y => write!(f, "Y"),
            VPD::N => write!(f, "N"),
        }
    }
}


pub enum Param<'a> {
    DateFrom { v: &'a String },
    DateTo { v: &'a String },
    CSVF { v: &'a CSVF },
    UsingCodes { v: &'a String },
    VPD { v: &'a VPD },
    VFD { v: &'a String },
}

impl<'a> Param<'a> {
    fn add_param_to_url(&self, url: &mut String) -> () {
        let url_param: String = match self {
            Param::DateFrom { v } => format!("&Datefrom={}", v),
            Param::DateTo { v } => format!("&Dateto={}", v),
            Param::CSVF { v } => format!("&CSVF={}", v.to_string()),
            Param::UsingCodes { v } => format!("&UsingCodes={}", v),
            Param::VPD { v } => format!("&VPD={}", v.to_string()),
            Param::VFD { v } => format!("&VFD={}", v),
        };
        url.push_str(&url_param);
    }
}


/// Make a request to the provided URL, validate the status code of the response, and return deserialized data.
async fn process_request(url: String) -> Result<Vec<IADBDataPoint>, Box<dyn Error>> {
    let user_agent: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.90 Safari/537.36";
    let client: Client = Client::builder().user_agent(user_agent).build()?;
    let response: Response = client.get(url).send().await?;
    let response_body: String = response.text().await?;
    // Deserialize the CSV
    let mut rdr: Reader<&[u8]> = ReaderBuilder::new().has_headers(true).from_reader(response_body.as_bytes());
    // Replace default CSV headers
    let _: &StringRecord = rdr.headers()?;
    rdr.set_headers(StringRecord::from(vec!["date", "value"]));
    // Deserialize data
    let mut data: Vec<IADBDataPoint> = Vec::<IADBDataPoint>::new();
    for entry in rdr.deserialize() {
        let entry: IADBDataPoint = entry?;
        data.push(entry);
    }
    Ok(data)
}


/// Constructs a URL for API request, sends the request, and returns the deserialzied response.
///
/// # Input
/// - `series_code`: Code of the time series in the IADB.
/// - `params`: List of parameters expected by the IADB API endpoint
/// - `additional_params`: Additional parameters to add to the request
pub async fn call_api_endpoint<'a>(series_code: &String, description: &Option<String>, params: Vec<Param<'a>>, additional_params: Option<String>) -> Result<IADBSeries, Box<dyn Error>> {
    // Set up a URL for the API endpoint
    let mut url: String = String::from(BASE_URL);
    // Add parameters to the URL
    url.push_str(&format!("?csv.x=yes&SeriesCodes={}", series_code));
    for param in params {
        param.add_param_to_url(&mut url);
    }
    // Add additional parameters to the URL
    match additional_params {
        Some(v) => url.push_str(&v),
        None => (),
    }
    // Process API response
    let description: String = match description {
        Some(v) => v.clone(),
        None => String::from(""),
    };
    let data: Vec<IADBDataPoint> = process_request(url).await?;
    Ok(IADBSeries { name: series_code.to_string(), description: description, data, })
}


#[cfg(test)]
mod tests {
    use tokio;

    #[tokio::test]
    async fn unit_test_request() -> () {
        use reqwest::{Client, Response};
        let user_agent: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.90 Safari/537.36";
        let client: Client = Client::builder().user_agent(user_agent).build().unwrap();
        // Request
        let url: String = String::from("http://www.bankofengland.co.uk/boeapps/iadb/fromshowcolumns.asp?csv.x=yes&Datefrom=01/Jan/2000&Dateto=01/Oct/2018&SeriesCodes=IUMBV34,IUMBV37,IUMBV42,IUMBV45&CSVF=TT&UsingCodes=Y&VPD=Y&VFD=N");
        let response: Response = client.get(url).send().await.unwrap();
        let response_body: String = response.text().await.unwrap();
        println!("{}", response_body)
    }
}