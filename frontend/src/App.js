// const nearAPI = require('near-api-js');

// async function mintNFT() {
//     const config = {
//     networkId: 'default',
//     nodeUrl: 'https://rpc.testnet.near.org',
//     walletUrl: 'https://wallet.testnet.near.org',
//     helperUrl: 'https://helper.testnet.near.org',
//     explorerUrl: 'https://explorer.testnet.near.org',
//     };  // Network configuration

//     const keyStore = new nearAPI.keyStores.UnencryptedFileSystemKeyStore('//path of .credential folder');
//     const near = await nearAPI.connect({...config, keyStore});
//     const account = await near.account('bhuttag.testnet');

//     const contract = new nearAPI.Contract(account, 'bhuttag.testnet', {
//         viewMethods: [],
//         changeMethods: ['mint_nft'],
//         sender: 'bhuttag.testnet',
//     });

//     const metadata = {
//         title: 'My NFT Title',
//         description: 'My NFT Description',
//         image: 'https://example.com/my-nft-image.png',
//     };

//     await contract.mint_nft({ token_id: 'token1', metadata });
// }

// mintNFT().catch(console.error);
