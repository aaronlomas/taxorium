//! Canonicalización XML C14N 1.0 — W3C REC-xml-c14n-20010315, sin comentarios.
//!
//! Sustituye a `xml-canonicalization`, que produce una forma canónica distinta a la
//! que recalcula SUNAT al validar la firma. Ese crate:
//!
//! 1. Ordenaba las declaraciones de espacio de nombres de forma inconsistente, por
//!    lo que `xmlns="..."` (que debe ir siempre primero) acababa al final del elemento
//!    raíz. Como el digest se calcula sobre esos bytes, SUNAT obtenía otro hash y
//!    respondía "Incorrect reference digest value".
//! 2. No re-emitía los espacios de nombres heredados en el nodo ápice, así que la
//!    forma canónica de `SignedInfo` no coincidía con la del documento completo.
//!
//! Aquí se implementa el algoritmo sobre `quick-xml`, que sí resuelve las entidades
//! y los CDATA del documento antes de serializar.

use std::cmp::Ordering;

use quick_xml::escape::unescape;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct C14nError {
    pub message: String,
}

impl C14nError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for C14nError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for C14nError {}

type Result<T> = std::result::Result<T, C14nError>;

/// Declaração de espacio de nombres: `(prefijo, URI)`. El prefijo vacío es el
/// namespace por defecto (`xmlns`).
type Decl = (String, String);

struct RawAttr {
    qname: String,
    value: String,
}

/// Canoniza el documento completo.
///
/// Equivale a `xml-c14n-20010315` sobre el conjunto de nodos "todos los nodos".
pub fn canonicalize(input: &str) -> Result<String> {
    Canonicalizer::whole_document().run(input)
}

/// Canoniza un único elemento (el que tenga `Id="<id>"`) con C14N inclusivo.
///
/// Este es el pre-imagen que XMLDSIG usa para `SignedInfo`: C14N inclusivo exige
/// declarar en el nodo ápice todos los espacios de nombres que están en alcance,
/// incluidos los heredados de los ancestros. Por eso no basta con recortar el
/// fragmento del texto renderizado.
pub fn canonicalize_element_by_id(input: &str, id: &str) -> Result<String> {
    Canonicalizer::element_by_id(id).run(input)
}

struct Canonicalizer<'a> {
    apex_id: Option<&'a str>,
    out: String,
    /// Declaraciones de espacio de nombres que introduce cada elemento abierto.
    ns_stack: Vec<Vec<Decl>>,
    depth: usize,
    /// `true` cuando ya se está escribiendo el subárbol que forma el nodo ápice.
    writing: bool,
    /// Profundidad del elemento raíz del nodo ápice, si ya se encontró.
    apex_depth: Option<usize>,
    found_apex: bool,
}

impl<'a> Canonicalizer<'a> {
    fn whole_document() -> Self {
        Self {
            apex_id: None,
            out: String::new(),
            ns_stack: Vec::new(),
            depth: 0,
            writing: false,
            apex_depth: None,
            found_apex: false,
        }
    }

    fn element_by_id(id: &'a str) -> Self {
        Self {
            apex_id: Some(id),
            ..Self::whole_document()
        }
    }

    fn run(mut self, input: &str) -> Result<String> {
        let mut reader = Reader::from_str(input);
        reader.config_mut().check_end_names = true;
        reader.config_mut().expand_empty_elements = true;
        reader.config_mut().trim_text(false);

        let mut buf = Vec::new();
        loop {
            let event = reader
                .read_event_into(&mut buf)
                .map_err(|e| C14nError::new(format!("XML inválido: {e}")))?;

            match event {
                // La declaración XML y el DTD no forman parte de la forma canónica.
                Event::Decl(_) | Event::DocType(_) => {}
                Event::Eof => break,

                Event::Start(start) => {
                    let attrs = parse_attributes(&start)?;
                    let decls = collect_declarations(&attrs);
                    let in_scope = self.in_scope();

                    let is_apex = if self.writing {
                        false
                    } else {
                        match self.apex_id {
                            None => self.depth == 0,
                            Some(id) => attrs.iter().any(|a| a.qname == "Id" && a.value == id),
                        }
                    };
                    if is_apex {
                        self.found_apex = true;
                        self.apex_depth = Some(self.depth);
                    }

                    if self.writing || is_apex {
                        let rendered = if is_apex {
                            // Nodo ápice: se emiten todos los espacios de nombres en
                            // alcance, incluidos los heredados.
                            let mut merged = in_scope;
                            for decl in &decls {
                                match merged.iter_mut().find(|(p, _)| p == &decl.0) {
                                    Some(slot) => slot.1 = decl.1.clone(),
                                    None => merged.push(decl.clone()),
                                }
                            }
                            merged
                        } else {
                            // Descendiente: solo las declaraciones nuevas, y solo si
                            // no replican un valor ya visible desde el ancestro.
                            decls
                                .iter()
                                .filter(|(prefix, uri)| {
                                    lookup(&in_scope, prefix)
                                        .map(|u| u != uri.as_str())
                                        .unwrap_or(true)
                                })
                                .cloned()
                                .collect()
                        };
                        self.write_start(&start, &attrs, &rendered)?;
                    }

                    self.ns_stack.push(decls.clone());
                    self.writing = self.writing || is_apex;
                    self.depth += 1;
                }

                Event::End(end) => {
                    if self.writing {
                        self.out.push_str("</");
                        self.out
                            .push_str(&String::from_utf8_lossy(end.name().as_ref()));
                        self.out.push('>');
                        if self.apex_depth == self.depth.checked_sub(1) {
                            self.writing = false;
                        }
                    }
                    self.ns_stack.pop();
                    self.depth = self.depth.saturating_sub(1);
                }

                Event::Text(text) => {
                    if !self.writing {
                        continue;
                    }
                    let raw = String::from_utf8_lossy(&text);
                    let normalized = normalize_line_endings(&raw);
                    let resolved = unescape(&normalized)
                        .map_err(|e| C14nError::new(format!("Entidad inválida: {e}")))?;
                    self.out.push_str(&escape_text(&resolved));
                }

                // C14N 1.0 §2.4: una sección CDATA se reemplaza por su contenido.
                Event::CData(cdata) => {
                    if !self.writing {
                        continue;
                    }
                    let raw = String::from_utf8_lossy(&cdata);
                    self.out
                        .push_str(&escape_text(&normalize_line_endings(&raw)));
                }

                Event::PI(pi) => {
                    if !self.writing {
                        continue;
                    }
                    let content = String::from_utf8_lossy(&pi.into_inner()).into_owned();
                    self.out
                        .push_str(&format!("<?{}?>", tidy_processing_instruction(&content)));
                }

                // C14N 1.0 sin comentarios: se descartan.
                Event::Comment(_) => {}

                other => {
                    return Err(C14nError::new(format!(
                        "El documento usa una construcción XML no soportada: {other:?}"
                    )))
                }
            }
            buf.clear();
        }

        if !self.found_apex {
            let target = self.apex_id.unwrap_or("documento");
            return Err(C14nError::new(format!(
                "No se encontró el nodo raíz «{target}» para canonicalizar"
            )));
        }
        // C14N 1.0 no añade salto de línea final: cualquier byte extra cambia el
        // digest frente al que calcula el validador de SUNAT.
        Ok(self.out)
    }

    /// Espacios de nombres declarados y visibles desde la posición actual; el más
    /// interno gana. El prefijo `xml`, ligado de forma implícita en todo documento,
    /// no se incluye porque C14N no lo renderiza salvo que se redeclare.
    fn in_scope(&self) -> Vec<Decl> {
        let mut merged: Vec<Decl> = Vec::new();
        for frame in &self.ns_stack {
            for decl in frame {
                match merged.iter_mut().find(|(p, _)| p == &decl.0) {
                    Some(slot) => slot.1 = decl.1.clone(),
                    None => merged.push(decl.clone()),
                }
            }
        }
        merged
    }

    fn resolve_prefix(&self, in_scope: &[Decl], prefix: &str) -> String {
        if let Some(uri) = lookup(in_scope, prefix) {
            return uri.to_string();
        }
        if prefix == "xml" {
            return "http://www.w3.org/XML/1998/namespace".to_string();
        }
        prefix.to_string()
    }

    fn write_start(
        &mut self,
        start: &BytesStart<'_>,
        attrs: &[RawAttr],
        namespaces: &[Decl],
    ) -> Result<()> {
        let qname = String::from_utf8_lossy(start.name().into_inner()).into_owned();
        let mut out = String::with_capacity(qname.len() + 64);
        out.push('<');
        out.push_str(&qname);

        // C14N 1.0 §2.3: `xmlns` por defecto primero, luego los prefijados por prefijo.
        let mut namespaces = namespaces.to_vec();
        namespaces.sort_by(|a, b| a.0.cmp(&b.0));
        for (prefix, uri) in &namespaces {
            if prefix.is_empty() {
                out.push_str(&format!(" xmlns=\"{}\"", escape_attribute(uri)));
            } else {
                out.push_str(&format!(" xmlns:{}=\"{}\"", prefix, escape_attribute(uri)));
            }
        }

        // C14N 1.0 §3.6.2: atributos sin prefijo ordenados por nombre, luego los
        // prefijados por (URI del espacio de nombres, nombre local). El prefijo se
        // conserva tal cual; solo cambia el orden.
        let in_scope = self.in_scope();
        let mut rendered: Vec<(String, String, String, String)> = attrs
            .iter()
            .filter(|a| a.qname != "xmlns" && !a.qname.starts_with("xmlns:"))
            .map(|a| {
                let (prefix, local) = split_qname(&a.qname);
                let uri = if prefix.is_empty() {
                    String::new()
                } else {
                    self.resolve_prefix(&in_scope, &prefix)
                };
                (uri, local, a.qname.clone(), a.value.clone())
            })
            .collect();
        rendered.sort_by(|a, b| match (a.0.is_empty(), b.0.is_empty()) {
            (true, true) => a.1.cmp(&b.1),
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)),
        });
        for (_, _, qname, value) in rendered {
            out.push_str(&format!(" {qname}=\"{}\"", escape_attribute(&value)));
        }

        out.push('>');
        self.out.push_str(&out);
        Ok(())
    }
}

fn lookup<'a>(decls: &'a [Decl], prefix: &str) -> Option<&'a str> {
    decls
        .iter()
        .find(|(p, _)| p == prefix)
        .map(|(_, uri)| uri.as_str())
}

fn split_qname(qname: &str) -> (String, String) {
    match qname.split_once(':') {
        Some((prefix, local)) => (prefix.to_string(), local.to_string()),
        None => (String::new(), qname.to_string()),
    }
}

fn parse_attributes(start: &BytesStart<'_>) -> Result<Vec<RawAttr>> {
    let mut attrs = Vec::new();
    for attr in start.attributes().with_checks(true) {
        let attr = attr.map_err(|e| C14nError::new(format!("Atributo XML inválido: {e}")))?;
        let qname = String::from_utf8_lossy(attr.key.as_ref()).into_owned();
        // Normalización de valores de atributo (XML 1.0 §3.3.3) sobre el texto
        // literal, antes de resolver las referencias de carácter: un `&#x9;` debe
        // sobrevivir como tabulador, no convertirse en un espacio.
        let raw = String::from_utf8_lossy(&attr.value);
        let raw = raw.replace("\r\n", " ");
        let raw = raw.replace(['\r', '\n', '\t'], " ");
        let value = unescape(&raw).map_err(|e| C14nError::new(format!("Entidad inválida: {e}")))?;
        attrs.push(RawAttr {
            qname,
            value: value.into_owned(),
        });
    }
    Ok(attrs)
}

fn collect_declarations(attrs: &[RawAttr]) -> Vec<Decl> {
    attrs
        .iter()
        .filter_map(|a| {
            if a.qname == "xmlns" {
                Some((String::new(), a.value.clone()))
            } else {
                a.qname
                    .strip_prefix("xmlns:")
                    .map(|prefix| (prefix.to_string(), a.value.clone()))
            }
        })
        .collect()
}

fn normalize_line_endings(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

/// C14N 1.0 §2.3 para contenido de texto.
fn escape_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '\r' => out.push_str("&#xD;"),
            _ => out.push(ch),
        }
    }
    out
}

/// C14N 1.0 §2.3 para valores de atributo.
fn escape_attribute(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '"' => out.push_str("&quot;"),
            '\t' => out.push_str("&#x9;"),
            '\n' => out.push_str("&#xA;"),
            '\r' => out.push_str("&#xD;"),
            _ => out.push(ch),
        }
    }
    out
}

/// C14N 1.0 §2.3 para processing instructions.
fn tidy_processing_instruction(content: &str) -> String {
    let content = normalize_line_endings(content);
    let (target, rest) = match content.find(|c: char| c.is_whitespace()) {
        Some(i) => (&content[..i], &content[i..]),
        None => (content.as_str(), ""),
    };
    let rest = rest.trim_matches(' ');
    let mut out = String::from(target);
    if !rest.is_empty() {
        out.push(' ');
        out.push_str(rest);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Golden vectors contrastados contra `lxml.etree.tostring(method="c14n")`.
    #[test]
    fn default_namespace_is_rendered_first() {
        let out =
            canonicalize(r#"<Invoice xmlns="urn:inv" xmlns:cac="urn:cac"><a>1</a></Invoice>"#)
                .unwrap();
        assert_eq!(
            out,
            r#"<Invoice xmlns="urn:inv" xmlns:cac="urn:cac"><a>1</a></Invoice>"#
        );
    }

    #[test]
    fn attributes_are_sorted_canonically() {
        let out = canonicalize(
            r#"<a xmlns:p="urn:p" z="26" m="13" b:id="1" a:q="2" p:m="3" a="1" b="2"/>"#,
        )
        .unwrap();
        assert_eq!(
            out,
            r#"<a xmlns:p="urn:p" a="1" b="2" m="13" z="26" a:q="2" b:id="1" p:m="3"></a>"#
        );
    }

    #[test]
    fn cdata_is_expanded_and_entities_normalised() {
        let out =
            canonicalize("<a><![CDATA[ACME & CO <SRL>]]><b>&amp;x&#x9;&#xD;y</b></a>").unwrap();
        assert_eq!(out, "<a>ACME &amp; CO &lt;SRL&gt;<b>&amp;x\t&#xD;y</b></a>");
    }

    #[test]
    fn xml_declaration_comments_and_empty_tags() {
        let out = canonicalize(
            r#"<?xml version="1.0" encoding="utf-8"?><!-- c --><a><b/></a><!-- d -->"#,
        )
        .unwrap();
        assert_eq!(out, "<a><b></b></a>");
    }

    #[test]
    fn inherited_namespaces_are_hoisted_to_the_apex_node() {
        let doc = r#"<Invoice xmlns="urn:inv" xmlns:cbc="urn:cbc"><ds:SignedInfo Id="SignedInfo" xmlns:ds="urn:ds"><ds:R>1</ds:R></ds:SignedInfo></Invoice>"#;
        let out = canonicalize_element_by_id(doc, "SignedInfo").unwrap();
        assert_eq!(
            out,
            r#"<ds:SignedInfo xmlns="urn:inv" xmlns:cbc="urn:cbc" xmlns:ds="urn:ds" Id="SignedInfo"><ds:R>1</ds:R></ds:SignedInfo>"#
        );
    }

    #[test]
    fn missing_apex_is_an_error() {
        assert!(canonicalize_element_by_id("<a/>", "Nope").is_err());
    }

    #[test]
    fn malformed_xml_is_an_error() {
        assert!(canonicalize("<a><b></a>").is_err());
    }
}
