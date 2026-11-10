use crate::communication_manager::CommunicationManager;
use chrono::NaiveDate;
use extendr_api::prelude::*;

#[extendr]
#[derive(Debug)]
/// Client to access JD+ grpc API
pub struct JDClient {
    /// Handler for connection and requests to API
    pub manager: CommunicationManager,
}

impl JDClient {
    pub fn vectors_to_tuples(
        dates: &[String],
        values: &[f64],
    ) -> Result<Vec<(NaiveDate, f64)>, chrono::ParseError> {
        dates
            .iter()
            .zip(values.iter())
            .map(|(d, v)| NaiveDate::parse_from_str(d, "%Y-%m-%d").map(|date| (date, *v)))
            .collect::<Result<Vec<(NaiveDate, f64)>, chrono::ParseError>>()
    }

    pub fn datatable_to_tuples(data: List) -> Result<Vec<(NaiveDate, f64)>, Error> {
        let date_col = data
            .dollar("date")
            .map_err(|e| Error::Other(format!("Failed to retrieve column 'date': {e:?}")))?;
        let dates = date_col
            .as_string_vector()
            .ok_or(Error::Other("Column 'date' is not a string".to_string()))?;

        let values_col = data
            .dollar("value")
            .map_err(|e| Error::Other(format!("Failed to retrieve column 'value': {e:?}")))?;
        let values = values_col
            .as_real_vector()
            .ok_or(Error::Other("Column 'value' is not a real".to_string()))?;

        let tuples = Self::vectors_to_tuples(&dates, &values);

        match tuples {
            Ok(tuples) => Ok(tuples),
            Err(e) => Err(Error::Other(format!("{:?}", e))),
        }
    }
}
