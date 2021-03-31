use near_sdk::serde_json::json;
use near_sdk_sim::{init_simulator, to_yocto, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT};

const ACA_ID: &str = "aca";
const LINKTOKEN_ID: &str = "lt";

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    // update `contract.wasm` for your contract's name
    ACA_WASM_BYTES => "target/wasm32-unknown-unknown/debug/AccessControlledAggregator.wasm",

    // if you run `cargo build` without `--release` flag:
    LINKTOKEN_WASM_BYTES => "target/wasm32-unknown-unknown/debug/LinkToken.wasm",
}

pub fn init_without_macros() -> (UserAccount, UserAccount, UserAccount, UserAccount) {
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
    aca.call(
        ACA_ID.into(),
        "new",
        &json!({
            "link_id": link.account_id(), "owner_id": root.account_id(), "_timeout": "1800", "_validator": root.valid_account_id(), "_min_submission_value": "1", "_max_submission_value": "10",
    "_decimals": "18", "_description": "NEAR/USD", "_payment_amount": "5"
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        0, // attached deposit
    ).assert_success();

    let oracle_one = root.create_user(
        "oracle_one".to_string(),
        to_yocto("1000000"), // initial balance
    );

    (root, aca, link, oracle_one)
}

// pub fn init_with_macros() -> (
//     UserAccount,
//     ContractAccount<LinkToken>,
//     ContractAccount<AccessControlledAggregator>,
//     UserAccount,
// ) {
//     let root = init_simulator(None);
//     // uses default values for deposit and gas
//     let link_token = deploy!(
//         // Contract Proxy
//         contract: LinkToken,
//         // Contract account id
//         contract_id: LINKTOKEN_ID,
//         // Bytes of contract
//         bytes: &LINKTOKEN_WASM_BYTES,
//         // User deploying the contract,
//         signer_account: root,
//         // init method
//         init_method: new(
//             root.account_id(), 100000
//         )
//     );
//     let aca = deploy!(
//         contract: AccessControlledAggregator,
//         contract_id: ACA_ID,
//         bytes: &ACA_WASM_BYTES,
//         signer_account: root,
//         init_method: new(
//             link_token.account_id(), root.account_id(),  10,  root.valid_account_id(),  1, 10,
//      18,  "NEAR/USD"
//         )
//     );
//     let oracle_one = root.create_user(
//         "oracle_one".to_string(),
//         to_yocto("1000000"), // initial balance
//     );

//     (root, link_token, aca, oracle_one)
// }
