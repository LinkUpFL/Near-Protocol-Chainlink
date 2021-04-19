use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk_sim::DEFAULT_GAS;

use crate::utils::init_without_macros as init;

#[test]
fn external_access_tests() {
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
    oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": "1", "_submission": "3"})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
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

    let _withdraw = oracle_one.call(
        aca.account_id(),
        "withdraw_payment",
        &json!({"_oracle": oracle_one.account_id(), "_recipient": oracle_one.account_id(), "_amount": "1"}).to_string().into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    );
    // println!("{:?}", withdraw.promise_results());
    let _add_access = root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": oracle_one.account_id()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let _add_access_1 = root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": eac_without_access_controller.account_id()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
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

    // let latest_answer: U128 = oracle_one
    // .call(
    //     aca.account_id(),
    //     "latest_answer",
    //     &json!({})
    //     .to_string()
    //     .into_bytes(),
    //     DEFAULT_GAS,
    //     0, // deposit
    // )
    // .unwrap_json();
    // println!("{:?}", latest_answer);
    // let _update_aggregator = root
    // .call(
    //     eac.account_id(),
    //     "set_aggregator",
    //     &json!({"_aggregator": oracle_one.account_id()}).to_string().into_bytes(),
    //     DEFAULT_GAS,
    //     0, // deposit
    // ).assert_success();
    oracle_one
    .call(
        eac_without_access_controller.account_id(),
        "decimals",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );


    // Callers can call view functions without explicit access

        test_helper.call(
        eac_without_access_controller.account_id(),
            "latest_answer",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();
        test_helper.call(
            eac_without_access_controller.account_id(),
                "latest_timestamp",
                &json!({}).to_string().into_bytes(),
                DEFAULT_GAS,
                0, // deposit
            ).assert_success();
            // test_helper.call(
            //     eac_without_access_controller.account_id(),
            //         "get_answer",
            //         &json!({}).to_string().into_bytes(),
            //         DEFAULT_GAS,
            //         0, // deposit
            //     ).assert_success();
            //     test_helper.call(
            //         eac_without_access_controller.account_id(),
            //             "get_answer",
            //             &json!({}).to_string().into_bytes(),
            //             DEFAULT_GAS,
            //             0, // deposit
            //         ).assert_success();
        assert_eq!(true, true);
}