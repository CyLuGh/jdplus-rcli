use chrono::NaiveDate;
use jdplusextendr::communication_manager::CommunicationManager;
use jdplusextendr::models::{AggregationType, Frequency};

#[tokio::main]
async fn main() {
    let manager = CommunicationManager::builder().build();

    let input = vec![(NaiveDate::from_ymd(2022, 1, 1), 1.0),(NaiveDate::from_ymd(2022, 2, 1), 1.0)];
    let result = manager.build_ts_data(input,AggregationType::Sum,Frequency::Yearly,true,true).await;

    println!("{:?}", result);
}