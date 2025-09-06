use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;
use sp_core::crypto::Ss58Codec;
use sp_core::sr25519::Public;
use hex;


// Include the generated interface.
mod westend_interface;
use westend_interface::api as westend;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client for the Westend network.
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://westend-rpc.polkadot.io:443").await?;

    let from = dev::alice();
    let dest_subxt: subxt::utils::AccountId32 = dev::bob().public_key().into();
    let dest_sp: sp_core::crypto::AccountId32 = dest_subxt.0.into();

    println!("Sender (Alice) address: {}", Public::from_raw(from.public_key().0).to_ss58check());
    println!("Receiver (Bob) address: {}", dest_sp.to_ss58check());

    // Build a balance transfer extrinsic.
    let balance_transfer_tx = westend::tx().balances().transfer_allow_death(subxt::utils::MultiAddress::Id(dest_subxt), 1_000_000);

    // Sign the transaction, and then get the encoded version of it to be printed.
    let signed_tx = api.tx().create_signed(&balance_transfer_tx, &from, Default::default()).await?;

    println!("Hex-encoded extrinsic: 0x{}", hex::encode(signed_tx.encoded()));

    // Submit the balance transfer extrinsic from Alice, and wait for it to be successful
    // and in a finalized block. We get back the extrinsic events if all is well.
    let events = signed_tx
        .submit_and_watch()
        .await?
        .wait_for_finalized_success()
        .await?;

    // Find a Transfer event and print it.
    let transfer_event = events.find_first::<westend::balances::events::Transfer>()?;

    if let Some(event) = transfer_event {
        println!("Balance transfer success: {event:?}");
    }

    let tx_hash = events.extrinsic_hash();
    println!("Transaction hash: {:?}", tx_hash);

    Ok(())
}
