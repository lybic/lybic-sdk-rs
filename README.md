<p align="center">
  <a href="https://lybic.ai/">
    <img src="https://avatars.githubusercontent.com/lybic" alt="Lybic Logo" width="120" height="120">
  </a>
</p>

<h1 align="center">Lybic SDK for Rust</h1>

Developing, testing, and deploying GUI-based AI agents is complex. Developers waste precious time wrestling with cloud instances, VNC servers, and environment configurations instead of focusing on what matters: building intelligent agents.

**Lybic is the infrastructure layer for your GUI agents.**

**Lybic** (/ˈlaɪbɪk/) provides a robust, on-demand infrastructure platform designed specifically for the AI agent development lifecycle. This SDK for Go is your command center for programmatically controlling the entire Lybic ecosystem, empowering you to build, test, and scale your agents with unprecedented speed and simplicity.

## Usage

```toml
[dependencies]
lybic-sdk-rs = "0.1.0"
```

example:

```rust
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
```
