elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait OwnerModule:
    crate::merkle_tree::MerkleTreeModule + crate::nft_module::NftModule + crate::sale::SaleModule
{
    /**
     * Update the sale price.
     *
     * Validation
     * [x] Caller must be the owner
     *
     * Actions
     * [x] Update the sale price storage
     */
    #[only_owner]
    #[endpoint(setSalePrice)]
    fn set_sale_price(&self, new_price: BigUint) {
        self.sale_price().set(&new_price);
    }

    /**
     * Update the merkle root hash.
     *
     * Validation
     * [x] Caller must be the owner
     *
     * Actions
     * [x] Update the merkle root storage
     */
    #[only_owner]
    #[endpoint(setMerkleTreeRootHash)]
    fn set_merkle_tree_root_hash(&self, root: ManagedByteArray<Self::Api, 32>) {
        self.merkle_tree_root().set(root);
    }

    /**
     * Toggle the state of private sale.
     *
     * Validation
     * [x] Caller must be the owner
     * [x] The merkle tree root, used for whitelist, should must not be empty
     * [x] The public sale must not be enabled
     *
     * Actions
     * [x] The private sale state storage
     */
    #[only_owner]
    #[endpoint(togglePrivateSale)]
    fn toggle_private_sale(&self, state: bool) {
        require!(
            !self.merkle_tree_root().is_empty(),
            "Can't toggle private sale since MT root isn't set"
        );

        require!(
            !self.public_sale_open().get(),
            "Can't toggle private sale if public is open"
        );

        self.private_sale_open().set(state);
    }

    /**
     * Toggle the public sale state.
     *
     * Validation
     * [x] The caller must be the owner
     * [x] The private sale should be closed
     *
     * Actions
     * [x] Update the public sale state storage
     */
    #[only_owner]
    #[endpoint(togglePublicSale)]
    fn toggle_public_sale(&self, state: bool) {
        require!(
            !self.private_sale_open().get(),
            "Can't toggle public sale if private is open"
        );

        self.public_sale_open().set(state);
    }

    /**
     * Withdraw EGLD (mint fees + royalties) from the contract.
     *
     * Validation
     * [x] The caller must be the owner
     *
     * Actions
     * [x] Withdraw EGLD contract balance
     */
    #[only_owner]
    #[endpoint(withdraw)]
    fn withdraw(&self) {
        self.send().direct_egld(
            &self.blockchain().get_owner_address(),
            &self
                .blockchain()
                .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0u64),
        );
    }
}
