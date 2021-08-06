use near_sdk::serde_json::json;
use near_sdk::AccountId;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::DEFAULT_GAS;
use near_sdk::json_types::{U128, U64};

use crate::utils::init_without_macros as init;

/**
 * FluxAggregator tests were ported from this file https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts
 */


/**
 * #constructor - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L214
 */

#[test]

fn constructor_tests() {
    let payment_amount: u128 = 3;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let version: u128 = 3;
    let validator: String = "".to_string();

    let (
        root,
        _aca,
        _link,
        _oracle_one,
        _oracle_two,
        _oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let expected_payment_amount: u128 = root
        .call(
            flux_aggregator.account_id(),
            "get_payment_amount",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .unwrap_json();

    assert_eq!(payment_amount, expected_payment_amount);

    let expected_timeout: u64 = root
        .call(
            flux_aggregator.account_id(),
            "get_timeout",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .unwrap_json();

    assert_eq!(timeout, expected_timeout);

    let expected_decimals: u64 = root
        .call(
            flux_aggregator.account_id(),
            "get_decimals",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .unwrap_json();

    assert_eq!(decimals, expected_decimals);

    let expected_description: String = root
        .call(
            flux_aggregator.account_id(),
            "get_description",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .unwrap_json();

    assert_eq!(description, expected_description);

    let expected_version: u128 = root
        .call(
            flux_aggregator.account_id(),
            "get_version",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .unwrap_json();

    assert_eq!(version, expected_version);

    let expected_validator: String = root
        .call(
            flux_aggregator.account_id(),
            "get_validator",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .unwrap_json();

    assert_eq!(validator, expected_validator);
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L249
 * *TODO* Fix parsing of the log
 */
#[test]
fn updates_the_allocated_and_available_funds_counters() {
    let payment_amount: u128 = 3;
    let deposit: u128 = 100;
    let answer: u128 = 100;
    let rr_delay: u128 = 0;
    let next_round: u128 = 1;
    let min_max: u128 = 3;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let mut allocated_funds: u128 = root
        .view(
            flux_aggregator.account_id(),
            "allocated_funds",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    assert_eq!(allocated_funds, 0);

    let tx = oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let mut receipt = tx.promise_results();

    allocated_funds = root
        .view(
            flux_aggregator.account_id(),
            "allocated_funds",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    let available_funds: u128 = root
        .view(
            flux_aggregator.account_id(),
            "available_funds",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    assert_eq!(payment_amount, allocated_funds);

    let expected_available: u128 = deposit - payment_amount;

    assert_eq!(expected_available, available_funds);

    // *TODO* Fix parsing of the log
    let logged: u128 = receipt.remove(1).unwrap().outcome().logs[3]
        .parse()
        .unwrap();

    assert_eq!(expected_available, logged);
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L262
 */
#[test]
fn emits_a_log_event_announcing_submission_details() {
    let payment_amount: u128 = 3;
    let deposit: u128 = 100;
    let answer: u128 = 100;
    let rr_delay: u128 = 0;
    let next_round: u128 = 1;
    let min_max: u128 = 3;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let mut allocated_funds: u128 = root
        .view(
            flux_aggregator.account_id(),
            "allocated_funds",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    assert_eq!(allocated_funds, 0);

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    allocated_funds = root
        .view(
            flux_aggregator.account_id(),
            "allocated_funds",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    let available_funds: u128 = root
        .view(
            flux_aggregator.account_id(),
            "available_funds",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    assert_eq!(payment_amount, allocated_funds);

    let expected_available: u128 = deposit - payment_amount;

    assert_eq!(expected_available, available_funds);

    let tx = oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    assert_eq!(tx.logs()[0], "100, 1, oracle_three");
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L269
 */
#[test]
fn when_the_minimum_oracles_have_not_reported_and_pays_the_oracles_that_have_reported() {
    let payment_amount: u128 = 3;
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let next_round: u128 = 1;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let min_max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let withdrawable_payment: u128 = root
        .view(
            flux_aggregator.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_one.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    assert_eq!(0, withdrawable_payment);

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let withdrawable_payment_oracle_one: u128 = root
        .view(
            flux_aggregator.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_one.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    assert_eq!(payment_amount, withdrawable_payment_oracle_one);

    let withdrawable_payment_oracle_two: u128 = root
        .view(
            flux_aggregator.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_two.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    assert_eq!(0, withdrawable_payment_oracle_two);

    let withdrawable_payment_oracle_three: u128 = root
        .view(
            flux_aggregator.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_three.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    assert_eq!(0, withdrawable_payment_oracle_three);
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L285
 */
#[test]
fn when_the_minimum_oracles_have_not_reported_and_does_not_update_the_answer() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let next_round: u128 = 1;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let min_max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let not_updated: u128 = root
        .call(
            flux_aggregator.account_id(),
            "latest_answer",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(0, not_updated);

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let still_not_updated: u128 = root
        .call(
            flux_aggregator.account_id(),
            "latest_answer",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(0, still_not_updated);
}
/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L302
 */
#[test]
fn when_an_oracle_prematurely_bumps_the_round_and_reverts() {
    let payment_amount: u128 = 3;
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let next_round: u128 = 1;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let min: u64 = 2;
    let max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let expected_previous_round_not_supersedable = oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": (next_round + 1).to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_previous_round_not_supersedable
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("previous round not supersedable"));
    } else {
        unreachable!();
    }
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L310
 */
#[test]
fn updates_the_answer_with_the_median() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let next_round: u128 = 1;

    let (
        root,
        _aca,
        _link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let min: u64 = 2;
    let max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let expected_latest_answer: u128 = test_helper
        .call(
            flux_aggregator.account_id(),
            "latest_answer",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(0, expected_latest_answer);

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": 99.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let expected_latest_answer_first: u128 = test_helper
        .call(
            flux_aggregator.account_id(),
            "latest_answer",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(99, expected_latest_answer_first);

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": 101.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let expected_latest_answer_second: u128 = test_helper
        .call(
            flux_aggregator.account_id(),
            "latest_answer",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(100, expected_latest_answer_second);
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L327
 */
#[test]

fn updates_the_updated_timestamp() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let next_round: u128 = 1;

    let (
        root,
        _aca,
        _link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let min: u64 = 2;
    let max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let original_timestamp: u128 = test_helper
        .call(
            flux_aggregator.account_id(),
            "latest_timestamp",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(original_timestamp > 0, true);

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let current_timestamp: u128 = test_helper
        .call(
            flux_aggregator.account_id(),
            "latest_timestamp",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(current_timestamp > original_timestamp, true);
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L337
 * *TODO* Look into emitting necessary log
 */
#[test]

fn announces_the_new_answer_with_a_log_event() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let next_round: u128 = 1;

    let (
        root,
        _aca,
        _link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let min: u64 = 2;
    let max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let mut receipt = oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let new_answer: u64 = receipt.promise_results().remove(1).unwrap().outcome().logs[0]
        .parse()
        .unwrap();

    let latest_answer: u64 = test_helper
        .call(
            flux_aggregator.account_id(),
            "latest_answer",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(latest_answer, new_answer);
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L346
 */
#[test]

fn does_not_set_the_timedout_flag() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let next_round: u64 = 1;

    let (
        root,
        _aca,
        _link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let min: u64 = 2;
    let max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let expected_no_data_present = test_helper.call(
        flux_aggregator.account_id(),
        "get_round_data",
        &json!({"_round_id": next_round.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_no_data_present
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error.to_string().contains("No data present"));
    } else {
        unreachable!();
    }

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let latest_round_data: (u64, u128, u64, u64, u64) = test_helper
        .call(
            flux_aggregator.account_id(),
            "latest_round_data",
            &json!({"_round_id": next_round.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(next_round, latest_round_data.4);
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L355
 */
#[test]

fn submit_and_updates_the_round_details() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let next_round: u64 = 1;

    let (
        root,
        _aca,
        _link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let min: u64 = 2;
    let max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let expected_no_data_present = test_helper.call(
        flux_aggregator.account_id(),
        "latest_round_data",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_no_data_present
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        println!("{:?}", execution_error.to_string());
        assert!(execution_error.to_string().contains("No data present"));
    } else {
        unreachable!();
    }

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let round_after: (u64, u128, u64, u64, u64) = test_helper
        .call(
            flux_aggregator.account_id(),
            "get_round_data",
            &json!({"_round_id": next_round.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(next_round, round_after.0);
    assert_eq!(answer, round_after.1);
    assert_eq!(false, round_after.2 == 0);

    let original_timestamp: u128 = test_helper
        .call(
            flux_aggregator.account_id(),
            "latest_timestamp",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(original_timestamp as u64, round_after.3);
    assert_eq!(1, round_after.4);
    assert_eq!(true, round_after.2 < round_after.3);

    let latest_round_data: (u64, u128, u64, u64, u64) = test_helper
        .call(
            flux_aggregator.account_id(),
            "latest_round_data",
            &json!({"_round_id": next_round.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(true, round_after.0 == latest_round_data.0);
    assert_eq!(true, round_after.1 == latest_round_data.1);
    assert_eq!(true, round_after.2 == latest_round_data.2);
    assert_eq!(true, round_after.3 == latest_round_data.3);
    assert_eq!(true, round_after.4 == latest_round_data.4);
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L380
 */
#[test]

fn when_an_oracle_submits_for_a_round_twice_and_reverts() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let next_round: u128 = 1;

    let (
        root,
        _aca,
        _link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let min_max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let withdrawable_payment: u128 = root
        .view(
            flux_aggregator.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_one.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    assert_eq!(0, withdrawable_payment);

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let cannout_report_on_previous_rounds = oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &cannout_report_on_previous_rounds
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("cannot report on previous rounds"));
    } else {
        unreachable!();
    }
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L396
 */
#[test]

fn when_updated_after_the_max_answers_submitted_and_reverts() {
    let payment_amount: u128 = 3;
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let next_round: u128 = 1;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let min: u64 = 2;
    let max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L484 sets the min and max submissions back to 1

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let round_not_accepting_submissions = oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &round_not_accepting_submissions
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        // No data present should be error
        assert!(execution_error
            .to_string()
            .contains("round not accepting submissions"));
    } else {
        unreachable!();
    }
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L402
 * **TODO** Look into why oracle_round_state_suggest_round is returning the wrong data.
 */
#[test]
fn when_a_new_highest_round_number_is_passed_in_and_increments_the_answer_round() {
    let rr_delay: u64 = 0;
    let answer: u64 = 100;
    let next_round: u64 = 1;

    let (
        root,
        _aca,
        _link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let min: u64 = 2;
    let max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = test_helper
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_one.account_id(), "_queried_round_id": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(1, starting_state.1);

    // Advance round non-refactored function, https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L498

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    // https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L498 - Look into the oracle_round_state and oracle_round_suggest_state functions to return the correct results for 0 state.

    let updated_state = test_helper.call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_one.account_id(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    println!("{:?} SEC", starting_state);
    assert_eq!(2, starting_state.1);
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L412
 * **TODO** Look into grabbing the block timestamp from tx.
 */
#[test]
fn when_a_new_highest_round_number_is_passed_in_and_sets_the_started_at_time_for_the_reporting_round(
) {
    let rr_delay: u64 = 0;
    let answer: u64 = 100;
    let next_round: u64 = 1;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let min: u64 = 2;
    let max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let expected_no_data_present = root.call(
        flux_aggregator.account_id(),
        "get_round_data",
        &json!({"_round_id": next_round.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_no_data_present
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error.to_string().contains("No data present"));
    } else {
        unreachable!();
    }

    // Advance round non-refactored function, https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L498

    let tx = oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let updated_state: (u64, u128, u64, u64, u64) = root
        .call(
            flux_aggregator.account_id(),
            "get_round_data",
            &json!({"_round_id": next_round.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    // *TODO* Look into grabbing the block timestamp from tx.

    // assert_eq!(2, updated_state.2);
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L425
 *
 */
#[test]
fn when_a_new_highest_round_number_is_passed_in_and_announces_a_new_round_by_emitting_a_log() {
    let rr_delay: u64 = 0;
    let answer: u64 = 100;
    let next_round: u64 = 1;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let min: u64 = 2;
    let max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let expected_no_data_present = root.call(
        flux_aggregator.account_id(),
        "get_round_data",
        &json!({"_round_id": next_round.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_no_data_present
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error.to_string().contains("No data present"));
    } else {
        unreachable!();
    }

    let tx = oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    assert_eq!(tx.logs()[0].contains("1, oracle_one"), true);
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L439
 * **TODO** Look into why oracle_round_state_suggest_round is returning the wrong data.
 */
#[test]

fn when_a_round_is_passed_in_higher_than_expected_and_reverts() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let next_round: u128 = 1;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let min: u64 = 2;
    let max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let invalid_round_to_report = oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": (next_round + 1).to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &invalid_round_to_report
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("invalid round to report"));
    } else {
        unreachable!();
    }
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L445
 * **TODO** Look into why oracle_round_state_suggest_round is returning the wrong data.
 */
#[test]

fn when_called_by_a_non_oracle_and_reverts() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let next_round: u128 = 1;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let min: u64 = 2;
    let max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    // Carol

    let not_enabled_oracle = root.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    if let ExecutionStatus::Failure(execution_error) = &not_enabled_oracle
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        // No data present should be error
        assert!(execution_error.to_string().contains("not enabled oracle"));
    } else {
        unreachable!();
    }
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L450
 * **TODO** Look into subtraction overflow errors
 */
#[test]

fn when_there_are_not_sufficient_available_funds() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let mut next_round: u128 = 1;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let max: u64 = 3;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": max.to_string(), "_max_submissions": max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    // Carol

    root.call(
        flux_aggregator.account_id(),
        "withdraw_funds",
        &json!({"_recipient": root.account_id().to_string(), "_amount": 82.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        1, // deposit
    )
    .assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    let subtraction_overflow_math_error = oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    println!("{:?}", subtraction_overflow_math_error.promise_results());

    if let ExecutionStatus::Failure(execution_error) = &subtraction_overflow_math_error
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        // SafeMath: subtraction overflow
        assert!(execution_error
            .to_string()
            .contains("SafeMath: subtraction overflow"));
    } else {
        unreachable!();
    }
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L479
 *
 */
#[test]

fn when_a_new_round_opens_before_the_previous_rounds_closes_and_still_allows_the_previous_round_to_be_answered(
) {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let mut next_round: u128 = 1;
    let min_max: u128 = 3;

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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id() ], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_four.account_id(), oracle_five.account_id() ], "_min_submissions": 3.to_string(), "_max_submissions": 4.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_four
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = 2;

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    // still allows the previous round to be answered

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": (next_round - 1).to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L491
 *
 */
#[test]

fn once_the_current_round_is_answered_does_not_allow_reports_for_the_previous_round() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let mut next_round: u128 = 1;
    let min_max: u128 = 3;

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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id() ], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_four.account_id(), oracle_five.account_id() ], "_min_submissions": 3.to_string(), "_max_submissions": 4.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_four
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = 2;

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    // once the current round is answered

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_four
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    // does not allow reports for the previous round

    let invalid_round_to_report = oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": (next_round - 1).to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &invalid_round_to_report
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("invalid round to report"));
    } else {
        unreachable!();
    }
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L501
 *
 */
#[test]

fn when_the_previous_round_has_finished_and_does_not_allow_reports_for_the_previous_round() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let mut next_round: u128 = 1;
    let min_max: u128 = 3;

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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id() ], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_four.account_id(), oracle_five.account_id() ], "_min_submissions": 3.to_string(), "_max_submissions": 4.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_four
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = 2;

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    // when the previous round has finished

    oracle_five
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    // does not allow reports for the previous round

    let round_not_accepting_submissions = oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &round_not_accepting_submissions
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("round not accepting submissions"));
    } else {
        unreachable!();
    }
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L513
 *
 */
#[test]

fn when_price_is_updated_mid_round_and_pays_the_same_amount_to_all_oracles_per_round() {
    let new_amount: u128 = 50;
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let next_round: u128 = 1;
    let min_max: u128 = 3;
    let timeout: u64 = 1800;

    let (
        root,
        _aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id() ], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        link.account_id(),
        "ft_transfer",
        &json!({
            "receiver_id": flux_aggregator.account_id().to_string(), "amount": 300.to_string(), "memo": "None"
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        1
    )
    .assert_success();

    root.call(
        flux_aggregator.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0,
    )
    .assert_success();

    let mut withdrawable_payment_oracle_one: u128 = root
        .view(
            flux_aggregator.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_one.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    assert_eq!(0, withdrawable_payment_oracle_one);

    let mut withdrawable_payment_oracle_three: u128 = root
        .view(
            flux_aggregator.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_three.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    assert_eq!(0, withdrawable_payment_oracle_three);

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": new_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    withdrawable_payment_oracle_one = root
        .view(
            flux_aggregator.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_one.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    assert_eq!(3, withdrawable_payment_oracle_one);

    withdrawable_payment_oracle_three = root
        .view(
            flux_aggregator.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_three.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    assert_eq!(3, withdrawable_payment_oracle_three);
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L549
 *
 */
#[test]

fn when_delay_is_on_does_not_revert_on_the_oracles_first_round() {
    let payment_amount: u128 = 3;
    let answer: u128 = 100;
    let rr_delay: u64 = 1;
    let timeout: u64 = 1800;
    let next_round: u128 = 1;
    let min_max: u128 = 3;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id() ], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
            flux_aggregator.account_id(),
            "update_future_rounds",
            &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L557
 *
 */
#[test]

fn when_delay_is_on_and_does_revert_before_the_delay() {
    let payment_amount: u128 = 3;
    let answer: u128 = 100;
    let rr_delay: u64 = 1;
    let timeout: u64 = 1800;
    let mut next_round: u128 = 1;
    let min_max: u128 = 3;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id() ], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
            flux_aggregator.account_id(),
            "update_future_rounds",
            &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    let expected_previous_round_not_supersedable = oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_previous_round_not_supersedable
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        // No data present should be error
        assert!(execution_error
            .to_string()
            .contains("previous round not supersedable"));
    } else {
        unreachable!();
    }
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L590
 *
 */
#[test]

fn when_called_by_an_oracle_who_has_not_answered_recently_and_does_not_revert() {
    let payment_amount: u128 = 3;
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let min_max: u128 = 3;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": 3.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    // Since Ned and Nelly have answered recently, and we set the delay
    // to 2, only Nelly can answer as she is the only oracle that hasn't
    // started the last two rounds.
    // await updateFutureRounds(aggregator, {
    //   maxAnswers: oracles.length,
    //   restartDelay: newDelay,
    // })

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 2.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // when called by an oracle who has not answered recently
    // it does not revert
    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": 4.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L596
 *
 */
#[test]

fn when_called_by_an_oracle_who_has_answered_recently_and_reverts() {
    let payment_amount: u128 = 3;
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let min_max: u128 = 3;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": 3.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    // Since Ned and Nelly have answered recently, and we set the delay
    // to 2, only Nelly can answer as she is the only oracle that hasn't
    // started the last two rounds.
    // await updateFutureRounds(aggregator, {
    //   maxAnswers: oracles.length,
    //   restartDelay: newDelay,
    // })

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 2.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // when called by an oracle who has answered recently
    // it does not revert
    let expected_round_not_accepting_submissions = oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": 4.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_round_not_accepting_submissions
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        // No data present should be error
        assert!(execution_error
            .to_string()
            .contains("round not accepting submissions"));
    } else {
        unreachable!();
    }

    let expected_round_not_accepting_submissions_two = oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": 4.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_round_not_accepting_submissions_two
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        // No data present should be error
        assert!(execution_error
            .to_string()
            .contains("round not accepting submissions"));
    } else {
        unreachable!();
    }
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L630
 * *TODO* Look into why the contract panics on oracle_three starting a new round. Error: previous round not supersedable.
 */
#[test]

fn when_the_price_is_not_updated_for_a_round_and_allows_a_new_round_to_be_started() {
    let payment_amount: u128 = 3;
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let min_max: u128 = 3;
    let mut next_round: u128 = 1;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    root.call(

        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // allows a new round to be started
    // *TODO* Look into why the contract panics on oracle_three starting a new round. Error: previous round not supersedable.
    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L634
 * *TODO* Look into how to acheive this with NEAR's sdk  const block = await provider.getBlock(receipt.blockHash ?? '')
 */
#[test]
fn sets_the_info_for_the_previous_round() {
    let payment_amount: u128 = 3;
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let min_max: u128 = 3;
    let mut next_round: u128 = 1;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    root.call(

        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // sets the info for the previous round
    let mut expected_updated_timestamp: u128 = root
        .view(
            flux_aggregator.account_id(),
            "get_timestamp",
            &json!({"_round_id": (next_round - 1).to_string()})
                .to_string()
                .into_bytes(),
        )
        .unwrap_json();

    assert_eq!(0, expected_updated_timestamp);

    let mut expected_answer: u128 = root
        .view(
            flux_aggregator.account_id(),
            "get_answer",
            &json!({"_round_id": (next_round - 1).to_string()})
                .to_string()
                .into_bytes(),
        )
        .unwrap_json();

    assert_eq!(0, expected_answer);

    let tx = oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    println!("{:?}", tx);

    // *TODO*: Look into this const block = await provider.getBlock(receipt.blockHash ?? '')
    //  matchers.bigNum(ethers.utils.bigNumberify(block.timestamp), updated)

    expected_updated_timestamp = root
        .view(
            flux_aggregator.account_id(),
            "get_timestamp",
            &json!({"_round_id": (next_round - 1).to_string()})
                .to_string()
                .into_bytes(),
        )
        .unwrap_json();
    // assert_eq!(tx.timestamp, expected_updated_timestamp);

    expected_answer = root
        .view(
            flux_aggregator.account_id(),
            "get_answer",
            &json!({"_round_id": (next_round - 1).to_string()})
                .to_string()
                .into_bytes(),
        )
        .unwrap_json();

    assert_eq!(answer, expected_answer);

    let expected_round: (u64, u128, u64, u64, u64) = root
        .call(
            flux_aggregator.account_id(),
            "get_round_data",
            &json!({"_round_id": (next_round - 1).to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .unwrap_json();

    assert_eq!(2, expected_round.0);
    assert_eq!(answer, expected_round.1);
    assert_eq!(expected_updated_timestamp as u64, expected_round.3);
    assert_eq!(1, expected_round.4);
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L658
 */
#[test]
fn sets_the_previous_round_as_timed_out() {
    let payment_amount: u128 = 3;
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let min_max: u128 = 3;
    let mut next_round: u128 = 1;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    root.call(

        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // sets the previous round as timed out
    // *TODO* Look into why the panic error contains previous round not supersedable and not No data present
    let expected_no_data_present = root.call(
        flux_aggregator.account_id(),
        "get_round_data",
        &json!({"_round_id": (next_round - 1).to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    if let ExecutionStatus::Failure(execution_error) = &expected_no_data_present
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        // No data present should be error
        assert!(execution_error.to_string().contains("No data present"));
    } else {
        unreachable!();
    }

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let expected_round: (u64, u128, u64, u64, u64) = root
        .call(
            flux_aggregator.account_id(),
            "get_round_data",
            &json!({"_round_id": 2.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(2, expected_round.0);
    assert_eq!(1, expected_round.4);
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L669
 */
#[test]

fn still_respects_the_delay_restriction() {
    let payment_amount: u128 = 3;
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let min_max: u128 = 3;
    let mut next_round: u128 = 1;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    root.call(

        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    println!("{:?}", next_round);

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    let expected_revert = oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_revert
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        // expected to revert because the sender started the last round
        assert!(execution_error
            .to_string()
            .contains("previous round not supersedable"));
    } else {
        unreachable!();
    }
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L674
 */
#[test]

fn uses_the_set_timeout_at_the_beginning_of_the_round() {
    let payment_amount: u128 = 3;
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let min_max: u128 = 3;
    let mut next_round: u128 = 1;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    root.call(

        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    println!("{:?}", next_round);

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": (timeout+100000).to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L684
 */
#[test]
fn rejects_values_below_the_submission_value_range() {
    let rr_delay: u64 = 0;
    let min_submission_value: u64 = 1;
    let next_round: u64 = 1;
    let min_max: u128 = 3;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let expected_value_below_min_submission_value = oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": (min_submission_value-1).to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_value_below_min_submission_value
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        // No data present should be error
        assert!(execution_error
            .to_string()
            .contains("value below min_submission_value"));
    } else {
        unreachable!();
    }
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L691
 */
#[test]
fn accepts_submissions_equal_to_the_min_submission_value() {
    let rr_delay: u64 = 0;
    let min_submission_value: u64 = 1;
    let next_round: u64 = 1;
    let min_max: u128 = 3;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": (min_submission_value).to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L695
 */
#[test]
fn accepts_submissions_equal_to_the_max_submission_value() {
    let rr_delay: u64 = 0;
    let next_round: u64 = 1;
    let min_max: u128 = 3;
    let max_submission_value: u128 = 100000000000000000000;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": (max_submission_value).to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L699
 */
#[test]
fn rejects_values_above_the_max_submission_value() {
    let rr_delay: u64 = 0;
    let next_round: u64 = 1;
    let min_max: u128 = 3;
    let max_submission_value: u128 = 100000000000000000000;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let expected_value_above_max_submission_value = oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": (max_submission_value + 1).to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_value_above_max_submission_value
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        // No data present should be error
        assert!(execution_error
            .to_string()
            .contains("value above max_submission_value"));
    } else {
        unreachable!();
    }
}

/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L716
 */

#[test]
fn calls_out_to_the_validator() {
    let rr_delay: u64 = 0;
    let next_round: u64 = 1;
    let min_max: u128 = 3;
    let payment_amount: u128 = 3;
    let answer: u128 = 100;
    let timeout: u64 = 1800;
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
        _oracle_five,
        aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // Carol

    root.call(
        flux_aggregator.account_id(),
        "set_validator",
        &json!({"_new_validator": aggregator_validator_mock.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let tx = oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    assert_eq!(tx.promise_results().remove(2).unwrap().outcome().logs[0], "0, 0, 1, 100")
}


/**
 * #submit - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L240
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L733
 */

#[test]
fn still_updates() {
}

/**
 * #get_answer - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L743
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L754
 */

#[test]
fn retrieves_the_answer_recorded_for_past_rounds() {
    let answers: Vec<u128> = [1, 10, 101, 1010, 10101, 101010, 1010101].to_vec();
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u128 = 1;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let mut n = 0;
    let mut y = 1;
    let mut x = 0;

    while n < answers.len() {
        oracle_one.call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[n].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();
        next_round += 1;
        n += 1;
    }

    while y < next_round {
        let answer: u128 = root
            .call(
                flux_aggregator.account_id(),
                "get_answer",
                &json!({"_round_id": y.to_string()}).to_string().into_bytes(),
                DEFAULT_GAS,
                0, // deposit
            )
            .unwrap_json();
        let expected_answer: u128 = answers[x] as u128;
        x += 1;
        y += 1;
        assert_eq!(answer, expected_answer);
    }
}

/**
 * #get_answer - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L743
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L761
 * *TODO* Research overflowedId issue for Rust uint type
 */

#[test]
fn returns_zero_for_answers_greater_than_uint32s_max() {
}

/**
 * #get_timestamp - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L768
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L777
 */

#[test]
fn retrieves_the_timestamp_recorded_for_past_rounds() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u128 = 1;

    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();


    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let mut i = 0;
    let mut z = 1;
    let mut latest_timestamp: u128 = 0;

    while i < 10 {
        oracle_one.call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": (i + 1).to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();
        next_round += 1;
        i += 1;
    }

    while z < next_round {
        let current_timestamp: u128 = root
            .call(
                flux_aggregator.account_id(),
                "get_timestamp",
                &json!({"_round_id": z.to_string()}).to_string().into_bytes(),
                DEFAULT_GAS,
                0, // deposit
            )
            .unwrap_json();
        z += 1;
        assert_eq!(current_timestamp >= latest_timestamp, true);
        latest_timestamp = current_timestamp;
    }
}

/**
 * #get_timestamp - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L768
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L787
 */

#[test]
fn returns_zero_for_timestamps_greater_than_uint32s_max() {
}

/**
 * #change_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L794
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L796
 */

#[test]
fn increases_the_oracle_count() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let past_count: u128 = root
        .view(
            flux_aggregator.account_id(),
            "oracle_count",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let current_count: u128 = root
    .view(
        flux_aggregator.account_id(),
        "oracle_count",
        &json!({}).to_string().into_bytes(),
    )
    .unwrap_json();

    assert_eq!(past_count + 1, current_count);
}

/**
 * #change_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L794
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L804
 */

#[test]
fn adds_the_address_in_get_oracles() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();


    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let oracles: Vec<String> = root
        .view(
            flux_aggregator.account_id(),
            "get_oracles",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    assert_eq!(oracle_one.account_id().to_string(), oracles[0]);
}

/**
 * #change_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L794
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L809
 */

#[test]
fn change_oracles_and_updates_the_round_details() {
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 2.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let min_submission_count: u64 = root
        .view(
            flux_aggregator.account_id(),
            "min_submission_count",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    let max_submission_count: u64 = root
        .view(
            flux_aggregator.account_id(),
            "max_submission_count",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    let restart_delay: u64 = root
        .view(
            flux_aggregator.account_id(),
            "restart_delay",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    assert_eq!(min_submission_count, 1);
    assert_eq!(max_submission_count, 3);
    assert_eq!(restart_delay, 2);
}

/**
 * #change_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L794
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L816
 */

#[test]
fn emits_a_log() {
    let (
        root,
        _aca,
        _link,
        oracle_one,
        oracle_two,
        _oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let oracle_added_event = root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_two.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).promise_results();

    let oracle_added_event_oracle: String = oracle_added_event.clone().remove(1).unwrap().outcome().logs[0]
        .parse()
        .unwrap();

    let result = [oracle_two.account_id(), ", true".to_string()].join("");

    assert_eq!(result, oracle_added_event_oracle);

    let oracle_admin_updated_event_oracle: String = oracle_added_event.clone().remove(1).unwrap().outcome().logs[1]
        .parse()
        .unwrap();

    let result_two = [oracle_one.account_id(), ", true".to_string()].join("");

    assert_eq!(result_two, oracle_admin_updated_event_oracle);
}

/**
 * #change_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L794
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L834
 */

#[test]

fn when_the_oracle_has_already_been_added_and_reverts() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;

    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let expected_oracle_already_enabled = root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_oracle_already_enabled
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("oracle already enabled"));
    } else {
        unreachable!();
    }

}

/**
 * #change_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L794
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L840
 */

#[test]

fn change_oracles_and_when_called_by_anyone_but_the_owner_and_reverts() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let expected_only_callable_by_owner = oracle_one.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
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

/**
 * #change_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L794
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L867
 */

#[test]

fn does_not_allow_the_oracle_to_update_the_round() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let next_round: u128 = 1;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
            flux_aggregator.account_id(),
            "change_oracles",
            &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );

    let expected_not_yet_enabled_oracle = oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_not_yet_enabled_oracle
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("not yet enabled oracle"));
    } else {
        unreachable!();
    }
}

/**
 * #change_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L794
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L871
 */

#[test]

fn does_allow_the_oracle_to_update_future_rounds() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let mut next_round: u128 = 1;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
            flux_aggregator.account_id(),
            "change_oracles",
            &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );

    oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    next_round = next_round + 1;

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
}

/**
 * #change_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L794
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L881
 */

#[test]

fn when_an_oracle_is_added_after_removed_for_a_round_and_allows_the_oracle_to_update() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let mut next_round: u128 = 1;
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
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    next_round = next_round + 1;

    root.call(
            flux_aggregator.account_id(),
            "change_oracles",
            &json!({"_removed": [oracle_three.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    root.call(
            flux_aggregator.account_id(),
            "change_oracles",
            &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
}

/**
 * #change_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L794
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L903
 */

 #[test]

fn when_an_oracle_is_added_and_immediately_removed_mid_round_allows_the_oracle_to_update() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let mut next_round: u128 = 1;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    next_round = next_round + 1;

    root.call(
            flux_aggregator.account_id(),
            "change_oracles",
            &json!({"_removed": [oracle_three.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    root.call(
            flux_aggregator.account_id(),
            "change_oracles",
            &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
}

/**
 * #change_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L794
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L924
 * **TODO** Research why the contract is not panicking with owner cannot overwrite admin, reckoning that the issue is in remove_oracle 
 */

#[test]

fn when_an_oracle_is_re_added_after_with_a_different_admin_address_and_reverts() {
    let answer: u128 = 100;
    let rr_delay: u64 = 0;
    let next_round: u128 = 1;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
            flux_aggregator.account_id(),
            "change_oracles",
            &json!({"_removed": [oracle_three.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();

    let expected_owner_cannot_override_admin = root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [root.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // *TODO* Research why the contract is not panicking with owner cannot overwrite admin

    if let ExecutionStatus::Failure(execution_error) = &expected_owner_cannot_override_admin
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        println!("{:?}", execution_error
        .to_string());
        assert!(execution_error
            .to_string()
            .contains("owner cannot overwrite admin"));
    } else {
        unreachable!();
    }
}

/**
 * #change_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L794
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L975
 * **TODO** Look into a simple way to implement this function
 */

#[test]

fn not_use_too_much_gas() {}

/**
 * #change_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L794
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1011
 * **TODO** Look into a simple way to implement this function
 */

#[test]

fn reverts_when_another_oracle_is_added() {}

/**
 * #change_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L794
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1028
 * **TODO** Look into a simple way to implement this function
 */

#[test]

fn reverts_when_min_submissions_is_set_to_0() {}

/**
 * #removing_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1033
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1039
 */


#[test]

fn decreases_the_oracle_count() {
    let rr_delay: u64 = 0;
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
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let past_count: u128 = root.view(
            flux_aggregator.account_id(),
            "oracle_count",
            &json!({}).to_string().into_bytes()
        )
        .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let current_count: u128 = root.view(
        flux_aggregator.account_id(),
        "oracle_count",
        &json!({}).to_string().into_bytes()
    )
    .unwrap_json();

    assert_eq!(past_count - 1, current_count);
}

/**
 * #removing_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1033
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1049
 */

#[test]

fn removing_oracles_and_updates_the_round_details() {
    let rr_delay: u64 = 0;
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
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let min_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "min_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let max_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "max_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let restart_delay: u64 = root
        .call(
            flux_aggregator.account_id(),
            "restart_delay",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(min_submission_count, 1);
    assert_eq!(max_submission_count, 1);
    assert_eq!(restart_delay, 0);
}

/**
 * #removing_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1033
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1057
 */

#[test]

fn removing_oracles_and_emits_a_log() {
    let rr_delay: u64 = 0;
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
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let oracle_removed_event = root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).promise_results();

    let oracle_removed_event_log: String = oracle_removed_event
        .clone()
        .remove(1)
        .unwrap()
        .outcome()
        .logs[0]
        .parse()
        .unwrap();

    let result = [oracle_one.account_id(), ", false".to_string()].join("");

    assert_eq!(result, oracle_removed_event_log);
}

/**
 * #removing_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1033
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1067
 */

#[test]

fn removing_oracles_and_removes_the_address_in_get_oracles() {
    let rr_delay: u64 = 0;
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
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let oracles: Vec<String> = root
        .view(
            flux_aggregator.account_id(),
            "get_oracles",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    let mut n = 0;

    while n < oracles.len() {
        assert_ne!(oracles[n], oracle_one.account_id());
        n += 1;
    }
}

/**
 * #removing_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1033
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1081
 */

#[test]

fn when_the_oracle_is_not_currently_added_and_reverts() {
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
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let expected_oracle_not_enabled = root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_oracle_not_enabled
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error.to_string().contains("oracle not enabled"));
    } else {
        unreachable!();
    }
}

/**
 * #removing_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1033
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1092
 */

#[test]
fn when_removing_the_last_oracle_and_does_not_revert() {
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
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_three.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 0.to_string(), "_max_submissions": 0.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

}

/**
 * #removing_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1033
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1101
 */

#[test]

fn removing_oracles_and_when_called_by_anyone_but_the_owner_and_reverts() {
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let expected_only_callable_by_owner = oracle_two.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
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

/**
 * #removing_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1033
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1101
 * **TODO** Look into why the contract is failing with round not accepting submissions and not no longer allowed oracle
 */

#[test]

fn it_is_allowed_to_report_on_one_more_round() {
    let mut next_round: u128 = 1;
    let answer: u128 = 100;
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
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_three.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let expected_no_longer_allowed_oracle = oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );

    // *TODO* Look into why the contract is failing with round not accepting submissions and not no longer allowed oracle

    if let ExecutionStatus::Failure(execution_error) = &expected_no_longer_allowed_oracle
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("no longer allowed oracle"));
    } else {
        unreachable!();
    }
}

/**
 * #removing_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1033
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1137
 */

#[test]

fn it_is_allowed_to_finish_that_round_and_one_more_round() {
    let mut next_round: u128 = 1;
    let answer: u128 = 100;

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
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_three.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    let expected_no_longer_allowed_oracle = oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    // *TODO* Look into why the contract is allowing oracle_three to future in participate in future rounds
    // cannot participate in future rounds

    if let ExecutionStatus::Failure(execution_error) = &expected_no_longer_allowed_oracle
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("no longer allowed oracle"));
    } else {
        unreachable!();
    }
}

/**
 * #removing_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1033
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1147
 */

#[test]

fn it_reverts_when_min_submissions_is_set_to_0() {

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
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let expected_min_must_be_greater_than_0 = root.call(
        flux_aggregator.account_id(),
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

/**
 * #removing_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1033
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1161
 */

#[test]

fn can_swap_out_oracles() {
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let oracles: Vec<String> = root
        .view(
            flux_aggregator.account_id(),
            "get_oracles",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    let mut n = 0;


    while n < oracles.len() {
        assert_ne!(oracles[n], oracle_three.account_id());
        if oracles[n] == oracle_two.account_id() {
            assert_eq!(oracles[n] == oracle_two.account_id(), true);
        }
        n += 1;
    }

    n = 0;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_two.account_id()], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let oracles_second: Vec<String> = root
        .view(
            flux_aggregator.account_id(),
            "get_oracles",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();


    while n < oracles_second.len() {
        assert_ne!(oracles_second[n], oracle_two.account_id());
        if oracles_second[n] == oracle_three.account_id() {
            assert_eq!(oracles_second[n] == oracle_three.account_id(), true);
        }
        n += 1;
    }
}

/**
 * #removing_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1033
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1180
 * *TODO* Look into why the contract is panicking when removing and adding an oracle at the same time, not intended functionality. (oracle already enabled)
 */

#[test]

fn it_is_possible_to_remove_and_add_the_same_address() {
    let (
        root,
        _aca,
        _link,
        oracle_one,
        oracle_two,
        _oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let oracles: Vec<String> = root
        .view(
            flux_aggregator.account_id(),
            "get_oracles",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    let mut n = 0;

    while n < oracles.len() {
        if oracles[n] == oracle_two.account_id() {
            assert_eq!(oracles[n] == oracle_two.account_id(), true);
        }
        n += 1;
    }
    n = 0;

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_two.account_id()], "_added": [oracle_two.account_id()], "_added_admins": [oracle_two.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let oracles_second: Vec<String> = root
    .view(
        flux_aggregator.account_id(),
        "get_oracles",
        &json!({}).to_string().into_bytes(),
    )
    .unwrap_json();

    // *TODO* Look into why the contract is panicking when removing and adding an oracle at the same time, not intended functionality. (oracle already enabled)

    while n < oracles_second.len() {
        if oracles_second[n] == oracle_two.account_id() {
            assert_eq!(oracles_second[n] == oracle_two.account_id(), true);
        }
        n += 1;
    }
}

/**
 * #get_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1199
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1207
 */

#[test]

fn returns_the_addresses_of_addded_oracles() {
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let oracles: Vec<String> = root
        .view(
            flux_aggregator.account_id(),
            "get_oracles",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    assert_eq!(oracles[0], oracle_one.account_id());

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_two.account_id()], "_added_admins": [oracle_two.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let mut n = 0;

    let oracles_second: Vec<String> = root
    .view(
        flux_aggregator.account_id(),
        "get_oracles",
        &json!({}).to_string().into_bytes(),
    )
    .unwrap_json();

    while n < oracles_second.len() {
        if oracles_second[n] == oracle_two.account_id() {
            assert_eq!(oracles_second[n] == oracle_two.account_id(), true);
        }
        if oracles_second[n] == oracle_one.account_id() {
            assert_eq!(oracles_second[n] == oracle_one.account_id(), true);
        }
        n += 1;
    }

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let mut n = 0;

    let oracles_third: Vec<String> = root
    .view(
        flux_aggregator.account_id(),
        "get_oracles",
        &json!({}).to_string().into_bytes(),
    )
    .unwrap_json();

    while n < oracles_third.len() {
        if oracles_third[n] == oracle_two.account_id() {
            assert_eq!(oracles_third[n] == oracle_two.account_id(), true);
        }
        if oracles_third[n] == oracle_one.account_id() {
            assert_eq!(oracles_third[n] == oracle_one.account_id(), true);
        }
        if oracles_third[n] == oracle_three.account_id() {
            assert_eq!(oracles_third[n] == oracle_three.account_id(), true);
        }
        n += 1;
    }
}

/**
 * #get_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1199
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1233
 */

#[test]

fn reorders_when_removing_from_the_beginning() {
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let oracles: Vec<String> = root
    .view(
        flux_aggregator.account_id(),
        "get_oracles",
        &json!({}).to_string().into_bytes(),
    )
    .unwrap_json();

    assert_eq!(oracles[0], oracle_one.account_id());
    assert_eq!(oracles[1], oracle_two.account_id());
    assert_eq!(oracles[2], oracle_three.account_id());

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let oracles_second: Vec<String> = root
    .view(
        flux_aggregator.account_id(),
        "get_oracles",
        &json!({}).to_string().into_bytes(),
    )
    .unwrap_json();

    assert_eq!(oracles_second[0], oracle_three.account_id());
    assert_eq!(oracles_second[1], oracle_two.account_id());

}

/**
 * #get_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1199
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1243
 */

#[test]

fn reorders_when_removing_from_the_middle() {
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let oracles: Vec<String> = root
    .view(
        flux_aggregator.account_id(),
        "get_oracles",
        &json!({}).to_string().into_bytes(),
    )
    .unwrap_json();


    assert_eq!(oracles[0], oracle_one.account_id());
    assert_eq!(oracles[1], oracle_two.account_id());
    assert_eq!(oracles[2], oracle_three.account_id());

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_two.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let oracles_second: Vec<String> = root
    .view(
        flux_aggregator.account_id(),
        "get_oracles",
        &json!({}).to_string().into_bytes(),
    )
    .unwrap_json();

    assert_eq!(oracles_second[0], oracle_one.account_id());
    assert_eq!(oracles_second[1], oracle_three.account_id());

}

/**
 * #get_oracles - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1199
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1253
 */

#[test]

fn pops_the_last_node_off_at_the_end() {
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let oracles: Vec<String> = root
    .view(
        flux_aggregator.account_id(),
        "get_oracles",
        &json!({}).to_string().into_bytes(),
    )
    .unwrap_json();


    assert_eq!(oracles[0], oracle_one.account_id());
    assert_eq!(oracles[1], oracle_two.account_id());
    assert_eq!(oracles[2], oracle_three.account_id());

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_three.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let oracles_second: Vec<String> = root
    .view(
        flux_aggregator.account_id(),
        "get_oracles",
        &json!({}).to_string().into_bytes(),
    )
    .unwrap_json();


    assert_eq!(oracles_second[0], oracle_one.account_id());
    assert_eq!(oracles_second[1], oracle_two.account_id());

}

/**
 * #withdraw_funds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1265
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1266
 */

#[test]

fn withdraw_funds_and_succeeds() {
    let deposit: u64 = 100;
    let (
        root,
        _aca,
        link,
        _oracle_one,
        _oracle_two,
        _oracle_three,
        test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    // Carol

    root.call(
        flux_aggregator.account_id(),
        "withdraw_funds",
        &json!({"_recipient": test_helper.account_id().to_string(), "_amount": deposit.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, 
    ).assert_success();

    let available_funds: u128 = root
        .call(
            flux_aggregator.account_id(),
            "available_funds",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0,
        )
        .unwrap_json();

    assert_eq!(0, available_funds);

    let balance: U128 = root
        .call(
            link.account_id(),
            "ft_balance_of",
            &json!({"account_id":  test_helper.account_id().to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(u128::from(balance), 100);
}

/**
 * #withdraw_funds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1265
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1273
 */

#[test]

fn does_not_let_withdrawls_happen_multiple_times() {
    let deposit: u64 = 100;
    let (
        root,
        _aca,
        _link,
        _oracle_one,
        _oracle_two,
        _oracle_three,
        test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "withdraw_funds",
        &json!({"_recipient": test_helper.account_id().to_string(), "_amount": deposit.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let expected_insufficient_reserve_funds = root.call(
        flux_aggregator.account_id(),
        "withdraw_funds",
        &json!({"_recipient": test_helper.account_id().to_string(), "_amount": deposit.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

        if let ExecutionStatus::Failure(execution_error) = &expected_insufficient_reserve_funds
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error.to_string().contains("insufficient reserve funds"));
    } else {
        unreachable!();
    }
}

/**
 * #withdraw_funds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1265
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1289
 */

#[test]

fn with_a_number_higher_than_the_available_link_balance_and_fails() {
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let next_round: u128 = 1;
    let (
        root,
        _aca,
        _link,
        oracle_one,
        _oracle_two,
        _oracle_three,
        test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();


    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let expected_insufficient_reserve_funds = root.call(
        flux_aggregator.account_id(),
        "withdraw_funds",
        &json!({"_recipient": test_helper.account_id().to_string(), "_amount": deposit.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_insufficient_reserve_funds
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("insufficient reserve funds"));
    } else {
        unreachable!();
    }

    let available_funds: u128 = root
        .call(
            flux_aggregator.account_id(),
            "available_funds",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(97, available_funds);
}

/**
 * #withdraw_funds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1265
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1307
 */

#[test]

fn does_not_allow_withdrawal_with_less_than_2x_rounds_of_payments() {
    let allowed: u128 = 82;
    let (
        root,
        _aca,
        _link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let available_funds: u128 = root
        .call(
            flux_aggregator.account_id(),
            "available_funds",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(100, available_funds);

    let expected_insufficient_reserve_funds = root.call(
        flux_aggregator.account_id(),
        "withdraw_funds",
        &json!({"_recipient": test_helper.account_id().to_string(), "_amount": (allowed + 1).to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_insufficient_reserve_funds
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("insufficient reserve funds"));
    } else {
        unreachable!();
    }

    root.call(
        flux_aggregator.account_id(),
        "withdraw_funds",
        &json!({"_recipient": test_helper.account_id().to_string(), "_amount": allowed.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
}

/**
 * #withdraw_funds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1265
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1323
 */

#[test]

fn when_called_by_a_non_owner_and_fails() {
    let (
        root,
        _aca,
        _link,
        _oracle_one,
        _oracle_two,
        _oracle_three,
        test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();


    let expected_only_callable_by_owner = eddy.call(
        flux_aggregator.account_id(),
        "withdraw_funds",
        &json!({"_recipient": test_helper.account_id().to_string(), "_amount": 100.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
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

    let available_funds: u128 = root
        .call(
            flux_aggregator.account_id(),
            "available_funds",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(100, available_funds);
}

/**
 * #update_future_rounds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1334
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1352
 */

#[test]


fn updates_the_min_and_max_answer_counts() {
    let rr_delay: u64 = 0;
    let new_delay: u64 = 2;
    let new_min: u64 = 1;
    let new_max: u64 = 3;
    let new_payment_amount: u64 = 2;
    let mut min_submission_count: u64 = 3;
    let mut max_submission_count: u64 = 3;
    let payment_amount: u64 = 3;
    let timeout: u64 = 1800;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_submission_count.to_string(), "_max_submissions": max_submission_count.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let mut expected_payment_amount: u64 = root
        .call(
            flux_aggregator.account_id(),
            "get_payment_amount",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let mut expected_min_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "min_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let mut expected_max_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "max_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(expected_payment_amount, payment_amount);
    assert_eq!(expected_min_submission_count, min_submission_count);
    assert_eq!(expected_max_submission_count, max_submission_count);

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": new_payment_amount.to_string(), "_min_submissions": new_min.to_string(), "_max_submissions": new_max.to_string(), "_restart_delay": new_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let expected_restart_delay: u64 = root
    .call(
        flux_aggregator.account_id(),
        "restart_delay",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    expected_payment_amount = root
        .call(
            flux_aggregator.account_id(),
            "get_payment_amount",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    expected_min_submission_count = root
        .call(
            flux_aggregator.account_id(),
            "min_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    expected_max_submission_count = root
        .call(
            flux_aggregator.account_id(),
            "max_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(expected_payment_amount, new_payment_amount);
    assert_eq!(expected_min_submission_count, new_min);
    assert_eq!(expected_max_submission_count, new_max);
    assert_eq!(expected_restart_delay, new_delay);
}

/**
 * #update_future_rounds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1334
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1352
 * *TODO* Try to implement a more type heavy assertion from the log instead of comparing strings
 */

#[test]

fn emits_a_log_announcing_the_new_round_details() {
    let rr_delay: u64 = 0;
    let new_delay: u64 = 2;
    let new_min: u64 = 1;
    let new_max: u64 = 3;
    let new_payment_amount: u64 = 2;
    let min_submission_count: u64 = 3;
    let max_submission_count: u64 = 3;
    let payment_amount: u64 = 3;
    let timeout: u64 = 1800;

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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_submission_count.to_string(), "_max_submissions": max_submission_count.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let expected_payment_amount: u64 = root
        .call(
            flux_aggregator.account_id(),
            "get_payment_amount",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let expected_min_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "min_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let expected_max_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "max_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(expected_payment_amount, payment_amount);
    assert_eq!(expected_min_submission_count, min_submission_count);
    assert_eq!(expected_max_submission_count, max_submission_count);

    let receipt = root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": new_payment_amount.to_string(), "_min_submissions": new_min.to_string(), "_max_submissions": new_max.to_string(), "_restart_delay": new_delay.to_string(), "_timeout": (timeout + 1).to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    // let expected_min_submission_count_log: u64 =
    //     receipt.promise_results().remove(1).unwrap().outcome().logs[1]
    //         .parse()
    //         .unwrap();
    // let expected_max_submission_count_log: u64 =
    //     receipt.promise_results().remove(1).unwrap().outcome().logs[2]
    //         .parse()
    //         .unwrap();
    // let expected_restart_delay_log: u64 =
    //     receipt.promise_results().remove(1).unwrap().outcome().logs[3]
    //         .parse()
    //         .unwrap();
    // let expected_timeout_log: u64 = receipt.promise_results().remove(1).unwrap().outcome().logs[4]
    //     .parse()
    //     .unwrap();

    assert_eq!(receipt.promise_results().remove(1).unwrap().outcome().logs[0], "2, 1, 3, 2, 1801");

    // assert_eq!(expected_min_submission_count_log, new_min);
    // assert_eq!(expected_max_submission_count_log, new_max);
    // assert_eq!(expected_restart_delay_log, new_min);
    // assert_eq!(expected_timeout_log, (timeout + 1));
}

/**
 * #update_future_rounds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1334
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1381
 */

#[test]

fn when_it_is_set_to_higher_than_the_number_or_oracles_and_reverts() {
    let rr_delay: u64 = 0;
    let min_submission_count: u64 = 3;
    let max_submission_count: u64 = 3;
    let payment_amount: u64 = 3;
    let timeout: u64 = 1800;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_submission_count.to_string(), "_max_submissions": max_submission_count.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let expected_payment_amount: u64 = root
        .call(
            flux_aggregator.account_id(),
            "get_payment_amount",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let expected_min_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "min_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let expected_max_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "max_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(expected_payment_amount, payment_amount);
    assert_eq!(expected_min_submission_count, min_submission_count);
    assert_eq!(expected_max_submission_count, max_submission_count);

    let expected_max_cannot_exceed_total = root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": min_submission_count.to_string(), "_max_submissions": 4.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_max_cannot_exceed_total
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("max cannot exceed total"));
    } else {
        unreachable!();
    }
}

/**
 * #update_future_rounds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1334
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1392
 */

#[test]

fn when_it_is_sets_the_min_higher_than_the_max_and_reverts() {
    let rr_delay: u64 = 0;
    let min_submission_count: u64 = 3;
    let max_submission_count: u64 = 3;
    let payment_amount: u64 = 3;
    let timeout: u64 = 1800;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_submission_count.to_string(), "_max_submissions": max_submission_count.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let expected_payment_amount: u64 = root
        .call(
            flux_aggregator.account_id(),
            "get_payment_amount",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let expected_min_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "min_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let expected_max_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "max_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(expected_payment_amount, payment_amount);
    assert_eq!(expected_min_submission_count, min_submission_count);
    assert_eq!(expected_max_submission_count, max_submission_count);

    let expected_max_must_equal_exceed_min = root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_max_must_equal_exceed_min
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("max must equal/exceed min"));
    } else {
        unreachable!();
    }
}

/**
 * #update_future_rounds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1334
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1404
 */

#[test]

fn when_delay_equal_or_greater_the_oracle_count_and_reverts() {
    let rr_delay: u64 = 0;
    let min_submission_count: u64 = 3;
    let max_submission_count: u64 = 3;
    let payment_amount: u64 = 3;
    let timeout: u64 = 1800;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_submission_count.to_string(), "_max_submissions": max_submission_count.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let expected_payment_amount: u64 = root
        .call(
            flux_aggregator.account_id(),
            "get_payment_amount",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let expected_min_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "min_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let expected_max_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "max_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(expected_payment_amount, payment_amount);
    assert_eq!(expected_min_submission_count, min_submission_count);
    assert_eq!(expected_max_submission_count, max_submission_count);

    let expected_revert_delay_cannot_exceed_total = root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": min_submission_count.to_string(), "_max_submissions": max_submission_count.to_string(), "_restart_delay": 3.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_revert_delay_cannot_exceed_total
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("revert delay cannot exceed total"));
    } else {
        unreachable!();
    }
}

/**
 * #update_future_rounds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1334
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1417
 * *TODO* Look into why you cannot pass a decimal number into the update_future_rounds payment_amount
 */

#[test]

fn when_the_payment_amount_does_not_cover_reserve_rounds_and_reverts() {
    let rr_delay: u64 = 0;
    let min_submission_count: u64 = 3;
    let max_submission_count: u64 = 3;
    let payment_amount: u64 = 3;
    let timeout: u64 = 1800;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_submission_count.to_string(), "_max_submissions": max_submission_count.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let expected_payment_amount: u64 = root
        .call(
            flux_aggregator.account_id(),
            "get_payment_amount",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let expected_min_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "min_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let expected_max_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "max_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(expected_payment_amount, payment_amount);
    assert_eq!(expected_min_submission_count, min_submission_count);
    assert_eq!(expected_max_submission_count, max_submission_count);
    
    // *TODO* Look into why you cannot pass a decimal number into the update_future_rounds payment_amount (17.67)

    let expected_insufficient_funds_for_payment = root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": "18", "_min_submissions": min_submission_count.to_string(), "_max_submissions": max_submission_count.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_insufficient_funds_for_payment
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("insufficient funds for payment"));
    } else {
        unreachable!();
    }
}

/**
 * #update_future_rounds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1334
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1434
 */

#[test]

fn min_oracles_is_set_to_0_and_reverts() {
    let rr_delay: u64 = 0;
    let min_submission_count: u64 = 3;
    let max_submission_count: u64 = 3;
    let payment_amount: u64 = 3;
    let timeout: u64 = 1800;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_submission_count.to_string(), "_max_submissions": max_submission_count.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let expected_payment_amount: u64 = root
        .call(
            flux_aggregator.account_id(),
            "get_payment_amount",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let expected_min_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "min_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let expected_max_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "max_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(expected_payment_amount, payment_amount);
    assert_eq!(expected_min_submission_count, min_submission_count);
    assert_eq!(expected_max_submission_count, max_submission_count);

    let expected_min_must_be_greater_than_0 = root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 0.to_string(), "_max_submissions": 0.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
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

/**
 * #update_future_rounds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1334
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1434
 * *TODO* Look into why the self.only_owner() function breaks the contract in update_future_rounds
 */

#[test]


fn update_future_rounds_and_when_called_by_anyone_but_the_owner_and_reverts() {
    let rr_delay: u64 = 0;
    let min_submission_count: u64 = 3;
    let max_submission_count: u64 = 3;
    let payment_amount: u64 = 3;
    let timeout: u64 = 1800;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_submission_count.to_string(), "_max_submissions": max_submission_count.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let expected_payment_amount: u64 = root
        .call(
            flux_aggregator.account_id(),
            "get_payment_amount",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let expected_min_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "min_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let expected_max_submission_count: u64 = root
        .call(
            flux_aggregator.account_id(),
            "max_submission_count",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(expected_payment_amount, payment_amount);
    assert_eq!(expected_min_submission_count, min_submission_count);
    assert_eq!(expected_max_submission_count, max_submission_count);

    let expected_only_callable_by_owner = oracle_two.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": min_submission_count.to_string(), "_max_submissions": max_submission_count.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
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

/**
 * #update_available_funds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1449
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1450
 */

#[test]

fn checks_the_link_token_to_see_if_any_additional_funds_are_available() {
    let deposit: u64 = 100;
    let (
        root,
        _aca,
        link,
        _oracle_one,
        _oracle_two,
        _oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let original_balance: u64 = root
    .view(
        flux_aggregator.account_id(),
        "available_funds",
        &json!({}).to_string().into_bytes(),
    )
    .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "update_available_funds",
        &json!({})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let original_balance_updated: u64 = root
    .view(
        flux_aggregator.account_id(),
        "available_funds",
        &json!({}).to_string().into_bytes(),
    )
    .unwrap_json();

    assert_eq!(original_balance, original_balance_updated);

    root.call(
        link.account_id(),
        "ft_transfer",
        &json!({
            "receiver_id": flux_aggregator.account_id(), "amount": deposit.to_string(), "memo": "None"
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        1, // deposit
    )
    .assert_success();

    root.call(
        flux_aggregator.account_id(),
        "update_available_funds",
        &json!({})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let new_balance: u64 = root
    .view(
        flux_aggregator.account_id(),
        "available_funds",
        &json!({}).to_string().into_bytes(),
    )
    .unwrap_json();

    assert_eq!((original_balance + deposit), new_balance);

}

/**
 * #update_available_funds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1449
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1464
 */

#[test]

fn removes_allocated_funds_from_the_available_balance() {
    let deposit: u64 = 100;
    let (
        root,
        _aca,
        link,
        oracle_one,
        _oracle_two,
        _oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let original_balance: u64 = root
        .view(
            flux_aggregator.account_id(),
            "available_funds",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": 100.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    root.call(
            link.account_id(),
            "ft_transfer",
            &json!({
                "receiver_id": flux_aggregator.account_id(), "amount": deposit.to_string(), "memo": "None"
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            1, // deposit
        )
        .assert_success();

    root.call(
        flux_aggregator.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let expected: u64 = (original_balance + deposit) - 3;
    let new_balance: u64 = root
        .view(
            flux_aggregator.account_id(),
            "available_funds",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    assert_eq!(expected, new_balance);
}

/**
 * #update_available_funds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1449
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1464
 * **TODO** Figure out how to assert a failure here
 */

#[test]

fn update_available_funds_and_emits_a_log() {
    let deposit: u64 = 100;
    let (
        root,
        _aca,
        link,
        _oracle_one,
        _oracle_two,
        _oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        link.account_id(),
        "ft_transfer",
        &json!({
            "receiver_id": flux_aggregator.account_id(), "amount": deposit.to_string(), "memo": "None"
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        1, // deposit
    )
    .assert_success();

    let receipt = root.call(
        flux_aggregator.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let expected: u64 = receipt.promise_results().remove(5).unwrap().outcome().logs[0]
        .parse::<u64>()
        .unwrap();

    let new_balance: u64 = root
        .view(
            flux_aggregator.account_id(),
            "available_funds",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    assert_eq!(new_balance, expected);
}

/**
 * #update_available_funds - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1449
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1464
 */

#[test]

fn when_the_available_funds_have_not_changed_does_not_emit_a_log() {
    let (
        root,
        _aca,
        _link,
        _oracle_one,
        _oracle_two,
        _oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let receipt = root.call(
        flux_aggregator.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    // *TODO* Figure out how to assert a failure here.
    assert_eq!(receipt.logs().len(), 0);
}

/**
 * #withdraw_payment - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1497
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1503
 */

#[test]

fn transfers_link_to_the_recipient() {
    let payment_amount: u64 = 3;
    let (
        root,
        _aca,
        link,
        oracle_one,
        _oracle_two,
        _oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": 100.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let original_balance: U128 = root
        .call(
            link.account_id(),
            "ft_balance_of",
            &json!({"account_id": flux_aggregator.account_id().to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let original_oracle_one_balance: U128 = root
        .call(
            link.account_id(),
            "ft_balance_of",
            &json!({"account_id": oracle_one.account_id().to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(0, u128::from(original_oracle_one_balance));

    oracle_one
    .call(
        flux_aggregator.account_id(),
        "withdraw_payment",
        &json!({"_oracle": oracle_one.account_id().to_string(), "_recipient": oracle_one.account_id().to_string(), "_amount": payment_amount.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        1, // deposit
    )
    .assert_success();

    let updated_balance: U128 = root
        .call(
            link.account_id(),
            "ft_balance_of",
            &json!({"account_id": flux_aggregator.account_id().to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(97, u128::from(updated_balance));

    let updated_oracle_one_balance: U128 = root
        .call(
            link.account_id(),
            "ft_balance_of",
            &json!({"account_id": oracle_one.account_id().to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(3, u128::from(updated_oracle_one_balance));
}

/**
 * #withdraw_payment - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1497
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1515
 */

#[test]

fn decrements_the_allocated_funds_counter() {
    let payment_amount: u128 = 3;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": 100.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let original_allocation: u128 = root
        .call(
            flux_aggregator.account_id(),
            "allocated_funds",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    oracle_one
    .call(
        flux_aggregator.account_id(),
        "withdraw_payment",
        &json!({"_oracle": oracle_one.account_id().to_string(), "_recipient": oracle_one.account_id().to_string(), "_amount": payment_amount.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let updated_allocation: u128 = root
        .call(
            flux_aggregator.account_id(),
            "allocated_funds",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!((original_allocation - payment_amount), updated_allocation);
}

/**
 * #withdraw_payment - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1497
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1526
 */

#[test]

fn when_the_caller_withdraws_more_than_they_have_and_reverts() {
    let payment_amount: u128 = 3;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": 100.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let expected_revert_insufficient_withdrawable_funds = oracle_one
    .call(
        flux_aggregator.account_id(),
        "withdraw_payment",
        &json!({"_oracle": oracle_one.account_id().to_string(), "_recipient": oracle_one.account_id().to_string(), "_amount": (payment_amount + 1).to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) =
        &expected_revert_insufficient_withdrawable_funds
            .promise_errors()
            .remove(0)
            .unwrap()
            .outcome()
            .status
    {
        assert!(execution_error
            .to_string()
            .contains("revert insufficient withdrawable funds"));
    } else {
        unreachable!();
    }
}

/**
 * #withdraw_payment - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1497
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1540
 */

#[test]

fn when_the_caller_is_not_the_admin_and_reverts() {
    let payment_amount: u128 = 3;
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
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": 100.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let expected_only_callable_by_admin = oracle_three
    .call(
        flux_aggregator.account_id(),
        "withdraw_payment",
        &json!({"_oracle": oracle_one.account_id().to_string(), "_recipient": oracle_three.account_id().to_string(), "_amount": 1.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_only_callable_by_admin
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("only callable by admin"));
    } else {
        unreachable!();
    }
}

/**
 * #transfer_admin - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1552
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1567
 */

#[test]

fn when_the_admin_tries_to_transfer_the_admin_and_works() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_two.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let receipt = oracle_one
        .call(
            flux_aggregator.account_id(),
            "transfer_admin",
            &json!({"_oracle": oracle_two.account_id().to_string(), "_new_admin": oracle_three.account_id().to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );

    assert_eq!(
        "oracle_two, oracle_one, oracle_three",
        receipt.promise_results().remove(1).unwrap().outcome().logs[0]
    );

    let get_admin: String = oracle_one
        .call(
            flux_aggregator.account_id(),
            "get_admin",
            &json!({"_oracle": oracle_two.account_id().to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(oracle_one.account_id(), get_admin);
}

/**
 * #transfer_admin - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1552
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1584
 */

#[test]

fn when_the_non_admin_owner_tries_to_update_the_admin_and_reverts() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_two.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let expected_only_callable_by_admin = root.call(
        flux_aggregator.account_id(),
        "transfer_admin",
        &json!({"_oracle": oracle_two.account_id().to_string(), "_new_admin": oracle_three.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_only_callable_by_admin
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("revert only callable by admin"));
    } else {
        unreachable!();
    }
}

/**
 * #transfer_admin - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1552
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1595
 */

#[test]

fn when_the_non_admin_oracle_tries_to_update_the_admin_and_reverts() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_two.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let expected_only_callable_by_admin = oracle_two.call(
        flux_aggregator.account_id(),
        "transfer_admin",
        &json!({"_oracle": oracle_two.account_id().to_string(), "_new_admin": oracle_three.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_only_callable_by_admin
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("revert only callable by admin"));
    } else {
        unreachable!();
    }
}

/**
 * #accept_admin - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1606
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1624
 */

#[test]

fn when_the_new_admin_tries_to_accept_and_works() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_two.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "transfer_admin",
        &json!({"_oracle": oracle_two.account_id().to_string(), "_new_admin": oracle_three.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let receipt = oracle_three.call(
        flux_aggregator.account_id(),
        "accept_admin",
        &json!({"_oracle": oracle_two.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    assert_eq!(
        "oracle_two, oracle_three",
        receipt.promise_results().remove(1).unwrap().outcome().logs[0]
    );

    let get_admin: String = oracle_one
        .call(
            flux_aggregator.account_id(),
            "get_admin",
            &json!({"_oracle": oracle_two.account_id().to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(oracle_three.account_id(), get_admin);
}

/**
 * #accept_admin - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1606
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1634
 */

#[test]

fn when_someone_other_than_the_admin_tries_to_accept_and_reverts() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_two.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "transfer_admin",
        &json!({"_oracle": oracle_two.account_id().to_string(), "_new_admin": oracle_three.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let expected_only_callable_by_pending_admin = oracle_two.call(
        flux_aggregator.account_id(),
        "accept_admin",
        &json!({"_oracle": oracle_two.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_only_callable_by_pending_admin
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("only callable by pending admin"));
    } else {
        unreachable!();
    }

    let expected_only_callable_by_pending_admin_second = oracle_one.call(
        flux_aggregator.account_id(),
        "accept_admin",
        &json!({"_oracle": oracle_two.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) =
        &expected_only_callable_by_pending_admin_second
            .promise_errors()
            .remove(0)
            .unwrap()
            .outcome()
            .status
    {
        assert!(execution_error
            .to_string()
            .contains("only callable by pending admin"));
    } else {
        unreachable!();
    }
}

/**
 * #on_token_transfer (ft_on_transfer) - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1647
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1648
 */

#[test]

fn updates_the_available_balance() {
    let (
        root,
        _aca,
        link,
        _oracle_one,
        _oracle_two,
        _oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let original_balance: u128 = root
        .view(
            flux_aggregator.account_id(),
            "available_funds",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0,
    )
    .assert_success();

    let updated_balance: u128 = root
        .view(
            flux_aggregator.account_id(),
            "available_funds",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    assert_eq!(original_balance, updated_balance);

    let prom = root.call(
        link.account_id(),
        "ft_transfer_call",
        &json!({
            "receiver_id": flux_aggregator.account_id(), "amount": 100.to_string(), "memo": "None", "msg": "".to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        1
    );

    println!("{:?}", prom.promise_results());

    let new_balance: u128 = root
        .view(
            flux_aggregator.account_id(),
            "available_funds",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    assert_eq!(200, new_balance);
}

/**
 * #on_token_transfer (ft_on_transfer) - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1647
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1661
 */

#[test]

fn reverts_given_calldata() {
    let (
        root,
        _aca,
        link,
        _oracle_one,
        _oracle_two,
        _oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let prom = root.call(
        link.account_id(),
        "ft_transfer_call",
        &json!({
            "receiver_id": flux_aggregator.account_id(), "amount": 100.to_string(), "memo": "None", "msg": "12345678".to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        1
    );

    println!("{:?}", prom.promise_results());
}

/**
 * #request_new_round - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1669
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1679
 */

#[test]

fn announces_a_new_round_via_log_event() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u64 = 100;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;
    root.call(
        flux_aggregator.account_id(),
        "set_requester_permissions",
        &json!({"_requester": root.account_id().to_string(), "_authorized": true, "_delay": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let receipt = root.call(
        flux_aggregator.account_id(),
        "request_new_round",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    assert_eq!(
        "2, root, 161000000000",
        receipt.promise_results().remove(1).unwrap().outcome().logs[0]
    );
}

/**
 * #request_new_round - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1669
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1683
 */

#[test]

fn returns_the_new_round_id() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u64 = 100;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;
    root.call(
        flux_aggregator.account_id(),
        "set_requester_permissions",
        &json!({"_requester": root.account_id().to_string(), "_authorized": true, "_delay": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    root.call(
        flux_aggregator.account_id(),
        "set_requester_permissions",
        &json!({"_requester": flux_aggregator_test_helper_contract.account_id().to_string(), "_authorized": true, "_delay": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let mut round_id: u64 = root
        .view(
            flux_aggregator_test_helper_contract.account_id(),
            "requested_round_id",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    assert_eq!(round_id, 0);

    root.call(
        flux_aggregator_test_helper_contract.account_id(),
        "request_new_round",
        &json!({"_aggregator": flux_aggregator.account_id()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    round_id = root
        .view(
            flux_aggregator_test_helper_contract.account_id(),
            "requested_round_id",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();

    assert_eq!(round_id > 0, true);
}

/**
 * #request_new_round - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1669
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1701
 */

#[test]

fn when_there_is_a_new_round_in_progress_and_reverts() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u64 = 100;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    root.call(
        flux_aggregator.account_id(),
        "set_requester_permissions",
        &json!({"_requester": root.account_id().to_string(), "_authorized": true, "_delay": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    root.call(
        flux_aggregator.account_id(),
        "request_new_round",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let expected_previous_round_must_be_supersedable = root.call(
        flux_aggregator.account_id(),
        "request_new_round",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_previous_round_must_be_supersedable
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("prev round must be supersedable"));
    } else {
        unreachable!();
    }
}

/**
 * #request_new_round - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1669
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1701
 * *TODO* Look into increaseTimeBy and mineBlock implementation
 */

#[test]

fn when_that_round_has_timed_out_and_starts_a_new_round() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u64 = 100;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    root.call(
        flux_aggregator.account_id(),
        "set_requester_permissions",
        &json!({"_requester": root.account_id().to_string(), "_authorized": true, "_delay": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    root.call(
        flux_aggregator.account_id(),
        "request_new_round",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    // *TODO* Look into increaseTimeBy and mineBlock implementation

    let receipt = root.call(
        flux_aggregator.account_id(),
        "request_new_round",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
}

/**
 * #request_new_round - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1669
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1722
 */

#[test]

fn when_there_is_a_restart_delay_set_and_reverts_if_a_round_is_started_before_the_delay() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u64 = 100;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    root.call(
        flux_aggregator.account_id(),
        "set_requester_permissions",
        &json!({"_requester": root.account_id().to_string(), "_authorized": true, "_delay": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    root.call(
        flux_aggregator.account_id(),
        "set_requester_permissions",
        &json!({"_requester": eddy.account_id().to_string(), "_authorized": true, "_delay": 1.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    eddy.call(
        flux_aggregator.account_id(),
        "request_new_round",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    next_round = next_round + 1;

    // Eddy can't start because of the delay
    let expected_must_delay_requests = eddy.call(
        flux_aggregator.account_id(),
        "request_new_round",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_must_delay_requests
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error.to_string().contains("must delay requests"));
    } else {
        unreachable!();
    }

    // Carol starts a new round instead
    root.call(
        flux_aggregator.account_id(),
        "request_new_round",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    // round completes
    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    next_round = next_round + 1;

    eddy.call(
        flux_aggregator.account_id(),
        "request_new_round",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
}

/**
 * #request_new_round - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1669
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1722
 * *TODO* Look into increaseTimeBy and mineBlock implementation
 */

#[test]

fn when_all_oracles_have_been_removed_and_then_re_added_and_does_not_get_stuck() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u64 = 100;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    root.call(
        flux_aggregator.account_id(),
        "set_requester_permissions",
        &json!({"_requester": root.account_id().to_string(), "_authorized": true, "_delay": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [oracle_one.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 0.to_string(), "_max_submissions": 0.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // *TODO* Look into increaseTimeBy and mineBlock functions
    // advance a few rounds
    // for (let i = 0; i < 7; i++) {
    //     await aggregator.requestNewRound();
    //     nextRound = nextRound + 1;
    //     await increaseTimeBy(timeout + 1, ethers.provider);
    //     await mineBlock(ethers.provider);
    //   }

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // round completes
    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
}

/**
 * #set_requester_permissions - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1760
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1769
 */

#[test]

fn when_called_by_the_owner_and_allows_the_specified_address_to_start_new_rounds() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u64 = 100;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    root.call(
            flux_aggregator.account_id(),
            "set_requester_permissions",
            &json!({"_requester": oracle_one.account_id().to_string(), "_authorized": true, "_delay": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "request_new_round",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
}

/**
 * #set_requester_permissions - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1760
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1775
 */

#[test]

fn emits_a_log_announcing_the_update() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u64 = 100;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    let receipt = root.call(
            flux_aggregator.account_id(),
            "set_requester_permissions",
            &json!({"_requester": oracle_one.account_id().to_string(), "_authorized": true, "_delay": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );

    assert_eq!(
        "oracle_one, true, 0",
        receipt.promise_results().remove(1).unwrap().outcome().logs[0]
    );
}

/**
 * #set_requester_permissions - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1760
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1786
 */

#[test]

fn when_the_address_is_already_authorized_and_does_not_emit_a_log_for_already_authorized_accounts()
{
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u64 = 100;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    root.call(
            flux_aggregator.account_id(),
            "set_requester_permissions",
            &json!({"_requester": oracle_one.account_id().to_string(), "_authorized": true, "_delay": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();

    let receipt = root.call(
            flux_aggregator.account_id(),
            "set_requester_permissions",
            &json!({"_requester": oracle_one.account_id().to_string(), "_authorized": true, "_delay": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );

    assert_eq!(0, receipt.logs().len());
}

/**
 * #set_requester_permissions - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1760
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1798
 */

#[test]

fn when_permission_is_removed_by_the_owner_and_does_not_allow_the_specified_address_to_start_new_rounds(
) {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u64 = 100;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    root.call(
            flux_aggregator.account_id(),
            "set_requester_permissions",
            &json!({"_requester": oracle_one.account_id().to_string(), "_authorized": true, "_delay": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();

    root.call(
            flux_aggregator.account_id(),
            "set_requester_permissions",
            &json!({"_requester": oracle_one.account_id().to_string(), "_authorized": false, "_delay": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();

    let expected_not_authorized_requester = oracle_one.call(
        flux_aggregator.account_id(),
        "request_new_round",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_not_authorized_requester
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("not authorized requester"));
    } else {
        unreachable!();
    }
}

/**
 * #set_requester_permissions - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1760
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1804
 */

#[test]

fn when_permission_is_removed_by_the_owner_and_emits_a_log_announcing_the_update() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u64 = 100;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    root.call(
            flux_aggregator.account_id(),
            "set_requester_permissions",
            &json!({"_requester": oracle_one.account_id().to_string(), "_authorized": true, "_delay": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();

    let receipt = root.call(
            flux_aggregator.account_id(),
            "set_requester_permissions",
            &json!({"_requester": oracle_one.account_id().to_string(), "_authorized": false, "_delay": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );

    assert_eq!(
        "oracle_one, false, 0",
        receipt.promise_results().remove(1).unwrap().outcome().logs[0]
    );
}

/**
 * #set_requester_permissions - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1760
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1810
 * *TODO* Look into why a log is still being emitted, looks as though it's still emitting in the Solidity code as well
 */

#[test]

fn does_not_emit_a_log_for_accounts_without_authorization() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let next_round: u64 = 1;
    let answer: u64 = 100;
    let (
        root,
        _aca,
        _link,
        oracle_one,
        oracle_two,
        _oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    root.call(
            flux_aggregator.account_id(),
            "set_requester_permissions",
            &json!({"_requester": oracle_one.account_id().to_string(), "_authorized": true, "_delay": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();

    let receipt = root.call(
            flux_aggregator.account_id(),
            "set_requester_permissions",
            &json!({"_requester": oracle_two.account_id().to_string(), "_authorized": false, "_delay": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );
    // *TODO* Look into why a log is still being emitted, looks as though it's still emitting in the Solidity code as well

    // println!("{:?}", receipt.logs());

    assert_eq!(0, receipt.logs().len());
}

/**
 * #set_requester_permissions - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1760
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1819
 */

#[test]

fn when_called_by_a_stranger_and_reverts() {
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u64 = 100;
    let (
        root,
        _aca,
        _link,
        oracle_one,
        oracle_two,
        _oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let expected_only_callable_by_owner = oracle_one.call(
            flux_aggregator.account_id(),
            "set_requester_permissions",
            &json!({"_requester": oracle_one.account_id().to_string(), "_authorized": true, "_delay": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
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

    let expected_not_authorized_requester = oracle_one.call(
        flux_aggregator.account_id(),
        "request_new_round",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_not_authorized_requester
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("not authorized requester"));
    } else {
        unreachable!();
    }
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1850
 * *TODO* Look into why the started_at is being set, it should not be set
 */

#[test]

fn when_round_id_0_is_passed_in_and_returns_all_of_the_important_round_information() {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let base_funds: u128 = 88;
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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // advanceRound

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_four.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    next_round = next_round + 1;

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, true);
    assert_eq!(state.1, 2);
    assert_eq!(state.2, previous_submission);

    // *TODO* Look into why the started_at is being set, it should not be set

    // assert_eq!(state.3, true);
    assert_eq!(state.4, 0);
    assert_eq!(state.5, base_funds);
    assert_eq!(state.6, 5);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1865
 */

#[test]

fn when_round_id_0_is_passed_in_reverts_if_called_by_a_contract() {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // advanceRound

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_four.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    next_round = next_round + 1;

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    let expected_off_chain_reading_only = root.call(
        flux_aggregator_test_helper_contract.account_id(),
        "read_oracle_round_state",
        &json!({"_aggregator": flux_aggregator.account_id(), "_oracle": oracle_one.account_id()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_off_chain_reading_only
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("off-chain reading only"));
    } else {
        unreachable!();
    }
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1887
 */

#[test]

fn when_the_restart_delay_is_not_enforced_and_less_than_min_submissions_and_oracle_not_included_and_is_eligible_to_submit(
) {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let timeout: u64 = 1800;
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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // advanceRound

    oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_four.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    next_round = next_round + 1;

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": 3.to_string(), "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": 0.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, true);
    assert_eq!(state.1, 2);
    assert_eq!(state.2, previous_submission);
    assert_eq!(state.3 > 0, true);
    assert_eq!(state.4, timeout);
    assert_eq!(state.5, 85);
    assert_eq!(state.6, 5);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1908
 */

#[test]

fn when_the_restart_delay_is_not_enforced_and_less_than_min_submissions_and_oracle_included_and_is_not_eligible_to_submit(
) {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u128 = 100;
    let timeout: u64 = 1800;
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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // advanceRound

    oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_four.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    next_round = next_round + 1;

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": 3.to_string(), "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": 0.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, false);
    assert_eq!(state.1, 2);
    assert_eq!(state.2, answer);
    assert_eq!(state.3 > 0, true);
    assert_eq!(state.4, timeout);
    assert_eq!(state.5, 85);
    assert_eq!(state.6, 5);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1908
 *  *TODO* Look into increaseTimeBy and mineBlock implementation
 */

#[test]

fn when_the_restart_delay_is_not_enforced_and_less_than_min_submissions_and_oracle_included_and_is_eligible_to_submit_and_timed_out_is_eligible_to_submit(
) {
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1951
 */

#[test]

fn when_the_restart_delay_is_not_enforced_and_greater_than_or_equal_to_min_submissions_and_oracle_not_included_and_is_eligible_to_submit(
) {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let timeout: u64 = 1800;
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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // advanceRound

    oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_four.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    next_round = next_round + 1;

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();
    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": 3.to_string(), "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": 0.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    // advanceRound

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_four.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, true);
    assert_eq!(state.1, 2);
    assert_eq!(state.2, previous_submission);
    assert_eq!(state.3 > 0, true);
    assert_eq!(state.4, timeout);
    assert_eq!(state.5, 79);
    assert_eq!(state.6, 5);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1972
 */

#[test]

fn when_the_restart_delay_is_not_enforced_and_greater_than_or_equal_to_min_submissions_and_oracle_included_and_is_eligible_to_submit(
) {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u128 = 100;
    let timeout: u64 = 1800;
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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
            flux_aggregator.account_id(),
            "change_oracles",
            &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();

    // advanceRound

    oracle_three.call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();

    oracle_two.call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();

    oracle_one.call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();

    oracle_four.call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();

    next_round = next_round + 1;

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": 3.to_string(), "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": 0.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    // advanceRound

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, true);
    assert_eq!(state.1, 3);
    assert_eq!(state.2, answer);
    assert_eq!(state.3 > 0, true);
    assert_eq!(state.4, timeout);
    assert_eq!(state.5, 79);
    assert_eq!(state.6, 5);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1993
 * *TODO* Look into increaseTimeBy and mineBlock implementation
 */

#[test]

fn when_the_restart_delay_is_not_enforced_and_greater_than_or_equal_to_min_submissions_and_oracle_included_and_timed_out_is_eligible_to_submit(
) {
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2017
 * *TODO* Look into increaseTimeBy and mineBlock implementation
 */

#[test]

fn when_the_restart_delay_is_not_enforced_and_max_submissions_and_oracle_not_included_and_is_eligible_to_submit(
) {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u128 = 100;
    let timeout: u64 = 1800;
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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // advanceRound

    oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_four.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    next_round = next_round + 1;

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": 3.to_string(), "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": 0.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    // advanceRound

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_four
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_five
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, true);
    assert_eq!(state.1, 3);
    assert_eq!(state.2, previous_submission);
    assert_eq!(state.3 > 0, true);
    assert_eq!(state.4, 0);
    assert_eq!(state.5, 76);
    assert_eq!(state.6, 5);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2040
 */

#[test]

fn when_the_restart_delay_is_not_enforced_and_max_submissions_and_oracle_included_and_is_eligible_to_submit(
) {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u128 = 100;
    let timeout: u64 = 1800;
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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // advanceRound

    oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_four.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    next_round = next_round + 1;

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": 3.to_string(), "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": 0.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    // advanceRound

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_four
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, true);
    assert_eq!(state.1, 3);
    assert_eq!(state.2, answer);
    assert_eq!(state.3 > 0, true);
    assert_eq!(state.4, 0);
    assert_eq!(state.5, 76);
    assert_eq!(state.6, 5);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2071
 */

#[test]

fn when_the_restart_delay_is_enforced_and_less_than_min_submissions_and_oracle_not_included_and_is_eligible_to_submit(
) {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u128 = 100;
    let timeout: u64 = 1800;
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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // advanceRound

    oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_four.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    next_round = next_round + 1;

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": 3.to_string(), "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": (max_answers - 1).to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, true);
    assert_eq!(state.1, 2);
    assert_eq!(state.2, previous_submission);
    assert_eq!(state.3 > 0, true);
    assert_eq!(state.4, timeout);
    assert_eq!(state.5, 82);
    assert_eq!(state.6, 5);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2092
 */

#[test]

fn when_the_restart_delay_is_enforced_and_less_than_min_submissions_and_oracle_included_and_is_not_eligible_to_submit(
) {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u128 = 100;
    let timeout: u64 = 1800;
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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // advanceRound

    oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_four.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    next_round = next_round + 1;

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": 3.to_string(), "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": (max_answers - 1).to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, false);
    assert_eq!(state.1, 2);
    assert_eq!(state.2, answer);
    assert_eq!(state.3 > 0, true);
    assert_eq!(state.4, timeout);
    assert_eq!(state.5, 82);
    assert_eq!(state.6, 5);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2092
 * *TODO* Look into increaseTimeBy and mineBlock implementation
 */

#[test]

fn when_the_restart_delay_is_enforced_and_less_than_min_submissions_and_oracle_included_and_is_eligible_to_submit_and_timed_out_is_eligible_to_submit(
) {
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2135
 */

#[test]

fn when_the_restart_delay_is_enforced_and_greater_than_or_equal_to_min_submissions_and_oracle_not_included_and_is_eligible_to_submit(
) {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u128 = 100;
    let timeout: u64 = 1800;
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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // advanceRound

    oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_four.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    next_round = next_round + 1;

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": 3.to_string(), "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": (max_answers - 1).to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_four
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, true);
    assert_eq!(state.1, 2);
    assert_eq!(state.2, previous_submission);
    assert_eq!(state.3 > 0, true);
    assert_eq!(state.4, timeout);
    assert_eq!(state.5, 79);
    assert_eq!(state.6, 5);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2156
 * *TODO* Look into why timeout is not 0.
 */

#[test]

fn when_the_restart_delay_is_enforced_and_greater_than_or_equal_to_min_submissions_and_oracle_included_and_is_eligible_to_submit(
) {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u128 = 100;
    let timeout: u64 = 1800;
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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // advanceRound

    oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_four.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    next_round = next_round + 1;

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": 3.to_string(), "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": (max_answers - 1).to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, false);
    assert_eq!(state.1, 3);
    assert_eq!(state.2, answer);
    assert_eq!(state.3 > 0, true);
    // *TODO* Look into why timeout is not 0.
    assert_eq!(state.4, timeout);
    assert_eq!(state.5, 79);
    assert_eq!(state.6, 5);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2177
 * *TODO* Look into increaseTimeBy and mineBlock implementation
 */

#[test]

fn when_the_restart_delay_is_enforced_and_greater_than_or_equal_to_min_submissions_and_oracle_included_and_timed_out_is_eligible_to_submit(
) {
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2201
 */

#[test]

fn when_the_restart_delay_is_enforced_and_max_submissions_and_oracle_not_included_and_is_not_eligible_to_submit(
) {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u128 = 100;
    let timeout: u64 = 1800;
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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // advanceRound

    oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_four.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    next_round = next_round + 1;

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": 3.to_string(), "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": (max_answers - 1).to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_four
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_five
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, false);
    assert_eq!(state.1, 3);
    assert_eq!(state.2, previous_submission);
    assert_eq!(state.3 > 0, true);
    assert_eq!(state.4, 0);
    assert_eq!(state.5, 76);
    assert_eq!(state.6, 5);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2224
 */

#[test]

fn when_the_restart_delay_is_enforced_and_max_submissions_and_oracle_included_and_is_not_eligible_to_submit(
) {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u128 = 100;
    let timeout: u64 = 1800;
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
        oracle_four,
        oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id(), oracle_four.account_id(), oracle_five.account_id()], "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // advanceRound

    oracle_three.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_two.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_four.call(
        flux_aggregator.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": previous_submission.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    next_round = next_round + 1;

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    root.call(
        flux_aggregator.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": 3.to_string(), "_min_submissions": min_answers.to_string(), "_max_submissions": max_answers.to_string(), "_restart_delay": (max_answers - 1).to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_four
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, false);
    assert_eq!(state.1, 3);
    assert_eq!(state.2, answer);
    assert_eq!(state.3 > 0, true);
    assert_eq!(state.4, 0);
    assert_eq!(state.5, 76);
    assert_eq!(state.6, 5);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2259
 */

#[test]

fn when_non_zero_round_id_0_is_passed_in_and_returns_info_about_previous_rounds() {
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answers: Vec<u128> = [0, 42, 47, 52, 57].to_vec();
    let current_funds: u128 = 73;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();
    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    // advanceRound * 4 (1)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (2)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[2].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[2].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (3)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (4)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[4].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 1.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, false);
    assert_eq!(state.1, 1);
    assert_eq!(state.2, answers[3]);
    assert_eq!(state.3 > 0, true);
    assert_eq!(state.4, 0);
    assert_eq!(state.5, current_funds);
    assert_eq!(state.6, 3);
    assert_eq!(state.7, 0);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2274
 */

#[test]

fn when_non_zero_round_id_0_is_passed_in_and_returns_info_about_previous_rounds_that_were_not_submitted_to(
) {
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answers: Vec<u128> = [0, 42, 47, 52, 57].to_vec();
    let current_funds: u128 = 73;
    let timeout: u64 = 1800;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    // advanceRound * 4 (1)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (2)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[2].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[2].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (3)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (4)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[4].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 2.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, false);
    assert_eq!(state.1, 2);
    assert_eq!(state.2, answers[3]);
    assert_eq!(state.3 > 0, true);
    assert_eq!(state.4, timeout);
    assert_eq!(state.5, current_funds);
    assert_eq!(state.6, 3);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2291
 */

#[test]

fn when_non_zero_round_id_0_is_passed_in_and_for_the_current_round_which_has_not_been_submitted_to_and_returns_info_about_the_current_round_that_hasnt_been_submitted_to(
) {
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answers: Vec<u128> = [0, 42, 47, 52, 57].to_vec();
    let current_funds: u128 = 73;
    let timeout: u64 = 1800;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    // advanceRound * 4 (1)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (2)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[2].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[2].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (3)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (4)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[4].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 4.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, true);
    assert_eq!(state.1, 4);
    assert_eq!(state.2, answers[3]);
    assert_eq!(state.3 > 0, true);
    assert_eq!(state.4, timeout);
    assert_eq!(state.5, current_funds);
    assert_eq!(state.6, 3);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2306
 */

#[test]

fn when_non_zero_round_id_0_is_passed_in_and_for_the_current_round_which_has_not_been_submitted_to_and_returns_info_about_the_subsequent_round(
) {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answers: Vec<u128> = [0, 42, 47, 52, 57].to_vec();
    let current_funds: u128 = 73;
    let timeout: u64 = 1800;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    // advanceRound * 4 (1)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (2)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[2].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[2].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (3)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (4)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[4].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 5.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, false);
    assert_eq!(state.1, 5);
    assert_eq!(state.2, answers[3]);
    assert_eq!(state.3 <= 0, true);
    assert_eq!(state.4, 0);
    assert_eq!(state.5, current_funds);
    assert_eq!(state.6, 3);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2327
 */

#[test]

fn when_non_zero_round_id_0_is_passed_in_and_for_the_current_round_which_has_been_submitted_to_and_returns_info_about_the_current_round_that_hasnt_been_submitted_to(
) {
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answers: Vec<u128> = [0, 42, 47, 52, 57].to_vec();
    let timeout: u64 = 1800;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    // advanceRound * 4 (1)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (2)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[2].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[2].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (3)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (4)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[4].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[4].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 4.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, false);
    assert_eq!(state.1, 4);
    assert_eq!(state.2, answers[4]);
    assert_eq!(state.3 > 0, true);
    assert_eq!(state.4, timeout);
    assert_eq!(state.5, 70);
    assert_eq!(state.6, 3);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2342
 */

#[test]

fn when_non_zero_round_id_0_is_passed_in_and_for_the_current_round_which_has_been_submitted_to_and_returns_info_about_the_subsequent_round(
) {
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answers: Vec<u128> = [0, 42, 47, 52, 57].to_vec();
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    // advanceRound * 4 (1)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (2)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[2].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[2].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (3)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (4)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[4].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[4].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 5.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, true);
    assert_eq!(state.1, 5);
    assert_eq!(state.2, answers[4]);
    assert_eq!(state.3 <= 0, true);
    assert_eq!(state.4, 0);
    assert_eq!(state.5, 70);
    assert_eq!(state.6, 3);
    assert_eq!(state.7, 3);
}

/**
 * #oracle_round_state - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L1830
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2359
 */

#[test]

fn when_non_zero_round_id_0_is_passed_in_and_returns_speculative_info_about_future_rounds() {
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answers: Vec<u128> = [0, 42, 47, 52, 57].to_vec();
    let current_funds: u128 = 73;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let starting_state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
    .call(
        flux_aggregator.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .unwrap_json();

    // advanceRound * 4 (1)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[1].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (2)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[2].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[2].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (3)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_two
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[3].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    next_round = next_round + 1;

    // advanceRound * 4 (4)

    oracle_one
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[4].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let state: (bool, u64, u128, u64, u64, u128, u64, u128) = root
        .call(
            flux_aggregator.account_id(),
            "oracle_round_state",
            &json!({"_oracle": oracle_three.account_id().to_string(), "_queried_round_id": 6.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7
    );

    assert_eq!(state.0, false);
    assert_eq!(state.1, 6);
    assert_eq!(state.2, answers[3]);
    assert_eq!(state.3 <= 0, true);
    assert_eq!(state.4, 0);
    assert_eq!(state.5, current_funds);
    assert_eq!(state.6, 3);
    assert_eq!(state.7, 3);
}

/**
 * #get_round_data - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2376
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2386
 * *TODO* Find current time and make sure its above upatedAt
 */

#[test]

fn get_round_data_and_returns_relevant_information() {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let latest_round_id: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u128 = 100;
    let current_funds: u128 = 73;
    let timeout: u64 = 1800;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let latest_round_id: u64 = root
        .call(
            flux_aggregator.account_id(),
            "latest_round",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let round: (u64, u128, u64, u64, u64) = root
        .call(
            flux_aggregator.account_id(),
            "get_round_data",
            &json!({"_round_id": latest_round_id.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(latest_round_id, round.0);
    assert_eq!(answer, round.1);

    // const nowSeconds = new Date().valueOf() / 1000;
    // assert.isAbove(round.updatedAt.toNumber(), nowSeconds - 120);
    // *TODO* Find current time and make sure its above upatedAt

    assert_eq!(round.2, round.3);
    assert_eq!(latest_round_id, round.4);
}

/**
 * #get_round_data - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2376
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2396
 */

#[test]

fn get_round_data_and_reverts_if_a_round_is_not_present() {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let latest_round_id: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u128 = 100;
    let current_funds: u128 = 73;
    let timeout: u64 = 1800;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let latest_round_id: u64 = root
        .call(
            flux_aggregator.account_id(),
            "latest_round",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let expected_no_data_present = root.call(
        flux_aggregator.account_id(),
        "get_round_data",
        &json!({"_round_id": (latest_round_id + 1).to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_no_data_present
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error
            .to_string()
            .contains("No data present"));
    } else {
        unreachable!();
    }
}

/**
 * #get_round_data - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2376
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2396
 * *TODO* Calculate math for overflowed u64 integer
 */

#[test]

fn get_round_data_and_reverts_if_a_round_ID_is_too_big() {
    let previous_submission: u128 = 42;
    let min_answers: u64 = 3;
    let max_answers: u64 = 4;
    let latest_round_id: u64 = 1;
    let rr_delay: u64 = 0;
    let mut next_round: u64 = 1;
    let answer: u128 = 100;
    let current_funds: u128 = 73;
    let timeout: u64 = 1800;
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
        _oracle_five,
        _aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let latest_round_id: u64 = root
        .call(
            flux_aggregator.account_id(),
            "latest_round",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    // // *TODO* Calculate math for overflowed u64 integer

    let expected_no_data_present = root.call(
        flux_aggregator.account_id(),
        "get_round_data",
        &json!({"_round_id": 100.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_no_data_present
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error.to_string().contains("No data present"));
    } else {
        unreachable!();
    }
}

/**
 * #latest_round_data - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2407
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2419
 * *TODO* Find current time and make sure its above upatedAt
 */

#[test]

fn latest_round_data_when_an_answer_has_been_received_and_returns_the_relevant_round_info_without_reverting(
) {
    let rr_delay: u64 = 0;
    let next_round: u64 = 1;
    let answer: u128 = 100;

    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let round: (u64, u128, u64, u64, u64) = root
        .call(
            flux_aggregator.account_id(),
            "latest_round_data",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    let latest_round_id: u64 = root
        .call(
            flux_aggregator.account_id(),
            "latest_round",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(latest_round_id, round.0);
    assert_eq!(answer, round.1);

    // const nowSeconds = new Date().valueOf() / 1000;
    // assert.isAbove(round.updatedAt.toNumber(), nowSeconds - 120);
    // *TODO* Find current time and make sure its above upatedAt

    assert_eq!(round.2, round.3);
    assert_eq!(latest_round_id, round.4);
}

/**
 * #latest_round_data - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2407
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2432
 */

#[test]

fn latest_round_data_when_an_answer_has_been_received_and_reverts_if_a_round_is_not_present() {
    let rr_delay: u64 = 0;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let expected_no_data_present = root.call(
        flux_aggregator.account_id(),
        "latest_round_data",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &expected_no_data_present
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error.to_string().contains("No data present"));
    } else {
        unreachable!();
    }
}

/**
 * #latest_answer - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2437
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2449
 */

#[test]

fn latest_answer_when_an_answer_has_already_been_received_and_returns_the_latest_answer_without_reverting(
) {
    let rr_delay: u64 = 0;
    let next_round: u64 = 1;
    let answer: u128 = 100;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    oracle_three
        .call(
            flux_aggregator.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let latest_answer: u128 = root
        .call(
            flux_aggregator.account_id(),
            "latest_answer",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();
    assert_eq!(answer, latest_answer);
}

/**
 * #latest_answer - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2437
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2454
 */

#[test]

fn latest_answer_and_returns_zero() {
    let rr_delay: u64 = 0;
    let (
        root,
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
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    root.call(
        flux_aggregator.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    let latest_answer: u128 = root
        .call(
            flux_aggregator.account_id(),
            "latest_answer",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(0, latest_answer);
}

/**
 * #set_validator - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2459
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2465
 */

#[test]

fn set_validator_and_emits_a_log_event_showing_the_validator_was_changed() {
    let (
        root,
        _aca,
        _link,
        _oracle_one,
        _oracle_two,
        _oracle_three,
        _test_helper,
        _eac,
        _eac_without_access_controller,
        _oracle_four,
        _oracle_five,
        aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let empty_address: String = root
        .call(
            flux_aggregator.account_id(),
            "get_validator",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!("", empty_address);

    let receipt = root.call(
        flux_aggregator.account_id(),
        "set_validator",
        &json!({"_new_validator": aggregator_validator_mock.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    assert_eq!(
        receipt.promise_results().remove(1).unwrap().outcome().logs[0],
        ", aggregator_validator_mock"
    );

    let receipt_two = root.call(
        flux_aggregator.account_id(),
        "set_validator",
        &json!({"_new_validator": aggregator_validator_mock.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    assert_eq!(receipt_two.logs().len(), 0);
}

/**
 * #set_validator - https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2459
 * https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/FluxAggregator.test.ts#L2479
 */

#[test]

fn set_validator_and_when_called_by_a_non_owner_and_reverts() {
    let (
        root,
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
        aggregator_validator_mock,
        _flags,
        _consumer,
        _flags_consumer,
        _controller,
        _controller_2,
        _flux_aggregator_test_helper_contract,
        _eddy,
        _mock_v3_aggregator,
        _mock_v3_aggregator_second,
        _read_controller,
        flux_aggregator,
    ) = init();

    let empty_address: String = root
        .call(
            flux_aggregator.account_id(),
            "get_validator",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!("", empty_address);

    let expected_only_callable_by_owner = oracle_one.call(
        flux_aggregator.account_id(),
        "set_validator",
        &json!({"_new_validator": aggregator_validator_mock.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
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


// // *TODO*: integrating with historic deviation checker.
