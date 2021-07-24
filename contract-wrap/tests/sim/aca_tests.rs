// use crate::utils::init_without_macros as init;
// use near_sdk::json_types::{U128, U64};
// use near_sdk::serde_json::json;
// use near_sdk_sim::transaction::ExecutionStatus;
// use near_sdk_sim::DEFAULT_GAS;

// // #get_answer https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L143
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L160

// #[test]
// fn get_answer_when_read_by_a_contract_without_explicit_access_and_reverts() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//         eddy
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     let expected_no_access = eddy.call(
//         flux_aggregator_test_helper_contract.account_id(),
//         "read_get_answer",
//         &json!({"_aggregator": aca.account_id(), "_round_id": 0.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
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
// // #get_answer https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L143
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L169

// #[test]
// fn get_answer_when_read_by_a_contract_with_access_and_succeeds() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//         eddy
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": flux_aggregator_test_helper_contract.account_id()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     eddy
//         .call(
//             flux_aggregator_test_helper_contract.account_id(),
//             "read_get_answer",
//             &json!({"_aggregator": aca.account_id(), "_round_id": 0.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();
// }

// // #get_answer https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L143
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L178

// #[test]
// fn get_answer_when_read_by_a_regular_account_without_explicit_access_and_succeeds() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     let round: u64 = eddy
//         .call(
//             aca.account_id(),
//             "latest_round",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     eddy.call(
//         aca.account_id(),
//         "get_answer",
//         &json!({"_round_id": round.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();
// }

// // #get_answer https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L143
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L185

// #[test]
// fn get_answer_when_read_by_a_regular_account_with_access_and_succeeds() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": eddy.account_id()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let round: u64 = eddy
//         .call(
//             aca.account_id(),
//             "latest_round",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     eddy.call(
//         aca.account_id(),
//         "get_answer",
//         &json!({"_round_id": round.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();
// }

// // #get_timestamp https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L196
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L213

// #[test]
// fn get_timestamp_when_read_by_a_contract_without_explicit_access_and_reverts() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//         eddy
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     let expected_no_access = eddy.call(
//         flux_aggregator_test_helper_contract.account_id(),
//         "read_get_timestamp",
//         &json!({"_aggregator": aca.account_id(), "_round_id": 0.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
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

// // #get_timestamp https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L196
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L222

// #[test]
// fn get_timestamp_when_read_by_a_contract_with_access_and_succeeds() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//         eddy
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": flux_aggregator_test_helper_contract.account_id()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     eddy
//         .call(
//             flux_aggregator_test_helper_contract.account_id(),
//             "read_get_timestamp",
//             &json!({"_aggregator": aca.account_id(), "_round_id": 0.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();
// }

// // #get_timestamp https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L196
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L231

// #[test]
// fn get_timestamp_when_read_by_a_regular_account_without_explicit_access_and_succeeds() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     let round: u64 = eddy
//         .call(
//             aca.account_id(),
//             "latest_round",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     let current_timestamp: u64 = eddy
//         .call(
//             aca.account_id(),
//             "get_timestamp",
//             &json!({"_round_id": round.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(current_timestamp > 0, true)
// }

// // #get_timestamp https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L196
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L231

// #[test]
// fn get_timestamp_when_read_by_a_regular_account_with_access_and_succeeds() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": eddy.account_id()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let round: u64 = eddy
//         .call(
//             aca.account_id(),
//             "latest_round",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     let current_timestamp: u64 = eddy
//         .call(
//             aca.account_id(),
//             "get_timestamp",
//             &json!({"_round_id": round.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(current_timestamp > 0, true)
// }

// // #latest_answer https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L255
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L272

// #[test]
// fn latest_answer_when_read_by_a_contract_without_explicit_access_and_reverts() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     let expected_no_access = eddy.call(
//         flux_aggregator_test_helper_contract.account_id(),
//         "read_latest_answer",
//         &json!({"_aggregator": aca.account_id()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
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

// // #latest_answer https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L255
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L281

// #[test]
// fn latest_answer_when_read_by_a_contract_with_access_and_succeeds() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": flux_aggregator_test_helper_contract.account_id()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     eddy
//         .call(
//             flux_aggregator_test_helper_contract.account_id(),
//             "read_latest_answer",
//             &json!({"_aggregator": aca.account_id()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();
// }

// // #latest_answer https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L255
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L290

// #[test]
// fn latest_answer_when_read_by_a_regular_account_without_explicit_access_and_succeeds() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     eddy.call(
//         aca.account_id(),
//         "latest_answer",
//         &json!({}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();
// }

// // // #latest_answer https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L255
// // // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L296

// #[test]
// fn latest_answer_when_read_by_a_regular_account_with_access_and_succeeds() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": eddy.account_id()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     eddy.call(
//         aca.account_id(),
//         "latest_answer",
//         &json!({}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();
// }

// // #latest_timestamp https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L306
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L323

// #[test]
// fn latest_timestamp_when_read_by_a_contract_without_explicit_access_and_reverts() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     let expected_no_access = eddy.call(
//         flux_aggregator_test_helper_contract.account_id(),
//         "read_latest_timestamp",
//         &json!({"_aggregator": aca.account_id()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     println!("{:?}", expected_no_access.promise_results());

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

// // #latest_timestamp https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L306
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L332

// #[test]
// fn latest_timestamp_when_read_by_a_contract_with_access_and_succeeds() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": flux_aggregator_test_helper_contract.account_id()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     eddy.call(
//         flux_aggregator_test_helper_contract.account_id(),
//         "read_latest_timestamp",
//         &json!({"_aggregator": aca.account_id()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

// }

// // #latest_timestamp https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L306
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L341

// #[test]
// fn latest_timestamp_when_read_by_a_regular_account_without_explicit_access_and_succeeds() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     let current_timestamp: u64 = eddy
//         .call(
//             aca.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(current_timestamp > 0, true);
// }

// // #latest_timestamp https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L306
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L350

// #[test]
// fn latest_timestamp_when_read_by_a_regular_account_with_access_and_succeeds() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         _test_helper,
//         _eac,
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
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": eddy.account_id()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let current_timestamp: u64 = eddy
//         .call(
//             aca.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(current_timestamp > 0, true);
// }
