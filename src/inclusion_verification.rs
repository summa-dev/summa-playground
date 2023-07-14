use std::fs::File;
use std::io::prelude::*;

use bincode;
use dialoguer::{Confirm, Input};
use num_bigint::BigInt;

use halo2_proofs::halo2curves::{bn256::Fr as Fp, ff::PrimeField};

use summa_backend::apis::snapshot::Snapshot;
use summa_solvency::{circuits::utils::full_verifier, merkle_sum_tree::Entry};

fn generate_leaf_hash<const N_ASSETS: usize>(user_name: String, balances: Vec<usize>) -> Fp {
    // Convert usize to BigInt for the `Entry` struct
    let balances_big_int: Vec<BigInt> = balances
        .clone()
        .into_iter()
        .map(|balance| BigInt::from(balance))
        .collect();

    let entry: Entry<N_ASSETS> =
        Entry::new(user_name, balances_big_int.try_into().unwrap()).unwrap();

    entry.compute_leaf().hash
}

pub fn verify_inclusion_proof(snapshot: &Snapshot<4, 6, 2, 8>) {
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

    // Convert type from `root_hash_str` to Fp
    let root_hash =
        Fp::from_str_vartime(
            &BigInt::from_bytes_be(num_bigint::Sign::Plus, root_hash_str.as_bytes())
                .to_str_radix(10)[..],
        )
        .unwrap();

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

    let leaf_hash = generate_leaf_hash::<2>(user_name.clone(), balances_usize.clone());

    // Get confirmation from the user
    let proceed = Confirm::new()
        .with_prompt(format!(
            "Your leaf hash is {:?}.\nAre you ready to proceed with the proof verification?",
            leaf_hash
        ))
        .interact()
        .unwrap();

    let (params, _, vk) = snapshot.get_trusted_setup_for_mst_inclusion();

    let verification_result: bool =
        full_verifier(&params, &vk, proof, vec![vec![leaf_hash], vec![root_hash]]);

    if verification_result {
        // Perform verification
        println!("==========================");
        println!("    root_hash :  \"{}\"", root_hash_str);
        println!("    leaf_hash: \"{:?}\"", leaf_hash);
        println!("    balances : {:?}", balances_usize);
        println!("  ");
        println!("  The proof has been validated ✅");
        println!("==========================");
    } else {
        println!("The proof is invalid ❌");
    }
}

#[cfg(test)]
mod tests {
    use super::{generate_leaf_hash, Fp, PrimeField};

    #[test]
    fn test_generate_leaf_hash() {
        let user_name = "dxGaEAii".to_string();
        let balances = vec![11888, 41163];
        let leaf_hash = generate_leaf_hash::<2>(user_name, balances);

        assert_eq!(
            leaf_hash,
            // "0x0e113acd03b98f0bab0ef6f577245d5d008cbcc19ef2dab3608aa4f37f72a407"
            Fp::from_str_vartime(
                "6362822108736413915574850018842190920390136280184018644072260166743334495239"
            )
            .unwrap()
        );
    }
}
