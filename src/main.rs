/*
 * Read (contractAddress, RPC_URL) from cli
 * Query All the standard storage slots for storing implementation address in proxy
 * Return True and implementation address in case a non-zero address is found is slot
 * Else False and an empty string
*/

use clap::{arg, Command};
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json::json;

const EIP_1967_LOGIC_SLOT: &str =
    "0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc";
const EIP_1967_BEACON_SLOT: &str =
    "0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50";
const OPEN_ZEPPELIN_IMPLEMENTATION_SLOT: &str =
    "0x7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c3";
const EIP_1822_LOGIC_SLOT: &str =
    "0xc5f16f0fcc639fa48a6947836d9850f504798523bf8c9a3a87d5876cf622bcf7";
const GNOSIS_SAFE_PROXY_INTERFACE: &str =
    "0xa619486e00000000000000000000000000000000000000000000000000000000";

#[derive(Serialize, Deserialize, Debug)]
struct EthGetStorageAtResponse {
    jsonrpc: String,
    id: u8,
    result: String,
}

async fn get_storage(addr: &str, slot: &str, rpc_url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let data = json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "eth_getStorageAt",
        "params": [
            addr,
            slot,
            "latest"
        ]
    });

    let response = client
        .post(rpc_url)
        .header(CONTENT_TYPE, "application/json")
        .json(&data)
        .send()
        .await?;

    let api_response: EthGetStorageAtResponse = response.json().await?;
    Ok(api_response.result)
}

async fn is_proxy(rpc_url: &str, addr: &str) -> (bool, String) {
    let slots: Vec<&str> = vec![
        EIP_1967_LOGIC_SLOT,
        EIP_1967_BEACON_SLOT,
        OPEN_ZEPPELIN_IMPLEMENTATION_SLOT,
        EIP_1822_LOGIC_SLOT,
        GNOSIS_SAFE_PROXY_INTERFACE,
    ];
    let zero_address =
        String::from("0x0000000000000000000000000000000000000000000000000000000000000000");
    let mut implementation_addr: String = String::from("0x");
    let mut is_address_proxy: bool = false;

    for slot in slots.iter() {
        let storage_value = get_storage(addr, slot, rpc_url).await.unwrap();
        if storage_value != zero_address {
            implementation_addr = storage_value;
            is_address_proxy = true;
            break;
        }
    }
    return (is_address_proxy, implementation_addr);
}

#[tokio::main]
async fn main() {
    let matches = Command::new("isProxy")
        .version("1.0")
        .author("Rishabh S.")
        .about("Analyzes an address for it's mutability")
        .arg(arg!(--rpc <VALUE>).required(true))
        .arg(arg!(--addr <VALUE>).required(true))
        .get_matches();

    let rpc_url = matches.get_one::<String>("rpc").expect("required");
    let addr = matches.get_one::<String>("addr").expect("required");
    let (is_proxy, implementation_addr) = is_proxy(rpc_url, addr).await;
    println!("{}: {}", is_proxy, implementation_addr);
}
