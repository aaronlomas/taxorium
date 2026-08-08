use rusqlite::{params, Connection};
use machine_uid;

fn main() {
    let password = machine_uid::get().unwrap();
    let db_path = std::path::Path::new("/home/us-dala/.local/share/com.taxorium.app/taxorium.db");
    let conn = Connection::open(db_path).unwrap();
    conn.pragma_update(None, "key", &password).unwrap();
    
    let res = conn.execute(
        "INSERT INTO products (internal_code, unit_code, name, sunat_code, gsl_code, currency, \
         price_unit_sale, price_unit_purchase, stock_minimo, afectacion_venta, afectacion_compra, \
         has_icbper, brand, category, branch) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
        params![
            Some("asd12".to_string()), 
            "NIU".to_string(), 
            "arroz".to_string(), 
            Some("d2dsadfa".to_string()), 
            Some("df12d2scf".to_string()), 
            "PEN".to_string(), 
            5.3, 
            20.0, 
            1.0, 
            "20".to_string(), 
            "NO_GRAVADO".to_string(), 
            0, 
            None::<String>, 
            Some("Abarrotes".to_string()), 
            Some("oficina-01".to_string()), 
        ],
    );
    println!("INSERT RESULT: {:?}", res);
}
