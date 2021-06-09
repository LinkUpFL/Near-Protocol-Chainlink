use near_sdk::serde_json::json;
use near_sdk::AccountId;
use near_sdk_sim::{init_simulator, to_yocto, UserAccount, DEFAULT_GAS};

const ACA_ID: &str = "aca";
const LINKTOKEN_ID: &str = "lt";
const EAC_ID: &str = "eac";
const EAC_WITHOUT_ACCESS_CONTROLLER_ID: &str = "eac_without_access_controller";

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    // update `contract.wasm` for your contract's name
    ACA_WASM_BYTES => "target/wasm32-unknown-unknown/debug/AccessControlledAggregator.wasm",

    // if you run `cargo build` without `--release` flag:
    LINKTOKEN_WASM_BYTES => "target/wasm32-unknown-unknown/debug/LinkToken.wasm",

    EAC_WASM_BYTES => "target/wasm32-unknown-unknown/debug/EACAggregatorProxy.wasm"
}

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L251
// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/AccessControlledAggregator.test.ts
// Initialization and constructor tests

pub fn init_without_macros() -> (
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
) {
    // Use `None` for default genesis configuration; more info below
    let root = init_simulator(None);
    let link = root.deploy(
        &LINKTOKEN_WASM_BYTES,
        LINKTOKEN_ID.to_string(),
        to_yocto("1000"), // attached deposit
    );
    link.call(
        LINKTOKEN_ID.into(),
        "new",
        &json!({
            "owner_id": root.account_id().to_string(), "total_supply": "100000"
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        0, // attached deposit
    )
    .assert_success();
    let aca = root.deploy(
        &ACA_WASM_BYTES,
        ACA_ID.to_string(),
        to_yocto("1000"), // attached deposit
    );

    let payment_amount: u128 = 3;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let version: u128 = 3;
    let validator: String = "".to_string();
    let deposit: u64 = 100;

    aca.call(
        ACA_ID.into(),
        "new",
        &json!({
            "link_id": link.account_id(),
            "owner_id": root.account_id(),
            "_payment_amount": payment_amount.to_string(),
            "_timeout": timeout.to_string(),
            "_validator": validator,
            "_min_submission_value": min_submission_value.to_string(),
            "_max_submission_value": max_submission_value.to_string(),
            "_decimals": decimals.to_string(),
            "_description": description,
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        0, // attached deposit
    )
    .assert_success();

    let expected_payment_amount: u128 = root
        .call(
            aca.account_id(),
            "get_payment_amount",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(payment_amount, expected_payment_amount);

    let expected_timeout: u64 = root
        .call(
            aca.account_id(),
            "get_timeout",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(timeout, expected_timeout);

    let expected_decimals: u64 = root
        .call(
            aca.account_id(),
            "get_decimals",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(decimals, expected_decimals);

    let expected_description: String = root
        .call(
            aca.account_id(),
            "get_description",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(description, expected_description);

    let expected_version: u128 = root
        .call(
            aca.account_id(),
            "get_version",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(version, expected_version);

    let expected_validator: String = root
        .call(
            aca.account_id(),
            "get_validator",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();

    assert_eq!(validator, expected_validator);

    // Deployment function body as done on line 180-196 -> https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L180 (beforeEach)
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

    root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();


    let oracle_one = root.create_user(
        "oracle_one".to_string(),
        to_yocto("1000000"), // initial balance
    );

    let oracle_two = root.create_user(
        "oracle_two".to_string(),
        to_yocto("1000000"), // initial balance
    );

    let oracle_three = root.create_user(
        "oracle_three".to_string(),
        to_yocto("1000000"), // initial balance
    );

    let test_helper = root.create_user(
        "test_helper".to_string(),
        to_yocto("1000"), // initial balance
    );

    // *TODO* Create FluxAggregator test factory contract here
    // let test_helper_contract = root.create_user(
    //     "test_helper".to_string(),
    //     to_yocto("1000"), // initial balance
    // );

    let eac = root.deploy(
        &EAC_WASM_BYTES,
        EAC_ID.to_string(),
        to_yocto("1000"), // attached deposit
    );
    eac.call(
        EAC_ID.into(),
        "new",
        &json!({
            "owner_id": eac.account_id(),
            "_aggregator": aca.account_id(),
            "_access_controller": "null"
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        0, // attached deposit
    )
    .assert_success();

    let eac_without_access_controller = root.deploy(
        &EAC_WASM_BYTES,
        EAC_WITHOUT_ACCESS_CONTROLLER_ID.to_string(),
        to_yocto("1000"), // attached deposit
    );

    eac_without_access_controller
        .call(
            EAC_WITHOUT_ACCESS_CONTROLLER_ID.into(),
            "new",
            &json!({
                "owner_id": eac_without_access_controller.account_id(),
                "_aggregator": aca.account_id(),
                "_access_controller": "null"
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS / 2,
            0, // attached deposit
        )
        .assert_success();
    (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        eac,
        eac_without_access_controller,
    )
}
