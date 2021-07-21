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

#[test]
fn raise_flag_when_called_by_an_enabled_setter_sets_the_flags() {
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
        flags_consumer,
        controller,
    ) = init();

    oracle_three
        .call(
            controller.account_id(),
            "add_access",
            &json!({
                "_user": oracle_one.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    oracle_one
        .call(
            flags.account_id(),
            "raise_flag",
            &json!({
                "subject": flags_consumer.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    let expected_true_value: bool = oracle_three
        .call(
            flags.account_id(),
            "get_flag",
            &json!({
                "subject": flags_consumer.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            36500000000000000000000, // deposit
        )
        .unwrap_json();

    assert_eq!(true, expected_true_value);
}

// #raise_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L65
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L117

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
//         flags_consumer,
//         controller
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
//         flags_consumer,
//         controller
//     ) = init();

//     oracle_three
//         .call(
//             controller.account_id(),
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
//                 "subjects": [flags_consumer.account_id().to_string()]
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
//                 "subject": flags_consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(true, expected_true_value);
// }

// #raise_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L160
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L212

// #[test]
// fn raise_flags_when_called_by_a_non_enabled_setter_and_reverts() {
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
//         flags_consumer,
//         controller
//     ) = init();

//     let expected_not_allowed_to_raise_flags = oracle_one.call(
//         flags.account_id(),
//         "raise_flags",
//         &json!({
//             "subjects": [flags_consumer.account_id().to_string()]
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

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

// #raise_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L65
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L52

// #[test]
// fn raise_flags_when_called_when_there_is_no_raising_access_controller_and_succeeds_for_the_owner() {
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
//         flags_consumer,
//         controller
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

//     assert_eq!(tx.logs()[0], "controller, ");

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
//             "raise_flags",
//             &json!({
//                 "subjects": [flags_consumer.account_id().to_string()]
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
//                 "subject": flags_consumer.account_id().to_string()
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
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L52
// #[test]
// fn raise_flags_when_called_when_there_is_no_raising_access_controller_and_reverts_for_non_owner() {
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
//         flags_consumer,
//         controller
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

//     assert_eq!(tx.logs()[0], "controller, ");

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
//         "raise_flags",
//         &json!({
//             "subjects": [flags_consumer.account_id().to_string()]
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     if let ExecutionStatus::Failure(execution_error) = &expected_not_allowed_to_raise_flags
//         .promise_errors()
//         .remove(0)
//         .unwrap()
//         .outcome()
//         .status
//     {
//         assert!(execution_error
//             .to_string()
//             .contains("Raising access controller account ID is invalid."));
//     } else {
//         unreachable!();
//     }
// }

// #lower_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L65
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L52
// #[test]
// fn lower_flags_when_called_by_the_owner_updates_the_warning_flag() {
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
//         flags_consumer,
//         controller,
//     ) = init();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "raise_flags",
//             &json!({
//                 "subjects": [flags_consumer.account_id().to_string()]
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
//                 "subject": flags_consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(true, expected_true_value);

//     oracle_three
//         .call(
//             flags.account_id(),
//             "lower_flags",
//             &json!({
//                 "subjects": [flags_consumer.account_id().to_string()]
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let expected_false_value: bool = oracle_three
//         .call(
//             flags.account_id(),
//             "get_flag",
//             &json!({
//                 "subject": flags_consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             36500000000000000000000, // deposit
//         )
//         .unwrap_json();

//     assert_eq!(false, expected_false_value);
// }

// #lower_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L65
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L75
// *TODO* Look into naming Events and parsing the names for verification the event exists.

// #[test]
// fn lower_flags_when_called_by_the_owner_and_emits_an_event_log() {
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
//         flags_consumer,
//         controller,
//     ) = init();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "raise_flags",
//             &json!({
//                 "subjects": [flags_consumer.account_id().to_string()]
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let tx = oracle_three.call(
//         flags.account_id(),
//         "lower_flags",
//         &json!({
//             "subjects": [flags_consumer.account_id().to_string()]
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     // *TODO* Look into naming Events and parsing the names for verification the event exists.

//     assert_eq!(tx.logs()[0], flags_consumer.account_id().to_string());
// }

// #lower_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L65
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L93

// #[test]
// fn lower_flags_when_called_by_the_owner_and_if_a_flag_has_already_been_raised_emits_an_event_log() {
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
//         flags_consumer,
//         controller,
//     ) = init();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "raise_flags",
//             &json!({
//                 "subjects": [flags_consumer.account_id().to_string()]
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     oracle_three.call(
//         flags.account_id(),
//         "lower_flags",
//         &json!({
//             "subjects": [flags_consumer.account_id().to_string()]
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     ).assert_success();

//     let tx = oracle_three.call(
//         flags.account_id(),
//         "lower_flags",
//         &json!({
//             "subjects": [flags_consumer.account_id().to_string()]
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     assert_eq!(0, tx.logs().len());
// }

// #lower_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L190
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L223

// #[test]
// fn lower_flags_when_called_by_a_non_owner_and_reverts() {
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
//         flags_consumer,
//         controller,
//     ) = init();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "raise_flags",
//             &json!({
//                 "subjects": [flags_consumer.account_id().to_string()]
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let tx = oracle_one.call(
//         flags.account_id(),
//         "lower_flags",
//         &json!({
//             "subjects": [flags_consumer.account_id().to_string()]
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     if let ExecutionStatus::Failure(execution_error) = &tx
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

// #get_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L190
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L223

// #[test]
// fn get_flag_if_the_access_control_is_turned_on_and_reverts() {
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
//         flags_consumer,
//         controller,
//     ) = init();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "enable_access_check",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     let tx = flags_consumer.call(
//         flags_consumer.account_id(),
//         "get_flag",
//         &json!({
//             "subject": flags_consumer.account_id().to_string()
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     if let ExecutionStatus::Failure(execution_error) =
//         &tx.promise_errors().remove(0).unwrap().outcome().status
//     {
//         assert!(execution_error.to_string().contains("No access"));
//     } else {
//         unreachable!();
//     }
// }

// // #get_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L190
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L223

// #[test]
// fn get_flag_if_the_access_control_is_turned_on_and_if_access_is_granted_to_the_address_and_does_not_revert(
// ) {
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
//         flags_consumer,
//         controller,
//     ) = init();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "enable_access_check",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "add_access",
//             &json!({"_user": flags_consumer.account_id()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     flags_consumer
//         .call(
//             flags_consumer.account_id(),
//             "get_flag",
//             &json!({
//                 "subject": flags_consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();
// }

// // #get_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L190
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L223

// #[test]
// fn get_flag_if_the_access_control_is_turned_off_and_and_does_not_revert() {
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
//         flags_consumer,
//         controller,
//     ) = init();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "disable_access_check",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     flags_consumer
//         .call(
//             flags_consumer.account_id(),
//             "get_flag",
//             &json!({
//                 "subject": flags_consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();
// }

// // #get_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L190
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L223

// #[test]
// fn get_flag_if_the_access_control_is_turned_off_and_if_access_is_granted_to_the_address_and_does_not_revert(
// ) {
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
//         flags_consumer,
//         controller,
//     ) = init();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "disable_access_check",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "add_access",
//             &json!({"_user": flags_consumer.account_id()})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     flags_consumer
//         .call(
//             flags_consumer.account_id(),
//             "get_flag",
//             &json!({
//                 "subject": flags_consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();
// }


// // #get_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L190
// // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L223

// #[test]
// fn get_flags_and_respects_the_access_controls_of_get_flag() {
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
//         oracle_five,
//         aggregator_validator_mock,
//         flags,
//         consumer,
//         flags_consumer,
//         controller,
//     ) = init();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "disable_access_check",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "raise_flags",
//             &json!({"subjects": [oracle_one.account_id(), oracle_five.account_id()]})
//                 .to_string()
//                 .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();

//     oracle_three
//         .call(
//             flags.account_id(),
//             "enable_access_check",
//             &json!({}).to_string().into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();
        
//     let tx = flags_consumer.call(
//         flags_consumer.account_id(),
//         "get_flag",
//         &json!({
//             "subject": flags_consumer.account_id().to_string()
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     );

//     if let ExecutionStatus::Failure(execution_error) =
//         &tx.promise_errors().remove(0).unwrap().outcome().status
//     {
//         assert!(execution_error.to_string().contains("No access"));
//     } else {
//         unreachable!();
//     }

//     oracle_three
//     .call(
//         flags.account_id(),
//         "add_access",
//         &json!({
//             "_user": flags_consumer.account_id().to_string()
//         })
//         .to_string()
//         .into_bytes(),
//         DEFAULT_GAS,
//         0,
//     )
//     .assert_success();

//     flags_consumer
//         .call(
//             flags_consumer.account_id(),
//             "get_flag",
//             &json!({
//                 "subject": flags_consumer.account_id().to_string()
//             })
//             .to_string()
//             .into_bytes(),
//             DEFAULT_GAS,
//             0,
//         )
//         .assert_success();
// }


// #get_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L190
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/6bf9ede9130ff4206b2a81b12168ec007d419682/contracts/test/v0.6/Flags.test.ts#L223

#[test]
fn get_flags_and_returns_the_flags_in_the_order_they_are_requested() {
    let (
        root,
        _aca,
        _link,
        oracle_one,
        oracle_two,
        oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        oracle_five,
        aggregator_validator_mock,
        flags,
        consumer,
        flags_consumer,
        controller,
    ) = init();

    oracle_three
        .call(
            flags.account_id(),
            "disable_access_check",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    oracle_three
        .call(
            flags.account_id(),
            "raise_flags",
            &json!({"subjects": [oracle_one.account_id(), oracle_five.account_id()]})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();
        
    let tx = oracle_three.call(
        flags_consumer.account_id(),
        "get_flags",
        &json!({
            "subjects": [oracle_three.account_id(), oracle_one.account_id(), oracle_two.account_id(), oracle_five.account_id()]
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    println!("{:?}", tx.promise_results());
}
