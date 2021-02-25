0) Run  ./scripts/build.sh
1) Run near create-account eactest4.$NEAR_ACCT --masterAccount $NEAR_ACCT
2) Deploy: near deploy --accountId eactest14.$NEAR_ACCT --wasmFile ./res/eac_aggregator_proxy.wasm --initFunction new --initArgs '{"owner_id": "eactest14.'$NEAR_ACCT'", "_aggregator": "acasimple1.nolanjacobson.testnet", "_access_controller": "ikeeprunningout.testnet"}'
3) Call lower_flags: near call eactest14.$NEAR_ACCT aggregator '{}' --accountId $NEAR_ACCT