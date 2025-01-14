use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");
    blockchain.register_contract("mxsc:output/nft-login-project.mxsc.json", nft_login_project::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/nft_login_project.scen.json");
}
