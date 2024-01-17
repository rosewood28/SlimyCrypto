USER="user.pem"
DEV="slime_dev.pem"
SC_OWN="erd1qqqqqqqqqqqqqpgqtjxnqnejv8qn9fg5tep0mmq03u9pv3p8aa2qfg8der"
SC_GENETIC="erd1qqqqqqqqqqqqqpgq9ye6538vh0dcmx657qqpt5rgldy0zj96aa2qvkxe7l"
SC_SLK="erd1qqqqqqqqqqqqqpgqel7dl9dfp98nfh763rhdhsl8rhpffupraa2qqy2sfe"
SC_SHP="erd1qqqqqqqqqqqqqpgqk9xd4wz4s8vhzfw7hmg2z463hduze27vaa2q4fl9q9"
PROXY="https://testnet-api.multiversx.com"
API="https://testnet-api.multiversx.com"

dev_gen_wild() {
    read -p "Enter Hunter Address: " HUNTER
    mxpy contract call ${SC_OWN} --pem ${DEV} --gas-limit 5000000 --recall-nonce --function "createGenZeroSlime" --arguments ${HUNTER} --outfile="gen_wild.json" --send --proxy ${PROXY} --chain T

    RESULT_TX=$(mxpy data parse --file gen_wild.json --expression='data["emittedTransactionHash"]' | tail -n 1)
    curl -s -X 'GET' "${API}/transactions/${RESULT_TX}?fields=results" -H 'accept: application/json' > result.json
    OUTPUT=$(python3 parse_results.py)
    IFS='@' read -ra ADDR <<< "$OUTPUT"
    echo "WILD SLIME ID = ${ADDR[2]}"
}

dev_catch() {
    read -p "Enter Hunter Address: " HUNTER
    read -p "Enter Wild Slime ID (hex): " WILD_ID 
    mxpy contract call ${SC_OWN} --pem ${DEV} --gas-limit 5000000 --recall-nonce --function "catchSlime" --arguments ${HUNTER} ${WILD_ID} --outfile="catch_wild.json" --send --proxy ${PROXY} --chain T

    RESULT_TX=$(mxpy data parse --file catch_wild.json --expression='data["emittedTransactionHash"]' | tail -n 1)
    curl -s -X 'GET' "${API}/transactions/${RESULT_TX}?fields=results" -H 'accept: application/json' > result.json
    OUTPUT=$(python3 parse_results.py)
    IFS='@' read -ra ADDR <<< "$OUTPUT"

    echo "SLIME ID = ${ADDR[2]}"
}

dev_breed() {
    MATRON_ID=$1
    SIRE_ID=$2   
    
    mxpy contract call ${SC_OWN} --pem ${DEV} --gas-limit 50000000 --recall-nonce --function "breedWith" --arguments ${MATRON_ID} ${SIRE_ID} --outfile="breed.json" --send --proxy ${PROXY} --chain T

    RESULT_TX=$(mxpy data parse --file breed.json --expression='data["emittedTransactionHash"]' | tail -n 1)
    sleep 5
    curl -s -X 'GET' "${API}/transactions/${RESULT_TX}?fields=results" -H 'accept: application/json' > result.json
    OUTPUT=$(python3 parse_results.py)
    IFS='@' read -ra ADDR <<< "$OUTPUT"
    python3 slime_genes.py ${ADDR[2]}
}