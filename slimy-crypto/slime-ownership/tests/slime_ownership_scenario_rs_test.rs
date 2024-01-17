use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "file:../slime-genetic-alg/output/slime-genetic-alg.wasm",
        slime_genetic_alg::ContractBuilder,
    );
    blockchain.register_contract(
        "file:output/slime-ownership.wasm",
        slime_ownership::ContractBuilder,
    );

    blockchain
}

#[test]
fn breed_ok_rs() {
    world().run("scenarios/breed_ok.scen.json");
}

#[test]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}

#[test]
fn query_rs() {
    world().run("scenarios/query.scen.json");
}

#[test]
fn setup_accounts_rs() {
    world().run("scenarios/setup_accounts.scen.json");
}

#[test]
fn catch_gen_zero_rs() {
    world().run("scenarios/catch_gen_zero.scen.json");
}
