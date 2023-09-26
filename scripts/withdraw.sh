#/bin/bash

junod q bank balances juno1q6hu769ve7zhxyyxwxk2p8maf2u2wrkj4eau69

sleep 3

junod tx distribution withdraw-all-rewards --from staker -y

sleep 3

junod q bank balances juno1q6hu769ve7zhxyyxwxk2p8maf2u2wrkj4eau69
