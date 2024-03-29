
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