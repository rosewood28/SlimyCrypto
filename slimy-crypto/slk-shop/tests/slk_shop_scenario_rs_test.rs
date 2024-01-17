use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");

    blockchain.register_contract("file:output/slk-shop.wasm", slk_shop::ContractBuilder);
    blockchain
}

#[test]
fn adder_rs() {
    world().run("scenarios/slk_shop.scen.json");
}
