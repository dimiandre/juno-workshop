#/bin/bash

junod tx staking delegate junovaloper1tgq44h68ddt3dmxgs26dpjsmzaaadp7efscr8w 1000000000ujunox --from staker -y

# fund contract

junod tx gov submit-proposal ./proposal.json --from dripper --chain-id testing --gas 1000000 --gas-prices 0.01ujunox -y

sleep 5

junod tx gov vote 1 yes --from staker -y

sleep 5 

junod q gov votes 1