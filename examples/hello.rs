use lybic_sdk_rs::{types::CreateSandboxDto, Client};

const ORG_ID: &str = "lybic-sdk-rust-example";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create a client
    let client = Client::new("https://api.lybic.cn");

    // Create a sandbox
    let sandbox = client.create_sandbox(ORG_ID, &CreateSandboxDto {
        datacenter_id: None,
        max_life_seconds: Some(3600.0),
        name: ORG_ID.to_string(),
        project_id: None,
        spec_id: None,
    }).await?;

    println!("Sandbox created: {:?}", sandbox);

    // Delete the sandbox
    client.delete_sandbox(ORG_ID, &sandbox.sandbox.id).await?;

    println!("Sandbox deleted");

    Ok(())
}
