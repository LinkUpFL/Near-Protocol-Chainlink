use near_sdk::serde_json::json;
use near_sdk::AccountId;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::DEFAULT_GAS;

use crate::utils::init_without_macros as init;

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/AccessControlledAggregator.test.ts
// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L180
// Suite of simulation tests matching TypeScript tests for AccessControlledAggregator and FluxAggregator as closely as possible.

// #[test]

// // *TODO* Create FluxAggregator test factory contract here
// // *TODO* Assert success from regular account with/without access
// fn access_control_tests() {
//     let deposit: u64 = 100;
//     let answer: u128 = 100;
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         test_helper,
//         _ea,
//         _eac_without_access_controller,
//     ) = init();

//     // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/AccessControlledAggregator.test.ts#L144

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

//     // Unauthorized Calls
//     // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/AccessControlledAggregator.test.ts#L158

//     let mut get_answer_unauthorized = test_helper.call(
//         aca.account_id(),
//         "get_answer",
//         &json!({"_round_id": next_round.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );
//     assert_eq!(get_answer_unauthorized.promise_errors().len(), 1);

//     if let ExecutionStatus::Failure(execution_error) = &get_answer_unauthorized
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

//     // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/AccessControlledAggregator.test.ts#L196

//     get_answer_unauthorized = test_helper.call(
//         aca.account_id(),
//         "get_timestamp",
//         &json!({"_round_id": next_round.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );
//     assert_eq!(get_answer_unauthorized.promise_errors().len(), 1);

//     if let ExecutionStatus::Failure(execution_error) = &get_answer_unauthorized
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

//     // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/AccessControlledAggregator.test.ts#L255

//     get_answer_unauthorized = test_helper.call(
//         aca.account_id(),
//         "latest_answer",
//         &json!({"_round_id": next_round.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );
//     assert_eq!(get_answer_unauthorized.promise_errors().len(), 1);

//     if let ExecutionStatus::Failure(execution_error) = &get_answer_unauthorized
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

//     // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/AccessControlledAggregator.test.ts#L306

//     get_answer_unauthorized = test_helper.call(
//         aca.account_id(),
//         "latest_timestamp",
//         &json!({"_round_id": next_round.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );
//     assert_eq!(get_answer_unauthorized.promise_errors().len(), 1);

//     if let ExecutionStatus::Failure(execution_error) = &get_answer_unauthorized
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

//     // Authorized Contract Account Calls

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     // Authorized call from test_helper for get_answer
//     test_helper
//         .call(
//             aca.account_id(),
//             "get_answer",
//             &json!({"_round_id": next_round.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();
//     // Authorized call from test_helper for get_timestamp
//     test_helper
//         .call(
//             aca.account_id(),
//             "get_timestamp",
//             &json!({"_round_id": next_round.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();
//     // Authorized call from test_helper for latest_answer
//     test_helper
//         .call(
//             aca.account_id(),
//             "latest_answer",
//             &json!({"_round_id": next_round.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();
//     // Authorized call from test_helper for latest_timestamp
//     test_helper
//         .call(
//             aca.account_id(),
//             "latest_timestamp",
//             &json!({"_round_id": next_round.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L251
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L298, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L298
// // *TODO* https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L315 -> emit event log with with submission, round, oracle address (need to add to smart contract)
// #[test]
// fn updates_the_allocated_and_available_funds_counters_and_emits_a_log_event_announcing_submission_details(
// ) {
//     let payment_amount: u64 = 3;
//     let deposit: u64 = 100;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         _test_helper,
//         _eac,
//         _eac_without_access_controller,
//     ) = init();

//     // number of oracles
//     let min_max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let mut allocated_funds: u64 = root
//         .view(
//             aca.account_id(),
//             "allocated_funds",
//             &json!({}).to_string().into_bytes(),
//         )
//         .unwrap_json();

//     assert_eq!(
//         0, allocated_funds,
//         "updates the allocated and available funds counters"
//     );

//     let mut tx = oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     let mut receipt = tx.promise_results();

//     allocated_funds = root
//         .view(
//             aca.account_id(),
//             "allocated_funds",
//             &json!({}).to_string().into_bytes(),
//         )
//         .unwrap_json();

//     let available_funds: u64 = root
//         .view(
//             aca.account_id(),
//             "available_funds",
//             &json!({}).to_string().into_bytes(),
//         )
//         .unwrap_json();

//     assert_eq!(payment_amount, allocated_funds);

//     let expected_available: u64 = deposit - payment_amount;

//     assert_eq!(expected_available, available_funds);

//     let logged: u64 = receipt.remove(1).unwrap().outcome().logs[0]
//         .parse()
//         .unwrap();

//     assert_eq!(expected_available, logged);

//     // *TODO* https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L315 -> emit event log with with submission, round, oracle address (need to add to smart contract)

//     // tx = oracle_two.call(
//     //     aca.account_id(),
//     //     "submit",
//     //     &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//     //         .to_string()
//     //         .into_bytes(),
//     //     DEFAULT_GAS,
//     //     0, // deposit
//     // );
//     // receipt = tx.promise_results();
//     // println!("{:?}", receipt);
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L251
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L327, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L328

// #[test]
// fn when_the_minimum_oracles_have_not_reported() {
//     let payment_amount: u128 = 3;
//     let deposit: u64 = 100;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//     ) = init();
//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let min_max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let withdrawable_payment: u128 = root
//         .view(
//             aca.account_id(),
//             "withdrawable_payment",
//             &json!({
//                 "_oracle": oracle_one.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//         )
//         .unwrap_json();

//     assert_eq!(0, withdrawable_payment);

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

//     let withdrawable_payment_oracle_one: u128 = root
//         .view(
//             aca.account_id(),
//             "withdrawable_payment",
//             &json!({
//                 "_oracle": oracle_one.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//         )
//         .unwrap_json();

//     assert_eq!(payment_amount, withdrawable_payment_oracle_one);

//     let withdrawable_payment_oracle_two: u128 = root
//         .view(
//             aca.account_id(),
//             "withdrawable_payment",
//             &json!({
//                 "_oracle": oracle_two.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//         )
//         .unwrap_json();

//     assert_eq!(0, withdrawable_payment_oracle_two);

//     let withdrawable_payment_oracle_three: u128 = root
//         .view(
//             aca.account_id(),
//             "withdrawable_payment",
//             &json!({
//                 "_oracle": oracle_three.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//         )
//         .unwrap_json();

//     assert_eq!(0, withdrawable_payment_oracle_three);
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L251
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L327, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L358

// #[test]
// fn it_does_not_update_the_answer() {
//     let payment_amount: u128 = 3;
//     let deposit: u64 = 100;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//     ) = init();
//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let min_max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let not_updated: u128 = test_helper
//         .call(
//             aca.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(0, not_updated);

//     oracle_two
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

//     oracle_three
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

//     let still_not_updated: u128 = test_helper
//         .call(
//             aca.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(0, still_not_updated);
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L369
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L370, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L375
// // *TODO* Look into the issue here, the prev_round being 0 makes the code problematic. Line 981 in AccessControlledAggregator/lib.rs

// #[test]
// fn when_an_oracle_prematurely_bumps_the_round() {
//     let payment_amount: u128 = 3;
//     let deposit: u64 = 100;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let timeout: u64 = 1800;
//     let next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//     ) = init();
//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let min: u64 = 2;
//     let max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

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

//     let expected_previous_round_not_supersedable = oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": (next_round + 1).to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     // Note: https://github.com/smartcontractkit/chainlink/blob/95dd250a296042c81b7aafa887d8935c87cb1190/evm-contracts/test/v0.6/FluxAggregator.test.ts#L371
//     // Look into the issue here, the prev_round being 0 makes the code problematic. Line 981 in AccessControlledAggregator/lib.rs
//     if let ExecutionStatus::Failure(execution_error) = &expected_previous_round_not_supersedable
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         println!("{:?}", execution_error.to_string());
//         assert!(execution_error
//             .to_string()
//             .contains("previous round not supersedable"));
//     } else {
//         unreachable!();
//     }
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L383
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L389, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L400, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L413, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L426, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L438
// #[test]
// fn updates_the_answer_with_the_median() {
//     let deposit: u64 = 100;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let min: u64 = 2;
//     let max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
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

//     let expected_latest_answer: u128 = test_helper
//         .call(
//             aca.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(0, expected_latest_answer);

//     oracle_two
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": 99.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     let expected_latest_answer_first: u128 = test_helper
//         .call(
//             aca.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(99, expected_latest_answer_first);

//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": 101.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     let expected_latest_answer_second: u128 = test_helper
//         .call(
//             aca.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(100, expected_latest_answer_second);
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L383
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L389, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L400, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L413, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L426, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L438

// #[test]

// fn updates_the_updated_timestamp() {
//     let deposit: u64 = 100;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let min: u64 = 2;
//     let max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
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

//     let original_timestamp: u128 = test_helper
//         .call(
//             aca.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(original_timestamp > 0, true);

//     oracle_three
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

//     let current_timestamp: u128 = test_helper
//         .call(
//             aca.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(current_timestamp > original_timestamp, true);
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L383
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L413
// // *TODO* Look into emitting necessary log
// #[test]

// fn announces_the_new_answer_with_a_log_event() {
//     let deposit: u64 = 100;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let min: u64 = 2;
//     let max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
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

//     oracle_three
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

//     let mut receipt = oracle_three.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     let new_answer: u64 = receipt.promise_results().remove(1).unwrap().outcome().logs[0]
//         .parse()
//         .unwrap();

//     let latest_answer: u64 = test_helper
//         .call(
//             aca.account_id(),
//             "latest_answer",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(latest_answer, new_answer);
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L383
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L426
// #[test]

// fn does_not_set_the_timedout_flag() {
//     let deposit: u64 = 100;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let next_round: u64 = 1;
//     let (
//         root,
//         aca,
//         link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let min: u64 = 2;
//     let max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
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

//     let expected_no_data_present = test_helper.call(
//         aca.account_id(),
//         "get_round_data",
//         &json!({"_round_id": next_round.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );
//     if let ExecutionStatus::Failure(execution_error) = &expected_no_data_present
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error.to_string().contains("No data present"));
//     } else {
//         unreachable!();
//     }

//     oracle_three
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

//     let latest_round_data: (u64, u128, u64, u64, u64) = test_helper
//         .call(
//             aca.account_id(),
//             "latest_round_data",
//             &json!({"_round_id": next_round.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     // mismatched type issue here with next_round
//     assert_eq!(next_round, latest_round_data.4);
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L383
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L413
// #[test]

// fn updates_the_round_details() {
//     let deposit: u64 = 100;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let next_round: u64 = 1;
//     let (
//         root,
//         aca,
//         link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let min: u64 = 2;
//     let max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
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

//     let expected_no_data_present = test_helper.call(
//         aca.account_id(),
//         "latest_round_data",
//         &json!({}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );
//     if let ExecutionStatus::Failure(execution_error) = &expected_no_data_present
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         println!("{:?}", execution_error.to_string());
//         assert!(execution_error.to_string().contains("No data present"));
//     } else {
//         unreachable!();
//     }

//     oracle_three
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

//     let round_after: (u64, u128, u64, u64, u64) = test_helper
//         .call(
//             aca.account_id(),
//             "get_round_data",
//             &json!({"_round_id": next_round.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(next_round, round_after.0);
//     assert_eq!(answer, round_after.1);
//     assert_eq!(false, round_after.2 == 0);

//     let original_timestamp: u128 = test_helper
//         .call(
//             aca.account_id(),
//             "latest_timestamp",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(original_timestamp as u64, round_after.3);
//     assert_eq!(1, round_after.4);
//     assert_eq!(true, round_after.2 < round_after.3);

//     let latest_round_data: (u64, u128, u64, u64, u64) = test_helper
//         .call(
//             aca.account_id(),
//             "latest_round_data",
//             &json!({"_round_id": next_round.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(true, round_after.0 == latest_round_data.0);
//     assert_eq!(true, round_after.1 == latest_round_data.1);
//     assert_eq!(true, round_after.2 == latest_round_data.2);
//     assert_eq!(true, round_after.3 == latest_round_data.3);
//     assert_eq!(true, round_after.4 == latest_round_data.4);
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L471
// #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L472

// #[test]

// fn when_an_oracle_submits_for_a_round_twice() {
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//     ) = init();
//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let min_max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let withdrawable_payment: u128 = root
//         .view(
//             aca.account_id(),
//             "withdrawable_payment",
//             &json!({
//                 "_oracle": oracle_one.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//         )
//         .unwrap_json();

//     assert_eq!(0, withdrawable_payment);

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

//     let cannout_report_on_previous_rounds = oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );
//     if let ExecutionStatus::Failure(execution_error) = &cannout_report_on_previous_rounds
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error
//             .to_string()
//             .contains("cannot report on previous rounds"));
//     } else {
//         unreachable!();
//     }
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L482
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L488

// #[test]

// fn when_updated_after_the_max_answers_submitted() {
//     let payment_amount: u128 = 3;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let timeout: u64 = 1800;
//     let next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let min: u64 = 2;
//     let max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L484 sets the min and max submissions back to 1

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     let round_not_accepting_submissions = oracle_two.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     if let ExecutionStatus::Failure(execution_error) =
//         &round_not_accepting_submissions.promise_errors().remove(0).unwrap().outcome().status
//     {
//         // No data present should be error
//         assert!(execution_error
//             .to_string()
//             .contains("round not accepting submissions"));
//     } else {
//         unreachable!();
//     }
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L496
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L497,  https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L513,  https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L534
// // *TODO* Look into the oracle_round_state and oracle_round_suggest_state functions to return the correct results for 0 state. Finish all assertions in the describe (497, 513, 534)

// #[test]
// fn when_a_new_highest_round_number_is_passed_in() {
//     let rr_delay: u64 = 0;
//     let answer: u64 = 100;
//     let next_round: u64 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//     ) = init();
//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let min: u64 = 2;
//     let max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();
//     // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L498 - Look into the oracle_round_state and oracle_round_suggest_state functions to return the correct results for 0 state.

//     let starting_state = test_helper.call(
//         aca.account_id(),
//         "oracle_round_state",
//         &json!({"_oracle": oracle_one.account_id(), "_queried_round_id": 0.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     // println!("{:?}", starting_state.promise_results());
//     // assert_eq!();

//     // Advance round non-refactored function, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L498

//     oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     oracle_two.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     oracle_three.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L498 - Look into the oracle_round_state and oracle_round_suggest_state functions to return the correct results for 0 state.

//     let updated_state = test_helper.call(
//         aca.account_id(),
//         "oracle_round_state",
//         &json!({"_oracle": oracle_one.account_id(), "_queried_round_id": 0.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     // println!("{:?}", updated_state.promise_results());
//     // assert_eq!();

// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L549
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L550

// #[test]

// fn when_a_round_is_passed_in_higher_than_expected() {
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let min: u64 = 2;
//     let max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let invalid_round_to_report = oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": (next_round + 1).to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     if let ExecutionStatus::Failure(execution_error) =
//         &invalid_round_to_report.promise_errors().remove(0).unwrap().outcome().status
//     {
//         // No data present should be error
//         assert!(execution_error
//             .to_string()
//             .contains("invalid round to report"));
//     } else {
//         unreachable!();
//     }

// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L558
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L559

// #[test]

// fn when_called_by_a_non_oracle() {
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let min: u64 = 2;
//     let max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let not_enabled_oracle = test_helper.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );
//     if let ExecutionStatus::Failure(execution_error) =
//         &not_enabled_oracle.promise_errors().remove(0).unwrap().outcome().status
//     {
//         // No data present should be error
//         assert!(execution_error.to_string().contains("not enabled oracle"));
//     } else {
//         unreachable!();
//     }

// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L567
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L581
// // // *TODO* Look into subtraction overflow error handling

// #[test]

// fn when_there_are_not_sufficient_available_funds() {
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let deposit: u64 = 100;
//     let reserve_rounds: u64 = 2;
//     let oracles_length: u64 = 3;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "withdraw_funds",
//         &json!({"_recipient": test_helper.account_id().to_string(), "_amount": deposit.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     // Look into this https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L567
//     //        beforeEach(async () => {
//     //     await aggregator
//     //     .connect(personas.Carol)
//     //     .withdrawFunds(
//     //       personas.Carol.address,
//     //       deposit.sub(paymentAmount.mul(oracles.length).mul(reserveRounds)),
//     //     )

//     //   // drain remaining funds
//     //   await advanceRound(aggregator, oracles)
//     //   await advanceRound(aggregator, oracles)
//     // })

//     let subtraction_overflow_math_error = oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );
//     if let ExecutionStatus::Failure(execution_error) =
//         &subtraction_overflow_math_error.promise_errors().remove(0).unwrap().outcome().status
//     {
//         // SafeMath: subtraction overflow
//         println!("{:?}", subtraction_overflow_math_error.promise_results());
//         assert!(execution_error.to_string().contains("SafeMath: subtraction overflow"));
//     } else {
//         unreachable!();
//     }

// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L589
// #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L603
// #[test]

// fn still_allows_the_previous_round_to_be_answered() {
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let mut next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         oracle_four,
//         oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id() ], "_min_submissions": 3.to_string(), "_max_submissions": 4.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

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

//     oracle_three
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

//     oracle_four
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

//     next_round = 2;

//     // Start the next round
//     // Start the next round

//     oracle_three
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
//     // still allows the previous round to be answered

//     oracle_two
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L589
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L607

// #[test]

// fn once_the_current_round_is_answered() {
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let mut next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         oracle_four,
//         oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id() ], "_min_submissions": 3.to_string(), "_max_submissions": 4.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

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

//     oracle_three
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

//     oracle_four
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

//     next_round = 2;

//     // Start the next round

//     oracle_three
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
//     // once the current round is answered

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

//     oracle_four
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

//     // does not allow reports for the previous round

//     let invalid_round_to_report = oracle_two.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     if let ExecutionStatus::Failure(execution_error) = &invalid_round_to_report
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         // SafeMath: subtraction overflow
//         assert!(execution_error
//             .to_string()
//             .contains("invalid round to report"));
//     } else {
//         unreachable!();
//     }
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L589
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L623

// #[test]

// fn when_the_previous_round_has_finished() {
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let mut next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         oracle_four,
//         oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id() ], "_min_submissions": 3.to_string(), "_max_submissions": 4.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

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

//     oracle_three
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

//     oracle_four
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

//     // when the previous round has finished

//     oracle_five
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     // does not allow reports for the previous round

//     let round_not_accepting_submissions = oracle_two.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     if let ExecutionStatus::Failure(execution_error) = &round_not_accepting_submissions
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error
//             .to_string()
//             .contains("round not accepting submissions"));
//     } else {
//         unreachable!();
//     }
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#639
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L642
// // *TODO* Complete the function
// #[test]

// fn pays_the_same_amount_to_all_oracles_per_round() {}

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#683
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L692

// #[test]

// fn does_not_revert_on_the_oracles_first_round() {
//     let payment_amount: u128 = 3;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 1;
//     let timeout: u64 = 1800;
//     let next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         oracle_four,
//         oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let min: u64 = 2;
//     let max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L484 sets the min and max submissions back to 1

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

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
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#683
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L692

// #[test]

// fn does_revert_before_the_delay() {
//     let payment_amount: u128 = 3;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 1;
//     let timeout: u64 = 1800;
//     let mut next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         oracle_four,
//         oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let min: u64 = 2;
//     let max: u64 = 3;

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": max.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L484 sets the min and max submissions back to 1

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

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

//     next_round = next_round + 1;

//     let expected_previous_round_not_supersedable = oracle_two.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_previous_round_not_supersedable
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         // No data present should be error
//         assert!(execution_error
//             .to_string()
//             .contains("previous round not supersedable"));
//     } else {
//         unreachable!();
//     }
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#712
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L736

// #[test]

// fn when_called_by_an_oracle_who_has_not_answered_recently() {
//     let payment_amount: u128 = 3;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let timeout: u64 = 1800;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_two
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 3.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     // Since Ned and Nelly have answered recently, and we set the delay
//     // to 2, only Nelly can answer as she is the only oracle that hasn't
//     // started the last two rounds.
//     // await updateFutureRounds(aggregator, {
//     //   maxAnswers: oracles.length,
//     //   restartDelay: newDelay,
//     // })

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 2.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     // when called by an oracle who has not answered recently
//     // it does not revert
//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 4.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#712
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L742

// #[test]

// fn when_called_by_an_oracle_who_has_answered_recently() {
//     let payment_amount: u128 = 3;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let timeout: u64 = 1800;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_two
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 3.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     // Since Ned and Nelly have answered recently, and we set the delay
//     // to 2, only Nelly can answer as she is the only oracle that hasn't
//     // started the last two rounds.
//     // await updateFutureRounds(aggregator, {
//     //   maxAnswers: oracles.length,
//     //   restartDelay: newDelay,
//     // })

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 2.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     // when called by an oracle who has answered recently
//     // it does not revert
//     let expected_round_not_accepting_submissions = oracle_two.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": 4.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_round_not_accepting_submissions
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         // No data present should be error
//         assert!(execution_error
//             .to_string()
//             .contains("round not accepting submissions"));
//     } else {
//         unreachable!();
//     }

//     let expected_round_not_accepting_submissions_two = oracle_three.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": 4.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_round_not_accepting_submissions_two
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         // No data present should be error
//         assert!(execution_error
//             .to_string()
//             .contains("round not accepting submissions"));
//     } else {
//         unreachable!();
//     }
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#756
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L776
// // *TODO* Look into why the contract panics on oracle_three starting a new round. Error: previous round not supersedable.

// #[test]

// fn allows_a_new_round_to_be_started() {
//     let payment_amount: u128 = 3;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let timeout: u64 = 1800;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );
//     root.call(

//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_two
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_two
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     // allows a new round to be started
//     // *TODO* Look into why the contract panics on oracle_three starting a new round. Error: previous round not supersedable.
//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 3.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#756
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L780
// // *TODO* Look into how to acheive this with NEAR's sdk   const block = await provider.getBlock(receipt.blockHash ?? '')

// #[test]
// fn sets_the_info_for_the_previous_round() {
//     let payment_amount: u128 = 3;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let timeout: u64 = 1800;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_two
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_two
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     // sets the info for the previous round
//     let mut expected_updated_timestamp: u128 = test_helper
//         .call(
//             aca.account_id(),
//             "get_timestamp",
//             &json!({"_round_id": 2.to_string()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(0, expected_updated_timestamp);

//     let mut expected_answer: u128 = test_helper
//         .call(
//             aca.account_id(),
//             "get_answer",
//             &json!({"_round_id": 2.to_string()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(0, expected_answer);

//     oracle_three.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": 3.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );
//     // *TODO*: Look into this const block = await provider.getBlock(receipt.blockHash ?? '')
//     //  matchers.bigNum(ethers.utils.bigNumberify(block.timestamp), updated)

//     // expected_updated_timestamp = test_helper
//     //     .call(
//     //         aca.account_id(),
//     //         "get_timestamp",
//     //         &json!({"_round_id": 2.to_string()}).to_string().into_bytes(),
//     //         DEFAULT_GAS,
//     //         0, // deposit
//     //     )
//     //     .unwrap_json();
//     // assert_eq!(0, expected_updated_timestamp);

//     expected_answer = test_helper
//         .call(
//             aca.account_id(),
//             "get_answer",
//             &json!({"_round_id": 2.to_string()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(answer, expected_answer);

//     let expected_round: (u64, u128, u64, u64, u64) = test_helper
//         .call(
//             aca.account_id(),
//             "get_round_data",
//             &json!({"_round_id": 2.to_string()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(2, expected_round.0);
//     assert_eq!(answer, expected_round.1);
//     assert_eq!(expected_updated_timestamp as u64, expected_round.3);
//     assert_eq!(1, expected_round.4);
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#756
// #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L806
// *TODO* Look into why the panic error contains No data present and not previous round not supersedable

// #[test]
// fn sets_the_previous_round_as_timed_out() {
//     let payment_amount: u128 = 3;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let timeout: u64 = 1800;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_two
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_two
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     // sets the previous round as timed out
//     // *TODO* Look into why the panic error contains No data present and not previous round not supersedable
//     let expected_no_data_present = test_helper.call(
//         aca.account_id(),
//         "get_round_data",
//         &json!({"_round_id": 2.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );
//     if let ExecutionStatus::Failure(execution_error) =
//         &expected_no_data_present.promise_errors().remove(0).unwrap().outcome().status
//     {
//         // No data present should be error
//         assert!(execution_error.to_string().contains("No data present"));
//     } else {
//         unreachable!();
//     }

//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 3.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     let expected_round: (u64, u128, u64, u64, u64) = test_helper
//         .call(
//             aca.account_id(),
//             "get_round_data",
//             &json!({"_round_id": 2.to_string()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(2, expected_round.0);
//     assert_eq!(1, expected_round.4);
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#756
// #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L820
// *TODO* Check revert error message is correct

// #[test]
// fn still_respects_the_delay_restriction() {
//     let payment_amount: u128 = 3;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let timeout: u64 = 1800;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_two
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_two
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     let expected_revert = oracle_two.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": 3.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_revert
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         // expected to revert because the sender started the last round
//         // *TODO* Check revert error message is correct
//         // assert!(execution_error.to_string().contains(""));
//     } else {
//         unreachable!();
//     }
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#756
// #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L827
// *TODO* Check why the contract is throwing previous round not supersedable error, when oracle_three calls on round 3, there should be no reverting

// #[test]
// fn uses_the_timeout_set_at_the_beginning_of_the_round(
// ) {
//     let payment_amount: u128 = 3;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let timeout: u64 = 1800;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     oracle_one
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_two
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_two
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": (timeout+100000).to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     oracle_three
//         .call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": 3.to_string(), "_submission": answer.to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .assert_success();
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L836
// #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L837

// #[test]
// fn rejects_values_below_the_submission_value_range() {
//     let payment_amount: u128 = 3;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let timeout: u64 = 1800;
//     let min_submission_value: u64 = 1;
//     let next_round: u64 = 1;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     let expected_value_below_min_submission_value = oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": (min_submission_value-1).to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_value_below_min_submission_value
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         // No data present should be error
//         assert!(execution_error
//             .to_string()
//             .contains("value below min_submission_value"));
//     } else {
//         unreachable!();
//     }
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L836
// #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L846

// #[test]
// fn accepts_submissions_equal_to_the_min_submission_value() {
//     let payment_amount: u128 = 3;
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let timeout: u64 = 1800;
//     let min_submission_value: u64 = 1;
//     let next_round: u64 = 1;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": (min_submission_value).to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L836
// #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L852

// #[test]
// fn accepts_submissions_equal_to_the_max_submission_value() {
//     let payment_amount: u128 = 3;
//     let rr_delay: u64 = 0;
//     let timeout: u64 = 1800;
//     let next_round: u64 = 1;
//     let max_submission_value: u128 = 100000000000000000000;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": (max_submission_value).to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L836
// // #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L858

// #[test]
// fn rejects_values_above_the_max_submission_value_range() {
//     let payment_amount: u128 = 3;
//     let rr_delay: u64 = 0;
//     let timeout: u64 = 1800;
//     let next_round: u64 = 1;
//     let max_submission_value: u128 = 100000000000000000000;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//         aca.account_id(),
//         "update_future_rounds",
//         &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     let expected_value_above_max_submission_value = oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": (max_submission_value+ 1).to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_value_above_max_submission_value
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         // No data present should be error
//         assert!(execution_error
//             .to_string()
//             .contains("value above max_submission_value"));
//     } else {
//         unreachable!();
//     }
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L868
// #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L877
// *TODO* Create validator mock factory

// #[test]
// fn calls_out_to_the_validator() {
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L892
// #submit tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L904
// *TODO* Create validator mock factory

// #[test]
// fn still_updates() {
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L916
// #get_answer tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L927
// #[test]
// fn retrieves_the_answer_recorded_for_past_rounds() {
//     let new_amount: u128 = 50;
//     let payment_amount: u128 = 3;
//     let deposit: u64 = 100;
//     let answer: u128 = 100;
//     let answers: Vec<u128> = [1, 10, 101, 1010, 10101, 101010, 1010101].to_vec();
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let timeout: u64 = 1800;
//     let decimals: u64 = 24;
//     let description: String = "LINK/USD".to_string();
//     let reserve_rounds: u64 = 2;
//     let min_submission_value: u128 = 1;
//     let max_submission_value: u128 = 100000000000000000000;
//     let oracles: Vec<AccountId>;
//     let mut next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         eac_without_access_controller,
//         oracle_four,
//         oracle_five
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     let mut n = 0;
//     let mut y = 1;
//     let mut x = 0;

//     while n < answers.len() {
//         oracle_one.call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answers[n].to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         ).assert_success();
//         next_round += 1;
//         n += 1;
//     }

//     while y < next_round {
//         let answer: u128 = test_helper
//             .call(
//                 aca.account_id(),
//                 "get_answer",
//                 &json!({"_round_id": y.to_string()}).to_string().into_bytes(),
//                 DEFAULT_GAS,
//                 0, // deposit
//             )
//             .unwrap_json();
//         let expected_answer: u128 = answers[x] as u128;
//         x += 1;
//         y += 1;
//         assert_eq!(answer, expected_answer);
//     }

//     // research this
//     //     it("returns 0 for answers greater than uint32's max", async () => {
//     //   const overflowedId = h.bigNum(2).pow(32).add(1)
//     //   const answer = await aggregator.getAnswer(overflowedId)
//     //   matchers.bigNum(0, answer)
//     // })
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L916
// // #get_answer tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L934
// // *TODO* Research overflowedId issue for Rust uint type

// #[test]
// fn returns_zero_for_answers_greater_than_uint32s_max() {
//     let answers: Vec<u128> = [1, 10, 101, 1010, 10101, 101010, 1010101].to_vec();
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let mut next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     let mut n = 0;

//     while n < answers.len() {
//         oracle_one.call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": answers[n].to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         ).assert_success();
//         next_round += 1;
//         n += 1;
//     }

//     // *TODO* Research overflowedId issue for Rust uint type
//     //     it("returns 0 for answers greater than uint32's max", async () => {
//     //   const overflowedId = h.bigNum(2).pow(32).add(1)
//     //   const answer = await aggregator.getAnswer(overflowedId)
//     //   matchers.bigNum(0, answer)
//     // })

// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L941
// #get_timestamp tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L950

// #[test]
// fn retrieves_the_timestamp_recorded_for_past_rounds() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let mut next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     let mut i = 0;
//     let mut z = 1;
//     let mut latest_timestamp: u128 = 0;

//     while i < 10 {
//         oracle_one.call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": (i + 1).to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         ).assert_success();
//         next_round += 1;
//         i += 1;
//     }

//     while z < next_round {
//         let current_timestamp: u128 = test_helper
//             .call(
//                 aca.account_id(),
//                 "get_timestamp",
//                 &json!({"_round_id": z.to_string()}).to_string().into_bytes(),
//                 DEFAULT_GAS,
//                 0, // deposit
//             )
//             .unwrap_json();
//         z += 1;
//         assert_eq!(current_timestamp >= latest_timestamp, true);
//         latest_timestamp = current_timestamp;
//     }
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L941
// // #get_timestamp tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L960
// // *TODO* Research overflowedId issue for Rust uint type

// #[test]
// fn returns_zero_for_timestamps_greater_than_uint32s_max() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let mut next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     let mut i = 0;
//     let mut z = 1;
//     let mut latest_timestamp: u128 = 0;

//     while i < 10 {
//         oracle_one.call(
//             aca.account_id(),
//             "submit",
//             &json!({"_round_id": next_round.to_string(), "_submission": (i + 1).to_string()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         ).assert_success();
//         next_round += 1;
//         i += 1;
//     }

//     while z < next_round {
//         let current_timestamp: u128 = test_helper
//             .call(
//                 aca.account_id(),
//                 "get_timestamp",
//                 &json!({"_round_id": z.to_string()}).to_string().into_bytes(),
//                 DEFAULT_GAS,
//                 0, // deposit
//             )
//             .unwrap_json();
//         z += 1;
//         assert_eq!(current_timestamp >= latest_timestamp, true);
//         latest_timestamp = current_timestamp;
//     }

//     // research this
//     // it("returns 0 for answers greater than uint32's max", async () => {
//     //     const overflowedId = h.bigNum(2).pow(32).add(1)
//     //     const answer = await aggregator.getTimestamp(overflowedId)
//     //     matchers.bigNum(0, answer)
//     //   })
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L967
// #change_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L969

// #[test]
// fn increases_the_oracle_count() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let past_count: u128 = test_helper
//         .call(
//             aca.account_id(),
//             "oracle_count",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     let current_count: u128 = test_helper
//         .call(
//             aca.account_id(),
//             "oracle_count",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(past_count + 1, current_count);
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L967
// #change_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L977

// #[test]
// fn adds_the_address_in_get_oracles() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     let oracles: Vec<String> = test_helper
//         .call(
//             aca.account_id(),
//             "get_oracles",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(oracle_one.account_id().to_string(), oracles[0]);
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L967
// // #change_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L982

// #[test]
// fn updates_the_round_details() {
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();
//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 2.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     let min_submission_count: u64 = root
//         .call(
//             aca.account_id(),
//             "min_submission_count",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     let max_submission_count: u64 = root
//         .call(
//             aca.account_id(),
//             "max_submission_count",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     let restart_delay: u64 = root
//         .call(
//             aca.account_id(),
//             "restart_delay",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(min_submission_count, 1);
//     assert_eq!(max_submission_count, 3);
//     assert_eq!(restart_delay, 2);
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L967
// #change_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L995

// #[test]
// fn emits_a_log() {
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         _oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let oracle_added_event = root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_two.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).promise_results();

//     println!("{:?}", oracle_added_event);

//     let oracle_added_event_oracle: String = oracle_added_event.clone().remove(1).unwrap().outcome().logs[0]
//         .parse()
//         .unwrap();

//     let result = [oracle_two.account_id(), ", true".to_string()].join("");

//     assert_eq!(result, oracle_added_event_oracle);

//     let oracle_admin_updated_event_oracle: String = oracle_added_event.clone().remove(1).unwrap().outcome().logs[1]
//         .parse()
//         .unwrap();

//     let result_two = [oracle_one.account_id(), ", true".to_string()].join("");

//     assert_eq!(result_two, oracle_admin_updated_event_oracle);
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1016
// #change_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1021

#[test]
// fn when_the_oracle_has_already_been_added_and_reverts() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     let expected_oracle_already_enabled = root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_oracle_already_enabled
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error
//             .to_string()
//             .contains("oracle already enabled"));
//     } else {
//         unreachable!();
//     }

// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1029
// #change_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1030

// #[test]

// fn when_called_by_anyone_but_the_owner_and_reverts() {
//     let min_ans: u64 = 1;
//     let max_ans: u64 = 1;
//     let rr_delay: u64 = 0;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         _oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     let expected_only_callable_by_owner = oracle_one.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
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

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1047
// #change_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1069

// #[test]

// fn does_not_allow_the_oracle_to_update_the_round() {
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//             aca.account_id(),
//             "change_oracles",
//             &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         );

//     let expected_not_yet_enabled_oracle = oracle_three.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": "1", "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_not_yet_enabled_oracle
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error
//             .to_string()
//             .contains("not yet enabled oracle"));
//     } else {
//         unreachable!();
//     }
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1047
// #change_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1076
// #[test]

// fn does_allow_the_oracle_to_update_future_rounds() {
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let mut next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//             aca.account_id(),
//             "change_oracles",
//             &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         );

//     oracle_two.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     next_round = next_round + 1;

//     oracle_three
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
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1085
// #change_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1086
// #[test]

// fn when_an_oracle_is_added_after_removed_for_a_round_and_allows_the_oracle_to_update() {
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let mut next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     oracle_three.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     next_round = next_round + 1;

//     root.call(
//             aca.account_id(),
//             "change_oracles",
//             &json!({"_removed": [oracle_three.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         );

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

//     next_round = next_round + 1;

//     root.call(
//             aca.account_id(),
//             "change_oracles",
//             &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         );

//     oracle_three
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
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1113
// #change_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1114
// *TODO* Investigate why the contratc panicks and throws previous round not supersedable issue, as that is not intended behavior.
// #[test]

// fn when_an_oracle_is_added_and_immediately_removed_mid_round_allows_the_oracle_to_update() {
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let mut next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     oracle_three.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     next_round = next_round + 1;

//     root.call(
//             aca.account_id(),
//             "change_oracles",
//             &json!({"_removed": [oracle_three.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         );

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

//     next_round = next_round + 1;

//     root.call(
//             aca.account_id(),
//             "change_oracles",
//             &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         );

//     // *TODO* Investigate why the contratc panicks and throws previous round not supersedable issue, as that is not intended behavior.

//     oracle_three
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
// }

// // https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1140
// // #change_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1141
// // *TODO* Research why the contract is not panicking with owner cannot overwrite admin

// #[test]

// fn when_an_oracle_is_re_added_after_with_a_different_admin_address_and_reverts() {
//     let answer: u128 = 100;
//     let rr_delay: u64 = 0;
//     let next_round: u128 = 1;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     oracle_one.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//             aca.account_id(),
//             "change_oracles",
//             &json!({"_removed": [oracle_three.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         ).assert_success();

//     let expected_owner_cannot_override_admin = root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [root.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );
//     // *TODO* Research why the contract is not panicking with owner cannot overwrite admin

//     if let ExecutionStatus::Failure(execution_error) = &expected_owner_cannot_override_admin
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error
//             .to_string()
//             .contains("owner cannot overwrite admin"));
//     } else {
//         unreachable!();
//     }
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1173
// #change_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1204
// *TODO* Look into a simple way to implement this function

// #[test]

// fn not_use_too_much_gas() {}

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1173
// #change_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1242
// *TODO* Look into a simple way to implement this function

// #[test]

// fn reverts_when_another_oracle_is_added() {}

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1173
// #change_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1259
// *TODO* Look into a simple way to implement this function

// #[test]

// fn reverts_when_min_submissions_is_set_to_0() {}

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1267
// #removing_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1279

// #[test]

// fn decreases_the_oracle_count() {
//     let rr_delay: u64 = 0;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     let past_count: u128 = test_helper
//         .call(
//             aca.account_id(),
//             "oracle_count",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     let current_count: u128 = test_helper
//         .call(
//             aca.account_id(),
//             "oracle_count",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(past_count - 1, current_count);
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1267
// #removing_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1296
// #[test]

// fn removing_oracles_and_updates_the_round_details() {
//     let rr_delay: u64 = 0;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     let min_submission_count: u64 = root
//         .call(
//             aca.account_id(),
//             "min_submission_count",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     let max_submission_count: u64 = root
//         .call(
//             aca.account_id(),
//             "max_submission_count",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     let restart_delay: u64 = root
//         .call(
//             aca.account_id(),
//             "restart_delay",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(min_submission_count, 1);
//     assert_eq!(max_submission_count, 1);
//     assert_eq!(restart_delay, 0);
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1267
// #removing_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1306
// #[test]

// fn removing_oracles_and_emits_a_log() {
//     let rr_delay: u64 = 0;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     let oracle_removed_event = root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).promise_results();

//     let oracle_removed_event_log: String = oracle_removed_event
//         .clone()
//         .remove(1)
//         .unwrap()
//         .outcome()
//         .logs[0]
//         .parse()
//         .unwrap();

//     let result = [oracle_one.account_id(), ", false".to_string()].join("");

//     assert_eq!(result, oracle_removed_event_log);
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1267
// #removing_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1324

// #[test]

// fn removing_the_address_in_get_oracles() {
//     let rr_delay: u64 = 0;
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     let oracles: Vec<String> = test_helper
//         .call(
//             aca.account_id(),
//             "get_oracles",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();

//     let mut n = 0;

//     while n < oracles.len() {
//         assert_ne!(oracles[n], oracle_one.account_id());
//         n += 1;
//     }
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1341
// #removing_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1355
// #[test]

// fn when_the_oracle_is_not_currently_added_and_reverts() {
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     let expected_oracle_not_enabled = root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_oracle_not_enabled
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error.to_string().contains("oracle not enabled"));
//     } else {
//         unreachable!();
//     }
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1372
// #removing_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1373
// #[test]

// fn when_removing_the_last_oracle_and_does_not_revert() {
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [oracle_three.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 0.to_string(), "_max_submissions": 0.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1391
// #removing_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1392
// #[test]

// fn when_called_by_anyone_but_the_owner_and_reverts() {
//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     let expected_only_callable_by_owner = oracle_two.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
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

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1402
// #removing_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1409
// *TODO* Look into why the contract is failing with round not accepting submissions and not no longer allowed oracle
// #[test]

// fn it_is_allowed_to_report_on_one_more_round() {
//     let mut next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [oracle_three.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     oracle_three
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

//     next_round = next_round + 1;

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

//     let expected_no_longer_allowed_oracle = oracle_three
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

//     // *TODO* Look into why the contract is failing with round not accepting submissions and not no longer allowed oracle

//     if let ExecutionStatus::Failure(execution_error) = &expected_no_longer_allowed_oracle
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error
//             .to_string()
//             .contains("no longer allowed oracle"));
//     } else {
//         unreachable!();
//     }
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1423
// #removing_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1432
// *TODO* Look into why the contract is allowing oracle_three to future in participate in future rounds

// #[test]

// fn it_is_allowed_to_finish_that_round_and_one_more_round() {
//     let mut next_round: u128 = 1;
//     let answer: u128 = 100;

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

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
//         "change_oracles",
//         &json!({"_removed": [oracle_three.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     ).assert_success();

//     oracle_three
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

//     next_round = next_round + 1;

//     oracle_three
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

//     next_round = next_round + 1;

//     let expected_no_longer_allowed_oracle = oracle_three.call(
//         aca.account_id(),
//         "submit",
//         &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     // *TODO* Look into why the contract is allowing oracle_three to future in participate in future rounds
//     // cannot participate in future rounds

//     if let ExecutionStatus::Failure(execution_error) = &expected_no_longer_allowed_oracle
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error
//             .to_string()
//             .contains("no longer allowed oracle"));
//     } else {
//         unreachable!();
//     }
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1341
// #removing_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1445

// #[test]

// fn it_reverts_when_min_submissions_is_set_to_0() {

//     let (
//         root,
//         aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//     ) = init();

//     root.call(
//         aca.account_id(),
//         "add_access",
//         &json!({"_user": test_helper.account_id().to_string()})
//             .to_string()
//             .into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     )
//     .assert_success();

//     root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );


//     let expected_min_must_be_greater_than_0 = root.call(
//         aca.account_id(),
//         "change_oracles",
//         &json!({"_removed": [oracle_three.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 0.to_string(), "_max_submissions": 0.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
//         DEFAULT_GAS,
//         0, // deposit
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_min_must_be_greater_than_0
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error
//             .to_string()
//             .contains("min must be greater than 0"));
//     } else {
//         unreachable!();
//     }
// }

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1455
// #adding_and_removing_oracles tests, https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L1461

#[test]

fn can_swap_out_oracles() {

    let (
        root,
        aca,
        _link,
        oracle_one,
        _oracle_two,
        oracle_three,
        test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
    ) = init();

    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );


    let expected_min_must_be_greater_than_0 = root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_three.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 0.to_string(), "_max_submissions": 0.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_min_must_be_greater_than_0
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("min must be greater than 0"));
    } else {
        unreachable!();
    }
}
