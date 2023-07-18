use std::{fs::File, io::Write};

use dialoguer::Input;
use serde::Serialize;

use summa_backend::apis::snapshot::Snapshot;

fn export_data<T>(data: &T, file_name: &str, description: &str)
where
    T: Serialize,
{
    let encoded: Vec<u8> = bincode::serialize(&data).unwrap();

    let mut file = File::create(file_name).unwrap();
    file.write_all(&encoded).unwrap();

    println!("Exported {} to {}", description, file_name);
}

pub fn export_inclusion_proof(snapshot: &Snapshot<15, 6, 2, 8>) {
    let user_index: u64 = Input::new()
        .with_prompt("Enter user number")
        .interact()
        .unwrap();

    let inclusion_proof = snapshot
        .generate_proof_of_inclusion(user_index as usize)
        .unwrap();

    println!("Generated inclusion proof for user #{}", user_index);

    println!(
        "the root_hash is: {:?}",
        inclusion_proof.get_public_inputs()[0][1]
    );

    let file_name: String = Input::new()
        .with_prompt("Enter proof file name to save")
        .with_initial_text("proof.bin")
        .interact()
        .unwrap();

    export_data::<Vec<u8>>(inclusion_proof.get_proof(), &file_name, "inclusion proof");
}
