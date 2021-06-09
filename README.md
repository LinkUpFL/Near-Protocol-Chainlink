# Notes

- Need to clean up aca_flux_agg_tests to make more dev friendly, with easy to follow along notes
- Need to port the rest of Flux Aggregator tests
- Create a https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/src/v0.6/tests/FluxAggregatorTestHelper.sol contract account for tests
- Write a set of simulation tests for EACAggregatorProxy
- NEAR's simulation testing SDK does not allow you to nest tests, hence the instances of repeated code
- Brainstorm and implement a better way of handling is_none cases, such as 0 states.