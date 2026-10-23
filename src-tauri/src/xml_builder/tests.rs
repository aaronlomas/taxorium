use super::*;

fn payload_prueba() -> FacturaPayload {
    FacturaPayload {
        hash_xml: String::new(),
        firma: String::new(),
        certificado: String::new(),
        serie: "F001".to_string(),
        correlativo: "00000001".to_string(),
        establecimiento: "0000".to_string(),
        fecha_emision: "2023-10-24".to_string(),
        hora_emision: "15:30:00".to_string(),
        forma_pago: "Contado".to_string(),
        fecha_vencimiento: None,
        monto_neto_pendiente: None,
        tipo_comprobante: "01".to_string(),
        tipo_operacion: "0101".to_string(),
        moneda: "PEN".to_string(),
        tipo_cambio: None,
        emisor_ruc: "20123456789".to_string(),
        emisor_razon_social: "EMPRESA DE PRUEBA SAC".to_string(),
        emisor_ubigeo: "150101".to_string(),
        emisor_direccion: "AV LIMA 123".to_string(),
        cliente_tipo_doc: "6".to_string(),
        cliente_num_doc: "20987654321".to_string(),
        cliente_nombre: "CLIENTE DE PRUEBA SRL".to_string(),
        total_gravado: 100.0.into(),
        total_igv: 18.0.into(),
        total_pagar: 118.0.into(),
        items: vec![FacturaItem {
            unidad: "NIU".to_string(),
            cantidad: 1.0.into(),
            valor_unitario: 100.0.into(),
            precio_unitario: 118.0.into(),
            subtotal: 100.0.into(),
            igv: 18.0.into(),
            icbper: 0.0.into(),
            afectacion: "10".to_string(),
            descripcion: "PRODUCTO X".to_string(),
        }],
    }
}

/// El `DigestValue` tiene que ser reproducible por cualquier validador XMLDSIG
/// (SUNAT recalcula el digest por su cuenta). Los valores esperados se generaron
/// con `lxml.etree.tostring(method="c14n", exclusive=False, with_comments=False)`
/// sobre el mismo documento de prueba.
const EXPECTED_DIGEST: &str = "Vhs4b0ATN8Qngtnf9d4f4hipHNtQwEqB1++/WY/OJvo=";
const EXPECTED_SIGNED_INFO_DIGEST: &str = "499gt0RH7my+/QYmp+TR7ECuIOQbH7Zlpg1m0VoloSQ=";

#[test]
fn el_xml_declara_el_local_anexo_del_emisor() {
    for tipo in ["01", "03"] {
        let mut payload = payload_prueba();
        payload.tipo_comprobante = tipo.to_string();
        payload.establecimiento = "0001".to_string();

        let xml = generate_xml(&payload).unwrap();

        let supplier = xml
            .split("<cac:AccountingSupplierParty>")
            .nth(1)
            .and_then(|rest| rest.split("</cac:AccountingSupplierParty>").next())
            .unwrap();

        // SUNAT 3030 busca el local anexo en PartyLegalEntity, no en PartyTaxScheme.
        let legal_entity = supplier
            .split("<cac:PartyLegalEntity>")
            .nth(1)
            .and_then(|rest| rest.split("</cac:PartyLegalEntity>").next())
            .unwrap();
        assert!(
            legal_entity.contains("<cbc:AddressTypeCode>0001</cbc:AddressTypeCode>"),
            "{xml}"
        );

        let party_tax_scheme = supplier
            .split("<cac:PartyTaxScheme>")
            .nth(1)
            .and_then(|rest| rest.split("</cac:PartyTaxScheme>").next())
            .unwrap();
        assert!(
            !party_tax_scheme.contains("AddressTypeCode"),
            "el local anexo no debe ir en PartyTaxScheme: {xml}"
        );

        // Orden UBL 2.1 dentro de RegistrationAddress: ID, AddressTypeCode, AddressLine.
        let address = legal_entity.find("<cac:RegistrationAddress>").unwrap();
        let ubigeo = legal_entity.find("<cbc:ID ").unwrap();
        let local = legal_entity.find("<cbc:AddressTypeCode>").unwrap();
        let line = legal_entity.find("<cac:AddressLine>").unwrap();
        assert!(address < ubigeo && ubigeo < local && local < line, "{xml}");
    }
}

/// SUNAT exige el bloque simbólico `cac:Signature` (referencia a la firma) además
/// del `ds:Signature` real; sin él responde 3244. El `cbc:ProfileID` con el tipo
/// de operación evita el 3205.
#[test]
fn el_xml_declara_la_firma_simbolica_y_el_tipo_de_operacion() {
    for tipo in ["01", "03"] {
        let mut payload = payload_prueba();
        payload.tipo_comprobante = tipo.to_string();

        let xml = generate_xml(&payload).unwrap();

        assert!(xml.contains("<cac:Signature>"), "{xml}");
        assert!(
            xml.contains("<cbc:ID>20123456789</cbc:ID>"),
            "el ID de la firma debe ser el RUC del emisor: {xml}"
        );
        assert!(
            xml.contains("<cbc:URI>#SignTaxorium</cbc:URI>"),
            "la firma simbólica debe apuntar al Id del ds:Signature: {xml}"
        );
        assert!(
            xml.contains(
                "<cbc:ProfileID schemeURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo51\" \
                 schemeName=\"Tipo de Operación\">0101</cbc:ProfileID>"
            ),
            "{xml}"
        );
    }
}

/// Documento sin `ds:Signature`: es el que se canoniza para el digest de referencia
/// y el que `create_voucher` usa para calcular el hash (el `{% if firma %}` de la
/// plantilla deja `ExtensionContent` vacío cuando aún no hay firma).
fn payload_sin_firma() -> FacturaPayload {
    let mut payload = payload_prueba();
    payload.hash_xml = EXPECTED_DIGEST.to_string();
    payload.firma = String::new();
    payload.certificado = String::new();
    payload
}

fn payload_firmado() -> FacturaPayload {
    let mut payload = payload_sin_firma();
    payload.firma = "FIRMA".to_string();
    payload.certificado = "Q0VSVA==".to_string();
    payload
}

#[test]
fn digest_coincide_con_una_implementacion_de_referencia() {
    let sin_firma = generate_xml(&payload_sin_firma()).unwrap();
    let digest = hash_xml_sha256(&canonicalize_xml(&sin_firma).unwrap());

    assert_eq!(digest, EXPECTED_DIGEST);

    // El mismo digest tiene que salir de restarle la firma al XML firmado, que es
    // lo que valida `create_voucher` antes de guardar el comprobante.
    let firmado = generate_xml(&payload_firmado()).unwrap();
    let digest_tras_firmar =
        hash_xml_sha256(&canonicalize_xml(&remove_signature(&firmado).unwrap()).unwrap());
    assert_eq!(digest_tras_firmar, EXPECTED_DIGEST);
}

/// Lo mismo para el pre-imagen que se firma: si difiere, SUNAT rechaza la
/// firma aunque el `DigestValue` sea correcto.
#[test]
fn pre_imagen_de_la_firma_coincide_con_una_implementacion_de_referencia() {
    let firmado = generate_xml(&payload_firmado()).unwrap();
    let digest = hash_xml_sha256(&canonicalize_signed_info(&firmado).unwrap());
    assert_eq!(digest, EXPECTED_SIGNED_INFO_DIGEST);
}

#[test]
fn quitar_la_firma_no_altera_el_resto_del_documento() {
    let firmado = generate_xml(&payload_firmado()).unwrap();
    let sin_firma = remove_signature(&firmado).unwrap();
    assert!(!sin_firma.contains("ds:Signature"));
    assert!(sin_firma.contains("F001-00000001"));
    assert_eq!(
        sin_firma.matches("<ext:UBLExtensions>").count(),
        firmado.matches("<ext:UBLExtensions>").count()
    );
}

/// C14N inclusivo: `ds:SignedInfo` debe declarar los espacios de nombres
/// heredados de `<Invoice>`, si no la firma no valida en SUNAT.
#[test]
fn signed_info_arrastra_los_namespaces_heredados() {
    let firmado = generate_xml(&payload_firmado()).unwrap();
    let canonico = canonicalize_signed_info(&firmado).unwrap();

    assert!(canonico.starts_with("<ds:SignedInfo "));
    assert!(canonico.contains("xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\""));
    assert!(canonico.contains("xmlns=\"urn:oasis:names:specification:ubl:schema:xsd:Invoice-2\""));
    assert!(canonico.contains(
        "xmlns:cbc=\"urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2\""
    ));
    assert!(canonico.ends_with("</ds:SignedInfo>"));
}

#[test]
fn generate_xml_incluye_la_razon_social_y_el_id() {
    let raw_xml = generate_xml(&payload_prueba()).unwrap();
    assert!(raw_xml.contains("F001-00000001"));
    assert!(raw_xml.contains("EMPRESA DE PRUEBA SAC"));
}

/// Los importes se tipan como `xs:decimal`. Renderizar un `f64` con `Display`
/// produce `21.239999999999997`, que no es un decimal válido y hace que SUNAT
/// rechace el documento completo.
#[test]
fn los_importes_se_emiten_con_dos_decimales() {
    let mut payload = payload_prueba();
    // Valores típicos de la aritmética de coma flotante del frontend.
    payload.total_gravado = (10.30f64 * 1.5).into();
    payload.total_igv = (2.727).into();
    payload.total_pagar = 0.1f64.into();
    payload.items[0].cantidad = 1.5.into();
    payload.items[0].subtotal = (10.30f64 * 1.5).into();
    payload.items[0].igv = 2.727.into();

    let xml = generate_xml(&payload).unwrap();

    assert_eq!(valor_de(&xml, "cbc:PayableAmount"), "0.10");
    assert_eq!(valor_de(&xml, "cbc:LineExtensionAmount"), "15.45");
    assert_eq!(valor_de(&xml, "cbc:InvoicedQuantity"), "1.50");
    assert_eq!(valor_de(&xml, "cbc:TaxAmount"), "2.73");

    // Ningún importe puede llevar notación exponente ni residuo de coma flotante.
    assert!(!xml.contains("999999"), "hay un residuo binario en el XML");
    assert!(
        !xml.contains("e0") && !xml.contains("E0"),
        "notación exponente en el XML"
    );
}

/// `cbc:IssueDate` es `xs:date` y `cbc:IssueTime` es `xs:time`. El frontend
/// entrega la fecha como `DD/MM/YYYY HH:MM`, así que tiene que normalizarse antes
/// de llegar a la plantilla; si no, SUNAT responde
/// `IssueDate value '27/09/2026 00:00' is not a valid instance of type ...date`.
#[test]
fn issue_date_y_issue_time_cumplen_el_xsd() {
    use crate::validaciones::parse_fecha_emision_iso;
    use chrono::Local;

    // Exactamente lo que arma `VoucherModal.svelte` a partir del calendario.
    let fecha_ui = Local::now().format("%d/%m/%Y").to_string();
    let mut payload = payload_prueba();
    payload.fecha_emision = parse_fecha_emision_iso(&format!("{fecha_ui} 00:00")).unwrap();
    payload.hora_emision = Local::now().format("%H:%M:%S").to_string();

    let xml = generate_xml(&payload).unwrap();

    let issue_date = valor_de(&xml, "cbc:IssueDate");
    assert!(
        chrono::NaiveDate::parse_from_str(issue_date, "%Y-%m-%d").is_ok(),
        "cbc:IssueDate '{issue_date}' no es un xs:date válido"
    );

    let issue_time = valor_de(&xml, "cbc:IssueTime");
    assert!(
        chrono::NaiveTime::parse_from_str(issue_time, "%H:%M:%S").is_ok(),
        "cbc:IssueTime '{issue_time}' no es un xs:time válido"
    );
}

/// El frontend manda los importes como JSON crudo, así que llegan como enteros
/// cuando no tienen parte decimal (`"cantidad": 8`). `serde_json` enruta eso a
/// `visit_u64`, y si el visitor no lo contemplaba la creación del comprobante
/// moría con `invalid type: integer 8, expected un número o una cadena numérica`.
#[test]
fn monto_acepta_todas_las_formas_numericas_de_json() {
    assert_eq!(serde_json::from_str::<Monto>("8").unwrap().0, 8.0);
    assert_eq!(serde_json::from_str::<Monto>("0").unwrap().0, 0.0);
    assert_eq!(serde_json::from_str::<Monto>("-3").unwrap().0, -3.0);
    assert_eq!(serde_json::from_str::<Monto>("1.5").unwrap().0, 1.5);
    assert_eq!(serde_json::from_str::<Monto>("-0.25").unwrap().0, -0.25);
    assert_eq!(serde_json::from_str::<Monto>("\"8.75\"").unwrap().0, 8.75);
    assert_eq!(serde_json::from_str::<Monto>("\" 12 \"").unwrap().0, 12.0);
    assert_eq!(serde_json::from_str::<Monto>("1e3").unwrap().0, 1000.0);

    // Un precio sin decimales llega como entero, no como cadena.
    assert!(serde_json::from_str::<Monto>("\"abc\"").is_err());
    assert!(serde_json::from_str::<Monto>("true").is_err());
    assert!(serde_json::from_str::<Monto>("null").is_err());
}

/// El payload real de un ítem tal como lo manda `VoucherModal.svelte`.
#[test]
fn deserialize_de_una_factura_con_importes_enteros() {
    let json = r#"{
            "unidad": "NIU", "cantidad": 8, "valor_unitario": 8,
            "precio_unitario": 9.44, "subtotal": 8, "igv": 1.44,
            "afectacion": "10", "descripcion": "PRODUCTO"
        }"#;

    let item: FacturaItem = serde_json::from_str(json).unwrap();
    assert_eq!(item.cantidad.0, 8.0);
    assert_eq!(item.subtotal.0, 8.0);
    // Y al reemitir, el XML lleva los 2 decimales que exige el XSD.
    assert_eq!(item.cantidad.to_string(), "8.00");
    assert_eq!(item.subtotal.to_string(), "8.00");
}

/// Ítem de prueba tal como lo envía el frontend: precio con IGV incluido.
fn item_venta(afectacion: &str, cantidad: f64, unitario: f64) -> ItemVenta {
    ItemVenta {
        unidad: "NIU".to_string(),
        cantidad: cantidad.into(),
        precio_unitario: unitario.into(),
        total: (cantidad * unitario).into(),
        afectacion: afectacion.to_string(),
        descripcion: format!("ITEM {afectacion}"),
        tiene_icbper: false,
    }
}

/// Ítem ya normalizado por el backend, como llega a la plantilla.
fn item(afectacion: &str, cantidad: f64, unitario: f64) -> FacturaItem {
    item_venta(afectacion, cantidad, unitario)
        .into_item(0.5)
        .unwrap()
}

/// Ítem de bolsa plástica sujeto al ICBPER. Se usa una afectación exonerada
/// para que el ICBPER quede claramente separado del IGV y de la base.
fn item_bolsa(cantidad: f64, unitario: f64) -> ItemVenta {
    ItemVenta {
        tiene_icbper: true,
        ..item_venta("20", cantidad, unitario)
    }
}

fn payload_con_items(items: Vec<FacturaItem>) -> FacturaPayload {
    let mut payload = payload_prueba();
    let totales = totales_de(&items);
    payload.total_gravado = totales.base_total.into();
    payload.total_igv = totales.igv.into();
    payload.total_pagar = totales.total_pagar.into();
    payload.items = items;
    payload
}

fn payload_con_items_venta(items: Vec<ItemVenta>) -> FacturaPayload {
    let normalizados: Vec<FacturaItem> = items
        .into_iter()
        .map(|i| i.into_item(0.5).unwrap())
        .collect();
    payload_con_items(normalizados)
}

/// Devuelve el texto de todos los `cac:TaxSubtotal` de un elemento, para poder
/// comprobar bloque por bloque lo que se declara.
fn subtotales_de(xml: &str) -> Vec<String> {
    let mut bloques = Vec::new();
    let mut resto = xml;
    while let Some(inicio) = resto.find("<cac:TaxSubtotal>") {
        let desde = inicio + "<cac:TaxSubtotal>".len();
        let fin = resto[desde..]
            .find("</cac:TaxSubtotal>")
            .expect("TaxSubtotal sin cerrar")
            + desde;
        bloques.push(resto[desde..fin].to_string());
        resto = &resto[fin..];
    }
    bloques
}

/// Un ítem gravado siempre lleva IGV positivo, y un exonerado declara 0% en vez
/// de mentir con un 18% que no existe.
#[test]
fn el_tributo_se_deriva_de_la_afectacion() {
    assert_eq!(
        tributo_de("10"),
        Tributo {
            porcentaje: "18.00",
            esquema_id: "1000",
            esquema_nombre: "IGV",
            tipo_impuesto: "VAT",
            categoria_id: "S",
        }
    );
    assert_eq!(tributo_de("20").porcentaje, "0.00");
    // Exonerado usa esquema 9997, NO 1000 (observación 3111 de SUNAT).
    assert_eq!(tributo_de("20").esquema_id, "9997");
    assert_eq!(tributo_de("20").categoria_id, "E");
    // Inafecto usa esquema 9998.
    assert_eq!(tributo_de("30").esquema_id, "9998");
    assert_eq!(tributo_de("30").categoria_id, "O");
    assert_eq!(tributo_de("31").porcentaje, "0.00");
    assert_eq!(tributo_de("40").porcentaje, "0.00");
    // ICBPer va con el esquema 7152, no con IGV.
    assert_eq!(tributo_de("71").esquema_id, "7152");
    assert_eq!(tributo_de("72").esquema_id, "7152");
    // Un código desconocido se trata como gravado, que es lo que no rechaza SUNAT.
    assert_eq!(tributo_de("99").porcentaje, "18.00");
}

/// Regresión de la observación 3111: la plantilla fijaba 18% de IGV para todas
/// las líneas, y un ítem exonerado quedaba con monto 0.00 y Tributo 1000.
#[test]
fn un_item_exonerado_no_declara_igv_al_18() {
    let payload = payload_con_items(vec![item("20", 1.0, 100.0)]);
    let xml = generate_xml(&payload).unwrap();

    for bloque in subtotales_de(&xml) {
        assert!(
            !bloque.contains("<cbc:Percent>18.00</cbc:Percent>"),
            "{bloque}"
        );
        assert!(
            bloque.contains("<cbc:Percent>0.00</cbc:Percent>"),
            "{bloque}"
        );
        assert!(
            bloque.contains(">20</cbc:TaxExemptionReasonCode>"),
            "{bloque}"
        );
    }
    assert!(xml.contains("<cbc:TaxAmount currencyID=\"PEN\">0.00</cbc:TaxAmount>"));
}

/// Un ítem gravado debe llevar un IGV distinto de cero (observación 3111).
#[test]
fn un_item_gravado_lleva_igv_distinto_de_cero() {
    let payload = payload_con_items(vec![item("10", 1.0, 118.0)]);
    let xml = generate_xml(&payload).unwrap();

    for bloque in subtotales_de(&xml) {
        assert!(
            bloque.contains("<cbc:Percent>18.00</cbc:Percent>"),
            "{bloque}"
        );
        assert!(!bloque.contains(">0.00</cbc:TaxAmount>"), "{bloque}");
    }
    assert!(xml.contains("<cbc:TaxAmount currencyID=\"PEN\">18.00</cbc:TaxAmount>"));
}

/// Regresión del reporte de SUNAT: `Error en la linea: 1 Tributo: 1000: 3111
/// (nodo: "cac:TaxSubtotal/cbc:TaxAmount" valor: "0.00")`.
///
/// El frontend decidía el IGV con `afectacion === '10'` y mandaba 0 para todo lo
/// demás, mientras el backend declaraba IGV al 18% para toda la familia `10`-`17`.
/// Con la afectación `11` (Gravado - Retiro) la línea salía con Tributo 1000 y
/// monto 0.00. Ahora el reparto sale del tributo, así que da igual el código.
#[test]
fn una_afectacion_gravada_distinta_de_10_tambien_lleva_igv() {
    for afectacion in ["10", "11", "12", "13", "14", "15", "16", "17"] {
        let payload = payload_con_items_venta(vec![item_venta(afectacion, 2.0, 118.0)]);
        let item = &payload.items[0];
        assert_eq!(item.subtotal.0, 200.0, "afectación {afectacion}");
        assert_eq!(item.igv.0, 36.0, "afectación {afectacion}");

        let xml = generate_xml(&payload).unwrap();
        for bloque in subtotales_de(&xml) {
            assert!(
                !bloque.contains(">0.00</cbc:TaxAmount>"),
                "afectación {afectacion}: {bloque}"
            );
            assert!(
                bloque.contains(
                    "<cbc:ID schemeID=\"UN/ECE 5153\" schemeAgencyID=\"6\">1000</cbc:ID>"
                ),
                "afectación {afectacion}: {bloque}"
            );
        }
    }
}

/// El reparto base/IGV es responsabilidad del backend, no del cliente.
#[test]
fn el_importe_de_la_linea_se_reparte_segun_su_tributo() {
    // 118.00 con IGV incluido -> base 100.00 + IGV 18.00
    let gravado = item("10", 1.0, 118.0);
    assert_eq!(gravado.subtotal.0, 100.0);
    assert_eq!(gravado.igv.0, 18.0);
    assert_eq!(gravado.valor_unitario.0, 100.0);

    // Exonerado, inafecto y el resto de gravados no pagan IGV.
    for afectacion in ["20", "21", "30", "31", "40"] {
        let item = item(afectacion, 1.0, 118.0);
        assert_eq!(item.subtotal.0, 118.0, "afectación {afectacion}");
        assert_eq!(item.igv.0, 0.0, "afectación {afectacion}");
        assert_eq!(item.valor_unitario.0, 118.0, "afectación {afectacion}");
    }
}

/// Base e IGV se redondean de forma que la línea siempre vuelve a sumar el
/// importe que capturó el usuario, aunque el reparto no sea exacto.
#[test]
fn la_linia_siempre_devuelve_el_importe_capturado() {
    for (cantidad, unitario) in [(1.0, 10.30), (3.0, 10.30), (7.0, 0.85), (2.0, 33.33)] {
        let venta = item_venta("10", cantidad, unitario);
        let total = venta.total.0;
        let item = venta.into_item(0.5).unwrap();
        assert_eq!(
            redondear(item.subtotal.0 + item.igv.0),
            redondear(total),
            "{cantidad} x {unitario}"
        );
    }
}

/// Una afectación vacía se reporta con el detalle del ítem en vez de emitir un
/// `cbc:TaxExemptionReasonCode` en blanco que SUNAT no valida.
#[test]
fn una_afectacion_vacia_o_invalida_se_rechaza() {
    for afectacion in ["", "  ", "1", "ABC", "100"] {
        let error = item_venta(afectacion, 1.0, 118.0)
            .into_item(0.5)
            .expect_err("debería rechazar la afectación");
        assert!(
            error.contains("ITEM") && error.contains("afectación del IGV válida"),
            "{afectacion}: {error}"
        );
    }
    assert!(es_afectacion_valida("10"));
    assert!(es_afectacion_valida(" 20 "));
    assert!(!es_afectacion_valida(""));
}

/// Una línea de importe cero tampoco es declarable.
#[test]
fn una_linea_en_cero_se_rechaza() {
    let error = item_venta("10", 0.0, 0.0)
        .into_item(0.5)
        .expect_err("debería rechazar el importe en cero");
    assert!(error.contains("0.00"), "{error}");
}

/// Última red antes de gastar un ticket en SUNAT: si una línea llega ya
/// normalizada pero con Tributo 1000 y monto 0.00, se corta con el detalle.
#[test]
fn una_linea_gravada_sin_igv_no_llega_al_xml() {
    let mut roto = item("10", 1.0, 118.0);
    roto.igv = 0.0.into();
    let payload = payload_con_items(vec![roto]);

    let error = generate_xml(&payload).expect_err("debería cortar antes de firmar");
    assert!(error.contains("3111"), "{error}");
    assert!(error.contains("Línea 1"), "{error}");
}

/// Los totales del comprobante salen de las líneas, no de lo que declare el
/// cliente: `LineExtensionAmount` es la suma de las bases de todas ellas.
#[test]
fn los_totales_suman_las_lineas() {
    let items = vec![
        item("10", 2.0, 118.0), // base 200.00, igv 36.00
        item("20", 1.0, 50.0),  // base  50.00, igv  0.00
        item("10", 1.0, 59.0),  // base  50.00, igv  9.00
    ];
    let totales = totales_de(&items);

    assert_eq!(totales.base_gravada, 250.0);
    assert_eq!(totales.base_no_gravada, 50.0);
    assert_eq!(totales.base_total, 300.0);
    assert_eq!(totales.igv, 45.0);
    assert_eq!(totales.total_pagar, 345.0);

    // Y el documento firmado lleva esos mismos importes.
    let payload = payload_con_items(items);
    let xml = generate_xml(&payload).unwrap();
    assert!(xml
        .contains("<cbc:LineExtensionAmount currencyID=\"PEN\">300.00</cbc:LineExtensionAmount>"));
    assert!(xml.contains("<cbc:PayableAmount currencyID=\"PEN\">345.00</cbc:PayableAmount>"));
}

/// Una venta mixta necesita un `cac:TaxSubtotal` por grupo, no uno solo.
#[test]
fn una_venta_mixta_agrupa_los_subtotales_por_tributo() {
    let payload = payload_con_items(vec![
        item("10", 2.0, 118.0), // gravado  -> base 200.00, igv 36.00
        item("20", 1.0, 50.0),  // exonerado-> base  50.00, igv  0.00
        item("10", 1.0, 59.0),  // gravado  -> base  50.00, igv  9.00
    ]);
    let xml = generate_xml(&payload).unwrap();

    let bloques = subtotales_de(&xml);
    // 2 en la cabecera (por grupo) + 1 por cada una de las 3 líneas.
    assert_eq!(bloques.len(), 5, "{}", xml);

    let cabecera = &bloques[0..2];
    let mut bases: Vec<f64> = Vec::new();
    let mut impuestos: Vec<f64> = Vec::new();
    for bloque in cabecera {
        bases.push(
            valor_de(bloque, "cbc:TaxableAmount")
                .parse::<f64>()
                .unwrap(),
        );
        impuestos.push(valor_de(bloque, "cbc:TaxAmount").parse::<f64>().unwrap());
    }

    // Los gravados se suman en un solo subtotal.
    let mut suma = 0.0;
    for (base, impuesto) in bases.iter().zip(impuestos.iter()) {
        if *impuesto > 0.0 {
            assert!((base - impuesto / 0.18).abs() < 0.01);
            suma += *impuesto;
        }
    }
    assert!((suma - 45.00).abs() < 0.001, "IGV total {suma}");
    assert_eq!(bases.iter().sum::<f64>().round(), 300.0);

    // El total de impuestos de la cabecera cuadra con la suma de los grupos.
    let cabecera_tax = valor_de(&xml, "cac:TaxTotal").trim_start();
    assert!(
        cabecera_tax.starts_with("<cbc:TaxAmount currencyID=\"PEN\">45.00</cbc:TaxAmount>"),
        "{cabecera_tax}"
    );
}

/// ICBPER: impuesto fijo por bolsa (S/ 0.50), aparte del IGV y de la base.
#[test]
fn el_icbper_se_suma_como_impuesto_fijo_por_bolsa() {
    let bolsa = item_bolsa(3.0, 1.0).into_item(0.5).unwrap();
    assert_eq!(bolsa.subtotal.0, 3.0);
    assert_eq!(bolsa.igv.0, 0.0);
    assert_eq!(bolsa.icbper.0, 1.5);

    let payload = payload_con_items_venta(vec![item_bolsa(3.0, 1.0)]);
    let totales = totales_de(&payload.items);
    assert_eq!(totales.icbper, 1.5);
    assert_eq!(totales.total_pagar, 4.5);

    let xml = generate_xml(&payload).unwrap();

    // El total de impuestos incluye el ICBPER de la bolsa.
    assert!(xml.contains("<cbc:TaxAmount currencyID=\"PEN\">1.50</cbc:TaxAmount>"));

    // Cabecera + línea declaran el tributo con el esquema 7152 (ICBPER / OTH).
    let con_icbper: Vec<_> = subtotales_de(&xml)
        .into_iter()
        .filter(|b| b.contains(">7152</cbc:ID>"))
        .collect();
    assert_eq!(con_icbper.len(), 2, "{xml}");
    for bloque in &con_icbper {
        assert!(bloque.contains("<cbc:Name>ICBPER</cbc:Name>"), "{bloque}");
        assert!(
            bloque.contains("<cbc:TaxTypeCode>OTH</cbc:TaxTypeCode>"),
            "{bloque}"
        );
    }

    // El subtotal de la línea declara las bolsas y la tarifa unitaria.
    let linea = con_icbper
        .iter()
        .find(|b| b.contains("BaseUnitMeasure"))
        .expect("la línea debe declarar BaseUnitMeasure");
    assert!(
        linea.contains("<cbc:BaseUnitMeasure unitCode=\"NIU\">3</cbc:BaseUnitMeasure>"),
        "{linea}"
    );
    assert!(
        linea.contains("<cbc:PerUnitAmount currencyID=\"PEN\">0.50</cbc:PerUnitAmount>"),
        "{linea}"
    );

    // El total a pagar incluye la bolsa y su impuesto.
    assert!(xml.contains("<cbc:PayableAmount currencyID=\"PEN\">4.50</cbc:PayableAmount>"));
}

/// El total del documento debe ser la suma de las líneas, sin residuo de coma
/// flotante aunque las bases se sumen en cascada.
#[test]
fn los_subtotales_agrupados_no_arrastran_residuo_binario() {
    let payload = payload_con_items((0..6).map(|_| item("10", 3.0, 10.30)).collect());
    let xml = generate_xml(&payload).unwrap();

    assert!(!xml.contains("999999"), "hay un residuo binario en el XML");
    // 6 lineas x 3 x 10.30 = 185.40 con IGV incluido. Cada línea declara una base
    // de 26.19 y un IGV de 4.71, así que la cabecera suma lo que realmente dice el
    // documento (6 x 26.19 = 157.14), no el importe sin redondear (157.12), que
    // no aparece en ninguna línea.
    assert_eq!(valor_de(&xml, "cbc:TaxableAmount"), "157.14");
    assert_eq!(valor_de(&xml, "cbc:TaxAmount"), "28.26");
    assert!(xml.contains(">157.14</cbc:TaxableAmount>"));
    // Y la base más el IGV siguen dando el total cobrado.
    assert!(xml.contains("<cbc:PayableAmount currencyID=\"PEN\">185.40</cbc:PayableAmount>"));
}

/// El tipo de cambio se declara en UBL solo cuando la moneda no es soles.
#[test]
fn el_tipo_de_cambio_se_declara_en_moneda_extranjera() {
    let mut payload = payload_prueba();
    payload.moneda = "USD".to_string();
    payload.tipo_cambio = Some(3.44.into());

    let xml = generate_xml(&payload).unwrap();
    assert!(xml.contains("<cac:PricingExchangeRate>"), "{xml}");
    assert!(
        xml.contains("<cbc:SourceCurrencyCode>USD</cbc:SourceCurrencyCode>"),
        "{xml}"
    );
    assert!(
        xml.contains("<cbc:TargetCurrencyCode>PEN</cbc:TargetCurrencyCode>"),
        "{xml}"
    );
    assert!(
        xml.contains("<cbc:CalculationRate>3.44</cbc:CalculationRate>"),
        "{xml}"
    );

    // En soles se omite.
    let xml_pen = generate_xml(&payload_prueba()).unwrap();
    assert!(!xml_pen.contains("<cac:PricingExchangeRate>"), "{xml_pen}");
}

#[test]
fn la_factura_declara_forma_pago_contado_por_defecto() {
    let payload = payload_prueba();
    let xml = generate_xml(&payload).unwrap();
    assert!(xml.contains("<cac:PaymentTerms>"), "{xml}");
    assert!(xml.contains("<cbc:ID>FormaPago</cbc:ID>"), "{xml}");
    assert!(xml.contains("<cbc:PaymentMeansID>Contado</cbc:PaymentMeansID>"), "{xml}");
}

#[test]
fn la_factura_al_credito_declara_monto_pendiente_y_cuotas() {
    let mut payload = payload_prueba();
    payload.forma_pago = "Credito".to_string();
    payload.fecha_vencimiento = Some("2023-11-24".to_string());
    payload.monto_neto_pendiente = Some(118.0.into());

    let xml = generate_xml(&payload).unwrap();
    assert!(xml.contains("<cbc:PaymentMeansID>Credito</cbc:PaymentMeansID>"), "{xml}");
    assert!(xml.contains("<cbc:PaymentMeansID>Cuota001</cbc:PaymentMeansID>"), "{xml}");
    assert!(xml.contains("<cbc:PaymentDueDate>2023-11-24</cbc:PaymentDueDate>"), "{xml}");
    assert!(xml.contains("118.00"), "{xml}");
}

/// Devuelve el texto del primer elemento `<nombre ...>`, tolerando atributos.
fn valor_de<'a>(xml: &'a str, nombre: &str) -> &'a str {
    let apertura = xml
        .find(&format!("<{nombre} "))
        .or_else(|| xml.find(&format!("<{nombre}>")))
        .unwrap_or_else(|| panic!("no se encontró <{nombre}>"));
    let ini = xml[apertura..].find('>').expect("etiqueta sin cerrar") + apertura + 1;
    let fin = ini + xml[ini..].find(&format!("</{nombre}>")).expect(nombre);
    &xml[ini..fin]
}
