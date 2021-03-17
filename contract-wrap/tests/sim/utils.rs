use near_sdk_sim::{init_simulator, to_yocto, STORAGE_AMOUNT};
near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    // update `contract.wasm` for your contract's name
    CONTRACT_WASM_BYTES => "target/wasm32-unknown-unknown/release/AccessControlledAggregator.wasm",
    CONTRACT2_WASM_BYTES => "target/wasm32-unknown-unknown/release/LinkToken.wasm",
}
const CONTRACT_ID: &str = "contract";

pub fn init() -> None {
    // Use `None` for default genesis configuration; more info below
    let root = init_simulator(None);

    let contract = root.deploy(
        &CONTRACT_WASM_BYTES,
        CONTRACT_ID.to_string(),
        STORAGE_AMOUNT // attached deposit
    );

    let alice = root.create_user(
        "alice".to_string(),
        to_yocto("100") // initial balance
    );

    (root, contract, alice)
}