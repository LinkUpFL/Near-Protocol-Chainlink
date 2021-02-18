1) Run  ./scripts/build.sh
2) Deploy: near deploy --accountId testfunctions.$NEAR_ACCT --wasmFile ./res/testfunctions.wasm --initFunction new --initArgs '{"owner_id": "testfunctions.'$NEAR_ACCT'"}'
3) Call lower_flags: near call testfunctions.$NEAR_ACCT lower_flags '{"subjects": ["testing1.near", "testing2.near"]}' --accountId $NEAR_ACCT