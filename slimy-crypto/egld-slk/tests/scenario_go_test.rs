use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn swap_egld_go() {
    world().run("../scenarios/swap_egld.scen.json");
}
