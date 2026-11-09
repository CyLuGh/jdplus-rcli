pub mod build_ts;

use jdplusextendr::communication_manager::CommunicationManager;

#[tokio::main]
async fn main() {
    let manager = CommunicationManager::builder().build();
    println!("{}", manager.get_version().await);
}