use elrond_wasm_debug::{rust_biguint, testing_framework::*, tx_mock::TxResult, DebugApi};
use example_contract::*;

use elrond_wasm::types::Address;
use elrond_wasm_debug::num_bigint::BigUint;

const WASM_PATH: &str = "output/warriorz-dao.wasm";

type ContractBase = example_contract::ContractObj<DebugApi>;

pub trait ContractBuilder = where Self: 'static + Copy + Fn() -> ContractBase;

#[derive(Clone, Debug)]
pub struct Users {
  pub owner: Address,
  pub user1: Address,
  pub user2: Address,
}

pub struct TestSetup<T: ContractBuilder> {
  pub blockchain_wrapper: BlockchainStateWrapper,
  pub users: Users,
  pub contract: ContractObjWrapper<ContractBase, T>,
}

impl<T> TestSetup<T>
where
  T: ContractBuilder,
{
  pub fn run_test(&mut self, test_fn: fn(setup: &mut TestSetup<T>, user: &Users)) {
    let users = self.users.clone();
    test_fn(self, &users);
  }
}

pub fn setup_contract<T: ContractBuilder>(dao_contract_builder: T) -> TestSetup<T> {
  let mut blockchain_wrapper = BlockchainStateWrapper::new();
  let owner = blockchain_wrapper.create_user_account(&rust_biguint!(0));
  let contract_wrapper = blockchain_wrapper.create_sc_account(
    &rust_biguint!(0),
    Some(&owner),
    dao_contract_builder,
    WASM_PATH,
  );

  let user1 = blockchain_wrapper.create_user_account(&rust_biguint!(0));
  let user2 = blockchain_wrapper.create_user_account(&rust_biguint!(0));

  TestSetup {
    blockchain_wrapper,
    users: Users {
      owner,
      user1,
      user2,
    },
    contract: contract_wrapper,
  }
}

pub fn deploy_contract<T: ContractBuilder>(
  contract_setup: &mut TestSetup<T>,
  ping_amount: &BigUint,
  duration_in_seconds: u64,
  opt_activation_timestamp: Option<u64>,
  max_funds: Option<BigUint>,
) -> TxResult {
  contract_setup.blockchain_wrapper.execute_tx(
    &contract_setup.users.owner,
    &contract_setup.contract,
    &rust_biguint!(0),
    |sc| {
      sc.init(
        &elrond_wasm::types::BigUint::from(ping_amount),
        duration_in_seconds,
        opt_activation_timestamp,
        max_funds.map(elrond_wasm::types::BigUint::from).into(),
      );
    },
  )
}
