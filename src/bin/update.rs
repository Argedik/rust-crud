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

    // Örnek: ID = 1 olan kaydı güncelle
    let update_id = 2;
    let new_name = "Güncellenmiş Ürün";
    let new_desc = "Yeni açıklama";

    let rows_affected = client.execute(
        "UPDATE items SET name = $1, description = $2 WHERE id = $3",
        &[&new_name, &new_desc, &update_id]
    ).await?;

    if rows_affected == 0 {
        println!("ID {} için kayıt bulunamadı.", update_id);
    } else {
        println!("ID {} için güncelleme başarılı!", update_id);
    }

    Ok(())
}
