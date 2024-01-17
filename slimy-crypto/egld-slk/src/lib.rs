#![no_std]

multiversx_sc::imports!();

const EGLD_MIN : u64 = 1e16 as u64;
const SLK_PER_MIN : u64 = 50;
const GRANULARITY: u64 = 50;
#[multiversx_sc::contract]
pub trait EgldEsdtSwap: multiversx_sc_modules::pause::PauseModule {
    #[init]
    fn init(&self, slime_token_id: TokenIdentifier) {
        self.slime_token_id().set(&slime_token_id);
    }

    // endpoints
    #[payable("EGLD")]
    #[endpoint(swapEgld)]
    fn swap_egld(&self) -> EsdtTokenPayment<Self::Api> {
        self.require_not_paused();

        let payment_amount = self.call_value().egld_value();
        require!(*payment_amount > EGLD_MIN, "Payment must be more than 0.01 EGLD");

        let cents = (*payment_amount).clone().div(BigUint::from(EGLD_MIN / GRANULARITY));
        let tokens = cents.mul(BigUint::from(SLK_PER_MIN / GRANULARITY));

        let slime_token_id = self.slime_token_id().get();
        self.send()
            .esdt_local_mint(&slime_token_id, 0, &tokens);

        let caller = self.blockchain().get_caller();
        self.send()
            .direct_esdt(&caller, &slime_token_id, 0, &tokens);

        EsdtTokenPayment::new(slime_token_id, 0, tokens)
    }

    #[view(getSlimeTokenId)]
    #[storage_mapper("slimeTokenId")]
    fn slime_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
