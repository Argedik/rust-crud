use serde::{Serialize, Deserialize};
use std::fs::{OpenOptions, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;
pub mod config;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
}

// JSON dosyası adı
const DB_FILE: &str = "database.json";

/// JSON dosyasını oku ve `Vec<Item>` döndür.
/// Dosya yoksa boş bir vektör döndür.
pub fn read_db() -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    if !Path::new(DB_FILE).exists() {
        return Ok(vec![]);
    }
    let file = File::open(DB_FILE)?;
    let reader = BufReader::new(file);
    let items = serde_json::from_reader(reader)?;
    Ok(items)
}

/// Mevcut `Vec<Item>` verisini JSON dosyasına yaz.
pub fn write_db(items: &Vec<Item>) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(DB_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, items)?;
    Ok(())
}
