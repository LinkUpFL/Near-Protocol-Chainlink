# Notes

- Need to clean up the suite of tests more dev friendly (remove commented out code, clean up variables, assertions, update imports, Github lines, etc.)
- Create an AccessControl contract that Flags makes cross-contract calls to. (DONE)
- Create a Consumer contract that matches the one used in Flags tests (FlagsTestHelper). (DONE)
- Port the rest of Flux Aggregator tests (DONE)
- Port the rest of Flags tests (IN PROGRESS 90%)
- Port the rest of AccessControlledAggregator tests (IN PROGRESS 50%)
- Port the rest of EACAggregator tests (IN PROGRESS 80%)
- Fix bugs in suite of tests (IN PROGRESS)
- Implement transfer_and_call functionality into Fungible token standard, fix breaking changes. (IN PROGRESS)
- Create a https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/src/v0.6/tests/FluxAggregatorTestHelper.sol contract account for tests (DONE)
- Brainstorm and implement a better way of handling is_none cases, such as 0 states (DONE)
- Uncomment test with *TODO* comments to debug
- Look into Contract A calling Contract A function that returns bool or value from Promise