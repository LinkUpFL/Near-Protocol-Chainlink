describe("AccessControlledAggregator", function () {
  let near;
  let contract;
  let accountId;

  jasmine.DEFAULT_TIMEOUT_INTERVAL = 10000;

  beforeAll(async function () {
    console.log("nearConfig", nearConfig);
    near = await nearlib.connect(nearConfig);
    accountId = nearConfig.contractName;
    contract = await near.loadContract(nearConfig.contractName, {
      viewMethods: [],
      changeMethods: ["new"],
      sender: accountId,
    });
  });

  it("can be constructed", async function () {
    const newContract = await contract.new({
      "owner_id": "nolan.testnet",
      "link_id": "nolan.testnet",
      "_payment_amount": "0",
      "owner_id": "acasimple1.nolan.testnet",
      "_timeout": "10",
      "_validator": "nolan.testnet",
      "_min_submission_value": "1",
      "_max_submission_value": "10",
      "_decimals": "18",
      "_description": "testing",
    });
    // {
    //   owner: "nolan.testnet",
    //   link_id: "nolan.testnet",
    //   validator: "",
    //   payment_amount: 0,
    //   max_submission_count: 0,
    //   min_submission_count: 0,
    //   restart_delay: 0,
    //   timeout: 0,
    //   decimals: parseInt(0),
    //   description: "testing",
    //   min_submission_value: 1,
    //   max_submission_value: 10,
    //   check_enabled: true,
    //   access_list: [],
    //   reporting_round_id: 0,
    //   latest_round_id: 0,
    //   oracles: [],
    //   rounds: [],
    //   details: [],
    //   requesters: [],
    //   oracle_addresses: [],
    //   recorded_funds: { available: 0, allocated: 0 },
    // }
    expect(newContract).toEqual("");
  });
});
