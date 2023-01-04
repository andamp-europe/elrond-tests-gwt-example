# Testing smart contracts with Given-When-Then (GWT) style

The contract used in this repository was created based on the ping-pong example contract using the following command: `erdpy contract new example-contract --template ping-pong-egld`. You can find the original source code [here](https://github.com/ElrondNetwork/elrond-wasm-rs/tree/master/contracts/examples/ping-pong-egld).


The contract was not modified, but only supplemented by exemplary tests in the Given-When-Then (GWT) style. You can run the GWT tests like this: `cargo test --test gwt_test`

Find out more about the intentions behind this in [this blog post](https://medium.com/@michaeljank/47720f9b0297).
