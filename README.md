# nft_collection
This smart contract is designed to manage an NFT collection. It allows the owner to create new NFTs (minting) and associate each NFT with their NEAR account ID. The UnorderedMap is used to keep track of the ownership of each NFT by storing the mapping of token IDs to owner account IDs. The contract also ensures that it's properly initialized and that only the owner can mint NFTs
