import keccak256 from "keccak256";
import { MerkleTree } from "merkletreejs";
import WhitelistedAddressList from "./addresses.json";

const address =
  "erd1ss6u80ruas2phpmr82r42xnkd6rxy40g9jl69frppl4qez9w2jpsqj8x97";

const generateTree = () => {
  const leafs = WhitelistedAddressList.map((address: string) =>
    keccak256(address)
  );

  return new MerkleTree(leafs, keccak256, { sortPairs: true });
};

const run = () => {
  const merkleTree = generateTree();
  console.log("Merkle Tree:");
  console.log(merkleTree.toString());
  console.log("\n");

  console.log("Root Hash:", merkleTree.getHexRoot());

  console.log(`Get proof for address ${address}...`);

  const proofs = merkleTree.getHexProof(keccak256(address));
  console.log("Proofs:", proofs);
};

run();
