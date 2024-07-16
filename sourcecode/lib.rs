#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22 {
    use ink::prelude::string::ToString;
    use openbrush::{
        contracts::ownable::*,
        contracts::psp22::{extensions::metadata::*, PSP22Error},
        traits::{
            Storage,
            String,
        },
        
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
    }

    impl PSP22 for Contract {}

    impl Ownable for Contract {}
    
    impl PSP22Metadata for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance, name: Option <String>, symbol: Option <String>, decimal: u8) -> Self {
            let mut instance = Self::default();
            let caller: AccountId = instance.env().caller();

            instance.metadata.name = name.into();
            instance.metadata.symbol = symbol.into();
            instance.metadata.decimals = decimal;
            instance._init_with_owner(caller);
            instance
                ._mint_to(instance.env().caller(), total_supply)
                .expect("Should mint total_supply");

            instance
        }


        #[ink(message, payable)]
        #[openbrush::modifiers(only_owner)]
        pub fn withdraw_specific_amount(&mut self, value: Balance) -> Result<(), PSP22Error>{
            
            if value > self.env().balance() {
                return Err(PSP22Error::Custom("insufficient funds!".to_string().into()));
            }

            match self.env().transfer(self.env().caller(), value) {
                Ok(_) => Ok(()),
                Err(_) => Err(PSP22Error::Custom("Transfer error: insufficient fund".to_string().into())),
            }
        }


    }

}