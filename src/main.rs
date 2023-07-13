use std::{fs::File, io::Write};

use dialoguer::{theme::ColorfulTheme, Input, Select};

use summa_backend::apis::snapshot;

mod export_proof;
use export_proof::export_inclusion_proof;
mod inclusion_verification;
use inclusion_verification::verify_inclusion_proof;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // reuires these files for initialize snapshot
    let mut private_key = String::new();
    let mut csv_entry_path = String::new();
    let mut csv_signature_path = String::new();
    let mut ptau_path = String::new();

    // TODO: move outside of main function until initialize snapshot or initialize signers
    private_key = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter private key for Signer")
        .interact_text()
        .unwrap();

    csv_entry_path = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter path to entry CSV file")
        .with_initial_text("csv/two_assets_entry_2_15.csv")
        .interact_text()
        .unwrap();

    csv_signature_path = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter path to signature CSV file")
        .with_initial_text("csv/signatures.csv")
        .interact_text()
        .unwrap();

    ptau_path = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter path to ptau file for params")
        .with_initial_text("ptau/hermez-raw-13")
        .interact_text()
        .unwrap();

    // initialize snapshot
    let snapshot = snapshot::Snapshot::<15, 6, 2, 8>::new(
        &csv_entry_path,
        &csv_signature_path,
        "Summa proof of solvency for CryptoExchange".to_string(),
        &ptau_path,
    )
    .unwrap();

    // TODO: generate signers for interact with the contract

    loop {
        let selections = &[
            "1. Deploy on-chain verifier",
            "2. Generate and submit proof of wallet ownership",
            "3. Generate and submit proof of solvency",
            "4. Generate proof of inclusion",
            "5. Verify proof of inclusion",
            "6. Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an action")
            .default(0)
            .items(selections)
            .interact()
            .unwrap();

        match selections[selection] {
            "1. Deploy on-chain verifier" => {
                // TODO : generate sol contract file and deploy it target chain
            }
            "2. Generate and submit proof of wallet ownership" => {
                // TODO : generate proof of wallet ownership then submit it to the contract
            }
            "3. Generate and submit proof of solvency" => {
                // TODO : generate proof of solvency then submit it to the contract
            }
            "4. Generate proof of inclusion" => {
                export_inclusion_proof(&snapshot);
            }
            "5. Verify proof of inclusion" => {
                verify_inclusion_proof(&snapshot);
            }
            "6. Exit" => break, // Exit the program
            _ => unreachable!(),
        }
    }

    Ok(())
}
