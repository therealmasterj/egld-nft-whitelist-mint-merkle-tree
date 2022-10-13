elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait MerkleTreeModule {
    fn verify_proof(
        &self,
        proofs: ManagedVec<ManagedByteArray<Self::Api, 32>>,
        leaf: ManagedByteArray<Self::Api, 32>,
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

        computed_hash == self.merkle_tree_root().get()
    }

    fn efficient_hash(
        &self,
        a: &ManagedBuffer,
        b: &ManagedBuffer,
    ) -> ManagedByteArray<Self::Api, 32> {
        let mut buffer_to_hash = ManagedBuffer::new();
        buffer_to_hash.append(&a);
        buffer_to_hash.append(&b);
        self.crypto().keccak256(&buffer_to_hash)
    }

    #[view(getMerkleTreeRoot)]
    #[storage_mapper("merkleTreeRoot")]
    fn merkle_tree_root(&self) -> SingleValueMapper<ManagedByteArray<Self::Api, 32>>;
}
