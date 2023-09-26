#/bin/bash

junod tx wasm store ../artifacts/clock_example-aarch64.wasm --from dripper --gas 10000000 -y

sleep 5

junod tx wasm instantiate 1 '{}' --from dripper --gas 1000000 --label clocktest --no-admin -y

sleep 5

junod q wasm s smart juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 '{"get_config":{}}'

sleep 5 

# fund contract

junod tx bank send dripper juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 5000000000unice -y