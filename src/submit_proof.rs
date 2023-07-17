use std::{str::FromStr, sync::Arc};

use ethers::{
    abi::Address,
    prelude::SignerMiddleware,
    providers::{Http, Middleware, Provider, StreamExt},
    signers::LocalWallet,
    types::{Bytes, U256},
};
use halo2_proofs::halo2curves::bn256::Fr as Fp;

use summa_backend::apis::snapshot::Snapshot;

use super::mock_erc20::MockERC20;
use super::summa_contract::summa::{
    ExchangeAddressesSubmittedFilter, ProofOfSolvencySubmittedFilter, Summa,
};

fn update_balance(mut accumulator: Fp, balance: U256) -> Fp {
    let mut u8_balance = [0u8; 32];
    balance.to_little_endian(&mut u8_balance);
    accumulator += Fp::from_bytes(&u8_balance).unwrap();
    accumulator
}

fn get_contract_instance(
    client: &SignerMiddleware<Provider<Http>, LocalWallet>,
    contract_address: &str,
) -> Summa<SignerMiddleware<Provider<Http>, LocalWallet>> {
    let arc_client = Arc::new(client.clone());
    let contract_address = Address::from_str(contract_address).unwrap();
    Summa::new(contract_address, arc_client)
}

pub async fn generate_proof_of_solvency(
    snapshot: &Snapshot<15, 6, 2, 8>,
    client: &SignerMiddleware<Provider<Http>, LocalWallet>,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: get contract address from config
    let summa_contract =
        get_contract_instance(client, "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512");
    let mock_erc_20_address =
        Address::from_str("0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0").unwrap();
    let mock_erc_20_contract = MockERC20::new(mock_erc_20_address, Arc::new(client.clone()));

    // Fetch all registered addresses from onchain
    let mut registered_addresses: Vec<Address> = Vec::new();
    let mut registered_addresses_str: Vec<String> = Vec::new();

    let mut i = 0;

    while let Ok(addr) = summa_contract.cex_addresses(U256::from(i)).await {
        registered_addresses_str.push(addr.to_string());
        registered_addresses.push(addr);
        i += 1;
    }

    if registered_addresses_str.len() == 0 {
        println!("  No account addresses found on the verifier contract,\n  Please submit 'proof of ownership' first.");
        return Ok(());
    }

    println!(
        "Found {} addresses on the verifier contract",
        registered_addresses.len()
    );

    // Fetch balances from on-chain
    let mut sum_eth_balance = Fp::zero();
    let mut sum_erc20_balance = Fp::zero();

    for address in registered_addresses {
        let eth_balance: U256 = client.get_balance(address, None).await.unwrap();
        let erc_balance = mock_erc_20_contract.balance_of(address).await.unwrap();

        sum_eth_balance = update_balance(sum_eth_balance, eth_balance);
        sum_erc20_balance = update_balance(sum_erc20_balance, erc_balance);
    }

    if sum_eth_balance >= Fp::from(u64::MAX) || sum_erc20_balance >= Fp::from(u64::MAX) {
        Err("The CLI demo does not support a total balance sum exceeding 64 bits.")?;
    }

    let asset_sum: [Fp; 2] = [sum_eth_balance, sum_erc20_balance];

    let (solvency_data, _) = snapshot
        .generate_proof_of_solvency(registered_addresses_str.clone(), Some(asset_sum))
        .unwrap();

    // Convert data types to be compatible with the contract
    let public_inputs = solvency_data.get_public_inputs();
    let proof: &Bytes = solvency_data.get_proof_calldata();

    let block_number = client.get_block_number().await.unwrap();
    let event = summa_contract
        .event::<ProofOfSolvencySubmittedFilter>()
        .from_block(block_number);

    summa_contract
        .submit_proof_of_solvency(
            vec![mock_erc_20_address],
            public_inputs[1..].to_vec(), // first element is root hash
            public_inputs[0],            // maybe public_inputs[0] is roothash?
            proof.clone(),
        )
        .send()
        .await
        .unwrap();

    let mut stream = event.stream().await?.with_meta().take(1);
    while let Some(Ok((event, meta))) = stream.next().await {
        println!("       root hash: {:#066x}", event.mst_root);
        println!("transaction hash: {:?}", meta.transaction_hash);
        println!("The proof has been validated ✅");
    }

    Ok(())
}

pub async fn generate_proof_of_ownership(
    snapshot: &Snapshot<15, 6, 2, 8>,
    client: &SignerMiddleware<Provider<Http>, LocalWallet>,
) -> Result<(), Box<dyn std::error::Error>> {
    let summa_contract =
        get_contract_instance(client, "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512");

    // Converting types to be compatible with the contract
    let ownership_data = snapshot.get_proof_of_account_ownership();
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

    let block_number = client.get_block_number().await.unwrap();
    let event = summa_contract
        .event::<ExchangeAddressesSubmittedFilter>()
        .from_block(block_number);

    summa_contract
        .submit_proof_of_account_ownership(cex_addresses, cex_signatures, message.to_string())
        .send()
        .await
        .unwrap();

    let mut stream = event.stream().await?.with_meta().take(1);
    while let Some(Ok((event, meta))) = stream.next().await {
        println!(" CEX addresses:");
        for (i, addr) in event.cex_addresses.iter().enumerate() {
            println!("  {}: {:?}", i, addr);
        }
        println!("transaction hash: {:?}", meta.transaction_hash);
        println!("The proof has been validated ✅");
    }

    Ok(())
}
