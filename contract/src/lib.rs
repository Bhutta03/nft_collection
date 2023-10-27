// Import necessary dependencies
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::UnorderedMap;
use serde::{Deserialize, Serialize};

// Define the Metadata struct to store NFT metadata
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub description: String,
    pub image: String,
}

// Define the NFTCOLLECTION struct as a NEAR smart contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NFTCOLLECTION {
    // Define storage for NFT tokens as an UnorderedMap
    tokens: UnorderedMap<String, (AccountId, Metadata)>,
    owner: AccountId, // Store the owner's account ID
}

// Implement the default behavior for NFTCOLLECTION
impl Default for NFTCOLLECTION {
    fn default() -> Self {
        panic!("NFT should be initialized before usage")
    }
}

// Implement methods for NFTCOLLECTION
#[near_bindgen]
impl NFTCOLLECTION {
    // Constructor for initializing the NFT collection
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        // Ensure the contract is not already initialized
        assert!(!env::state_exists(), "Already initialized");
        // Initialize the NFTCOLLECTION with an empty UnorderedMap and owner's AccountId
        Self { tokens: UnorderedMap::new(b"t".to_vec()), owner: owner_id }
    }

    // Mint an NFT (payable function)
    #[payable]
    pub fn mint_nft(&mut self, token_id: String, metadata: Metadata) {
        // Ensure that only the owner can mint NFTs
        assert_eq!(env::predecessor_account_id(), self.owner, "Only the owner can mint NFTs");
        // Insert the NFT information (owner and metadata) into the tokens map
        self.tokens.insert(&token_id, &(self.owner.clone(), metadata));
    }
}
