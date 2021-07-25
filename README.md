# Notes

- Need to clean up the suite of tests more dev friendly (remove commented out code, clean up variables, assertions, update imports, Github lines, etc.)
- Create an AccessControl contract that Flags makes cross-contract calls to. (DONE)
- Create a Consumer contract that matches the one used in Flags tests (FlagsTestHelper). (DONE)
- Create a FluxTestHelper contract that matches the one used in ACA tests (FluxTestHelper). (DONE)
- Create a SimpleReadAccessController contract that matches the one used in EAC tests (SimpleReadAccessController) (DONE). 
- Create a MockV3Aggregator contract that matches the one used in EAC tests (MockV3Aggregator) (DONE). 
- Port the rest of Flux Aggregator tests (DONE)
- Port the rest of Flags tests (DONE 30/30 PASSING)
- Port the rest of AccessControlledAggregator tests (DONE 16/16 PASSING)
- Port the rest of EACAggregator tests (DONE 2/12 PASSING)
- Fix bugs in suite of tests (IN PROGRESS)
- Implement transfer_and_call functionality into Fungible token standard, fix breaking changes. (IN PROGRESS)
- Create a https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/src/v0.6/tests/FluxAggregatorTestHelper.sol contract account for tests (DONE)
- Brainstorm and implement a better way of handling is_none cases, such as 0 states (DONE)
- Uncomment test with *TODO* comments to debug
- Look into Contract A calling Contract A function that returns bool or value from Promise (DONE)
- Assert that contracts have limited interfaces
- Refactor smart contracts into traits/implementations for composability, optimize read/writes from storage for gas.