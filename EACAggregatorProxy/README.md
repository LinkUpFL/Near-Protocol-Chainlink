0) Run  ./scripts/build.sh
1) Run near create-account eactest2.$NEAR_ACCT --masterAccount $NEAR_ACCT
2) Deploy: near deploy --accountId eactest2.$NEAR_ACCT --wasmFile ./res/eac_aggregator_proxy.wasm --initFunction new --initArgs '{"owner_id": "eactest2.'$NEAR_ACCT'", "_aggregator": "nolanjacobson.testnet", "_access_controller": "ikeeprunningout.testnet"}'
3) Call lower_flags: near call eactest2.$NEAR_ACCT aggregator '{}' --accountId $NEAR_ACCT