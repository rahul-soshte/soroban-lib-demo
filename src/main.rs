use soroban_client::network::{Networks, NetworkPassphrase};
use soroban_client::server::Durability;
#[allow(warnings)]
use soroban_client::server::Options;
use soroban_client::contract_spec::native_to_sc_val;
use soroban_client::transaction::Account;
use soroban_client::transaction_builder::TransactionBuilder;
use soroban_client::{server::Server, keypair::Keypair};
use soroban_client::keypair::KeypairBehavior;
use soroban_client::transaction_builder::TransactionBuilderBehavior;
use soroban_client::account::AccountBehavior;
use stellar_baselib::{xdr, contract};
use stellar_xdr::next::{ScVec, WriteXdr, Limits, ScContractInstance};
use stellar_xdr::next::{HostFunction, ScSymbol, InvokeContractArgs, Hash, StringM, ScString, ScVal, ReadXdr, ScSpecType, ContractDataDurability};
use stellar_xdr::next::ScAddress;
use stellar_xdr::next::ScAddress::Contract;
use std::str::FromStr;
use stellar_baselib::transaction::TransactionBehavior;
use soroban_client::contract::ContractBehavior;
// Testnet -> https://soroban-testnet.stellar.org
// Futurenet -> https://rpc-futurenet.stellar.org:443



#[tokio::main]
async fn main() {

    let _server = Server::new("https://soroban-testnet.stellar.org", Options{ allow_http: None, timeout: Some(1000), headers: None });
    let source_secret_key = "SCCTADNI4B4FEFELEYEYSDUNQXVTXHRAOEXWWJWHJ57EO3VHGXJFL3TC";
    let source_keypair = Keypair::from_secret(source_secret_key).expect("Invalid secret key");
    let _source_public_key = source_keypair.public_key();
    let _source_public_key = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";

    let public_key = _source_public_key; // Replace with the actual public key
    let secret_string = source_secret_key; // Replace with the actual secret key
    let contract_id = "CCJYKPKPQADXVZVGNJIDIUFNBMV6FOKCFZZZMA2FCEZOMDIQA5BBPPCN"; // Replace with the actual contract ID

    let source_secret_key = "SCCTADNI4B4FEFELEYEYSDUNQXVTXHRAOEXWWJWHJ57EO3VHGXJFL3TC";
    let source_keypair = Keypair::from_secret(source_secret_key).expect("Invalid secret key");
    let _source_public_key = source_keypair.public_key();
    let _source_public_key = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";

    let account = _server.get_account(public_key).await.unwrap();
    let fee = 100;

    let contract = contract::Contracts::new(contract_id).unwrap();
}

// #[tokio::main]
// async fn main() {
//     let source_secret_key = "SCCTADNI4B4FEFELEYEYSDUNQXVTXHRAOEXWWJWHJ57EO3VHGXJFL3TC";
//     let source_keypair = Keypair::from_secret(source_secret_key).expect("Invalid secret key");
//     let _source_public_key = source_keypair.public_key();
//     let _source_public_key = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";

//     // Check Health of Node
//     let _contract_id = "CCJYKPKPQADXVZVGNJIDIUFNBMV6FOKCFZZZMA2FCEZOMDIQA5BBPPCN";
//     let _server = Server::new("https://soroban-testnet.stellar.org", Options{ allow_http: None, timeout: Some(1000), headers: None });
//     let health = _server.get_health().await.unwrap();
//     println!("{:?}", health);

//     // // Generate a key
//     // let seed = "hunterisworkingsodndokayalrightf";
//     // let keypair = Keypair::from_raw_ed25519_seed(seed.as_bytes()).unwrap();
//     // println!("{:?}", keypair.public_key());

//     // // Get the network
//     // let network = _server.get_network().await.unwrap();
//     // println!("{:?}", network );

//     // // Get the Transaction
//     // let tx = _server.get_transaction("4385bd2342151cf8cb914702c9b52e32f0c2403fec86006ef4241a857f6a993f").await;
//     // println!("{:?}", tx);

//     // // Get Account
//     // let account = _server.get_account(&_source_public_key).await.unwrap();
//     // println!("Account {:?}", account);
        
//     // Prepare Transaction
//     let contract = _server.get_contract_data("CCFEPVQM4UJ2NT7YBRMU5U7EV3JOOF47WQD7ZUZT5AM625F27OZJH5LQ", ScVal::LedgerKeyContractInstance, Durability::Persistent).await;
//     println!("{:?}", contract);
// }