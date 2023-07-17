use std::str::FromStr;

use dialoguer::{theme::ColorfulTheme, Input};
use ethers::{
    prelude::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
};

use summa_backend::apis::snapshot::Snapshot;

pub fn initialize_snapshot() -> Snapshot<15, 6, 2, 8> {
    // reuires these files for initialize snapshot
    let mut csv_entry_path = String::new();
    let mut csv_signature_path = String::new();
    let mut ptau_path = String::new();

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
        .with_initial_text("ptau/hermez-raw-15")
        .interact_text()
        .unwrap();

    // initialize snapshot
    Snapshot::<15, 6, 2, 8>::new(
        &csv_entry_path,
        &csv_signature_path,
        "Summa proof of solvency for CryptoExchange".to_string(),
        &ptau_path,
    )
    .unwrap()
}

pub async fn initialize_client() -> SignerMiddleware<Provider<Http>, LocalWallet> {
    let mut private_key = String::new();
    let mut provider_url = String::new();

    private_key = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter private key for Signer")
        .with_initial_text("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
        .interact_text()
        .unwrap();

    provider_url = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter provider for Signer")
        .with_initial_text("http://localhost:8545")
        .interact_text()
        .unwrap();

    let wallet: LocalWallet = LocalWallet::from_str(&private_key).unwrap();
    let provider = Provider::<Http>::try_from(provider_url).unwrap();
    let chain_id = provider.get_chainid().await.unwrap();

    SignerMiddleware::new(provider, wallet.with_chain_id(chain_id.as_u32()))
}
