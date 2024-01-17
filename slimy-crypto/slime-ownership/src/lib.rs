#![no_std]
#![allow(clippy::suspicious_operation_groupings)]

multiversx_sc::imports!();

use core::cmp::max;

use slime::{slime_genes::*, Slime};
use random::*;
    
#[multiversx_sc::contract]
pub trait SlimeOwnership {
    #[allow_multiple_var_args]
    #[init]
    fn init(
        &self,
        opt_gene_science_contract_address: OptionalValue<ManagedAddress>,
    ) {
        if let OptionalValue::Some(addr) = opt_gene_science_contract_address {
            self.gene_science_contract_address().set(&addr);
        }

        self.create_genesis_slime();
    }

    // endpoints - owner-only

    #[only_owner]
    #[endpoint(setGeneScienceContractAddress)]
    fn set_gene_science_contract_address_endpoint(&self, address: ManagedAddress) {
        self.gene_science_contract_address().set(&address);
    }

    // views/endpoints - ERC721 required

    #[view(totalSupply)]
    fn total_supply(&self) -> u32 {
        self.total_slimes().get() - 1 // not counting genesis Slime
    }

    #[view(balanceOf)]
    fn balance_of(&self, address: ManagedAddress) -> u32 {
        self.nr_owned_slimes(&address).get()
    }

    #[view(ownerOf)]
    fn owner_of(&self, slime_id: u32) -> ManagedAddress {
        if self.is_valid_id(slime_id) {
            self.slime_owner(slime_id).get()
        } else {
            ManagedAddress::zero()
        }
    }

    #[endpoint]
    fn approve(&self, to: ManagedAddress, slime_id: u32) {
        let caller = self.blockchain().get_caller();

        require!(self.is_valid_id(slime_id), "Invalid slime id!");
        require!(
            self.slime_owner(slime_id).get() == caller,
            "You are not the owner of that slime!"
        );

        self.approved_address(slime_id).set(&to);
        self.approve_event(&caller, &to, slime_id);
    }

    #[endpoint]
    fn transfer(&self, to: ManagedAddress, slime_id: u32) {
        let caller = self.blockchain().get_caller();

        require!(self.is_valid_id(slime_id), "Invalid slime id!");
        require!(!to.is_zero(), "Can't transfer to default address 0x0!");
        require!(
            to != self.blockchain().get_sc_address(),
            "Can't transfer to this contract!"
        );
        require!(
            self.slime_owner(slime_id).get() == caller,
            "You are not the owner of that slime!"
        );

        self.perform_transfer(&caller, &to, slime_id);
    }

    #[endpoint]
    fn transfer_from(&self, from: ManagedAddress, to: ManagedAddress, slime_id: u32) {
        let caller = self.blockchain().get_caller();

        require!(self.is_valid_id(slime_id), "Invalid slime id!");
        require!(!to.is_zero(), "Can't transfer to default address 0x0!");
        require!(
            to != self.blockchain().get_sc_address(),
            "Can't transfer to this contract!"
        );
        require!(
            self.slime_owner(slime_id).get() == from,
            "ManagedAddress _from_ is not the owner!"
        );
        require!(
            self.slime_owner(slime_id).get() == caller
                || self.get_approved_address_or_default(slime_id) == caller,
            "You are not the owner of that slime nor the approved address!"
        );

        self.perform_transfer(&from, &to, slime_id);
    }

    #[view(tokensOfOwner)]
    fn tokens_of_owner(&self, address: ManagedAddress) -> MultiValueEncoded<u32> {
        let nr_owned_slimes = self.nr_owned_slimes(&address).get();
        let total_slimes = self.total_slimes().get();
        let mut slime_list = ManagedVec::new();
        let mut list_len = 0; // more efficient than calling the API over and over

        for slime_id in 1..total_slimes {
            if nr_owned_slimes as usize == list_len {
                break;
            }

            if self.slime_owner(slime_id).get() == address {
                slime_list.push(slime_id);
                list_len += 1;
            }
        }

        slime_list.into()
    }

    // create gen zero slime
    // returns new slime id
    #[only_owner]
    #[endpoint(createGenZeroSlime)]
    fn create_gen_zero_slime(&self, behalf_of: ManagedAddress) -> u32 {
        let mut random = Random::new(
            self.blockchain().get_block_random_seed(),
            self.blockchain().get_tx_hash(),
        );
        let genes = SlimeGenes::get_random(&mut random);

        self.create_new_gen_zero_slime(genes, behalf_of)
    }

    #[only_owner]
    #[endpoint(catchSlime)]
    fn catch_slime(&self, behalf_of: ManagedAddress, slime_id: u32) -> u32 {
        require!(self.is_valid_wild_id(slime_id), "Invalid wild slime!");
        require!(behalf_of == self.hunter_address(slime_id).get(), "Only the hunter can catch!");
        let wild = self.wild_slime_by_id(slime_id).get();
        self.create_new_slime(wild.matron_id, wild.sire_id, wild.generation, wild.genes, &behalf_of)
    }

    // views - Slime Breeding

    #[view(getSlimeById)]
    fn get_slime_by_id_endpoint(&self, slime_id: u32) -> Slime {
        if self.is_valid_id(slime_id) {
            self.slime_by_id(slime_id).get()
        } else {
            sc_panic!("slime does not exist!")
        }
    }

    #[view(canBreedWith)]
    fn can_breed_with(&self, matron_id: u32, sire_id: u32) -> bool {
        require!(self.is_valid_id(matron_id), "Invalid matron id!");
        require!(self.is_valid_id(sire_id), "Invalid sire id!");

        self.is_valid_mating_pair(matron_id, sire_id)
            && self.is_siring_permitted(matron_id, sire_id)
    }

    // endpoints - Slime Breeding

    #[only_owner]
    #[endpoint(breedWith)]
    fn breed_with(&self, matron_id: u32, sire_id: u32) {
        require!(self.is_valid_id(matron_id), "Invalid matron id!");
        require!(self.is_valid_id(sire_id), "Invalid sire id!");

        let matron = self.slime_by_id(matron_id).get();
        let sire = self.slime_by_id(sire_id).get();

        require!(
            self.is_valid_mating_pair(matron_id, sire_id),
            "Not a valid mating pair!"
        );

        let gene_science_contract_address = self.get_gene_science_contract_address_or_default();
        if !gene_science_contract_address.is_zero() {
            self.slime_genetic_alg_proxy(gene_science_contract_address)
                .generate_slime_genes(matron, sire)
                .async_call()
                .with_callback(
                    self.callbacks()
                        .generate_slime_genes_callback(matron_id, sire_id),
                )
                .call_and_exit()
        } else {
            sc_panic!("Gene science contract address not set!")
        }
    }

    // private

    fn perform_transfer(&self, from: &ManagedAddress, to: &ManagedAddress, slime_id: u32) {
        if from == to {
            return;
        }

        let mut nr_owned_to = self.nr_owned_slimes(to).get();
        nr_owned_to += 1;

        if !from.is_zero() {
            let mut nr_owned_from = self.nr_owned_slimes(from).get();
            nr_owned_from -= 1;

            self.nr_owned_slimes(from).set(nr_owned_from);
            self.approved_address(slime_id).clear();
        }

        self.nr_owned_slimes(to).set(nr_owned_to);
        self.slime_owner(slime_id).set(to);

        self.transfer_event(from, to, slime_id);
    }

    // checks should be done in the caller function
    // returns the newly created slime id
    fn create_new_slime(
        &self,
        matron_id: u32,
        sire_id: u32,
        generation: u16,
        genes: SlimeGenes,
        owner: &ManagedAddress,
    ) -> u32 {
        let mut total_slimes = self.total_slimes().get();
        let new_slime_id = total_slimes;
        let slime = Slime::new(
            genes,
            self.blockchain().get_block_timestamp(),
            matron_id,
            sire_id,
            generation,
        );

        total_slimes += 1;
        self.total_slimes().set(total_slimes);
        self.slime_by_id(new_slime_id).set(slime);

        self.perform_transfer(&ManagedAddress::zero(), owner, new_slime_id);

        new_slime_id
    }

    fn create_new_wild_slime(
        &self,
        matron_id: u32,
        sire_id: u32,
        generation: u16,
        genes: SlimeGenes,
        owner: &ManagedAddress,
    ) -> u32 {
        let mut total_slimes = self.total_wild_slimes().get();
        let new_slime_id = total_slimes;
        let slime = Slime::new(
            genes,
            self.blockchain().get_block_timestamp(),
            matron_id,
            sire_id,
            generation,
        );

        total_slimes += 1;
        self.total_wild_slimes().set(total_slimes);
        self.wild_slime_by_id(new_slime_id).set(slime);
        self.hunter_address(new_slime_id).set(owner);

        new_slime_id
    }

    fn create_new_gen_zero_slime(&self, genes: SlimeGenes, behalf_of: ManagedAddress) -> u32 {
        self.create_new_wild_slime(0, 0, 0, genes, &behalf_of)
    }

    fn create_genesis_slime(&self) {
        let genesis_slime = Slime::default();

        self.create_new_slime(
            genesis_slime.matron_id,
            genesis_slime.sire_id,
            genesis_slime.generation,
            genesis_slime.genes,
            &ManagedAddress::zero(),
        );
    }

    // private - Slime checks. These should be in the Slime struct,
    // but unfortunately, they need access to the contract-only functions

    fn is_valid_id(&self, slime_id: u32) -> bool {
        slime_id != 0 && slime_id < self.total_slimes().get()
    }

    fn is_valid_wild_id(&self, slime_id: u32) -> bool {
        slime_id != 0 && slime_id < self.total_wild_slimes().get()
    }

    fn is_siring_permitted(&self, matron_id: u32, sire_id: u32) -> bool {
        let sire_owner = self.slime_owner(sire_id).get();
        let matron_owner = self.slime_owner(matron_id).get();

        sire_owner == matron_owner
    }

    fn is_valid_mating_pair(&self, matron_id: u32, sire_id: u32) -> bool {
        let matron = self.slime_by_id(matron_id).get();
        let sire = self.slime_by_id(sire_id).get();

        // can't breed with itself
        if matron_id == sire_id {
            return false;
        }

        // can't breed with their parents
        if matron.matron_id == sire_id || matron.sire_id == sire_id {
            return false;
        }
        if sire.matron_id == matron_id || sire.sire_id == matron_id {
            return false;
        }

        // for gen zero slimes
        if sire.matron_id == 0 || matron.matron_id == 0 {
            return true;
        }

        // can't breed with full or half siblings
        if sire.matron_id == matron.matron_id || sire.matron_id == matron.sire_id {
            return false;
        }
        if sire.sire_id == matron.matron_id || sire.sire_id == matron.sire_id {
            return false;
        }

        true
    }

    // getters

    fn get_gene_science_contract_address_or_default(&self) -> ManagedAddress {
        if self.gene_science_contract_address().is_empty() {
            ManagedAddress::zero()
        } else {
            self.gene_science_contract_address().get()
        }
    }

    fn get_approved_address_or_default(&self, slime_id: u32) -> ManagedAddress {
        if self.approved_address(slime_id).is_empty() {
            ManagedAddress::zero()
        } else {
            self.approved_address(slime_id).get()
        }
    }

    // callbacks

    #[callback]
    fn  generate_slime_genes_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<SlimeGenes>,
        matron_id: u32,
        sire_id: u32,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(genes) => {
                let mut matron = self.slime_by_id(matron_id).get();
                let mut sire = self.slime_by_id(sire_id).get();

                let new_slime_generation = max(matron.generation, sire.generation) + 1;

                // new slime goes to the owner of the matron
                let new_slime_owner = self.slime_owner(matron_id).get();
                let _new_slime_id = self.create_new_slime(
                    matron_id,
                    sire_id,
                    new_slime_generation,
                    genes,
                    &new_slime_owner,
                );

                // update matron slime
                matron.nr_children += 1;
                self.slime_by_id(matron_id).set(&matron);

                // update sire slime
                sire.nr_children += 1;
                self.slime_by_id(sire_id).set(&sire);
            }
            ManagedAsyncCallResult::Err(_) => {
                // this can only fail if the slime_genes contract address is invalid
                // in which case, the only thing we can do is call this again later
            }
        }
    }

    // proxy

    #[proxy]
    fn slime_genetic_alg_proxy(&self, to: ManagedAddress) -> slime_genetic_alg::Proxy<Self::Api>;

    // storage - General

    #[storage_mapper("geneScienceContractAddress")]
    fn gene_science_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    // storage - Slime

    #[storage_mapper("totalSlimes")]
    fn total_slimes(&self) -> SingleValueMapper<u32>;

    #[storage_mapper("slime")]
    fn slime_by_id(&self, slime_id: u32) -> SingleValueMapper<Slime>;

    #[storage_mapper("wildSlime")]
    fn wild_slime_by_id(&self, slime_id: u32) -> SingleValueMapper<Slime>;

    #[storage_mapper("totalWildSlimes")]
    fn total_wild_slimes(&self) -> SingleValueMapper<u32>;

    #[storage_mapper("hunterAddress")]
    fn hunter_address(&self, slime_id: u32) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("owner")]
    fn slime_owner(&self, slime_id: u32) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("nrOwnedSlimes")]
    fn nr_owned_slimes(&self, address: &ManagedAddress) -> SingleValueMapper<u32>;

    #[storage_mapper("approvedAddress")]
    fn approved_address(&self, slime_id: u32) -> SingleValueMapper<ManagedAddress>;

    // events

    #[event("transfer")]
    fn transfer_event(
        &self,
        #[indexed] from: &ManagedAddress,
        #[indexed] to: &ManagedAddress,
        #[indexed] token_id: u32,
    );

    #[event("approve")]
    fn approve_event(
        &self,
        #[indexed] owner: &ManagedAddress,
        #[indexed] approved: &ManagedAddress,
        #[indexed] token_id: u32,
    );
}
