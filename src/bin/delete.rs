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

    // Örnek: ID = 1 olan kaydı sil
    let delete_id = 2;

    let rows_affected = client.execute(
        "DELETE FROM items WHERE id = $1",
        &[&delete_id]
    ).await?;

    if rows_affected == 0 {
        println!("ID {} için kayıt bulunamadı.", delete_id);
    } else {
        println!("ID {} başarıyla silindi.", delete_id);
    }

    Ok(())
}
