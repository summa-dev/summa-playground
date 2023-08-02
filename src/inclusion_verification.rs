use std::fs::File;
use std::io::prelude::*;

use dialoguer::{Confirm, Input};
use num_bigint::BigUint;

use halo2_proofs::halo2curves::bn256::Fr as Fp;

use summa_backend::apis::snapshot::Snapshot;
use summa_solvency::{
    circuits::utils::full_verifier,
    merkle_sum_tree::{big_uint_to_fp, Entry},
};

fn generate_leaf_hash<const N_ASSETS: usize>(user_name: String, balances: Vec<usize>) -> Fp {
    // Convert usize to BigUint for the `Entry` struct
    let balances_big_int: Vec<BigUint> = balances.into_iter().map(BigUint::from).collect();

    let entry: Entry<N_ASSETS> =
        Entry::new(user_name, balances_big_int.try_into().unwrap()).unwrap();

    entry.compute_leaf().hash
}

pub fn verify_inclusion_proof(snapshot: &Snapshot<15, 6, 2, 8>) {
    // Get the path of the proof file
    let proof_file: String = Input::new()
        .with_prompt("Please input the path to the proof file")
        .with_initial_text("proof.bin")
        .interact()
        .unwrap();

    // Load and deserialize the proof
    let mut file = File::open(proof_file).unwrap();
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded).unwrap();

    let proof: Vec<u8> = bincode::deserialize(&encoded[..]).unwrap();

    println!("Initiating verification of `leaf_hash`.");

    // Ask for user details
    let root_hash_str: String = Input::new()
        .with_prompt("Please provide the `root_hash`")
        .interact()
        .unwrap();

    let root_hash_bytes =
        hex::decode(root_hash_str.strip_prefix("0x").unwrap()).expect("Decoding failed");

    let root_hash_big_int = BigUint::from_bytes_be(&root_hash_bytes);

    let root_hash = big_uint_to_fp(&root_hash_big_int);

    // Ask for user details
    let user_name: String = Input::new()
        .with_prompt("Please provide your `user_name`")
        .interact()
        .unwrap();

    let mut balances_usize = Vec::new();
    for i in 1..=2 {
        let balance: usize = Input::new()
            .with_prompt(&format!("Please provide your balance for asset#{}", i))
            .interact()
            .unwrap();
        balances_usize.push(balance);
    }

    let leaf_hash = generate_leaf_hash::<2>(user_name, balances_usize.clone());

    // Get confirmation from the user
    let _proceed = Confirm::new()
        .with_prompt(format!(
            "Your leaf hash is {:?}.\nAre you ready to proceed with the proof verification?",
            leaf_hash
        ))
        .interact()
        .unwrap();

    let (params, _, vk) = snapshot.get_trusted_setup_for_mst_inclusion();

    let verification_result: bool =
        full_verifier(params, vk, proof, vec![vec![leaf_hash, root_hash]]);

    if verification_result {
        // Perform verification
        println!("==========================");
        println!("    root hash : {:?}", root_hash);
        println!("    leaf hash : {:?}", leaf_hash);
        println!("     balances : {:?}", balances_usize);
        println!("  ");
        println!("  The proof has been validated ✅");
        println!("==========================");
    } else {
        println!("The proof is invalid ❌");
    }
}

#[cfg(test)]
mod tests {
    use super::{generate_leaf_hash, Fp};
    use halo2_proofs::halo2curves::ff::PrimeField;

    #[test]
    fn test_generate_leaf_hash() {
        let user_name = "dxGaEAii".to_string();
        let balances = vec![11888, 58946];
        let leaf_hash = generate_leaf_hash::<2>(user_name, balances);

        assert_eq!(
            leaf_hash,
            // "0x0e113acd03b98f0bab0ef6f577245d5d008cbcc19ef2dab3608aa4f37f72a407"
            // "0x0b38859334883b90f5c17cd250d166b40a6f754ed24d3b11169f70ed49780550"
            Fp::from_str_vartime(
                "5075306670952085051678079899488543418523134581904400252624929033793270449488"
            )
            .unwrap()
        );
    }
}
