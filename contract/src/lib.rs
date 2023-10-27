//use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};
// use near_sdk::collections::UnorderedMap;
// use serde::Deserialize;

// #[derive(BorshDeserialize, BorshSerialize, Deserialize)]
// pub struct NFTMetadata {
//     // title: String,
//     description: String,
//     external_link: String,
//     copies: u64,
// }

// const MINIMUM_DEPOSIT: Balance = 1_000_000_000_000_000_000_000; // 1 NEAR

// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize)]
// pub struct NFTCOLLECTION {
//     tokens: UnorderedMap<String, AccountId>,
//     owner: AccountId,
//     nft_collection: UnorderedMap<String, NFTMetadata>,
//     // receiver_id: Option<AccountId>,

// }
// impl Default for NFTCOLLECTION {
//     fn default() -> Self {
//         panic!("NFT should be initialized before usage");
//     }
// }

// #[near_bindgen]
// impl NFTCOLLECTION {
//     #[init]
//     pub fn new(owner_id: AccountId) -> Self {
//         assert!(!env::state_exists(), "Already initialized");
//         Self {
//             tokens: UnorderedMap::new(b"t".to_vec()),
//             owner: owner_id,
//             nft_collection: UnorderedMap::new(b"n".to_vec()),
//         }
//     }


//     #[payable]
//     pub fn mint_nft(&mut self, token_id: String, metadata: NFTMetadata) {
//         // Get the attached deposit
//         let deposit = env::attached_deposit();
    
//         // Check if the sender is the owner of the contract
//         if env::predecessor_account_id() != self.owner {
//             env::panic(b"Only the owner can mint NFTs");
//         }
    
//         // Check if the attached deposit meets the minimum requirement
//         if deposit < MINIMUM_DEPOSIT {
//             env::panic(b"Insufficient deposit");
//         }
    
//         // Insert the NFT into your collection
//         self.nft_collection.insert(&token_id, &metadata);
    
//         // If there is an excess deposit, return it to the sender
//         if deposit > MINIMUM_DEPOSIT {
//             let sender = env::predecessor_account_id();
//             let refund_amount = deposit - MINIMUM_DEPOSIT;
//             Promise::new(sender).transfer(refund_amount);
//         }
//     }
    
// }





//Working simple ....
// The contract represents an NFT (Non-Fungible Token) collection, allowing the owner to mint NFTs.
// use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::{env, near_bindgen, AccountId};
// use near_sdk::collections::UnorderedMap;

// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize)]
// pub struct NFTCOLLECTION {
//     tokens: UnorderedMap<String, AccountId>,
//     owner: AccountId,
// }

// impl Default for NFTCOLLECTION {
//     fn default() -> Self {
//         panic!("NFT should be initialized before usage")
//     }
// }

// #[near_bindgen]
// impl NFTCOLLECTION {
//     #[init]
//     pub fn new(owner_id: AccountId) -> Self {
//         assert!(!env::state_exists(), "Already initialized");
//         Self { tokens: UnorderedMap::new(b"t".to_vec()), owner: owner_id }
//     }

//     #[payable]
//     pub fn mint_nft(&mut self, token_id: String) {
//         assert_eq!(env::predecessor_account_id(), self.owner, "Only the owner can mint NFTs");
//         self.tokens.insert(&token_id, &self.owner);
//     }
// }


//Assoiate metadata ..
// This code defines NFT collection on the NEAR blockchain 
// with a minting function that can only be called by the
// owner of the collection. NFTs are identified by unique token IDs and have associated metadata.
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::UnorderedMap;
use serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub description: String,
    pub image: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NFTCOLLECTION {
    tokens: UnorderedMap<String, (AccountId, Metadata)>,
    owner: AccountId,
}

impl Default for NFTCOLLECTION {
    fn default() -> Self {
        panic!("NFT should be initialized before usage")
    }
}

#[near_bindgen]
impl NFTCOLLECTION {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self { tokens: UnorderedMap::new(b"t".to_vec()), owner: owner_id }
    }

    #[payable]
    pub fn mint_nft(&mut self, token_id: String, metadata: Metadata) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Only the owner can mint NFTs");
        self.tokens.insert(&token_id, &(self.owner.clone(), metadata));
    }
}
