use redfish_model::service_root::ServiceRoot;
use reqwest::blocking::ClientBuilder;
use std::env::args;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = args()
        .nth(1)
        .ok_or("Specify URL `https://127.0.0.1:443`.")?;

    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let json = client.get(format!("{url}/redfish/v1")).send()?.text()?;
    let root = serde_json::from_str::<ServiceRoot>(&json)?;

    dbg!(&root);

    Ok(())
}
