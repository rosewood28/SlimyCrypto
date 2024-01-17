#![no_std]

multiversx_sc::imports!();

const CATCHER_PRICE: u8 = 5;
const ATK_PRICE: u8 = 10;
const HEALTH_PRICE: u8 = 10;

#[multiversx_sc::contract]
pub trait SlkShop: multiversx_sc_modules::pause::PauseModule {
    #[init]
    fn init(&self, slime_token_id: TokenIdentifier, att_buff_token_id: TokenIdentifier, health_pot_token_id: TokenIdentifier, catcher_token_id: TokenIdentifier) {
        self.slime_token_id().set(&slime_token_id);
        self.att_buff_token_id().set(&att_buff_token_id);
        self.health_pot_token_id().set(&health_pot_token_id);
        self.catcher_token_id().set(&catcher_token_id);
    }

    // endpoints
    #[payable("*")]
    #[endpoint(buyCatcher)]
    fn buy_catcher(&self) -> EsdtTokenPayment<Self::Api> {
        self.require_not_paused();

        let (token_id, payment_amount) = self.call_value().single_fungible_esdt();
        require!(token_id == self.slime_token_id().get(), "Payment must be in SLK!");
        require!(payment_amount > BigUint::from(CATCHER_PRICE), "Payment must be more than 5 SLK!");

        let amt = payment_amount.clone().div(BigUint::from(CATCHER_PRICE));

        let res_token_id = self.catcher_token_id().get();
        self.send()
            .esdt_local_mint(&res_token_id, 0, &amt);

        let caller = self.blockchain().get_caller();
        self.send()
            .direct_esdt(&caller, &res_token_id, 0, &amt);

        EsdtTokenPayment::new(res_token_id, 0, amt)
    }

    #[payable("*")]
    #[endpoint(buyHealthPot)]
    fn buy_health_pot(&self) -> EsdtTokenPayment<Self::Api> {
        self.require_not_paused();

        let (token_id, payment_amount) = self.call_value().single_fungible_esdt();
        require!(token_id == self.slime_token_id().get(), "Payment must be in SLK!");
        require!(payment_amount > BigUint::from(HEALTH_PRICE), "Payment must be more than 10 SLK!");

        let amt = payment_amount.clone().div(BigUint::from(HEALTH_PRICE));

        let res_token_id = self.health_pot_token_id().get();
        self.send()
            .esdt_local_mint(&res_token_id, 0, &amt);

        let caller = self.blockchain().get_caller();
        self.send()
            .direct_esdt(&caller, &res_token_id, 0, &amt);

        EsdtTokenPayment::new(res_token_id, 0, amt)
    }
    
    #[payable("*")]
    #[endpoint(buyATTBuff)]
    fn buy_att_buff(&self) -> EsdtTokenPayment<Self::Api> {
        self.require_not_paused();

        let (token_id, payment_amount) = self.call_value().single_fungible_esdt();
        require!(token_id == self.slime_token_id().get(), "Payment must be in SLK!");
        require!(payment_amount > BigUint::from(ATK_PRICE)  , "Payment must be more than 10 SLK!");

        let amt = payment_amount.clone().div(BigUint::from(ATK_PRICE));

        let res_token_id = self.att_buff_token_id().get();
        self.send()
            .esdt_local_mint(&res_token_id, 0, &amt);

        let caller = self.blockchain().get_caller();
        self.send()
            .direct_esdt(&caller, &res_token_id, 0, &amt);

        EsdtTokenPayment::new(res_token_id, 0, amt)
    }


    #[view(getSlimeTokenId)]
    #[storage_mapper("slimeTokenId")]
    fn slime_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getAttBuffTokenId)]
    #[storage_mapper("attBuffTokenId")]
    fn att_buff_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getHealthPotTokenId)]
    #[storage_mapper("healthPotTokenId")]
    fn health_pot_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getCatcherTokenId)]
    #[storage_mapper("catcherTokenId")]
    fn catcher_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
