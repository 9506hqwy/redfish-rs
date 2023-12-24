use redfish_model::chassis::Chassis;
use redfish_model::chassis_collection::ChassisCollection;
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

    if let Some(idref) = &root.chassis {
        if let Some(uri) = &idref.odata_id {
            let json = client.get(format!("{url}{uri}")).send()?.text()?;
            let collections = serde_json::from_str::<ChassisCollection>(&json)?;

            for member in collections.members {
                if let Some(uri) = &member.odata_id {
                    let json = client.get(format!("{url}{uri}")).send()?.text()?;
                    let chassis = serde_json::from_str::<Chassis>(&json)?;

                    dbg!(&chassis);
                }
            }
        }
    }

    Ok(())
}
