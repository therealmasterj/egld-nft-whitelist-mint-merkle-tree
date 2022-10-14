elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Debug)]
pub struct NftAttributes<M: ManagedTypeApi> {
    pub metadata: ManagedBuffer<M>,
}

#[elrond_wasm::module]
pub trait NftModule {
    // Issue a new NFT Token collection.
    // The name and the collection ticker are specified.
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(&self, collection_name: ManagedBuffer, collection_ticker: ManagedBuffer) {
        require!(self.nft_token_id().is_empty(), "Token already issued");

        let payment_amount = self.call_value().egld_value();
        self.send()
            .esdt_system_sc_proxy()
            .issue_non_fungible(
                payment_amount,
                &collection_name,
                &collection_ticker,
                NonFungibleTokenProperties {
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_change_owner: false,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().issue_callback())
            .call_and_exit()
    }

    // Apply local roles to the contract.
    // Current applied role is NFT Creation
    #[only_owner]
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self) {
        self.require_token_issued();

        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.nft_token_id().get(),
                [EsdtLocalRole::NftCreate][..].iter().cloned(),
            )
            .async_call()
            .call_and_exit()
    }

    #[callback]
    fn issue_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<EgldOrEsdtTokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.nft_token_id().set(&token_id.unwrap_esdt());
            }
            ManagedAsyncCallResult::Err(_) => {
                let caller = self.blockchain().get_owner_address();
                let returned = self.call_value().egld_or_single_esdt();
                if returned.token_identifier.is_egld() && returned.amount > 0 {
                    self.send()
                        .direct(&caller, &returned.token_identifier, 0, &returned.amount);
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_nft_with_attributes(
        &self,
        name: &ManagedBuffer,
        attributes: NftAttributes<Self::Api>,
        video_uri: ManagedBuffer,
        metadata_uri: ManagedBuffer,
        royalties: &BigUint,
    ) -> u64 {
        self.require_token_issued();

        let mut uris = ManagedVec::new();
        uris.push(video_uri);
        uris.push(metadata_uri);

        self.send().esdt_nft_create::<NftAttributes<Self::Api>>(
            &self.nft_token_id().get(),
            &BigUint::from(1u32),
            name,
            royalties,
            &ManagedBuffer::new(),
            &attributes,
            &uris,
        )
    }

    fn require_token_issued(&self) {
        require!(!self.nft_token_id().is_empty(), "Token not issued");
    }

    #[view(getNftTokenId)]
    #[storage_mapper("nftTokenId")]
    fn nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
