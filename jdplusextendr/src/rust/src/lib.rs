pub mod communication_manager;
pub mod jd_client;
pub mod models;
pub mod samples;

use crate::communication_manager::CommunicationManager;
use crate::jd_client::JDClient;
use crate::models::{AggregationType, Frequency, TsMoniker};
use extendr_api::prelude::*;

use crate::models::tramo_output::TramoOutput;
use crate::models::tsdata::TsData;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

mod jdplus_main_ws_v1 {
    tonic::include_proto!("jdplus.main.ws.v1");
}

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

/// Return jdplus version to R.
/// @export
#[extendr]
fn get_version() -> String {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let manager = CommunicationManager::builder().build();
        manager.get_version().await
    })
}

#[extendr]
impl TsMoniker {
    pub fn new(source: String, id: String) -> Self {
        Self { source, id }
    }

    pub fn get_moniker(&self) -> List {
        list!(source = self.source.clone(), id = self.id.clone(),)
    }
}

#[extendr]
impl TsData {
    pub fn get_data_frame(&self) -> Robj {
        let map = self.get_date_values();
        let mut entries: Vec<_> = map.into_iter().collect();
        entries.sort_by_key(|(date, _)| *date);

        let dates: Vec<String> = entries.iter().map(|(d, _)| d.to_string()).collect();
        let values: Vec<f64> = entries.iter().map(|(_, v)| *v).collect();

        data_frame!(date = dates, value = values)
    }
}

#[extendr]
impl JDClient {
    /// Creates new client, if url is NULL or empty, it uses default value `http://localhost:4566`.
    pub fn new(#[extendr(default = "NULL")] url: Option<String>) -> Self {
        let mut builder = CommunicationManager::builder();
        let address = url.unwrap_or_default();
        if !address.is_empty() {
            builder = builder.url(address);
        }
        Self {
            manager: builder.build(),
        }
    }

    /// Returns API version
    pub fn get_version(&self) -> String {
        RUNTIME.block_on(self.manager.get_version())
    }

    // client <- JDClient$new()
    // df <- data.frame( date = c("2024-01-01","2024-02-01","2024-03-01"), value = c(10,20,13.5))
    // res <- client$build_ts_data(df, AggregationType$Sum)$get_data_frame()
    /// Build TS Data from raw input
    ///
    /// Arguments
    ///
    /// * `aggregation_type` defines how to aggregate data if there is more than one occurrence for the same period.
    /// | Value | Meaning |
    /// |:--:|:--|
    /// |0|None|
    /// |1|Sum|
    /// |2|Average|
    /// |3|First|
    /// |4|Last|
    /// |5|Max|
    /// |6|Min|
    /// * `frequency` defines the frequency of build series
    /// | Value | Meaning |
    /// |:--:|:--|
    /// |0|Undefined|
    /// |1|Yearly|
    /// |2|Half yearly|
    /// |3|Quadrimonthly|
    /// |4|Quarterly|
    /// |6|Bimonthly|
    /// |12|Monthly|
    ///
    /// Returns
    /// A TsData
    pub fn build_ts_data(
        &self,
        data: List,
        #[extendr(default = "1")] aggregation_type: i32,
        #[extendr(default = "1")] frequency: i32,
        #[extendr(default = "TRUE")] allow_partial_aggregation: bool,
        #[extendr(default = "TRUE")] include_missing_values: bool,
    ) -> Result<TsData, Error> {
        let try_vec = JDClient::datatable_to_tuples(data);

        match try_vec {
            Ok(vec) => Ok(RUNTIME.block_on(self.manager.build_ts_data(
                vec,
                AggregationType::from(aggregation_type),
                Frequency::from(frequency),
                allow_partial_aggregation,
                include_missing_values,
            ))?),
            Err(e) => Err(Error::Other(format!("Couldn't build ts-data: {}", e))),
        }
    }

    pub fn tramo_full_process_simple_def(
        &self,
        data: List,
        #[extendr(default = "'TRfull'")] spec: String,
    ) -> Result<TramoOutput, Error> {
        let series = self.build_ts_data(data, 0, 0, true, true);

        match series {
            Ok(series) => Ok(RUNTIME.block_on(self.manager.get_tramo_full_process(series, spec))?),
            Err(e) => Err(Error::Other(format!(
                "Ts data wasn't properly built: {}",
                e
            ))),
        }
    }
}

#[extendr]
impl TramoOutput {
    pub fn get_outliers_as_data_frame(&self) -> List {
        let outliers = self.get_outliers();

        let mut entries: Vec<_> = outliers.into_iter().collect();
        entries.sort_by_key(|o| o.period);

        let dates: Vec<String> = entries.iter().map(|o| o.period.to_string()).collect();
        let values: Vec<f64> = entries.iter().map(|o| o.value).collect();
        let codes: Vec<String> = entries.iter().map(|o| o.code.clone()).collect();
        let coefficients: Vec<f64> = entries.iter().map(|o| o.coefficient.value).collect();

        List::from_pairs([
            ("date", r!(dates)),
            ("values", r!(values)),
            ("codes", r!(codes)),
            ("coefficients", r!(coefficients)),
        ])
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod jdplusextendr;

    fn hello_world;
    fn get_version;

    impl TsMoniker;
    impl TsData;
    impl JDClient;
    impl TramoOutput;
}
