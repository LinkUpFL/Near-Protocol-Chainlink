describe("AccessControlledAggregator", function () {
    let near;
    let contract;
    let accountId;
    let accountOne;
    let accountTwo;
    const paymentAmount = 3
    const deposit = 100
    const answer = 100
    const minAns = 1
    const maxAns = 1
    const rrDelay = 0
    const timeout = 1800
    const decimals = 24
    const description = "LINK/USD"
    const minSubmissionValue = 1
    const maxSubmissionValue = 100000000000000000000
    const emptyAddress = ""

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
      owner_id: "",
      link_id: "link-near.nolanjacobson.testnet",
      _payment_amount: paymentAmount,
      _timeout: timeout,
      _validator: emptyAddress,
      _min_submission_value: minSubmissionValue,
      _max_submission_value: maxSubmissionValue,
      _decimals: decimals,
      _description: description,
    });
    accountOne = await near.account("test-account-1614816912569-4232549");
    accountTwo = await near.account("test-account-1614870841763-3263362");
    });

    describe('#constructor', () => {
        it('sets the paymentAmount', async () => {
          matchers.bigNum(h.bigNum(paymentAmount), await contract.paymentAmount())
        })

        it('sets the timeout', async () => {
          matchers.bigNum(h.bigNum(timeout), await contract.timeout())
        })

        it('sets the decimals', async () => {
          matchers.bigNum(h.bigNum(decimals), await contract.decimals())
        })

        it('sets the description', async () => {
          assert.equal(
            description,
            await contract.description(),
            )
        })
    })

    it("can be changed", async function () {
    const changeOracles = await contract.change_oracles({
      _removed: [],
      _added: ["test-account-1614816912569-4232549"],
      _added_admins: ["test-account-1614816912569-4232549"],
      _min_submissions: minAns,
      _max_submissions: maxAns,
      _restart_delay: rrDelay,
    });
    expect(changeOracles).toEqual("");
    });

    it("can be submitted", async function () {
    const submitAnswer = await accountOne.functionCall(
      accountId,
      "submit",
      {
        "_round_id": "1",
        "_submission": "1",
      },
      "300000000000000"
    );
    expect(submitAnswer).not.toBe("");
    });
});
