use std::fmt;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
/// Data series.
pub struct IADBSeries {
    /// IADB series code. 
    pub name: String,
    pub data: Vec<IADBDataPoint>,
}

impl fmt::Display for IADBSeries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IADB Series ({})\n", self.name)?;
        for item in self.data.iter() {
            write!(f, "{}\n", item.to_string())?;
        }
        Ok(())
    }
}


#[derive(Serialize, Deserialize)]
/// Describes a single entry in the series.
pub struct IADBDataPoint {
    // #[serde(rename = "DATE")]
    /// Date of the data point.
    pub date: String,
    /// Value of the data point.
    pub value: f64,
}

impl fmt::Display for IADBDataPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IADB Data Point ({}): {}", self.date, self.value)
    }
}