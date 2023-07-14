use std::{fs::File, io::Write};

use dialoguer::{theme::ColorfulTheme, Select};
use figlet_rs::FIGfont;

mod export_proof;
use export_proof::export_inclusion_proof;
mod inclusion_verification;
use inclusion_verification::verify_inclusion_proof;
mod initialization;
use initialization::{initialize_client, initialize_snapshot};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Splash with figlet
    let font = FIGfont::from_file("src/fonts/block.flf").unwrap();
    let figure = font.convert("Summa").unwrap();
    println!("{}", figure);

    let snapshot = initialize_snapshot();
    let client = initialize_client();

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
