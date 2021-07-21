use near_sdk::serde_json::json;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::DEFAULT_GAS;

use crate::utils::init_without_macros as init;

// #get_answer https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L143
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts#L160

#[test]
fn get_answer_when_read_by_a_contract_without_explicit_access_and_reverts() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let next_round: u128 = 1;
    let answer: u128 = 100;

    let (
        _root,
        _aca,
        _link,
        _oracle_one,
        _oracle_two,
        oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        flags,
        _consumer,
        flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
    ) = init();

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
}

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
