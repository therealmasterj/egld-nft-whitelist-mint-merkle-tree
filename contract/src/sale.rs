elrond_wasm::imports!();
elrond_wasm::derive_imports!();

// Constants
const SLASH: &[u8] = "/".as_bytes();
const METADATA_KEY_NAME: &[u8] = "metadata:".as_bytes();
const PNG_FILE_EXTENSION: &[u8] = ".png".as_bytes();
const JSON_FILE_EXTENSION: &[u8] = ".json".as_bytes();

#[elrond_wasm::module]
pub trait SaleModule {
    fn build_token_name(&self, token_id: &ManagedBuffer) -> ManagedBuffer {
        let mut token_name = ManagedBuffer::new();
        token_name.append(&self.base_name().get());
        token_name.append(&ManagedBuffer::new_from_bytes(" #".as_bytes()));
        token_name.append(token_id);

        token_name
    }

    fn build_metadata(&self, token_id: &ManagedBuffer) -> ManagedBuffer {
        let metadata_key_name = ManagedBuffer::new_from_bytes(METADATA_KEY_NAME);

        let mut metadata = ManagedBuffer::new();
        metadata.append(&metadata_key_name);
        metadata.append(&self.metadata_cid().get());
        metadata.append(&ManagedBuffer::new_from_bytes(SLASH));
        metadata.append(token_id);
        metadata.append(&ManagedBuffer::new_from_bytes(JSON_FILE_EXTENSION));

        metadata
    }

    fn build_metadata_uri(&self, token_id: &ManagedBuffer) -> ManagedBuffer {
        let mut metadata_uri = ManagedBuffer::new();
        metadata_uri.append(&ManagedBuffer::new_from_bytes(
            "https://ipfs.io/ipfs/".as_bytes(),
        ));
        metadata_uri.append(&self.metadata_cid().get());
        metadata_uri.append(&&ManagedBuffer::new_from_bytes(SLASH));
        metadata_uri.append(token_id);
        metadata_uri.append(&ManagedBuffer::new_from_bytes(JSON_FILE_EXTENSION));

        metadata_uri
    }

    fn build_image_uri(&self, token_id: &ManagedBuffer) -> ManagedBuffer {
        let mut metadata_uri = ManagedBuffer::new();
        metadata_uri.append(&ManagedBuffer::new_from_bytes(
            "https://ipfs.io/ipfs/".as_bytes(),
        ));
        metadata_uri.append(&self.image_cid().get());
        metadata_uri.append(&&ManagedBuffer::new_from_bytes(SLASH));
        metadata_uri.append(token_id);
        metadata_uri.append(&ManagedBuffer::new_from_bytes(PNG_FILE_EXTENSION));

        metadata_uri
    }

    #[view(getNextTokenId)]
    #[storage_mapper("nextTokenId")]
    fn next_token_id(&self) -> SingleValueMapper<u32>;

    #[view(getMaxSupply)]
    #[storage_mapper("maxSupply")]
    fn max_supply(&self) -> SingleValueMapper<u32>;

    #[view(getWhitelistedMinterStatus)]
    #[storage_mapper("whitelistedMinterStatus")]
    fn whitelisted_minter_status(&self, minter: &ManagedAddress) -> SingleValueMapper<bool>;

    #[view(getSalePrice)]
    #[storage_mapper("salePrice")]
    fn sale_price(&self) -> SingleValueMapper<BigUint<Self::Api>>;

    #[view(isPublicSaleOpen)]
    #[storage_mapper("publicSaleOpen")]
    fn public_sale_open(&self) -> SingleValueMapper<bool>;

    #[view(isPrivateSaleOpen)]
    #[storage_mapper("privateSaleOpen")]
    fn private_sale_open(&self) -> SingleValueMapper<bool>;

    // Storage Mapper for NFT data during mint process

    #[view(getBaseName)]
    #[storage_mapper("baseName")]
    fn base_name(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view(getBaseImageUri)]
    #[storage_mapper("baseImageUri")]
    fn image_cid(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view(getBaseMetadataUri)]
    #[storage_mapper("baseMetadataUri")]
    fn metadata_cid(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view(getRoyalties)]
    #[storage_mapper("royalties")]
    fn royalties(&self) -> SingleValueMapper<BigUint<Self::Api>>;
}
