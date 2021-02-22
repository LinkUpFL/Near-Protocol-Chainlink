0) Run  ./scripts/build.sh
1) Run near create-account crosscontract1.$NEAR_ACCT --masterAccount $NEAR_ACCT
2) Deploy: near deploy --accountId crosscontract1.$NEAR_ACCT --wasmFile ./res/crosscontracttesting.wasm --initFunction new --initArgs '{"owner_id": "crosscontract1.'$NEAR_ACCT'"}'
3) Call lower_flags: near call testfunctions3.$NEAR_ACCT lower_flags '{"account": "testing11.near"}' --accountId $NEAR_ACCT