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
    print!("{}", figure);
    println!("Proof of solvency for CryptoExchange\n\n");

    // Initialize snapshot and client
    let snapshot = initialize_snapshot();
    let client = initialize_client();

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
                // TODO : generate proof of wallet ownership then submit it to the contract
            }
            "2. Generate and submit proof of solvency" => {
                // TODO : generate proof of solvency then submit it to the contract
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
