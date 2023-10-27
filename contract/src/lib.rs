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
