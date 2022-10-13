# NFT Whitelist Mint

### Disclaimer

This contract is not currently used in production and can be optimized. Use it at your own risks.

### Introduction

This smart contract allows to mint NFTs from a private presale (to several addresses whitelisted beforehand) and public sale. The smart contract use the Merkle Tree mechanism to verify if an user is authorized to mint during private sale. This means that for minting during private sale, the owner must provide the root hash of the tree beforehand and the final whitelisted user has to provide the proofs.

### Prerequisites

- npm installed (for executing about Merkle Tree)
- rust and erdpy installed
- Having NFT collection hosted on IPFS

### Details

The contract offers the following functionalities

- Managing the logic for the NFT name (metadata works fine also and are retrieved from Elrond API, see https://docs.elrond.com/tokens/nft-tokens/#docsNav)
- Setting NFT max supply
- Opening/closing private and public sale (sale starts when the owner decide to enable it)
- Minting NFT (only one mint for private, one NFT / transaction for public) using EGLD
- Withdrawing collected funds (from mint fees and royalties)

### Steps

- Go to the script folder and install npm dependencies `npm install`
- Collect all the whitelisted addresses and paste it in `addresses.json`
- Execute `npm run start` and store the `rootHash` and the `proofs` somewhere
- Go to the contract interaction folder. Replace in the script the variables if you want (including the root hash). Replace the root hash variable and proofs with the data previously stored
- Get the deployer pem key and store it into `/wallets/owner.pem`

### Contract deployment and some interactions

First go to interaction folder and execute the following steps in orders

- run `source devnet.snippets.sh`
- run `deploy` will deploy the contract and grab the contract address and paste it under `CONTRACT_ADDRESS` variable
- run `issueToken` (will create the NFT collection)
- run `setLocalRoles` (allows your contract to mint NFT tokens)
- run `setMerkleTreeRootHash`
- run `openPrivateSale` to open the private sale
