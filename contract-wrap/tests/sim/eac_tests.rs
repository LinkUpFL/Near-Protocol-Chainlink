// use crate::utils::init_without_macros as init;
// use near_sdk::serde_json::json;
// use near_sdk_sim::transaction::ExecutionStatus;
// use near_sdk_sim::DEFAULT_GAS;
// const MOCKV3AGGREGATOR_ID_2: &str = "mock_v3_aggregator_2";
// const SIMPLEREADACCESSCONTROLLER_ID: &str = "read_controller";

// // #callers_can_call_view_functions_without_explicit_access https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L95
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L96 ----- https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L136

// #[test]
// fn callers_can_call_view_functions_without_explicit_access() {
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         _aggregator_validator_mock,
//         _flags,
//         _consumer,
//         _flags_consumer,
//         _controller,
//         _controller_2,
//         flux_aggregator_test_helper_contract,
//         eddy,
//         mock_v3_aggregator,
//         mock_v3_aggregator_second,
//         read_controller,
//     ) = init();

//     root.call(
//         eac.account_id(),
//         "latest_answer",
//         &json!({}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0,
//     )
//     .assert_success();

//     root.call(
//         eac.account_id(),
//         "latest_timestamp",
//         &json!({}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0,
//     )
//     .assert_success();

//     // root.call(
//     //     eac.account_id(),
//     //     "get_answer",
//     //     &json!({}).to_string().into_bytes(),
//     //     DEFAULT_GAS,
//     //     0,
//     // ).assert_success();
//     // root.call(
//     //     eac.account_id(),
//     //     "get_timestamp",
//     //     &json!({}).to_string().into_bytes(),
//     //     DEFAULT_GAS,
//     //     0,
//     // ).assert_success();

//     root.call(
//         eac.account_id(),
//         "latest_round",
//         &json!({}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0,
//     )
//     .assert_success();

//     //     root.call(
//     //         eac.account_id(),
//     //         "get_round_data",
//     //         &json!({}).to_string().into_bytes(),
//     //         DEFAULT_GAS,
//     //         0,
//     //     ).assert_success();
// }

// // #if_the_caller_is_granted_access https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L138
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L150
// // *TODO* Look into 54321 overflow issue

// #[test]
// fn if_the_caller_is_granted_access_and_pulls_the_rate_from_the_aggregator() {
//     let (
//         root,
//         _aca,
//         _link,
//         _oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         _aggregator_validator_mock,
//         _flags,
//         _consumer,
//         _flags_consumer,
//         controller,
//         _controller_2,
//         _flux_aggregator_test_helper_contract,
//         eddy,
//         mock_v3_aggregator,
//         mock_v3_aggregator_second,
//         read_controller,
//     ) = init();

//     oracle_three
//         .call(
//             controller.account_id(),
//             "add_access",
//             &json!({"_user": eddy.account_id()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let mut expected_answer: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     // *TODO* Look into 54321 overflow issue

//     assert_eq!(expected_answer, 54320);

//     let expected_timestamp: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_timestamp != 0, true);

//     expected_answer = eddy
//         .call(
//             eac.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();
//     // *TODO* Look into 54321 overflow issue

//     assert_eq!(expected_answer, 54320);

//     let latest_round: u128 = eddy
//         .call(
//             eac.account_id(),
//             "latest_round",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     let get_answer: u128 = eddy
//         .call(
//             eac.account_id(),
//             "get_answer",
//             &json!({ "_round_id": latest_round.to_string() })
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     // *TODO* Look into 54321 overflow issue

//     assert_eq!(get_answer, 54320);
// }

// // #if_the_caller_is_granted_access https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L138
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L156
// // *TODO* Look into 54321 overflow issue

// #[test]
// fn if_the_caller_is_granted_access_and_pulls_the_timestamp_from_the_aggregator() {
//     let (
//         root,
//         _aca,
//         _link,
//         _oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         _aggregator_validator_mock,
//         _flags,
//         _consumer,
//         _flags_consumer,
//         controller,
//         _controller_2,
//         _flux_aggregator_test_helper_contract,
//         eddy,
//         mock_v3_aggregator,
//         mock_v3_aggregator_second,
//         read_controller,
//     ) = init();

//     oracle_three
//         .call(
//             controller.account_id(),
//             "add_access",
//             &json!({"_user": eddy.account_id()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let mut expected_answer: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_answer, 54320);

//     let mut expected_timestamp: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_timestamp != 0, true);

//     expected_answer = eddy
//         .call(
//             eac.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_answer, 54320);

//     expected_timestamp = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     let latest_timestamp: u128 = eddy
//         .call(
//             eac.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(latest_timestamp == expected_timestamp, true);

//     let latest_round: u128 = eddy
//         .call(
//             eac.account_id(),
//             "latest_round",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     expected_timestamp = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     let get_timestamp: u128 = eddy
//         .call(
//             eac.account_id(),
//             "get_timestamp",
//             &json!({ "_round_id": latest_round.to_string() })
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_timestamp == get_timestamp, true);
// }

// // #if_the_caller_is_granted_access https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L138
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L168
// // *TODO* Look into 54321 overflow issue

// #[test]
// fn if_the_caller_is_granted_access_and_get_round_data_works() {
//     let (
//         root,
//         _aca,
//         _link,
//         _oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         _aggregator_validator_mock,
//         _flags,
//         _consumer,
//         _flags_consumer,
//         controller,
//         _controller_2,
//         _flux_aggregator_test_helper_contract,
//         eddy,
//         mock_v3_aggregator,
//         mock_v3_aggregator_second,
//         read_controller,
//     ) = init();

//     oracle_three
//         .call(
//             controller.account_id(),
//             "add_access",
//             &json!({"_user": eddy.account_id()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let mut expected_answer: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_answer, 54320);

//     let expected_timestamp: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_timestamp != 0, true);

//     expected_answer = eddy
//         .call(
//             eac.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_answer, 54320);

//     let proxy_round_id: u128 = eddy
//         .call(
//             eac.account_id(),
//             "latest_round",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     let round: (u128, u128, u128, u128, u128) = eddy
//         .call(
//             eac.account_id(),
//             "get_round_data",
//             &json!({ "_round_id": proxy_round_id.to_string() })
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(proxy_round_id, round.0);
//     assert_eq!(54320, round.1);
//     assert_eq!(677, round.2);
//     assert_eq!(678, round.3);
// }

// // #if_the_caller_is_granted_access https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L138
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L185
// // *TODO* Look into 54321 overflow issue

// #[test]
// fn if_the_caller_is_granted_access_and_an_aggregator_has_been_proposed_and_proposed_get_round_data_works(
// ) {
//     let (
//         root,
//         _aca,
//         _link,
//         _oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         _aggregator_validator_mock,
//         _flags,
//         _consumer,
//         _flags_consumer,
//         controller,
//         _controller_2,
//         _flux_aggregator_test_helper_contract,
//         eddy,
//         mock_v3_aggregator,
//         mock_v3_aggregator_second,
//         read_controller,
//     ) = init();

//     oracle_three
//         .call(
//             controller.account_id(),
//             "add_access",
//             &json!({"_user": eddy.account_id()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let mut expected_answer: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_answer, 54320);

//     let expected_timestamp: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_timestamp != 0, true);

//     expected_answer = eddy
//         .call(
//             eac.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_answer, 54320);

//     mock_v3_aggregator_second
//         .call(
//             MOCKV3AGGREGATOR_ID_2.into(),
//             "new",
//             &json!({
//                 "_decimals": 18.to_string(),
//                 "_initial_answer": 54320.to_string(),
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS / 2,
//             0, // attached deposit
//         )
//         .assert_success();

//     eac.call(
//         eac.account_id(),
//         "propose_aggregator",
//         &json!({ "_aggregator": MOCKV3AGGREGATOR_ID_2 })
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     )
//     .assert_success();

//     let latest_round: u128 = eddy
//         .call(
//             mock_v3_aggregator_second.account_id(),
//             "latest_round",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     let round: (u128, u128, u128, u128, u128) = eddy
//         .call(
//             eac.account_id(),
//             "proposed_get_round_data",
//             &json!({ "_round_id": latest_round.to_string() })
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(latest_round, round.0);
//     assert_eq!(54319, round.1);
// }

// // #if_the_caller_is_granted_access https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L138
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L192
// // *TODO* Look into 54321 overflow issue

// #[test]
// fn if_the_caller_is_granted_access_and_an_aggregator_has_been_proposed_and_proposed_latest_round_data_works(
// ) {
//     let (
//         root,
//         _aca,
//         _link,
//         _oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         _aggregator_validator_mock,
//         _flags,
//         _consumer,
//         _flags_consumer,
//         controller,
//         _controller_2,
//         _flux_aggregator_test_helper_contract,
//         eddy,
//         mock_v3_aggregator,
//         mock_v3_aggregator_second,
//         read_controller,
//     ) = init();

//     oracle_three
//         .call(
//             controller.account_id(),
//             "add_access",
//             &json!({"_user": eddy.account_id()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let mut expected_answer: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_answer, 54320);

//     let expected_timestamp: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_timestamp != 0, true);

//     expected_answer = eddy
//         .call(
//             eac.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_answer, 54320);

//     mock_v3_aggregator_second
//         .call(
//             MOCKV3AGGREGATOR_ID_2.into(),
//             "new",
//             &json!({
//                 "_decimals": 18.to_string(),
//                 "_initial_answer": 54320.to_string(),
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS / 2,
//             0, // attached deposit
//         )
//         .assert_success();

//     eac.call(
//         eac.account_id(),
//         "propose_aggregator",
//         &json!({ "_aggregator": MOCKV3AGGREGATOR_ID_2 })
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     )
//     .assert_success();

//     let latest_round: u128 = eddy
//         .call(
//             mock_v3_aggregator_second.account_id(),
//             "latest_round",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     let round: (u128, u128, u128, u128, u128) = eddy
//         .call(
//             eac.account_id(),
//             "proposed_latest_round_data",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(latest_round, round.0);
//     assert_eq!(54319, round.1);
// }

// // #if_the_caller_is_granted_access https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L138
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L201
// // *TODO* Look into 54321 overflow issue

// #[test]
// fn if_the_caller_is_granted_access_without_a_proposed_aggregator_and_proposed_get_round_data_reverts(
// ) {
//     let (
//         root,
//         _aca,
//         _link,
//         _oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         _aggregator_validator_mock,
//         _flags,
//         _consumer,
//         _flags_consumer,
//         controller,
//         _controller_2,
//         _flux_aggregator_test_helper_contract,
//         eddy,
//         mock_v3_aggregator,
//         mock_v3_aggregator_second,
//         read_controller,
//     ) = init();

//     oracle_three
//         .call(
//             controller.account_id(),
//             "add_access",
//             &json!({"_user": eddy.account_id()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let mut expected_answer: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_answer, 54320);

//     let expected_timestamp: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_timestamp != 0, true);

//     expected_answer = eddy
//         .call(
//             eac.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_answer, 54320);

//     let expected_no_proposed_aggregator_present = eddy.call(
//         eac.account_id(),
//         "proposed_get_round_data",
//         &json!({ "_round_id": 1.to_string() })
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_no_proposed_aggregator_present
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error
//             .to_string()
//             .contains("No proposed aggregator present"));
//     } else {
//         unreachable!();
//     }
// }

// // #if_the_caller_is_granted_access https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L138
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L207
// // *TODO* Look into 54321 overflow issue

// #[test]
// fn if_the_caller_is_granted_access_without_a_proposed_aggregator_and_proposed_latest_round_data_reverts(
// ) {
//     let (
//         root,
//         _aca,
//         _link,
//         _oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         _aggregator_validator_mock,
//         _flags,
//         _consumer,
//         _flags_consumer,
//         controller,
//         _controller_2,
//         _flux_aggregator_test_helper_contract,
//         eddy,
//         mock_v3_aggregator,
//         mock_v3_aggregator_second,
//         read_controller,
//     ) = init();

//     oracle_three
//         .call(
//             controller.account_id(),
//             "add_access",
//             &json!({"_user": eddy.account_id()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let mut expected_answer: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_answer, 54320);

//     let expected_timestamp: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_timestamp != 0, true);

//     expected_answer = eddy
//         .call(
//             eac.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_answer, 54320);

//     let expected_no_proposed_aggregator_present = eddy.call(
//         eac.account_id(),
//         "proposed_latest_round_data",
//         &json!({}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_no_proposed_aggregator_present
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error
//             .to_string()
//             .contains("No proposed aggregator present"));
//     } else {
//         unreachable!();
//     }
// }

// // #if_the_caller_is_granted_access https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L138
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L219
// // *TODO* Look into 54321 overflow issue

// #[test]
// fn if_the_caller_is_granted_access_when_read_from_a_contract_that_is_not_permissioned_and_does_not_allow_reading(
// ) {
//     let (
//         root,
//         _aca,
//         _link,
//         _oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         _aggregator_validator_mock,
//         _flags,
//         _consumer,
//         _flags_consumer,
//         controller,
//         _controller_2,
//         flux_aggregator_test_helper_contract,
//         eddy,
//         mock_v3_aggregator,
//         mock_v3_aggregator_second,
//         read_controller,
//     ) = init();

//     oracle_three
//         .call(
//             controller.account_id(),
//             "add_access",
//             &json!({"_user": eddy.account_id()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let mut expected_answer: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_answer, 54320);

//     let expected_timestamp: u128 = root
//         .call(
//             mock_v3_aggregator.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_timestamp != 0, true);

//     expected_answer = eddy
//         .call(
//             eac.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(expected_answer, 54320);

//     let expected_no_access = flux_aggregator_test_helper_contract.call(
//         eac.account_id(),
//         "read_latest_round_data",
//         &json!({}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_no_access
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error.to_string().contains("No access"));
//     } else {
//         unreachable!();
//     }
// }

// // #set_controller https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L228
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L238
// // *TODO* Look into 54321 overflow issue

// #[test]
// fn set_controller_when_called_by_a_stranger_and_reverts() {
//     let (
//         root,
//         _aca,
//         _link,
//         _oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         _aggregator_validator_mock,
//         _flags,
//         _consumer,
//         _flags_consumer,
//         _controller,
//         _controller_2,
//         _flux_aggregator_test_helper_contract,
//         eddy,
//         _mock_v3_aggregator,
//         _mock_v3_aggregator_second,
//         read_controller,
//     ) = init();

//     read_controller
//         .call(
//             SIMPLEREADACCESSCONTROLLER_ID.into(),
//             "new",
//             &json!({
//                 "owner_id": root.account_id()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS / 2,
//             0, // attached deposit
//         )
//         .assert_success();

//     let expected_only_callable_by_owner = eddy.call(
//         eac.account_id(),
//         "set_controller",
//         &json!({"_access_controller": read_controller.account_id()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_only_callable_by_owner
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error
//             .to_string()
//             .contains("Only callable by owner"));
//     } else {
//         unreachable!();
//     }
// }

// // #set_controller https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L228
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L248
// // *TODO* Look into 54321 overflow issue

// #[test]
// fn set_controller_when_called_by_the_owner_updates_the_controller_contract() {
//     let (
//         root,
//         _aca,
//         _link,
//         _oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         _aggregator_validator_mock,
//         _flags,
//         _consumer,
//         _flags_consumer,
//         _controller,
//         _controller_2,
//         _flux_aggregator_test_helper_contract,
//         eddy,
//         _mock_v3_aggregator,
//         _mock_v3_aggregator_second,
//         read_controller,
//     ) = init();

//     read_controller
//         .call(
//             SIMPLEREADACCESSCONTROLLER_ID.into(),
//             "new",
//             &json!({
//                 "owner_id": root.account_id()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS / 2,
//             0, // attached deposit
//         )
//         .assert_success();

//     eac.call(
//         eac.account_id(),
//         "set_controller",
//         &json!({"_access_controller": read_controller.account_id()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     )
//     .assert_success();

//     let expected_access_controller: String = root
//         .view(
//             eac.account_id(),
//             "access_controller",
//             &json!({}).to_string().into_bytes(),
//         )
//         .unwrap_json();

//     assert_eq!(
//         expected_access_controller == read_controller.account_id(),
//         true
//     )
// }

// // #set_controller https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L254
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/EACAggregatorProxy.test.ts#L259

// #[test]
// fn set_controller_when_set_to_the_zero_address_and_allows_anyone_to_read() {
//     let (
//         root,
//         _aca,
//         _link,
//         _oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         _aggregator_validator_mock,
//         _flags,
//         _consumer,
//         _flags_consumer,
//         _controller,
//         _controller_2,
//         flux_aggregator_test_helper_contract,
//         eddy,
//         _mock_v3_aggregator,
//         _mock_v3_aggregator_second,
//         read_controller,
//     ) = init();

//     let expected_no_access = eddy.call(
//         flux_aggregator_test_helper_contract.account_id(),
//         "read_latest_round_data",
//         &json!({"_aggregator": eac.account_id()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_no_access
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error.to_string().contains("No access"));
//     } else {
//         unreachable!();
//     }

//     eac.call(
//         eac.account_id(),
//         "set_controller",
//         &json!({"_access_controller": "".to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     )
//     .assert_success();

//     eddy.call(
//         flux_aggregator_test_helper_contract.account_id(),
//         "read_latest_round_data",
//         &json!({"_aggregator": eac.account_id()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     )
//     .assert_success();
// }
