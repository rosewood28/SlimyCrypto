#![allow(non_snake_case)]

use slime_ownership::ProxyTrait as _;
use slime_ownership::*;

use multiversx_sc_snippets::{
    env_logger,
    erdrs::wallet::Wallet,
    multiversx_sc::{codec::multi_types::*, types::*},
    multiversx_sc_scenario::{
        api::StaticApi,
        bech32,
        scenario_format::interpret_trait::{InterpretableFrom, InterpreterContext},
        scenario_model::*,
        ContractInfo,
    },
    sdk, tokio, Interactor,
};


const GATEWAY: &str = sdk::blockchain::DEVNET_GATEWAY;
const PEM: &str = "alice.pem";
const SC_ADDRESS: &str = "";

const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";
const DEFAULT_ADDRESS_EXPR: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";
const TOKEN_ISSUE_COST: u64 = 50_000_000_000_000_000;

type ContractType = ContractInfo<slime_ownership::Proxy<StaticApi>>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut state = State::new().await;
    match cmd.as_str() {
        "deploy" => state.deploy().await,
        "setGeneScienceContractAddress" => state.set_gene_science_contract_address_endpoint().await,
        "totalSupply" => state.total_supply().await,
        "balanceOf" => state.balance_of().await,
        "ownerOf" => state.owner_of().await,
        "approve" => state.approve().await,
        "transfer" => state.transfer().await,
        "transfer_from" => state.transfer_from().await,
        "tokensOfOwner" => state.tokens_of_owner().await,
        "createGenZeroSlime" => state.create_gen_zero_slime().await,
        "catchSlime" => state.catch_slime().await,
        "getSlimeById" => state.get_slime_by_id_endpoint().await,
        "canBreedWith" => state.can_breed_with().await,
        "breedWith" => state.breed_with().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

struct State {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    contract: ContractType,
}

impl State {
    async fn new() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(Wallet::from_pem_file(PEM).unwrap());
        let sc_addr_expr = if SC_ADDRESS == "" {
            DEFAULT_ADDRESS_EXPR.to_string()
        } else {
            "bech32:".to_string() + SC_ADDRESS
        };
        let contract_code = BytesValue::interpret_from(
            "file:../output/slime-ownership.wasm",
            &InterpreterContext::default(),
        );
        let contract = ContractType::new(sc_addr_expr);

        State {
            interactor,
            wallet_address,
            contract_code,
            contract,
        }
    }

    async fn deploy(&mut self) {
        let opt_gene_science_contract_address = OptionalValue::Some(bech32::decode(""));

        let (new_address, _) = self
            .interactor
            .sc_deploy_get_result::<_, ()>(
                ScDeployStep::new()
                    .call(self.contract.init(opt_gene_science_contract_address))
                    .from(&self.wallet_address)
                    .code(&self.contract_code)
                    .expect(TxExpect::ok().additional_error_message("deploy failed: ")),
            )
            .await;
s
        let new_address_bech32 = bech32::encode(&new_address);
        println!("new address: {new_address_bech32}");
    }

    async fn set_gene_science_contract_address_endpoint(&mut self) {
        let address = bech32::decode("");

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.set_gene_science_contract_address_endpoint(address))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn total_supply(&mut self) {
        let result_value: u32 = self
            .interactor
            .vm_query(self.contract.total_supply())
            .await;

    }

    async fn balance_of(&mut self) {
        let address = bech32::decode("");

        let result_value: u32 = self
            .interactor
            .vm_query(self.contract.balance_of(address))
            .await;

    }

    async fn owner_of(&mut self) {
        let slime_id = 0u32;

        let result_value: ManagedAddress<StaticApi> = self
            .interactor
            .vm_query(self.contract.owner_of(slime_id))
            .await;

    }

    async fn approve(&mut self) {
        let to = bech32::decode("");
        let slime_id = 0u32;

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.approve(to, slime_id))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn transfer(&mut self) {
        let to = bech32::decode("");
        let slime_id = 0u32;

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.transfer(to, slime_id))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn transfer_from(&mut self) {
        let from = bech32::decode("");
        let to = bech32::decode("");
        let slime_id = 0u32;

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.transfer_from(from, to, slime_id))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn tokens_of_owner(&mut self) {
        let address = bech32::decode("");

        let result_value: MultiValueVec<u32> = self
            .interactor
            .vm_query(self.contract.tokens_of_owner(address))
            .await;

    }

    async fn create_gen_zero_slime(&mut self) {
        let behalf_of = bech32::decode("");

        let response: TypedResponse<u32> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.create_gen_zero_slime(behalf_of))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn catch_slime(&mut self) {
        let behalf_of = bech32::decode("");
        let slime_id = 0u32;

        let response: TypedResponse<u32> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.catch_slime(behalf_of, slime_id))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn get_slime_by_id_endpoint(&mut self) {
        let slime_id = 0u32;

        let result_value: Slime<StaticApi> = self
            .interactor
            .vm_query(self.contract.get_slime_by_id_endpoint(slime_id))
            .await;

    }

    async fn can_breed_with(&mut self) {
        let matron_id = 0u32;
        let sire_id = 0u32;

        let result_value: bool<StaticApi> = self
            .interactor
            .vm_query(self.contract.can_breed_with(matron_id, sire_id))
            .await;

    }

    async fn breed_with(&mut self) {
        let matron_id = 0u32;
        let sire_id = 0u32;

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.breed_with(matron_id, sire_id))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

}
