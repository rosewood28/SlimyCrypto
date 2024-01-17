USER="user.pem"
SC_OWN="erd1qqqqqqqqqqqqqpgqtjxnqnejv8qn9fg5tep0mmq03u9pv3p8aa2qfg8der"
SC_GENETIC="erd1qqqqqqqqqqqqqpgq9ye6538vh0dcmx657qqpt5rgldy0zj96aa2qvkxe7l"
SC_SLK="erd1qqqqqqqqqqqqqpgqel7dl9dfp98nfh763rhdhsl8rhpffupraa2qqy2sfe"
SC_SHP="erd1qqqqqqqqqqqqqpgqk9xd4wz4s8vhzfw7hmg2z463hduze27vaa2q4fl9q9"
PROXY="https://testnet-api.multiversx.com"
API="https://testnet-api.multiversx.com"

buy_SLK() {
    read -p "Enter Amount of mEGLD to swap (>10): " EGLD
    EGLD=$((${EGLD} * 1000000000000000))
    mxpy contract call ${SC_SLK} --pem ${USER} --gas-limit 5000000 --recall-nonce --function "swapEgld" --value ${EGLD} --send --proxy ${PROXY} --chain T
}

buy_catcher() {
    read -p "Enter Amount of Catchers to buy (>=1, 5 SLK each): " CATCHERS
    CATCHERS=$((${CATCHERS} * 5))
    CATCHERS=$(printf '0x%x\n' ${CATCHERS})
    FUNC_NAME=$(echo -n "buyCatcher" | xxd -p)
    FUNC_NAME=$(echo "0x${FUNC_NAME}")
    mxpy contract call ${SC_SHP} --pem ${USER} --gas-limit 5000000 --recall-nonce --function "ESDTTransfer" --arguments 0x534c4b2d376435616633 ${CATCHERS} ${FUNC_NAME} --send --proxy ${PROXY} --chain T
}

buy_buff() {
    read -p "Enter Amount of Attack Buff to buy (>=1, 10 SLK each): " ATT
    ATT=$((${ATT} * 10))
    ATT=$(printf '0x%x\n' ${ATT})
    FUNC_NAME=$(echo -n "buyATTBuff" | xxd -p)
    FUNC_NAME=$(echo "0x${FUNC_NAME}")
    mxpy contract call ${SC_SHP} --pem ${USER} --gas-limit 5000000 --recall-nonce --function "ESDTTransfer" --arguments 0x534c4b2d376435616633 ${ATT} ${FUNC_NAME} --send --proxy ${PROXY} --chain T
}

buy_heal() {
    read -p "Enter Amount of Health Pots to buy (>=1, 10 SLK each): " POT
    POT=$((${POT} * 10))
    POT=$(printf '0x%x\n' ${POT})
    FUNC_NAME=$(echo -n "buyHealthPot" | xxd -p)
    FUNC_NAME=$(echo "0x${FUNC_NAME}")
    mxpy contract call ${SC_SHP} --pem ${USER} --gas-limit 5000000 --recall-nonce --function "ESDTTransfer" --arguments 0x534c4b2d376435616633 ${POT} ${FUNC_NAME} --send --proxy ${PROXY} --chain T
}

buy_breed() {
    read -p "Enter matron slime_id (hex): " MATRON_ID
    read -p "Enter sire slime_id (hex): " SIRE_ID
    FUNC_NAME=$(echo -n "breed ${MATRON_ID} ${SIRE_ID}" | xxd -p)

    mxpy tx new --pem ${USER} --recall-nonce --receiver erd1vxhrppythmchz5el6yk6h5d0kl9zn0sm8aj7xgs2ntvtllkeaa2q0cqsrj --gas-limit 500000 --proxy ${PROXY} --chain T --data ESDTTransfer@534c4b2d376435616633@14@${FUNC_NAME} --send --outfile=breed_user.json > /dev/null
    RESULT_TX=$(mxpy data parse --file breed_user.json --expression='data["emittedTransactionHash"]' | tail -n 1)

    curl localhost:8080/breed/${RESULT_TX}
    echo
}

get_slime_id() {
    read -p "Enter slime_id (hex): " NUMBER
	mxpy contract query ${SC_OWN} --function="getSlimeById" --arguments ${NUMBER} --proxy=${PROXY}
}

parse_slime() {
    read -p "Enter Slime Data (hex): " SLIME_DATA
    python3 slime_genes.py ${SLIME_DATA} 
}
