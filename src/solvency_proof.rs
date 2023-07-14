use std::{fs::File, io::Write};

use ethers::{
    prelude::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};
use halo2_proofs::halo2curves::bn256::{Bn256, Fr as Fp};

use summa_backend::apis::snapshot::Snapshot;

pub async fn generate_proof_of_solvency(
    snapshot: &Snapshot<4, 6, 2, 8>,
    client: &SignerMiddleware<Provider<Http>, LocalWallet>,
) -> Vec<String> {
    let account_ownership = snapshot.get_proof_of_account_ownership();

    // TODO: replace hard coded balances
    let asset_sum: [Fp; 2] = [Fp::from(556863u64), Fp::from(556863u64)];

    let (_, proof) = snapshot
        .generate_proof_of_solvency(account_ownership.addresses.clone(), Some(asset_sum))
        .unwrap();

    proof
}
