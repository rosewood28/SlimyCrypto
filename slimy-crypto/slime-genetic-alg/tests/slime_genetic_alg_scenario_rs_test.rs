use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "file:output/slime-genetic-alg.wasm",
        slime_genetic_alg::ContractBuilder,
    );
    blockchain
}

#[test]
fn generate_slime_genes_rs() {
    world().run("scenarios/generate-slime-genes.scen.json");
}

#[test]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}
