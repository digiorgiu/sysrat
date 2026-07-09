use zbus_systemd::{systemd1::ManagerProxy, zbus::Connection};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let connection = Connection::system().await?;
    let proxy = ManagerProxy::new(&connection).await?;
    
    let units = proxy.list_units().await?;
    
    println!("{:?}", units);
    Ok(())
}
