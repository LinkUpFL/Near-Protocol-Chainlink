1) Run  ./scripts/build.sh
2) Run near create-account testfunctions3.$NEAR_ACCT --masterAccount $NEAR_ACCT
2) Deploy: near deploy --accountId testfunctions3.$NEAR_ACCT --wasmFile ./res/testfunctions.wasm --initFunction new --initArgs '{"owner_id": "testfunctions3.'$NEAR_ACCT'"}'
3) Call lower_flags: near call testfunctions3.$NEAR_ACCT lower_flags '{"account": "testing11.near"}' --accountId $NEAR_ACCT