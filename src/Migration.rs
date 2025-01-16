#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;
use multiversx_sc::types::Address;

#[multiversx_sc::contract]
pub trait Migrations {
    #[init]
    fn init(&self) {
        let caller = self.blockchain().get_caller();
        self.owner().set(&caller);
    }

    #[endpoint(setCompleted)]
    fn set_completed(&self, completed: u64) {
        let caller = self.blockchain().get_caller();
        require!(caller == self.owner().get(), "Restricted to owner");
        self.last_completed_migration().set(completed);
    }

    #[view(getOwner)]
    #[storage_mapper("owner")]
    fn owner(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getLastCompletedMigration)]
    #[storage_mapper("lastCompletedMigration")]
    fn last_completed_migration(&self) -> SingleValueMapper<u64>;
}
