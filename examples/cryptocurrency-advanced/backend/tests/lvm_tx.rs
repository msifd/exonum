#[macro_use]
extern crate serde_json;

use common::{
    testkit::create_testkit,
};

mod common;

#[test]
fn create_contract() {
    let (mut testkit, api) = create_testkit();

    let code = "some code";
    let (tx, contract_pub) = api.create_contract(code);
    testkit.create_block();
    api.assert_tx_status(tx.hash(), &json!({ "type": "success" }));

    let contract = api.get_contract(contract_pub);
    assert!(contract.is_some());
    let contract = contract.unwrap();
    assert_eq!(contract.pub_key, contract_pub);
    assert_eq!(contract.code, code);
}
