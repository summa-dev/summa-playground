#[cfg(test)]
mod tests {
    use std::{str::FromStr, sync::Arc};

    use ethers::{
        abi::Address,
        prelude::SignerMiddleware,
        providers::{Http, Middleware, Provider, StreamExt},
        signers::{LocalWallet, Signer},
        types::{Bytes, U256},
    };
    use halo2_proofs::halo2curves::bn256::Fr as Fp;

    use summa_backend::apis::snapshot::Snapshot;

    use crate::summa_contract::{summa::Summa, ProofOfSolvencySubmittedFilter};
    // use summa_contract::summa::Summa;

    #[tokio::test]
    async fn get_event() -> Result<(), Box<dyn std::error::Error>> {
        let snapshot = Snapshot::<4, 6, 2, 8>::new(
            &"csv/entry_16.csv".to_string(),
            &"csv/signatures.csv".to_string(),
            "Summa proof of solvency for CryptoExchange".to_string(),
            &"ptau/hermez-raw-11".to_string(),
        )
        .unwrap();

        // TODO: check chainid from prompt

        let private_key =
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string();
        let provider_url = "http://localhost:8545".to_string();

        let wallet: LocalWallet = LocalWallet::from_str(&private_key).unwrap();
        let provider = Provider::<Http>::try_from(provider_url).unwrap();

        let client = SignerMiddleware::new(provider, wallet.with_chain_id(31337u32));
        let client2 = Arc::new(client.clone());
        let contract_address =
            Address::from_str("0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512").unwrap();
        let summa_contract = Summa::new(contract_address, client2);

        let ownership_data = snapshot.get_proof_of_account_ownership();
        let asset_addresses = ownership_data.get_addresses(); // No needed actually

        // // TODO: replace hard coded balances
        // let asset_sum: [Fp; 2] = [Fp::from(1140453377u64), Fp::from(1642368559u64)];

        // let (solvency_data, _) = snapshot
        //     .generate_proof_of_solvency(asset_addresses.clone(), Some(asset_sum))
        //     .unwrap();

        // // Convert data types to be compatible with the contract
        // let mock_erc_20_address =
        //     Address::from_str("0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0").unwrap();
        // let public_inputs = solvency_data.get_public_inputs();
        // let proof: &Bytes = solvency_data.get_proof_calldata();

        // let block_number = client.get_block_number().await.unwrap();
        // println!("current block number: {:?}", block_number);

        // let event = summa_contract
        //     .event::<ProofOfSolvencySubmittedFilter>()
        //     .from_block(block_number);

        // summa_contract
        //     .submit_proof_of_solvency(
        //         vec![mock_erc_20_address],
        //         public_inputs[1..].to_vec(), // first element is root hash
        //         public_inputs[0],            // maybe public_inputs[0] is roothash?
        //         proof.clone(),
        //     )
        //     .send()
        //     .await
        //     .unwrap();

        // let mut stream = event.stream().await?.with_meta().take(1);
        // while let Some(Ok((event, meta))) = stream.next().await {
        //     println!("The proof has been validated ✅");
        //     println!("        mst_root: 0x{:02x}", event.mst_root);
        //     println!("transaction hash: {:?}", meta.transaction_hash);
        // }

        Ok(())
    }
}
