#[allow(warnings)]
use soroban_client::server::Options;
use soroban_client::{server::Server, keypair::Keypair};
use soroban_client::keypair::KeypairBehavior;

#[tokio::main]
async fn main() {
    let source_secret_key = "SCCTADNI4B4FEFELEYEYSDUNQXVTXHRAOEXWWJWHJ57EO3VHGXJFL3TC";
    let source_keypair = Keypair::from_secret(source_secret_key).expect("Invalid secret key");
    let _source_public_key = source_keypair.public_key();
    let _source_public_key = "GDWH3P3MNTCMOY42CA7RVEACUUAUPZ73XDYKPYUL3TWOFRF37FD6OVM6";

    let _contract_id = "CCJYKPKPQADXVZVGNJIDIUFNBMV6FOKCFZZZMA2FCEZOMDIQA5BBPPCN";
    let _server = Server::new("https://rpc-futurenet.stellar.org:443/", Options{ allow_http: Some(true), timeout: Some(1000), headers: None });
    let health = _server.get_health().await.unwrap();
    println!("{:?}", health);
}