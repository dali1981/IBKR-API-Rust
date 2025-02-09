use twsapi::core::errors::IBKRApiLibError;

// //! Binary for manually testing crate
//
// use log::*;
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::Duration;
// use twsapi::core::client::EClient;
// use twsapi::core::contract::Contract;
// // use twsapi::examples::test_helpers::TestWrapper;
//
// /// Example of using client and wrapper.
// /// Requires a running instance of TWS or IB Gateway connected to the port in main.
// /// Upon connecting, TWS will send the next valid order ID which will cause the wrapper callback method
// /// next_valid_id to be called, which will start sending tests requests to TWS (see the
// /// start_requests function inn TestWrapper which is called by next_valid_id
// //==================================================================================================
pub fn main() -> Result<(), IBKRApiLibError> {
//     match log4rs::init_file("./log_config.yml", Default::default()) {
//         Ok(_) => (),
//         Err(_) => {
//             return Err(IBKRApiLibError::ApiError(TwsApiReportableError::new(
//                 -1,
//                 "-1".to_string(),
//                 "Failed to create logger!!".to_string(),
//             )))
//         }
//     };
//
//     let wrapper = Arc::new(Mutex::new(TestWrapper::new()));
//     let app = Arc::new(Mutex::new(EClient::new(wrapper.clone())));
//
//     info!("getting connection...");
//
//
//
//     wrapper.lock().expect("Wrapper mutex was poisoned").client = Option::from(app.clone());
//
//     //use port 7497 for TWS or 4002 for IB Gateway, depending on the port you have set
//     app.lock()
//         .expect("EClient mutex was poisoned")
//         .connect("127.0.0.1", 4002, 0)?;
//
//     thread::sleep(Duration::new(2, 0));
//
//
//     let mut contract = Contract::default();
//     contract.symbol = "NOKIA".to_string();
//     contract.sec_type = "STK".to_string();
//     contract.currency = "EUR".to_string();
//     contract.exchange = "SMART".to_string();
//     contract.primary_exchange = "HEX".to_string();
//
//     {
//         let mut app_ = app.lock().unwrap();
//         let result = &app_.req_historical_data(
//             123, &contract,
//             "20230201-00:00:00",
//             "1 M",
//             "1 day",
//             "TRADE",
//             1,
//             1,
//             true,
//             Vec::default()
//         );
//     }
//
//     thread::sleep(Duration::new(2, 0));
//
    Ok(())
}
