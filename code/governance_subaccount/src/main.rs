use ic_types::{CanisterId, PrincipalId};
use ledger_canister::{AccountIdentifier, Subaccount};
use ic_crypto_sha::Sha256;
use candid::Principal;


fn compute_neuron_staking_subaccount(controller: PrincipalId, nonce: u64) -> Subaccount {
    // The equivalent function in the UI is
    // https://github.com/dfinity/dfinity_wallet/blob/351e07d3e6d007b090117161a94ce8ec9d5a6b49/js-agent/src/canisters/createNeuron.ts#L63
    Subaccount({
        let mut state = Sha256::new();
        state.write(&[0x0c]);
        state.write(b"neuron-stake");
        state.write(controller.as_slice());
        state.write(&nonce.to_be_bytes());
        state.finish()
    })
}


fn main() {
    let controller: PrincipalId = PrincipalId::from(Principal::from_text("yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae").unwrap());
    let governance: PrincipalId = PrincipalId::from(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap());
    let ai: AccountIdentifier = AccountIdentifier::new(governance, Some(compute_neuron_staking_subaccount(controller, 0)));
    println!("{:?}", ai.to_hex());
}