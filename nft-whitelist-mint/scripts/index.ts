import { Address } from "@elrondnetwork/erdjs/out";
import keccak256 from "keccak256";
import { MerkleTree } from "merkletreejs";
import WhitelistedAddressList from "./addresses.json";

const address =
  "erd105el2j0ulvlz8rs3qejn7pyqphsnsw5fw4gtq056sluxgac4dvkqxstfg8";

const generateTree = () => {
  const leafs = WhitelistedAddressList.map((address: string) =>
    keccak256(bech32ErdAddressToHex(address))
  );

  return new MerkleTree(leafs, keccak256, { sortPairs: true });
};

const bech32ErdAddressToHex = (bech32Address: string) => {
  return `0x${new Address(bech32Address).hex()}`;
};

const run = () => {
  const merkleTree = generateTree();
  console.log("Merkle Tree:");
  console.log(merkleTree.toString());
  console.log("\n");

  console.log("Root Hash:", merkleTree.getHexRoot());

  console.log(`Get proof for address ${address}...`);

  const proofs = merkleTree.getHexProof(
    keccak256(bech32ErdAddressToHex(address))
  );
  console.log("Proofs:", proofs);
};

run();

// hex user address 7d33f549fcfb3e238e1106653f04800de1383a897550b03e9a87f86477156b2c
// 01e581c45f9487ebb736eaa3f291d25a3ff24e8970d5e58cb2f92a64201bf2d4
