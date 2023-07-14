use dialoguer::{theme::ColorfulTheme, Select};
use ethers::providers::Middleware;
use figlet_rs::FIGfont;

mod export_proof;
use export_proof::export_inclusion_proof;
mod inclusion_verification;
use inclusion_verification::verify_inclusion_proof;
mod initialization;
use initialization::{initialize_client, initialize_snapshot};
mod solvency_proof;
use solvency_proof::generate_proof_of_solvency;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Splash with figlet
    let font = FIGfont::from_file("src/fonts/block.flf").unwrap();
    let figure = font.convert("Summa").unwrap();
    print!("{}", figure);
    println!("Proof of solvency for CryptoExchange\n\n");

    // Initialize snapshot and client
    let snapshot = initialize_snapshot();
    let client = initialize_client();

    println!(
        "Chain connected, chain id is : {}",
        client.get_chainid().await.unwrap()
    );

    loop {
        let selections = &[
            "1. Generate and submit proof of wallet ownership",
            "2. Generate and submit proof of solvency",
            "3. Generate proof of inclusion",
            "4. Verify proof of inclusion",
            "5. Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an action")
            .default(0)
            .items(selections)
            .interact()
            .unwrap();

        match selections[selection] {
            "1. Generate and submit proof of wallet ownership" => {
                let _account_ownership = snapshot.get_proof_of_account_ownership();

                // TODO: send `submitProofOfAccountOwnership` transaction
            }
            "2. Generate and submit proof of solvency" => {
                let _proof = generate_proof_of_solvency(&snapshot, &client).await;

                // TODO: send `submitProofOfSolvency` transaction
            }
            "3. Generate proof of inclusion" => {
                export_inclusion_proof(&snapshot);
            }
            "4. Verify proof of inclusion" => {
                verify_inclusion_proof(&snapshot);
            }
            "5. Exit" => break, // Exit the program
            _ => unreachable!(),
        }
    }

    Ok(())
}
