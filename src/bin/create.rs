use tokio_postgres::NoTls;
use rust_crud_project::config::Config;  // <-- crate adın bu!

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ortam değişkenini yükle
    let config = Config::from_env();

    // PostgreSQL bağlantısı
    let (client, connection) = tokio_postgres::connect(&config.database_url, NoTls).await?;

    // Arka planda bağlantı yönetimi
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Bağlantı hatası: {}", e);
        }
    });

    // Örnek kayıt ekleyelim (CREATE)
    let name = "Deneme Ürün";
    let description = "Bu bir açıklamadır.";
    client.execute(
        "INSERT INTO items (name, description) VALUES ($1, $2)",
        &[&name, &description],
    ).await?;

    println!("✅ Kayıt eklendi!");

    Ok(())
}
