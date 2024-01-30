use soroban_client::server::Options;
use soroban_client::{server::Server, keypair::Keypair};
use stellar_baselib::network::Networks;
use stellar_xdr::next::{InvokeHostFunctionOp, VecM, SorobanAuthorizationEntry, InvokeContractArgs, Hash, ScSymbol, StringM};
use std::str::FromStr;
use std::time::Duration;
use std::thread::sleep;
use stellar_strkey::{Strkey, Contract};
use soroban_client::keypair::KeypairBehavior;

#[tokio::main]
async fn main() {
//     // The source account is the account we will be signing and sending from.
// Public Key -> GBDAMUQPKPMOHGEFZNE4NHGT5NGXT33VRO6L25YTKYM64KHAXXXGBQ34
let source_secret_key = "SBH7ZY57FDWA7QNMHSFMVSSKIRE5KTOK77QW6HCMKWY2PR3V662SZGN7";

//     // Derive Keypair object and public key (that starts with a G) from the secret
let source_keypair = Keypair::from_secret(source_secret_key).expect("Invalid secret key");
let source_public_key = source_keypair.public_key();
let receiver_public_key = "GAIRISXKPLOWZBMFRPU5XRGUUX3VMA3ZEWKBM5MSNRU3CHV6P4PYZ74D";
let contract_id = "CDA254AZTEXRTOXLQPAD2PCYJ3L5IE32ZLPUZWM2JBAIGG5WASVQJCBP";
let server = Server::new("https://rpc-futurenet.stellar.org:443", Options{ allow_http: Some(true), timeout: Some(1000), headers: None });

//     // Transactions require a valid sequence number that is specific to this account.
//     // We can fetch the current sequence number for the source account.
let account = server.get_account(&source_public_key).await.unwrap();
// let fee = 100;

// let contract = Contracts::new(contract_id);

// let mut transaction2 = TransactionBuilder::new(account, "Test SDF Future Network ; October 2022")
//     .add_operation(contract.call("hello"))
//     .build();

// let transaction3 = server.prepare_transaction(transaction2, Some("Test SDF Future Network ; October 2022")).await.unwrap();
// println!("{:?}", transaction3);
//     transaction.sign(&source_keypair);

//     // Let's see the XDR (encoded in base64) of the transaction we just built
//     println!("XDR: {}", transaction.to_envelope().to_xdr_base64().expect("Failed to convert to XDR"));

//     match server.send_transaction(transaction) {
//         Ok(mut response) => {
//             println!("Sent! Transaction ID: {}", response.id);

//             while response.status == "pending" {
//                 response = server.get_transaction_status(&response.id).expect("Failed to get transaction status");
//                 sleep(Duration::from_secs(1));
//             }

//             println!("Transaction status: {}", response.status);
//             println!("{:?}", response);
//         },
//         Err(e) => {
//             eprintln!("An error has occured: {:?}", e);
//         }
//     }
}