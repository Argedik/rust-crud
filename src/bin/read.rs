use tokio_postgres::NoTls;
use rust_crud_project::config::Config;  

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env();
    let (client, connection) = tokio_postgres::connect(&config.database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Bağlantı hatası: {}", e);
        }
    });

    // items tablosundaki kayıtları okuyalım
    let rows = client.query("SELECT id, name, description FROM items", &[]).await?;

    if rows.is_empty() {
        println!("Hiç kayıt yok.");
    } else {
        for row in rows {
            let id: i32 = row.get("id");
            let name: String = row.get("name");
            let desc: String = row.get("description");
            println!("ID: {}, Name: {}, Description: {}", id, name, desc);
        }
    }

    Ok(())
}
