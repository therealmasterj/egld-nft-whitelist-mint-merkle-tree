#![no_std]

elrond_wasm::imports!();

pub mod merkle_tree;
pub mod nft_module;
pub mod owner;
pub mod sale;

use nft_module::NftAttributes;

#[elrond_wasm::contract]
pub trait NftWhitelistMintContract:
    merkle_tree::MerkleTreeModule + nft_module::NftModule + owner::OwnerModule + sale::SaleModule
{
    #[init]
    fn init(
        &self,
        sale_price: BigUint,
        base_name: ManagedBuffer,
        image_cid: ManagedBuffer,
        metadata_cid: ManagedBuffer,
        base_royalties: BigUint,
        max_supply: u32,
    ) {
        self.sale_price().set_if_empty(&sale_price);
        self.base_name().set_if_empty(&base_name);
        self.image_cid().set_if_empty(&image_cid);
        self.metadata_cid().set_if_empty(&metadata_cid);
        self.royalties().set_if_empty(&base_royalties);

        self.private_sale_open().set_if_empty(&false);
        self.public_sale_open().set_if_empty(&false);

        self.next_token_id().set_if_empty(&1u32);
        self.max_supply().set_if_empty(&max_supply);
    }

    /**
     * Mint from whitelist.
     *
     * Validations
     * [x] The private sale must be opened
     * [x] The payment should be egld and the amount should be equal to the price
     * [x] The caller should not have minted before
     * [x] The caller should be whitelisted
     *
     * Actions
     * [x] Mint a NFT and send it to the caller
     */
    #[payable("EGLD")]
    #[endpoint(whitelistMint)]
    fn whitelist_mint(&self, proofs: ManagedVec<ManagedByteArray<Self::Api, 32>>) {
        require!(
            self.private_sale_open().get(),
            "Private sale must be opened."
        );

        self.require_can_mint();

        let caller = self.blockchain().get_caller();
        require!(
            !self.whitelisted_minter_status(&caller).get(),
            "Already minted"
        );

        require!(
            self.verify_proof(proofs, self.crypto().keccak256(caller.as_managed_buffer())),
            "Caller is not whitelisted"
        );

        self.mint();
    }

    #[payable("EGLD")]
    #[endpoint(publicMint)]
    fn public_mint(&self) {
        self.require_can_mint();
        self.mint();
    }

    fn require_can_mint(&self) {
        require!(
            self.call_value().egld_value() == self.sale_price().get(),
            "Invalid amount provided"
        );

        require!(
            self.next_token_id().get() <= self.max_supply().get(),
            "Should exceed max supply."
        );
    }

    fn mint(&self) {
        let token_id = self.next_token_id().get();
        let mut token_id_managed_buffer = ManagedBuffer::new();
        token_id_managed_buffer.append_u32_be(token_id);

        let nonce = self.create_nft_with_attributes(
            &self.build_token_name(&token_id_managed_buffer),
            NftAttributes {
                metadata: self.build_metadata(&token_id_managed_buffer),
            },
            self.build_image_uri(&token_id_managed_buffer),
            self.build_metadata_uri(&token_id_managed_buffer),
            &self.royalties().get(),
        );

        // Update the next token ID.
        self.next_token_id().update(|id| *id += 1);

        // Send NFT to caller
        self.send().direct(
            &self.blockchain().get_caller(),
            &EgldOrEsdtTokenIdentifier::esdt(self.nft_token_id().get()),
            nonce,
            &BigUint::from(1u32),
        );
    }
}
