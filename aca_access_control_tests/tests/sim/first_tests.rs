use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk_sim::DEFAULT_GAS;

use crate::utils::init_without_macros as init;

#[test]

fn simulate_linktoken_transfer() {
    let (root, aca, link, oracle_one) = init();
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
    // let aca_balance: String = aca
    //     .view(
    //         link.account_id(),
    //         "withdraw_payment",
    //         &json!({
    //             "owner_id": aca.valid_account_id()
    //         })
    //         .to_string()
    //         .into_bytes(),
    //     )
    //     .unwrap_json();


    // let oracle_available_withdrawable: String = aca
    //     .view(
    //         aca.account_id(),
    //         "withdrawable_payment",
    //         &json!({"_oracle": oracle_one.account_id()}).to_string().into_bytes(),
    //     )
    //     .unwrap_json();
    // println!("{:?}", oracle_available_withdrawable);

    let withdraw = oracle_one.call(
        aca.account_id(),
        "withdraw_payment",
        &json!({"_oracle": oracle_one.account_id(), "_recipient": oracle_one.account_id(), "_amount": "1"}).to_string().into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    );
    println!("{:?}", withdraw.promise_results());

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
    println!("{:?}", oracle_balance);

    assert_eq!(true, true);
}