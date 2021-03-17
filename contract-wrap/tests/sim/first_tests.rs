// use near_sdk_sim::{init_simulator, to_yocto, STORAGE_AMOUNT};

// const CONTRACT_ID: &str = "contract";

// pub fn init() -> (UserAccount, UserAccount, UserAccount) {
//     // Use `None` for default genesis configuration; more info below
//     let root = init_simulator(None);

//     let contract = root.deploy(
//         &CONTRACT_WASM_BYTES,
//         CONTRACT_ID.to_string(),
//         STORAGE_AMOUNT // attached deposit
//     );

//     let alice = root.create_user(
//         "alice".to_string(),
//         to_yocto("100") // initial balance
//     );

//     (root, contract, alice)
// }