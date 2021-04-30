use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::{AccountId};
use near_sdk_sim::DEFAULT_GAS;
use near_sdk_sim::transaction::ExecutionStatus;

use crate::utils::init_without_macros as init;

#[test]

fn simulate_linktoken_transfer() {
    let (root, aca, link, oracle_one, oracle_two, oracle_three, test_helper, _eac, eac_without_access_controller) = init();
    // Transfer from link_token contract to ACA.
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": "190"
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // First add oracle_one
    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": "1", "_max_submissions": "1", "_restart_delay": "0"}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Second, call submit from oracle_one
    oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": "1", "_submission": "1"}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let _root_balance: U128 = root
        .view(
            link.account_id(),
            "get_balance",
            &json!({
                "owner_id": root.valid_account_id()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    let withdraw = oracle_one.call(
        aca.account_id(),
        "withdraw_payment",
        &json!({"_oracle": oracle_one.account_id(), "_recipient": oracle_one.account_id(), "_amount": "1"}).to_string().into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    );

    let oracle_balance: U128 = root
    .view(
        link.account_id(),
        "get_balance",
        &json!({
            "owner_id": oracle_one.valid_account_id()
        })
        .to_string()
        .into_bytes(),
    )
    .unwrap_json();
    assert_eq!(1, u128::from(oracle_balance));
}

#[test]
fn access_control_tests() {
    let payment_amount: u64 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let empty_address: AccountId = "".to_string();
    let next_round: u128 = 1;
    let (root, aca, link, oracle_one, oracle_two, oracle_three, test_helper, _ea, eac_without_access_controller) = init();
    // Transfer from link_token contract to ACA.
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": deposit.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // First add oracle_one
    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Second, call submit from oracle_one
    oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();

    // Unauthorized Calls

    // Unauthorized call from test_helper for get_answer
    let mut get_answer_unauthorized = test_helper.call(
        aca.account_id(),
        "get_answer",
        &json!({"_round_id": next_round.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    assert_eq!(get_answer_unauthorized.promise_errors().len(), 1);

    if let ExecutionStatus::Failure(execution_error) =
        &get_answer_unauthorized.promise_errors().remove(0).unwrap().outcome().status
    {
        assert!(execution_error.to_string().contains("No access"));
    } else {
        unreachable!();
    }
    // Unauthorized call from test_helper for get_timestamp
    get_answer_unauthorized = test_helper.call(
        aca.account_id(),
        "get_timestamp",
        &json!({"_round_id": next_round.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    assert_eq!(get_answer_unauthorized.promise_errors().len(), 1);

    if let ExecutionStatus::Failure(execution_error) =
        &get_answer_unauthorized.promise_errors().remove(0).unwrap().outcome().status
    {
        assert!(execution_error.to_string().contains("No access"));
    } else {
        unreachable!();
    }
    // Unauthorized call from test_helper for latest_answer
    get_answer_unauthorized = test_helper.call(
        aca.account_id(),
        "latest_answer",
        &json!({"_round_id": next_round.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    assert_eq!(get_answer_unauthorized.promise_errors().len(), 1);

    if let ExecutionStatus::Failure(execution_error) =
        &get_answer_unauthorized.promise_errors().remove(0).unwrap().outcome().status
    {
        assert!(execution_error.to_string().contains("No access"));
    } else {
        unreachable!();
    }
    // Unauthorized call from test_helper for latest_timestamp
    get_answer_unauthorized = test_helper.call(
        aca.account_id(),
        "latest_timestamp",
        &json!({"_round_id": next_round.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    assert_eq!(get_answer_unauthorized.promise_errors().len(), 1);

    if let ExecutionStatus::Failure(execution_error) =
        &get_answer_unauthorized.promise_errors().remove(0).unwrap().outcome().status
    {
        assert!(execution_error.to_string().contains("No access"));
    } else {
        unreachable!();
    }

    // Authorized Calls

    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    // Authorized call from test_helper for get_answer
    test_helper.call(
        aca.account_id(),
        "get_answer",
        &json!({"_round_id": 1.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    // Authorized call from test_helper for get_timestamp
    test_helper.call(
        aca.account_id(),
        "get_timestamp",
        &json!({"_round_id": 1.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    // Authorized call from test_helper for latest_answer
    test_helper.call(
        aca.account_id(),
        "latest_answer",
        &json!({"_round_id": 1.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    // Authorized call from test_helper for latest_timestamp
    test_helper.call(
        aca.account_id(),
        "latest_timestamp",
        &json!({"_round_id": 1.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
}

#[test]
fn updates_the_allocated_and_available_funds_counters_and_emits_a_log_event_announcing_submission_details() {
    let payment_amount: u64 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (root, aca, link, oracle_one, oracle_two, oracle_three, test_helper, _eac, eac_without_access_controller) = init();
    // Transfer from link_token contract to ACA.
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": deposit.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // #submit
    println!("\n#submit");

    let mut min_max: u64 = 1;

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    min_max = min_max + 1;
    println!("first {:?}", min_max);
    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_two.account_id()], "_added_admins": [oracle_two.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    min_max = min_max + 1;
    println!("second {:?}", min_max);
    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    println!("after {:?}", min_max);
    // updates the allocated and available funds counters
    println!("updates the allocated and available funds counters");

    let mut allocated_funds: u64 = root
    .view(
        aca.account_id(),
        "allocated_funds",
        &json!({
                "": "".to_string()
            })
            .to_string()
            .into_bytes(),
    )
    .unwrap_json();
    assert_eq!(0, allocated_funds, "updates the allocated and available funds counters");

    let mut tx = oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    let mut receipt = tx.promise_results();

    allocated_funds = root
    .view(
        aca.account_id(),
        "allocated_funds",
        &json!({
                "": "".to_string()
            })
            .to_string()
            .into_bytes(),
    )
    .unwrap_json();
    let available_funds: u64 = root
    .view(
        aca.account_id(),
        "available_funds",
        &json!({
                "": "".to_string()
            })
            .to_string()
            .into_bytes(),
    )
    .unwrap_json();
    assert_eq!(payment_amount, allocated_funds);
    let expected_available: u64 = deposit - payment_amount;
    assert_eq!(expected_available, available_funds);
    let logged: u64 = receipt.remove(1).unwrap().outcome().logs[0].parse().unwrap();
    assert_eq!(expected_available, logged);

    // emits a log event announcing submission details
    println!("emits a log event announcing submission details");
    tx = oracle_two.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    receipt = tx.promise_results();
    println!("{:?}", receipt);
    // let round = receipt.events?.[1]
    //assert_eq(answer, round.submission)

    // when the minimum oracles have not reported
    // println!("when the minimum oracles have not reported");
    // let withdrawable_payment: u128 = root
    // .view(
    //     aca.account_id(),
    //     "withdrawable_payment",
    //     &json!({
    //             "_oracle": oracle_one.account_id().to_string()
    //         })
    //         .to_string()
    //         .into_bytes(),
    // )
    // .unwrap_json();
    // assert_eq!(0, withdrawable_payment);
}

#[test]
fn when_the_minimum_oracles_have_not_reported() {
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (root, aca, link, oracle_one, oracle_two, oracle_three, test_helper, _eac, eac_without_access_controller) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    // Transfer from link_token contract to ACA.
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": deposit.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // #submit
    println!("\n#submit");

    let min_max: u64 = 3;

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // min_max = min_max + 1;
    let withdrawable_payment: u128 = root
    .view(
        aca.account_id(),
        "withdrawable_payment",
        &json!({
                "_oracle": oracle_one.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
    )
    .unwrap_json();
    assert_eq!(0, withdrawable_payment);
    let tx =  oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    let receipt = tx.promise_results();
    println!("{:?}", receipt);
    // let round = receipt.events?.[1]
    //assert_eq(answer, round.submission)

    // when the minimum oracles have not reported
    println!("when the minimum oracles have not reported");
    let withdrawable_payment: u128 = root
    .view(
        aca.account_id(),
        "withdrawable_payment",
        &json!({
                "_oracle": oracle_one.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
    )
    .unwrap_json();
    assert_eq!(payment_amount, withdrawable_payment);
    let withdrawable_payment_1: u128 = root
    .view(
        aca.account_id(),
        "withdrawable_payment",
        &json!({
                "_oracle": oracle_two.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
    )
    .unwrap_json();
    assert_eq!(0, withdrawable_payment_1);
    let withdrawable_payment_2: u128 = root
    .view(
        aca.account_id(),
        "withdrawable_payment",
        &json!({
                "_oracle": oracle_three.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
    )
    .unwrap_json();
    assert_eq!(0, withdrawable_payment_2);
    // does not update the answer
    // oracle_two.call(
    //     aca.account_id(),
    //     "submit",
    //     &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()}).to_string().into_bytes(),
    //     DEFAULT_GAS,
    //     0, // deposit
    // ).assert_success();
    // oracle_three.call(
    //     aca.account_id(),
    //     "submit",
    //     &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()}).to_string().into_bytes(),
    //     DEFAULT_GAS,
    //     0, // deposit
    // ).assert_success();
    let not_updated = test_helper
    .call(
        aca.account_id(),
        "latest_answer",
        &json!({
             
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        
    );
    println!("{:?} logs", not_updated.promise_results());

    
    if let ExecutionStatus::Failure(execution_error) =
    &not_updated.promise_errors().remove(0).unwrap().outcome().status
{
    println!("{:?} logs111111", execution_error.to_string());
    assert!(execution_error.to_string().contains("Did not find"));
} else {
    unreachable!();
}
    // assert_eq!(0, not_updated);
}
#[test]
fn when_an_oracle_prematurely_bumps_the_round() {
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 2;
    let max_ans: u64 = 3;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (root, aca, link, oracle_one, oracle_two, oracle_three, test_helper, _eac, eac_without_access_controller) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    // Transfer from link_token contract to ACA.
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": deposit.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // #submit
    println!("\n#submit");

    let min_max: u64 = 3;

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx =  oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    let receipt = tx.promise_results();
    println!("{:?}", receipt);

    // oracle_two.call(
    //     aca.account_id(),
    //     "submit",
    //     &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()}).to_string().into_bytes(),
    //     DEFAULT_GAS,
    //     0, // deposit
    // ).assert_success();
    // oracle_three.call(
    //     aca.account_id(),
    //     "submit",
    //     &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()}).to_string().into_bytes(),
    //     DEFAULT_GAS,
    //     0, // deposit
    // ).assert_success();
 
    let tx_2 =  oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    let receipt_2 = tx_2.promise_results();
    println!("{:?} yooo", receipt_2);
    if let ExecutionStatus::Failure(execution_error) =
    &tx_2.promise_errors().remove(0).unwrap().outcome().status
{
    println!("{:?} jee", execution_error.to_string());
    assert!(execution_error.to_string().contains("previous round not supersedable"));
} else {
    unreachable!();
}
}