use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "file:output/egld-slk-swap-sc.wasm",
        egld_slk_swap::ContractBuilder,
    );
    blockchain
}

#[test]
fn swap_egld_rs() {
    world().run("scenarios/swap_egld.scen.json");
}
