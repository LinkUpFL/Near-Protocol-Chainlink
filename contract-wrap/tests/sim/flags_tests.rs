use near_sdk::serde_json::json;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::DEFAULT_GAS;

use crate::utils::init_without_macros as init;

// #raise_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L55
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L57

#[test]
fn raise_flag_when_called_by_the_owner_and_updates_the_warning_flag() {
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
        _eddy
    ) = init();

    let expected_false_value: bool = oracle_three
        .call(
            flags.account_id(),
            "get_flag",
            &json!({
                "subject": flags_consumer.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .unwrap_json();

    assert_eq!(false, expected_false_value);

    oracle_three
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
            0,
        )
        .unwrap_json();

    assert_eq!(true, expected_true_value);
}

// #raise_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L55
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L65
// *TODO* Look into naming Events and parsing the names for verification the event exists.

#[test]
fn raise_flag_when_called_by_the_owner_and_emits_an_event_log() {
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
        _eddy
    ) = init();

    let tx = oracle_three.call(
        flags.account_id(),
        "raise_flag",
        &json!({
            "subject": flags_consumer.account_id().to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    // *TODO* Look into naming Events and parsing the names for verification the event exists.

    assert_eq!(tx.logs()[0], flags_consumer.account_id().to_string());
}

// #raise_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L55
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L76

#[test]
fn raise_flag_when_called_by_the_owner_and_if_a_flag_has_already_been_raised_emits_an_event_log() {
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
        _eddy
    ) = init();

    oracle_three
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

    let tx = oracle_three.call(
        flags.account_id(),
        "raise_flag",
        &json!({
            "subject": flags_consumer.account_id().to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    assert_eq!(0, tx.logs().len());
}

// #raise_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L55
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L89

#[test]
fn raise_flag_when_called_by_an_enabled_setter_sets_the_flags() {
    let (
        _root,
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
        _aggregator_validator_mock,
        flags,
        _consumer,
        flags_consumer,
        controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy
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
            0,
        )
        .unwrap_json();

    assert_eq!(true, expected_true_value);
}

// #raise_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L55
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L96

#[test]
fn raise_flag_when_called_by_a_non_enabled_setter_and_reverts() {
    let (
        _root,
        _aca,
        _link,
        oracle_one,
        _oracle_two,
        _oracle_three,
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
        _eddy
    ) = init();

    let expected_not_allowed_to_raise_flags = oracle_one.call(
        flags.account_id(),
        "raise_flag",
        &json!({
            "subject": flags_consumer.account_id().to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

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

// #raise_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L55
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L111
// *TODO* Look into naming Events and parsing the names for verification the event exists.

#[test]
fn raise_flag_when_called_when_there_is_no_raising_access_controller_and_succeeds_for_the_owner() {
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
        _eddy
    ) = init();

    let tx = oracle_three.call(
        flags.account_id(),
        "set_raising_access_controller",
        &json!({
            "rac_address": "".to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    // *TODO* Look into naming Events and parsing the names for verification the event exists.

    assert_eq!(tx.logs()[0], "controller, ");

    let current_raising_access_controller: String = oracle_three
        .call(
            flags.account_id(),
            "get_raising_access_controller",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .unwrap_json();

    assert_eq!(current_raising_access_controller, "");

    oracle_three
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
            0,
        )
        .unwrap_json();

    assert_eq!(true, expected_true_value);
}

// #raise_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L55
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L116
// *TODO* Look into naming Events and parsing the names for verification the event exists.

#[test]
fn raise_flag_when_called_when_there_is_no_raising_access_controller_and_reverts_for_non_owner() {
    let (
        _root,
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
        _aggregator_validator_mock,
        flags,
        _consumer,
        flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy
    ) = init();

    let tx = oracle_three.call(
        flags.account_id(),
        "set_raising_access_controller",
        &json!({
            "rac_address": "".to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    assert_eq!(tx.logs()[0], "controller, ");

    let current_raising_access_controller: String = oracle_three
        .call(
            flags.account_id(),
            "get_raising_access_controller",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .unwrap_json();

    assert_eq!(current_raising_access_controller, "");

    let expected_not_allowed_to_raise_flags = oracle_one.call(
        flags.account_id(),
        "raise_flag",
        &json!({
            "subject": flags_consumer.account_id().to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

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

// #raise_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L122
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L124

#[test]
fn raise_flags_when_called_by_the_owner_and_updates_the_warning_flag() {
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
        _eddy
    ) = init();

    let expected_false_value: bool = oracle_three
        .call(
            flags.account_id(),
            "get_flag",
            &json!({
                "subject": flags_consumer.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(false, expected_false_value);

    oracle_three
        .call(
            flags.account_id(),
            "raise_flags",
            &json!({
                "subjects": [flags_consumer.account_id().to_string()]
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
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(true, expected_true_value);
}

// #raise_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L122
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L132
// *TODO* Look into naming Events and parsing the names for verification the event exists.

#[test]
fn raise_flags_when_called_by_the_owner_and_emits_an_event_log() {
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
        _eddy
    ) = init();

    let tx = oracle_three.call(
        flags.account_id(),
        "raise_flags",
        &json!({
            "subjects": [flags_consumer.account_id().to_string()]
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    // *TODO* Look into naming Events and parsing the names for verification the event exists.

    assert_eq!(tx.logs()[0], flags_consumer.account_id().to_string());
}

// #raise_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#122
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L143

#[test]
fn raise_flags_when_called_by_the_owner_and_if_a_flag_has_already_been_raised_emits_an_event_log() {
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
        _eddy
    ) = init();

    oracle_three
        .call(
            flags.account_id(),
            "raise_flags",
            &json!({
                "subjects": [flags_consumer.account_id().to_string()]
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    let tx = oracle_three.call(
        flags.account_id(),
        "raise_flags",
        &json!({
            "subjects": [flags_consumer.account_id().to_string()]
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    assert_eq!(0, tx.logs().len());
}

// #raise_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L122
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L156

#[test]
fn raise_flags_when_called_by_an_enabled_setter_sets_the_flags() {
    let (
        _root,
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
        _aggregator_validator_mock,
        flags,
        _consumer,
        flags_consumer,
        controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy
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
            "raise_flags",
            &json!({
                "subjects": [flags_consumer.account_id().to_string()]
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
            0,
        )
        .unwrap_json();

    assert_eq!(true, expected_true_value);
}

// #raise_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L122
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L162

#[test]
fn raise_flags_when_called_by_a_non_enabled_setter_and_reverts() {
    let (
        _root,
        _aca,
        _link,
        oracle_one,
        _oracle_two,
        _oracle_three,
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
        _eddy
    ) = init();

    let expected_not_allowed_to_raise_flags = oracle_one.call(
        flags.account_id(),
        "raise_flags",
        &json!({
            "subjects": [flags_consumer.account_id().to_string()]
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

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

// #raise_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L122
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L179

#[test]
fn raise_flags_when_called_when_there_is_no_raising_access_controller_and_succeeds_for_the_owner() {
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
        _eddy
    ) = init();

    let tx = oracle_three.call(
        flags.account_id(),
        "set_raising_access_controller",
        &json!({
            "rac_address": "".to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    assert_eq!(tx.logs()[0], "controller, ");

    let current_raising_access_controller: String = oracle_three
        .call(
            flags.account_id(),
            "get_raising_access_controller",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .unwrap_json();

    assert_eq!(current_raising_access_controller, "");

    oracle_three
        .call(
            flags.account_id(),
            "raise_flags",
            &json!({
                "subjects": [flags_consumer.account_id().to_string()]
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
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(true, expected_true_value);
}

// #raise_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L122
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L184

#[test]
fn raise_flags_when_called_when_there_is_no_raising_access_controller_and_reverts_for_non_owner() {
    let (
        _root,
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
        _aggregator_validator_mock,
        flags,
        _consumer,
        flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy
    ) = init();

    let tx = oracle_three.call(
        flags.account_id(),
        "set_raising_access_controller",
        &json!({
            "rac_address": "".to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    assert_eq!(tx.logs()[0], "controller, ");

    let current_raising_access_controller: String = oracle_three
        .call(
            flags.account_id(),
            "get_raising_access_controller",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .unwrap_json();

    assert_eq!(current_raising_access_controller, "");

    let expected_not_allowed_to_raise_flags = oracle_one.call(
        flags.account_id(),
        "raise_flags",
        &json!({
            "subjects": [flags_consumer.account_id().to_string()]
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

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

// #lower_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L190
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L196

#[test]
fn lower_flags_when_called_by_the_owner_updates_the_warning_flag() {
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
        _eddy
    ) = init();

    oracle_three
        .call(
            flags.account_id(),
            "raise_flags",
            &json!({
                "subjects": [flags_consumer.account_id().to_string()]
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
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(true, expected_true_value);

    oracle_three
        .call(
            flags.account_id(),
            "lower_flags",
            &json!({
                "subjects": [flags_consumer.account_id().to_string()]
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    let expected_false_value: bool = oracle_three
        .call(
            flags.account_id(),
            "get_flag",
            &json!({
                "subject": flags_consumer.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(false, expected_false_value);
}

// #lower_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L190
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L204
// *TODO* Look into naming Events and parsing the names for verification the event exists.

#[test]
fn lower_flags_when_called_by_the_owner_and_emits_an_event_log() {
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
        _eddy
    ) = init();

    oracle_three
        .call(
            flags.account_id(),
            "raise_flags",
            &json!({
                "subjects": [flags_consumer.account_id().to_string()]
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    let tx = oracle_three.call(
        flags.account_id(),
        "lower_flags",
        &json!({
            "subjects": [flags_consumer.account_id().to_string()]
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    // *TODO* Look into naming Events and parsing the names for verification the event exists.

    assert_eq!(tx.logs()[0], flags_consumer.account_id().to_string());
}

// #lower_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L190
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L215

#[test]
fn lower_flags_when_called_by_the_owner_and_if_a_flag_has_already_been_raised_emits_an_event_log() {
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
        _eddy
    ) = init();

    oracle_three
        .call(
            flags.account_id(),
            "raise_flags",
            &json!({
                "subjects": [flags_consumer.account_id().to_string()]
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    oracle_three
        .call(
            flags.account_id(),
            "lower_flags",
            &json!({
                "subjects": [flags_consumer.account_id().to_string()]
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    let tx = oracle_three.call(
        flags.account_id(),
        "lower_flags",
        &json!({
            "subjects": [flags_consumer.account_id().to_string()]
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    assert_eq!(0, tx.logs().len());
}

// #lower_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L190
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L224

#[test]
fn lower_flags_when_called_by_a_non_owner_and_reverts() {
    let (
        _root,
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
        _aggregator_validator_mock,
        flags,
        _consumer,
        flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy
    ) = init();

    oracle_three
        .call(
            flags.account_id(),
            "raise_flags",
            &json!({
                "subjects": [flags_consumer.account_id().to_string()]
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    let tx = oracle_one.call(
        flags.account_id(),
        "lower_flags",
        &json!({
            "subjects": [flags_consumer.account_id().to_string()]
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    if let ExecutionStatus::Failure(execution_error) =
        &tx.promise_errors().remove(0).unwrap().outcome().status
    {
        assert!(execution_error
            .to_string()
            .contains("Only callable by owner"));
    } else {
        unreachable!();
    }
}

// #get_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L232
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L238

#[test]
fn get_flag_if_the_access_control_is_turned_on_and_reverts() {
    let (
        _root,
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
        _aggregator_validator_mock,
        flags,
        _consumer,
        flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy
    ) = init();

    oracle_three
        .call(
            flags.account_id(),
            "enable_access_check",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    let tx = flags_consumer.call(
        flags_consumer.account_id(),
        "get_flag",
        &json!({
            "subject": flags_consumer.account_id().to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    if let ExecutionStatus::Failure(execution_error) =
        &tx.promise_errors().remove(0).unwrap().outcome().status
    {
        assert!(execution_error.to_string().contains("No access"));
    } else {
        unreachable!();
    }
}

// #get_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L232
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L247

#[test]
fn get_flag_if_the_access_control_is_turned_on_and_if_access_is_granted_to_the_address_and_does_not_revert(
) {
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
        _eddy
    ) = init();
    oracle_three
        .call(
            flags.account_id(),
            "enable_access_check",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    oracle_three
        .call(
            flags.account_id(),
            "add_access",
            &json!({"_user": flags_consumer.account_id()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    flags_consumer
        .call(
            flags_consumer.account_id(),
            "get_flag",
            &json!({
                "subject": flags_consumer.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();
}

// #get_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L232
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L258

#[test]
fn get_flag_if_the_access_control_is_turned_off_and_and_does_not_revert() {
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
        _eddy
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

    flags_consumer
        .call(
            flags_consumer.account_id(),
            "get_flag",
            &json!({
                "subject": flags_consumer.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();
}

// #get_flag https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L232
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L267

#[test]
fn get_flag_if_the_access_control_is_turned_off_and_if_access_is_granted_to_the_address_and_does_not_revert(
) {
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
        _eddy
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
            "add_access",
            &json!({"_user": flags_consumer.account_id()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    flags_consumer
        .call(
            flags_consumer.account_id(),
            "get_flag",
            &json!({
                "subject": flags_consumer.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();
}

// #get_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L274
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L282

#[test]
fn get_flags_and_respects_the_access_controls_of_get_flag() {
    let (
        _root,
        _aca,
        _link,
        oracle_one,
        _oracle_two,
        oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        flags,
        _consumer,
        flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy
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

    oracle_three
        .call(
            flags.account_id(),
            "enable_access_check",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    let tx = flags_consumer.call(
        flags_consumer.account_id(),
        "get_flag",
        &json!({
            "subject": flags_consumer.account_id().to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    if let ExecutionStatus::Failure(execution_error) =
        &tx.promise_errors().remove(0).unwrap().outcome().status
    {
        assert!(execution_error.to_string().contains("No access"));
    } else {
        unreachable!();
    }

    oracle_three
        .call(
            flags.account_id(),
            "add_access",
            &json!({
                "_user": flags_consumer.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    flags_consumer
        .call(
            flags_consumer.account_id(),
            "get_flag",
            &json!({
                "subject": flags_consumer.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();
}

// #get_flags https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L274
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L292
// *TODO* Parse promise results for assertion.

#[test]
fn get_flags_and_returns_the_flags_in_the_order_they_are_requested() {
    let (
        _root,
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
        _aggregator_validator_mock,
        flags,
        _consumer,
        flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy
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

    // *TODO* Parse promise results for assertion.

    println!("{:?}", tx.promise_results());
}

// #set_raising_access_controller https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L304
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L312

#[test]
fn set_raising_access_controller_and_updates_access_control_rules() {
    let (
        _root,
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
        _aggregator_validator_mock,
        flags,
        _consumer,
        flags_consumer,
        _controller,
        controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy
    ) = init();

    oracle_three
        .call(
            controller_2.account_id(),
            "enable_access_check",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    oracle_three
        .call(
            controller_2.account_id(),
            "add_access",
            &json!({"_user": oracle_three.account_id()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    // doesn't raise

    let expected_doesnt_raise = oracle_one.call(
        flags.account_id(),
        "raise_flags",
        &json!({"subjects": [flags_consumer.account_id()]})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_doesnt_raise
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

    oracle_three
        .call(
            flags.account_id(),
            "set_raising_access_controller",
            &json!({"rac_address": controller_2.account_id()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    let tx = oracle_one.call(
        flags.account_id(),
        "raise_flags",
        &json!({"subjects": [flags_consumer.account_id()]})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    if let ExecutionStatus::Failure(execution_error) =
        &tx.promise_errors().remove(0).unwrap().outcome().status
    {
        assert!(execution_error
            .to_string()
            .contains("Not allowed to raise flags"));
    } else {
        unreachable!();
    }
}

// #set_raising_access_controller https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L304
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L324

#[test]
fn set_raising_access_controller_and_emits_a_log_announcing_the_change() {
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
        _flags_consumer,
        _controller,
        controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy
    ) = init();

    oracle_three
        .call(
            controller_2.account_id(),
            "enable_access_check",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    oracle_three
        .call(
            controller_2.account_id(),
            "add_access",
            &json!({"_user": oracle_three.account_id()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    let tx = oracle_three.call(
        flags.account_id(),
        "set_raising_access_controller",
        &json!({"rac_address": controller_2.account_id()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    assert_eq!(tx.logs()[0], "controller, controller_2");
}

// #set_raising_access_controller https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L304
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L330

#[test]
fn set_raising_access_controller_and_does_not_emit_a_log_when_there_is_no_change() {
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
        _flags_consumer,
        _controller,
        controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy
    ) = init();

    oracle_three
        .call(
            controller_2.account_id(),
            "enable_access_check",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    oracle_three
        .call(
            controller_2.account_id(),
            "add_access",
            &json!({"_user": oracle_three.account_id()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    oracle_three
        .call(
            flags.account_id(),
            "set_raising_access_controller",
            &json!({"rac_address": controller_2.account_id()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    let tx = oracle_three.call(
        flags.account_id(),
        "set_raising_access_controller",
        &json!({"rac_address": controller_2.account_id()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    assert_eq!(tx.logs().len(), 0);
}

// #set_raising_access_controller https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L304
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/Flags.test.ts#L340

#[test]
fn set_raising_access_controller_and_when_called_by_a_non_owner_and_reverts() {
    let (
        _root,
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
        _aggregator_validator_mock,
        flags,
        _consumer,
        _flags_consumer,
        _controller,
        controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy
    ) = init();

    oracle_three
        .call(
            controller_2.account_id(),
            "enable_access_check",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    oracle_three
        .call(
            controller_2.account_id(),
            "add_access",
            &json!({"_user": oracle_three.account_id()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();

    let expected_only_callable_by_owner = oracle_one.call(
        flags.account_id(),
        "set_raising_access_controller",
        &json!({"rac_address": controller_2.account_id()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0,
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_only_callable_by_owner
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("Only callable by owner"));
    } else {
        unreachable!();
    }
}
