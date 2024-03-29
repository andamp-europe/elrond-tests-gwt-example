#![feature(trait_alias)]
mod common;

use common::basic::*;

#[test]
fn user_can_ping() {
  given_a_deployed_contract(5_000_000, 1, None, None).run_test(|setup, users| {
    given_user_has_egld(setup, &users.user1, 5_000_000);

    let tx = when_pinging(setup, &users.user1, 5_000_000);

    then_tx_should_succeed(&tx);
    then_user_should_have_egld(setup, &users.user1, 0);
    then_contract_should_have_egld(setup, 5_000_000);
  });
}

#[test]
fn user_cant_ping_with_wrong_egld_amount() {
  given_a_deployed_contract(5_000_000, 1, None, None).run_test(|setup, users| {
    given_user_has_egld(setup, &users.user1, 5_000_000);

    let tx = when_pinging(setup, &users.user1, 4_000_000);

    then_tx_should_fail(&tx, "the payment must match the fixed sum");
  });
}

#[test]
fn user_cant_ping_after_deadline() {
  given_a_deployed_contract(5_000_000, 1, None, None).run_test(|setup, users| {
    given_user_has_egld(setup, &users.user1, 5_000_000);

    when_block_timestamp_is(setup, DEPLOY_TIME + MIN);
    let tx = when_pinging(setup, &users.user1, 5_000_000);

    then_tx_should_fail(&tx, "deadline has passed");
  });
}
