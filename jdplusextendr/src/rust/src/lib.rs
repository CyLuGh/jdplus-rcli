use crate::communication_manager::CommunicationManager;
use crate::jd_client::JDClient;
use extendr_api::prelude::*;

mod jdplus_main_ws_v1 {
    tonic::include_proto!("jdplus.main.ws.v1");
}

pub mod communication_manager;
pub mod models;
pub mod jd_client;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

/// Return jplus version to R.
/// @export
#[extendr]
fn get_version() -> String {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let manager = CommunicationManager::builder().build();
        manager.get_version().await
    })
}



// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod jdplusextendr;
    fn hello_world;
    fn get_version;
    impl models::TsMoniker;
}
