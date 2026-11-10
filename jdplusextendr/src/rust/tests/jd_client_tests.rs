#[cfg(test)]
mod tests {
    use extendr_api::prelude::*;
    use jdplusextendr::jd_client::JDClient;
    use jdplusextendr::samples::sample;

    #[test]
    fn test_tramo_full_process_simple_def() {
        test! {
            let data = sample::sample_datable();
            let client = JDClient::new(None);
            let result = client.tramo_full_process_simple_def(data, String::from("TRfull"));
            assert!(result.is_ok());

            let outliers_count = result.map(|to| to.get_outliers_as_data_frame().len());
            assert_ne!(outliers_count.unwrap(), 0);
        }
    }
}
