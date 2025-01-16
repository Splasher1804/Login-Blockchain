#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;
use multiversx_sc::{BigUint, ManagedBuffer};

#[multiversx_sc::contract]
pub trait Authentication {
    #[init]
    fn init(&self) {
        self.nb_of_users().set(0u64);
    }

    #[endpoint(register)]
    fn register(&self, signature: ManagedBuffer) {
        let caller = self.blockchain().get_caller();
        require!(
            self.user_address(&caller).is_empty(),
            "already registered"
        );

        self.signature_hash(&caller).set(&signature);
        self.user_address(&caller).set(&caller);
        self.nb_of_users().update(|nb| *nb += 1);
    }

    #[view(getSignatureHash)]
    fn get_signature_hash(&self) -> ManagedBuffer {
        let caller = self.blockchain().get_caller();
        require!(
            caller == self.user_address(&caller).get(),
            "Not allowed"
        );

        self.signature_hash(&caller).get()
    }

    #[view(getUserAddress)]
    fn get_user_address(&self) -> ManagedAddress {
        let caller = self.blockchain().get_caller();
        self.user_address(&caller).get()
    }

    #[storage_mapper("nbOfUsers")]
    fn nb_of_users(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("userAddress")]
    fn user_address(&self, user: &ManagedAddress) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("signatureHash")]
    fn signature_hash(&self, user: &ManagedAddress) -> SingleValueMapper<ManagedBuffer>;
}
