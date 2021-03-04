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
      changeMethods: ["new", "change_oracles", "submit"],
      sender: accountId,
    });
    await contract.new({
      owner_id: accountId,
      link_id: "test.near",
      _payment_amount: "0",
      _timeout: "10",
      _validator: "test.near",
      _min_submission_value: "1",
      _max_submission_value: "10",
      _decimals: "18",
      _description: "LINK/USD",
    });
  });
  it("can be changed", async function () {
    const changeOracles = await contract.change_oracles({
      _removed: [],
      _added: [accountId],
      _added_admins: [accountId],
      _min_submissions: "1",
      _max_submissions: "1",
      _restart_delay: "0",
    });
    expect(changeOracles).toEqual("");
  });

  it("can be submitted", async function () {
    const submitAnswer = await contract.submit({
      _round_id: "1",
      _submission: "1",
    });
    expect(submitAnswer).toEqual("");
  });
});
