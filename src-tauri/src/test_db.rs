use rusqlite::Connection;
use std::fs;

fn main() {
    let db_path = std::path::Path::new("/home/us-dala/.local/share/com.taxorium.app/taxorium.db");
    let password = machine_uid::get().unwrap();
    println!("Password: {}", password);
    let conn = Connection::open(db_path).unwrap();
    conn.pragma_update(None, "key", &password).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM products").unwrap();
    let rows = stmt.query_map([], |row| {
        Ok(row.get::<_, String>(3).unwrap()) // name
    }).unwrap();
    for row in rows {
        println!("Product: {:?}", row.unwrap());
    }
}
