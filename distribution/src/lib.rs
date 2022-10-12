#![no_std]

elrond_wasm::imports!();

const HASH_LENGTH: usize = 32;

#[elrond_wasm::contract]
pub trait MerkleProofDistributionContract {
    #[init]
    fn init(&self, token_id: TokenIdentifier) {
        require!(
            token_id.is_valid_esdt_identifier(),
            "Invalid token provided"
        );
        self.airdrop_token_id().set(&token_id);
    }

    #[endpoint]
    #[only_owner]
    fn set_epoch_root(
        &self,
        epoch_number: BigUint,
        merkle_root: ManagedByteArray<Self::Api, HASH_LENGTH>,
    ) {
        self.epoch_root(&epoch_number).set(merkle_root);
    }

    #[endpoint]
    fn claim(
        &self,
        proof: ManagedVec<ManagedByteArray<Self::Api, HASH_LENGTH>>,
        index: BigUint,
        amount: BigUint,
        epoch_number: BigUint,
    ) {
        // check claim status
        let root = self.epoch_root(&epoch_number).get();
        let caller: ManagedAddress = self.blockchain().get_caller();
        require!(
            !self.claim_status(&caller, &epoch_number).get(),
            "User Already Claimed!"
        );
        // create_leaf
        let mut all_leaf_buffer = self.get_buffer_from_biguint(&index);
        all_leaf_buffer.append(&caller.as_managed_buffer());
        all_leaf_buffer.append(&self.get_buffer_from_biguint(&amount));
        let leaf_hash = self.crypto().keccak256(&all_leaf_buffer);
        // verify proof
        require!(
            self.verify(proof, root, leaf_hash),
            "Proof Missmatched Or Data Missmatch!"
        );
        // transfer ESDT
        let token_id = self.airdrop_token_id().get();

        // set claim status true
        self.claim_status(&caller, &epoch_number).set(true);
        // send airdrop to user wallet
        self.send().direct_esdt(&caller, &token_id, 0, &amount);
    }

    fn verify(
        &self,
        proofs: ManagedVec<ManagedByteArray<Self::Api, HASH_LENGTH>>,
        root: ManagedByteArray<Self::Api, HASH_LENGTH>,
        leaf: ManagedByteArray<Self::Api, HASH_LENGTH>,
    ) -> bool {
        let mut computed_hash = leaf;
        for proof_element in proofs.into_iter() {
            // Hash(current computed hash + current element of the proof)
            if &computed_hash.to_byte_array() <= &proof_element.to_byte_array() {
                computed_hash = self.efficient_hash(
                    &computed_hash.as_managed_buffer(),
                    &proof_element.as_managed_buffer(),
                );
            }
            // Hash(current element of the proof + current computed hash)
            else {
                computed_hash = self.efficient_hash(
                    &proof_element.as_managed_buffer(),
                    &computed_hash.as_managed_buffer(),
                );
            }
        }

        return computed_hash == root;
    }

    fn get_buffer_from_biguint(&self, number: &BigUint) -> ManagedBuffer {
        return number.to_bytes_be_buffer();
    }

    fn efficient_hash(
        &self,
        a: &ManagedBuffer,
        b: &ManagedBuffer,
    ) -> ManagedByteArray<Self::Api, HASH_LENGTH> {
        let mut buffer_to_hash = ManagedBuffer::new();
        buffer_to_hash.append(&a);
        buffer_to_hash.append(&b);
        return self.crypto().keccak256(&buffer_to_hash);
    }

    #[view(getTokenId)]
    #[storage_mapper("airdropTokenId")]
    fn airdrop_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getEpochRoot)]
    #[storage_mapper("epochRoot")]
    fn epoch_root(
        &self,
        epoch_number: &BigUint,
    ) -> SingleValueMapper<ManagedByteArray<Self::Api, HASH_LENGTH>>;

    #[view(getClaimStatus)]
    #[storage_mapper("claimStatus")]
    fn claim_status(
        &self,
        user: &ManagedAddress,
        epoch_number: &BigUint,
    ) -> SingleValueMapper<bool>;
}
