use std::{str::FromStr, sync::Arc};

use ethers::{
    abi::Address,
    prelude::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
    types::Bytes,
};
use halo2_proofs::halo2curves::bn256::Fr as Fp;

use summa_backend::apis::snapshot::Snapshot;

use super::summa_contract::summa::Summa;

pub async fn generate_proof_of_solvency(
    snapshot: &Snapshot<4, 6, 2, 8>,
    client: &SignerMiddleware<Provider<Http>, LocalWallet>,
) -> Vec<String> {
    let client2 = Arc::new(client.clone());
    let contract_address = Address::from_str("0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512").unwrap();
    let _summa_contract = Summa::new(contract_address, client2);

    let ownership_data = snapshot.get_proof_of_account_ownership();

    // TODO: replace hard coded balances
    let asset_sum: [Fp; 2] = [Fp::from(556863u64), Fp::from(556863u64)];

    let (_, proof) = snapshot
        .generate_proof_of_solvency(ownership_data.get_addresses().clone(), Some(asset_sum))
        .unwrap();

    proof
}

pub async fn generate_proof_of_ownership(
    snapshot: &Snapshot<4, 6, 2, 8>,
    client: &SignerMiddleware<Provider<Http>, LocalWallet>,
) {
    let ownership_data = snapshot.get_proof_of_account_ownership();
    let client2 = Arc::new(client.clone());
    let contract_address = Address::from_str("0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512").unwrap();
    let summa_contract = Summa::new(contract_address, client2);

    // Converting types to be compatible with the contract
    let addresses = ownership_data.get_addresses();

    let mut cex_addresses = Vec::<Address>::new();
    for addr in addresses {
        cex_addresses.push(Address::from_str(addr).unwrap());
    }

    let signatures = ownership_data.get_signatures();

    let mut cex_signatures = Vec::<Bytes>::new();
    for sig in signatures {
        cex_signatures.push(Bytes::from_str(sig).unwrap())
    }

    let message = ownership_data.get_message();

    summa_contract
        .submit_proof_of_account_ownership(cex_addresses, cex_signatures, message.to_string())
        .await
        .unwrap();

    println!("the proof has been validated!");
}
