use std::error::Error;

use bollard::{
    query_parameters::ListContainersOptions,
    secret::{ContainerSummary, Port, PortTypeEnum},
    Docker,
};
use serde::Serialize;
#[derive(Serialize, Default)]
struct WaybarModule {
    text: String,
    tooltip: String,
    class: String,
}

impl WaybarModule {
    pub fn new(text: String, tooltip: String, class: String) -> Self {
        Self {
            text,
            tooltip,
            class,
        }
    }
}

async fn get_running_containers() -> Result<Vec<ContainerSummary>, Box<dyn Error>> {
    let docker = Docker::connect_with_socket_defaults()?;
    let options = ListContainersOptions::default();
    let containers = docker.list_containers(Some(options)).await?;
    Ok(containers)
}

fn port_to_line(port: Port) -> String {
    let ip = port.ip.unwrap_or_default();
    let public_port = port.public_port.unwrap_or_default();
    let private_port = port.private_port;
    let typ = port.typ.unwrap_or(PortTypeEnum::EMPTY);
    format!("{ip}:{public_port}->{private_port}/{typ} ")
}
fn summary_to_line(summary: ContainerSummary) -> String {
    format!(
        "  {:<20}\n  Image: {:<15}\n  Status: {:<15}\n  Cmd:  {:<15}\n  Ports: {:<25}",
        &summary.names.unwrap_or_default()[0][1..],
        summary.image.unwrap_or_default(),
        summary.status.unwrap_or_default(),
        summary.command.unwrap_or_default(),
        summary
            .ports
            .unwrap_or_default()
            .iter()
            .map(|x| port_to_line(x.clone()))
            .collect::<String>(),
    )
}

async fn build_module() -> Result<WaybarModule, Box<dyn Error>> {
    let containers = get_running_containers().await?;
    let module = WaybarModule::new(
        format!("  {}", containers.len()),
        containers
            .iter()
            .map(|x| summary_to_line(x.clone()))
            .collect::<Vec<_>>()
            .join("\n"),
        "docker".to_string(),
    );
    Ok(module)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let module = build_module().await.unwrap_or_default();
    println!("{}", serde_json::to_string(&module).unwrap())
}
