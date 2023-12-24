use redfish_model::computer_system::ComputerSystem;
use redfish_model::computer_system_collection::ComputerSystemCollection;
use redfish_model::service_root::v1_16_0::ServiceRoot;
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

    if let Some(idref) = &root.systems {
        if let Some(uri) = &idref.odata_id {
            let json = client.get(format!("{url}{uri}")).send()?.text()?;
            let collections = serde_json::from_str::<ComputerSystemCollection>(&json)?;

            for member in collections.members {
                if let Some(uri) = &member.odata_id {
                    let json = client.get(format!("{url}{uri}")).send()?.text()?;
                    let system = serde_json::from_str::<ComputerSystem>(&json)?;

                    dbg!(&system);
                }
            }
        }
    }

    Ok(())
}
