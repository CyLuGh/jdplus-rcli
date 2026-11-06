use crate::communication_manager::CommunicationManager;
use extendr_api::prelude::*;

#[extendr]
/// Client to access JD+ grpc API
pub struct JDClient {
    /// Handler for connection and requests to API
    pub manager: CommunicationManager
}
