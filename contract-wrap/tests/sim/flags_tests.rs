use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::DEFAULT_GAS;

use crate::utils::init_without_macros as init;

// #raise_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L65
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L67
// #[test]
// fn raise_flag_when_called_by_the_owner_and_updates_the_warning_flag() {
//     let (
//         root,
//         _aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         aggregator_validator_mock,
//         flags,
//         consumer,
//     ) = init();

//     let expected_false_value: bool = oracle_three
//         .call(
//             flags.account_id(),
//             "get_flag",
//             &json!({
//                 "subject": consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();
//     assert_eq!(false, expected_false_value);

//     oracle_three
//         .call(
//             flags.account_id(),
//             "raise_flag",
//             &json!({
//                 "subject": consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let expected_true_value: bool = oracle_three
//         .call(
//             flags.account_id(),
//             "get_flag",
//             &json!({
//                 "subject": consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(true, expected_true_value);
// }

// #raise_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L65
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L75
// *TODO* Look into naming Events and parsing the names for verification the event exists.

// #[test]
// fn raise_flag_when_called_by_the_owner_and_emits_an_event_log() {
//     let (
//         root,
//         _aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         aggregator_validator_mock,
//         flags,
//         consumer,
//     ) = init();

//     let tx = oracle_three.call(
//         flags.account_id(),
//         "raise_flag",
//         &json!({
//             "subject": consumer.account_id().to_string()
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     // *TODO* Look into naming Events and parsing the names for verification the event exists.

//     assert_eq!(tx.logs()[0], consumer.account_id().to_string());
// }

// #raise_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L65
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L93

// #[test]
// fn raise_flag_when_called_by_the_owner_and_if_a_flag_has_already_been_raised_emits_an_event_log() {
//     let (
//         root,
//         _aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         aggregator_validator_mock,
//         flags,
//         consumer,
//     ) = init();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "raise_flag",
//             &json!({
//                 "subject": consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let tx = oracle_three.call(
//         flags.account_id(),
//         "raise_flag",
//         &json!({
//             "subject": consumer.account_id().to_string()
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     assert_eq!(0, tx.logs().len());
// }

// #raise_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L65
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L110

// #[test]
// fn raise_flag_when_called_by_an_enabled_setter_sets_the_flags() {
//     let (
//         root,
//         _aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         aggregator_validator_mock,
//         flags,
//         consumer,
//     ) = init();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "add_access",
//             &json!({
//                 "_user": oracle_one.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     oracle_one
//         .call(
//             flags.account_id(),
//             "raise_flag",
//             &json!({
//                 "subject": consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let expected_true_value: bool = oracle_three
//         .call(
//             flags.account_id(),
//             "get_flag",
//             &json!({
//                 "subject": consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(true, expected_true_value);
// }

// #raise_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L65
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L117
// *TODO* Look into how this can pass while access check is disabled.

// #[test]
// fn raise_flag_when_called_by_a_non_enabled_setter_and_reverts() {
//     let (
//         root,
//         _aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         aggregator_validator_mock,
//         flags,
//         consumer,
//     ) = init();

//     let expected_not_allowed_to_raise_flags = oracle_one.call(
//         flags.account_id(),
//         "raise_flag",
//         &json!({
//             "subject": consumer.account_id().to_string()
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     // *TODO* Look into how this can pass while access check is disabled.

//     if let ExecutionStatus::Failure(execution_error) = &expected_not_allowed_to_raise_flags
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error
//             .to_string()
//             .contains("Not allowed to raise flags"));
//     } else {
//         unreachable!();
//     }
// }


// #raise_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L65
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L52
// *TODO* Look into how this can pass while access check is disabled.
// #[test]
// fn raise_flag_when_called_when_there_is_no_raising_access_controller_and_succeeds_for_the_owner() {
//     let (
//         root,
//         _aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         aggregator_validator_mock,
//         flags,
//         consumer,
//     ) = init();

//     let tx = oracle_three.call(
//         flags.account_id(),
//         "set_raising_access_controller",
//         &json!({
//             "rac_address": "".to_string()
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     assert_eq!(tx.logs()[0], "oracle_three, ");

//     let current_raising_access_controller: String = oracle_three
//         .call(
//             flags.account_id(),
//             "get_raising_access_controller",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(current_raising_access_controller, "");

//     oracle_three
//         .call(
//             flags.account_id(),
//             "raise_flag",
//             &json!({
//                 "subject": consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let expected_true_value: bool = oracle_three
//         .call(
//             flags.account_id(),
//             "get_flag",
//             &json!({
//                 "subject": consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(true, expected_true_value);
// }
// #[test]
// fn raise_flag_when_called_when_there_is_no_raising_access_controller_and_reverts_for_non_owner() {
//     let (
//         root,
//         _aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         aggregator_validator_mock,
//         flags,
//         consumer,
//     ) = init();

//     let tx = oracle_three.call(
//         flags.account_id(),
//         "set_raising_access_controller",
//         &json!({
//             "rac_address": "".to_string()
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     assert_eq!(tx.logs()[0], "oracle_three, ");

//     let current_raising_access_controller: String = oracle_three
//         .call(
//             flags.account_id(),
//             "get_raising_access_controller",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .unwrap_json();

//     assert_eq!(current_raising_access_controller, "");

//     let expected_not_allowed_to_raise_flags = oracle_one.call(
//         flags.account_id(),
//         "raise_flag",
//         &json!({
//             "subject": consumer.account_id().to_string()
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     // *TODO* Look into how this can pass while access check is disabled.

//     if let ExecutionStatus::Failure(execution_error) = &expected_not_allowed_to_raise_flags
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error
//             .to_string()
//             .contains("Not allowed to raise flags"));
//     } else {
//         unreachable!();
//     }
// }


// #raise_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L160
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L162
// #[test]
// fn raise_flags_when_called_by_the_owner_and_updates_the_warning_flag() {
//     let (
//         root,
//         _aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         aggregator_validator_mock,
//         flags,
//         consumer,
//     ) = init();

//     let expected_false_value: bool = oracle_three
//         .call(
//             flags.account_id(),
//             "get_flag",
//             &json!({
//                 "subject": consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(false, expected_false_value);

//     oracle_three
//         .call(
//             flags.account_id(),
//             "raise_flags",
//             &json!({
//                 "subjects": [consumer.account_id().to_string()]
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let expected_true_value: bool = oracle_three
//         .call(
//             flags.account_id(),
//             "get_flag",
//             &json!({
//                 "subject": consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(true, expected_true_value);
// }

// #raise_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#160
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L170
// *TODO* Look into naming Events and parsing the names for verification the event exists.

// #[test]
// fn raise_flags_when_called_by_the_owner_and_emits_an_event_log() {
//     let (
//         root,
//         _aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         aggregator_validator_mock,
//         flags,
//         consumer,
//     ) = init();

//     let tx = oracle_three.call(
//         flags.account_id(),
//         "raise_flags",
//         &json!({
//             "subjects": [consumer.account_id().to_string()]
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     // *TODO* Look into naming Events and parsing the names for verification the event exists.

//     assert_eq!(tx.logs()[0], consumer.account_id().to_string());
// }

// #raise_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#160
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L188
// #[test]
// fn raise_flags_when_called_by_the_owner_and_if_a_flag_has_already_been_raised_emits_an_event_log() {
//     let (
//         root,
//         _aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         aggregator_validator_mock,
//         flags,
//         consumer,
//     ) = init();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "raise_flags",
//             &json!({
//                 "subjects": [consumer.account_id().to_string()]
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let tx = oracle_three.call(
//         flags.account_id(),
//         "raise_flags",
//         &json!({
//             "subjects": [consumer.account_id().to_string()]
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     assert_eq!(0, tx.logs().len());
// }

// #raise_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L160
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L205

// #[test]
// fn raise_flags_when_called_by_an_enabled_setter_sets_the_flags() {
//     let (
//         root,
//         _aca,
//         _link,
//         oracle_one,
//         _oracle_two,
//         oracle_three,
//         _test_helper,
//         _eac,
//         _eac_without_access_controller,
//         _oracle_four,
//         _oracle_five,
//         aggregator_validator_mock,
//         flags,
//         consumer,
//     ) = init();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "add_access",
//             &json!({
//                 "_user": oracle_one.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     oracle_one
//         .call(
//             flags.account_id(),
//             "raise_flags",
//             &json!({
//                 "subjects": [consumer.account_id().to_string()]
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let expected_true_value: bool = oracle_three
//         .call(
//             flags.account_id(),
//             "get_flag",
//             &json!({
//                 "subject": consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(true, expected_true_value);
// }

// #raise_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L160
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L212
// *TODO* Look into how this can pass while access check is disabled.

#[test]
fn raise_flags_when_called_by_a_non_enabled_setter_and_reverts() {
    let (
        root,
        _aca,
        _link,
        oracle_one,
        _oracle_two,
        oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        aggregator_validator_mock,
        flags,
        consumer,
    ) = init();

    let expected_not_allowed_to_raise_flags = oracle_one.call(
        flags.account_id(),
        "raise_flags",
        &json!({
            "subjects": [consumer.account_id().to_string()]
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    // *TODO* Look into how this can pass while access check is disabled.

    if let ExecutionStatus::Failure(execution_error) = &expected_not_allowed_to_raise_flags
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("Not allowed to raise flags"));
    } else {
        unreachable!();
    }
}