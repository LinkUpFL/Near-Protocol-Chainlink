import "regenerator-runtime/runtime";
import { assert } from "chai";
describe("AccessControlledAggregator", function () {
  let near;
  let contract;
  let contract_owner;
  let carol;
  let neil;
  let bob;
  const paymentAmount = "3";
  const deposit = "100";
  const answer = "100";
  const minAns = "1";
  const maxAns = "1";
  const rrDelay = "0";
  const timeout = "1800";
  const decimals = "24";
  const description = "LINK/USD";
  const minSubmissionValue = "1";
  const maxSubmissionValue = "100000000000000000000";
  const emptyAddress = "";
  jasmine.DEFAULT_TIMEOUT_INTERVAL = 10000;

  beforeAll(async function () {
    near = await nearlib.connect(nearConfig);
    contract_owner = nearConfig.contractName;
    contract = await near.loadContract(nearConfig.contractName, {
      viewMethods: [
        "get_description",
        "get_decimals",
        "get_timeout",
        "get_payment_amount",
        "get_answer",
      ],
      changeMethods: ["new", "add_access", "change_oracles"],
      sender: contract_owner,
    });
    let initialized = await contract.new({
      owner_id: contract_owner,
      link_id: "link-near.nolanjacobson.testnet",
      _payment_amount: paymentAmount,
      _timeout: timeout,
      _validator: emptyAddress,
      _min_submission_value: minSubmissionValue,
      _max_submission_value: maxSubmissionValue,
      _decimals: decimals,
      _description: description,
    });
    carol = await near.account("test-account-1615051526983-9863040");
    neil = await near.account("test-account-1615051629968-6231189");
    bob = await near.account("test-account-1615052048859-1533686");
    if (neil && initialized) {
      await contract.change_oracles({
        _removed: [],
        _added: [neil.accountId],
        _added_admins: [neil.accountId],
        _min_submissions: minAns,
        _max_submissions: maxAns,
        _restart_delay: rrDelay,
      });
      await neil.functionCall(
        contract_owner,
        "submit",
        {
          _round_id: "1",
          _submission: "1",
        },
        "300000000000000"
      );
    }
  });

  describe("#constructor", () => {
    it("sets the paymentAmount", async () => {
      const paymentAmount = await contract.get_payment_amount();
      assert.strictEqual(paymentAmount, parseInt(paymentAmount));
    });

    it("sets the timeout", async () => {
      const expectedTimeout = await contract.get_timeout();
      assert.strictEqual(expectedTimeout, parseInt(timeout));
    });

    it("sets the decimals", async () => {
      const expectedDecimals = await contract.get_decimals();
      assert.strictEqual(expectedDecimals, parseInt(decimals));
    });

    it("sets the description", async () => {
      const expectedDescription = await contract.get_description();
      assert.strictEqual(expectedDescription, description);
    });
  });

  describe("#get_answer", () => {
    describe("when read by a contract", () => {
      describe("without explicit access", () => {
        it("reverts", async () => {
          const noAccessGetAnswer = await bob.functionCall(
            contract_owner,
            "get_answer",
            {
              _round_id: "1",
            },
            "300000000000000"
          );
          console.log(JSON.parse(noAccessGetAnswer, "here"));
          assert.isString(noAccessGetAnswer);
        });
      });
      describe("with access", () => {
        it("succeeds", async () => {
          const addAccess = await contract.add_access({
            _user: bob.accountId,
          });
          if (addAccess) {
            const accessGetAnswer = await bob.functionCall(
              contract_owner,
              "get_answer",
              {
                _round_id: "1",
              },
              "300000000000000"
            );
            assert.equal(accessGetAnswer, 1);
          }
        });
      });
    });
  });
  it("can be changed", async function () {
    const changeOracles = await contract.change_oracles({
      _removed: [],
      _added: [carol.accountId],
      _added_admins: [carol.accountId],
      _min_submissions: minAns,
      _max_submissions: maxAns,
      _restart_delay: rrDelay,
    });
    if (changeOracles) {
      assert(changeOracles).equal("");
    }
  });

  // it("can be submitted", async function () {
  //   const submitAnswer = await accountOne.functionCall(
  //     accountId,
  //     "submit",
  //     {
  //       _round_id: "1",
  //       _submission: "1",
  //     },
  //     "300000000000000"
  //   );
  //   expect(submitAnswer).not.toBe("");
  // });
});
