use super::setup::*;
use elrond_wasm::elrond_codec::multi_types::IgnoreValue;
use elrond_wasm::types::Address;
use elrond_wasm_debug::num_bigint::BigUint;
use elrond_wasm_debug::tx_mock::TxResult;

use example_contract::PingPong;

pub const MIN: u64 = 60;
const D2022_11_11: u64 = 1668124800;

pub const DEPLOY_TIME: u64 = D2022_11_11;

pub fn given_user_has_egld<T: ContractBuilder>(
  setup: &mut TestSetup<T>,
  user: &Address,
  egld: &BigUint,
) {
  setup.blockchain_wrapper.set_egld_balance(user, egld);
}

pub fn given_a_deployed_contract(
  ping_amount: &BigUint,
  duration_in_seconds: u64,
  opt_activation_timestamp: Option<u64>,
  max_funds: Option<BigUint>,
) -> TestSetup<impl ContractBuilder> {
  let mut setup = setup_contract(example_contract::contract_obj);
  setup.blockchain_wrapper.set_block_timestamp(DEPLOY_TIME);
  deploy_contract(
    &mut setup,
    ping_amount,
    duration_in_seconds,
    opt_activation_timestamp,
    max_funds,
  )
  .assert_ok();
  setup
}

pub fn when_block_timestamp_is<T: ContractBuilder>(setup: &mut TestSetup<T>, time: u64) {
  setup.blockchain_wrapper.set_block_timestamp(time);
}

pub fn when_pinging<T: ContractBuilder>(
  setup: &mut TestSetup<T>,
  user: &Address,
  amount: &BigUint,
) -> TxResult {
  setup
    .blockchain_wrapper
    .execute_tx(&user, &setup.contract, amount, |sc| {
      sc.ping(IgnoreValue {});
    })
}

pub fn then_tx_should_succeed(tx: &TxResult) {
  tx.assert_ok();
}

pub fn then_tx_should_fail(tx: &TxResult, message: &str) {
  tx.assert_user_error(message);
}
