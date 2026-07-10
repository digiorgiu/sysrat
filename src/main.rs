mod models;

use zbus_systemd::{systemd1::ManagerProxy, zbus::Connection};

use models::SystemdService;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let connection = Connection::system().await?;
    let proxy = ManagerProxy::new(&connection).await?;
    
    let units = proxy.list_units().await?;
    
    let services: Vec<SystemdService> = units.into_iter().filter(|unit| unit.0.ends_with(".service")).map(SystemdService::from).collect();

    for service in services {
        println!("{}", service.name);
    }

    Ok(())
}
