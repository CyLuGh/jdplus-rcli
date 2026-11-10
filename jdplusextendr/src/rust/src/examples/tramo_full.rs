use std::io::Error;
use chrono::{NaiveDate, ParseError};
use jdplusextendr::communication_manager::CommunicationManager;
use jdplusextendr::jd_client::JDClient;
use jdplusextendr::models::{AggregationType, Frequency};

use jdplusextendr::models::tramo_output::TramoOutput;
use jdplusextendr::samples::sample;

#[tokio::main]
async fn main() {
    let manager = CommunicationManager::builder().build();

    let dates = sample::SAMPLE_DATES.iter().map(|s| String::from(*s))
        .collect::<Vec<String>>();
    let values = sample::SAMPLE_VALUES.to_vec();
    let try_vec = JDClient::vectors_to_tuples(&dates, &values);

    match try_vec {
        Ok(vec) => {
            let tsdata = manager
                .build_ts_data(vec, AggregationType::Sum, Frequency::Monthly, true, true)
                .await
                .unwrap();
            let output = manager
                .get_tramo_full_process(tsdata, String::from("TRfull"))
                .await
                .unwrap();

            println!("{:?}", output);

            let outliers = output.get_outliers();
            println!("{:?}",outliers);
        }

        _ => {}
    }
}

