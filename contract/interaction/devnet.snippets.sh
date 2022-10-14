## Network Config
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
PEM_PATH="../../../wallets/minter.pem"

# NFT Sales Config
BASE_NAME=0x436f7079204541504553 # Copy EAPES
IMAGE_CID=0x516d52503946567a6e6d317878613346457344454e58686166524461525759364c4b51646738477967484e484b77 # QmRP9FVznm1xxa3FEsDENXhafRDaRWY6LKQdg8GygHNHKw
METADATA_CID=0x516d536159397a5a6e4b574761386a6d4d464e4e364c724447796b6a53697279557a385965556a6a4a3937413877 # QmSaY9zZnKWGa8jmMFNN6LrDGykjSiryUz8YeUjjJ97A8w
SALE_PRICE=10000000000000000 # 0.01 EGLD
ROYALTIES=500 # 5%
MAX_SUPPLY_HEX=0x3E8 # Max Supply is 1000
COLLECTION_NAME="0x436f7079436f6c6c656374696f6e" # CopyCollection
COLLECTION_TICKER="0x4343415045" # CCAPE

CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqjn4qd330m60fav8pj59230dyyhfjnc9vdvkqnu6n7r"

ROOT_HASH="0xc649b787f5e56e9cc6ac84c1f83e55b7c87aec4c907e46dcb16449017a61e46b"
PROOFS_ARG_WL_MINT="0xf9a256e072898f2e2355a9e5d0611b5b91c09909c0319430a1f8285cdafe713f"

build() {
    cd ../ && erdpy contract build && cd interaction
}

deploy() {
    erdpy contract deploy \
        --gas-limit=300000000 \
        --recall-nonce \
        --pem=${PEM_PATH} \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --project="../" \
        --arguments $SALE_PRICE $BASE_NAME $IMAGE_CID $METADATA_CID $ROYALTIES $MAX_SUPPLY_HEX \
        --send 
}

upgrade() {
    erdpy contract upgrade ${CONTRACT_ADDRESS} \
        --gas-limit=300000000 \
        --recall-nonce \
        --pem=${PEM_PATH} \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --project="../" \
        --arguments $SALE_PRICE $BASE_NAME $IMAGE_CID $METADATA_CID $ROYALTIES $MAX_SUPPLY_HEX \
        --send 
}

issueToken() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --gas-limit=100000000 \
        --recall-nonce \
        --pem=${PEM_PATH} \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function "issueToken" \
        --arguments $COLLECTION_NAME $COLLECTION_TICKER \
        --value 50000000000000000 \
        --send 
}

setLocalRoles() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --gas-limit=200000000 \
        --recall-nonce \
        --pem=${PEM_PATH} \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function "setLocalRoles" \
        --send 
}

setRootHash() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --gas-limit=10000000 \
        --recall-nonce \
        --pem=${PEM_PATH} \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function "setMerkleTreeRootHash" \
        --arguments $ROOT_HASH \
        --send 
}

openPrivateSale() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --gas-limit=10000000 \
        --recall-nonce \
        --pem=${PEM_PATH} \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function "togglePrivateSale" \
        --arguments 0x01 \
        --send 
}

closePrivateSale() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --gas-limit=10000000 \
        --recall-nonce \
        --pem=${PEM_PATH} \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function "togglePrivateSale" \
        --arguments 0x00 \
        --send 
}

openPublicSale() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --gas-limit=10000000 \
        --recall-nonce \
        --pem=${PEM_PATH} \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function "togglePublicSale" \
        --arguments 0x01 \
        --send 
}

closePublicSale() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --gas-limit=10000000 \
        --recall-nonce \
        --pem=${PEM_PATH} \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function "togglePublicSale" \
        --arguments 0x00 \
        --send 
}

whitelistMint() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --gas-limit=10000000 \
        --recall-nonce \
        --pem=${PEM_PATH} \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function "whitelistMint" \
        --arguments $PROOFS_ARG_WL_MINT \
        --value $SALE_PRICE \
        --send 
}

publicMint() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --gas-limit=10000000 \
        --recall-nonce \
        --pem=${PEM_PATH} \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function "publicMint" \
        --value $SALE_PRICE \
        --send 
}