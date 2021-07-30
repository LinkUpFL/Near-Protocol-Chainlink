use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk_sim::{init_simulator, to_yocto, UserAccount, DEFAULT_GAS};

const ACA_ID: &str = "aca";
const FLUXAGGREGATOR_ID: &str = "flux_aggregator";
const LINKTOKEN_ID: &str = "link";
const EAC_ID: &str = "eac";
const EAC_WITHOUT_ACCESS_CONTROLLER_ID: &str = "eac_without_access_controller";
const AVM_ID: &str = "aggregator_validator_mock";
const FLAGS_ID: &str = "flags";
const CONSUMER_ID: &str = "consumer";
const FLAGSTESTHELPER_ID: &str = "flags_consumer";
const SIMPLEWRITEACCESSCONTROLLER_ID: &str = "controller";
const SIMPLEWRITEACCESSCONTROLLER_ID_2: &str = "controller_2";
const SIMPLEREADACCESSCONTROLLER_ID: &str = "read_controller";
const FLUXAGGREGATORTESTHELPER_ID: &str = "flux_aggregator_test_helper_contract";
const MOCKV3AGGREGATOR_ID: &str = "mock_v3_aggregator";
const MOCKV3AGGREGATOR_ID_2: &str = "mock_v3_aggregator_2";

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    ACA_WASM_BYTES => "target/wasm32-unknown-unknown/debug/AccessControlledAggregator.wasm",
    FLUXAGGREGATOR_WASM_BYTES => "target/wasm32-unknown-unknown/debug/FluxAggregator.wasm",
    LINKTOKEN_WASM_BYTES => "target/wasm32-unknown-unknown/debug/LinkToken.wasm",
    EAC_WASM_BYTES => "target/wasm32-unknown-unknown/debug/EACAggregatorProxy.wasm",
    AVM_WASM_BYTES => "target/wasm32-unknown-unknown/debug/AggregatorVaildatorMock.wasm",
    FLAGS_WASM_BYTES => "target/wasm32-unknown-unknown/debug/Flags.wasm",
    CONSUMER_WASM_BYTES => "target/wasm32-unknown-unknown/debug/Consumer.wasm",
    SIMPLEWRITEACCESSCONTROLLER_WASM_BYTES => "target/wasm32-unknown-unknown/debug/SimpleWriteAccessController.wasm",
    SIMPLEREADACCESSCONTROLLER_WASM_BYTES => "target/wasm32-unknown-unknown/debug/SimpleReadAccessController.wasm",
    FLAGSTESTHELPER_WASM_BYTES => "target/wasm32-unknown-unknown/debug/FlagsTestHelper.wasm",
    FLUXAGGREGATORTESTHELPER_WASM_BYTES => "target/wasm32-unknown-unknown/debug/FluxAggregatorTestHelper.wasm",
    MOCKV3AGGREGATOR_WASM_BYTES => "target/wasm32-unknown-unknown/debug/MockV3Aggregator.wasm"
}

// Register the given `user` with FT contract
pub fn register_user(user: &near_sdk_sim::UserAccount) {
    user.call(
        LINKTOKEN_ID.to_string(),
        "storage_deposit",
        &json!({
            "account_id": user.valid_account_id()
        })
        .to_string()
        .into_bytes(),
        near_sdk_sim::DEFAULT_GAS / 2,
        near_sdk::env::storage_byte_cost() * 125, // attached deposit
    )
    .assert_success();
}
/**
* TODO -> MATCH THESE
 Default: Signer;
 Carol: Signer;
 Eddy: Signer;
 Nancy: Signer;
 Ned: Signer;
 Neil: Signer;
 Nelly: Signer;
 Norbert: Signer;
*/

// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L251
// https://github.com/smartcontractkit/chainlink-brownie-contracts/blob/8071761a5b0e5444fc0de1751b7b398caf69ced4/contracts/test/v0.6/AccessControlledAggregator.test.ts
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
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
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
    // Alias: Carol
    let root = init_simulator(None);

    let link = root.deploy(
        &LINKTOKEN_WASM_BYTES,
        LINKTOKEN_ID.to_string(),
        to_yocto("1000"), // attached deposit
    );

    link.call(
        LINKTOKEN_ID.into(),
        "new_default_meta",
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

    let flux_aggregator = root.deploy(
        &FLUXAGGREGATOR_WASM_BYTES,
        FLUXAGGREGATOR_ID.to_string(),
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
            "owner_id": root.account_id(),
            "link_id": link.account_id(),
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

    flux_aggregator
        .call(
            FLUXAGGREGATOR_ID.into(),
            "new",
            &json!({
                "owner_id": root.account_id(),
                "link_id": link.account_id(),
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

    register_user(&aca);
    register_user(&flux_aggregator);

    // Deployment function body as done on line 180-196 -> https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L180 (beforeEach)
    root.call(
        link.account_id(),
        "ft_transfer",
        &json!({
            "receiver_id": aca.account_id().to_string(), "amount": deposit.to_string(), "memo": "None"
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        1
    )
    .assert_success();

    root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0,
    )
    .assert_success();

    root.call(
            link.account_id(),
            "ft_transfer",
            &json!({
                "receiver_id": flux_aggregator.account_id().to_string(), "amount": deposit.to_string(), "memo": "None"
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

    let aggregator_validator_mock_factory = root.deploy(
        &AVM_WASM_BYTES,
        AVM_ID.to_string(),
        to_yocto("1000"), // attached deposit
    );

    aggregator_validator_mock_factory
    .call(
        AVM_ID.to_string(),
        "new",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS / 2,
        0, // attached deposit
    )
    .assert_success();

    // Alias: Neil
    let oracle_one = root.create_user(
        "oracle_one".to_string(),
        to_yocto("1000000"), // initial balance
    );
    // Alias: Ned
    let oracle_two = root.create_user(
        "oracle_two".to_string(),
        to_yocto("1000000"), // initial balance
    );
    // Alias: Nelly
    let oracle_three = root.create_user(
        "oracle_three".to_string(),
        to_yocto("1000000"), // initial balance
    );
    // Alias: Nancy
    let oracle_four = root.create_user(
        "oracle_four".to_string(),
        to_yocto("1000000"), // initial balance
    );
    // Alias: Norbert
    let oracle_five = root.create_user(
        "oracle_five".to_string(),
        to_yocto("1000000"), // initial balance
    );

    register_user(&oracle_one);
    register_user(&oracle_two);
    register_user(&oracle_three);
    register_user(&oracle_four);
    register_user(&oracle_five);

    let test_helper = root.create_user(
        "test_helper".to_string(),
        to_yocto("1000"), // initial balance
    );

    let eddy = root.create_user(
        "eddy".to_string(),
        to_yocto("1000"), // initial balance
    );

    let flux_aggregator_test_helper_contract = root.deploy(
        &FLUXAGGREGATORTESTHELPER_WASM_BYTES,
        FLUXAGGREGATORTESTHELPER_ID.to_string(),
        to_yocto("1000"), // attached deposit
    );

    flux_aggregator_test_helper_contract
        .call(
            FLUXAGGREGATORTESTHELPER_ID.into(),
            "new",
            &json!({}).to_string().into_bytes(),
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
                "_access_controller": ""
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS / 2,
            0, // attached deposit
        )
        .assert_success();

    let controller = oracle_three.deploy(
        &SIMPLEWRITEACCESSCONTROLLER_WASM_BYTES,
        SIMPLEWRITEACCESSCONTROLLER_ID.to_string(),
        to_yocto("1000"), // attached deposit
    );

    controller
        .call(
            SIMPLEWRITEACCESSCONTROLLER_ID.into(),
            "new",
            &json!({
                "owner_id": oracle_three.account_id()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS / 2,
            0, // attached deposit
        )
        .assert_success();

    let mock_v3_aggregator = root.deploy(
        &MOCKV3AGGREGATOR_WASM_BYTES,
        MOCKV3AGGREGATOR_ID.to_string(),
        to_yocto("1000"), // attached deposit
    );

    mock_v3_aggregator
        .call(
            MOCKV3AGGREGATOR_ID.into(),
            "new",
            &json!({
                "_decimals": 18.to_string(),
                "_initial_answer": 0.to_string(),
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS / 2,
            0, // attached deposit
        )
        .assert_success();

    let mock_v3_aggregator_second = root.deploy(
        &MOCKV3AGGREGATOR_WASM_BYTES,
        MOCKV3AGGREGATOR_ID_2.to_string(),
        to_yocto("1000"), // attached deposit
    );

    // *TODO* Look into overflow issue with 54321

    mock_v3_aggregator
        .call(
            MOCKV3AGGREGATOR_ID.into(),
            "update_round_data",
            &json!({
                "_round_id": 17.to_string(),
                "_answer": 54320.to_string(),
                "_timestamp": 678.to_string(),
                "_started_at": 677.to_string()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS / 2,
            0, // attached deposit
        )
        .assert_success();

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
            "_aggregator": mock_v3_aggregator.account_id(),
            "_access_controller": controller.account_id()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        0, // attached deposit
    )
    .assert_success();

    let controller_2 = oracle_three.deploy(
        &SIMPLEWRITEACCESSCONTROLLER_WASM_BYTES,
        SIMPLEWRITEACCESSCONTROLLER_ID_2.to_string(),
        to_yocto("1000"), // attached deposit
    );

    controller_2
        .call(
            SIMPLEWRITEACCESSCONTROLLER_ID_2.into(),
            "new",
            &json!({
                "owner_id": oracle_three.account_id()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS / 2,
            0, // attached deposit
        )
        .assert_success();

    let read_controller = root.deploy(
        &SIMPLEREADACCESSCONTROLLER_WASM_BYTES,
        SIMPLEREADACCESSCONTROLLER_ID.to_string(),
        to_yocto("1000"), // attached deposit
    );

    let flags = root.deploy(
        &FLAGS_WASM_BYTES,
        FLAGS_ID.to_string(),
        to_yocto("1000"), // attached deposit
    );

    flags
        .call(
            FLAGS_ID.into(),
            "new",
            &json!({
                "owner_id": oracle_three.account_id().to_string(),
                "rac_address": controller.account_id().to_string(),
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS / 2,
            0, // attached deposit
        )
        .assert_success();

    oracle_three
        .call(
            FLAGS_ID.into(),
            "disable_access_check",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS / 2,
            0, // attached deposit
        )
        .assert_success();

    let flags_consumer = oracle_three.deploy(
        &FLAGSTESTHELPER_WASM_BYTES,
        FLAGSTESTHELPER_ID.to_string(),
        to_yocto("1000"), // attached deposit
    );

    flags_consumer
        .call(
            FLAGSTESTHELPER_ID.into(),
            "new",
            &json!({
                "flags_contract": flags.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS / 2,
            0, // attached deposit
        )
        .assert_success();

    let consumer = root.deploy(
        &CONSUMER_WASM_BYTES,
        CONSUMER_ID.to_string(),
        to_yocto("1000"), // attached deposit
    );

    consumer
        .call(
            CONSUMER_ID.into(),
            "new",
            &json!({
                "oracle_account": oracle_one.account_id()
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
        oracle_four,
        oracle_five,
        aggregator_validator_mock_factory,
        flags,
        consumer,
        flags_consumer,
        controller,
        controller_2,
        flux_aggregator_test_helper_contract,
        eddy,
        mock_v3_aggregator,
        mock_v3_aggregator_second,
        read_controller,
        flux_aggregator,
    )
}
