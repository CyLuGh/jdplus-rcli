use chrono::NaiveDate;
use crate::communication_manager::CommunicationManager;
use extendr_api::prelude::*;

#[extendr]
#[derive(Debug)]
/// Client to access JD+ grpc API
pub struct JDClient {
    /// Handler for connection and requests to API
    pub manager: CommunicationManager
}

impl JDClient {
    pub fn datatable_to_tuples(data: List) -> Vec<(NaiveDate,f64)> {
        let dates: Vec<String> = data.dollar("date").unwrap().as_string_vector().unwrap();
        let values: Vec<f64> = data.dollar("value").unwrap().as_real_vector().unwrap();
        let vec: Vec<(NaiveDate, f64)> = dates
            .iter()
            .zip(values.iter())
            .map(|(d, v)| (NaiveDate::parse_from_str(d, "%Y-%m-%d").unwrap(), *v))
            .collect();

        vec
    }
}
