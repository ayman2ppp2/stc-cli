use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

use sha2::{Digest, Sha256};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

fn temp_file(name: &str) -> PathBuf {
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!("stc-cli-test-{}-{id}-{name}", std::process::id()))
}

fn run_cli(input: &str) -> (Output, PathBuf) {
    let input_path = temp_file("input.xml");
    let output_path = temp_file("output.xml");

    fs::write(&input_path, input).expect("write input XML");
    let output = Command::new(env!("CARGO_BIN_EXE_stc-cli"))
        .arg(&input_path)
        .arg(&output_path)
        .output()
        .expect("run stc-cli");

    let _ = fs::remove_file(input_path);
    (output, output_path)
}

fn canonicalize(input: &str) -> String {
    let (output, output_path) = run_cli(input);
    assert!(
        output.status.success(),
        "stc-cli failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let canonicalized = fs::read_to_string(&output_path).expect("read canonicalized XML");
    let _ = fs::remove_file(output_path);
    canonicalized
}

fn sha256_hex(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
}

#[test]
fn removes_comments() {
    let output = canonicalize("<root><!-- remove me --><child>text</child></root>");

    assert_eq!(output, "<root><child>text</child></root>");
}

#[test]
fn orders_attributes() {
    let output = canonicalize(r#"<root b="2" a="1"></root>"#);

    assert_eq!(output, r#"<root a="1" b="2"></root>"#);
}

#[test]
fn orders_namespaces() {
    let output = canonicalize(
        r#"<root xmlns:z="urn:z" xmlns:a="urn:a"><z:child a:attr="1"></z:child></root>"#,
    );

    let a_pos = output.find("xmlns:a=").expect("a namespace exists");
    let z_pos = output.find("xmlns:z=").expect("z namespace exists");
    assert!(a_pos < z_pos, "namespaces should be ordered: {output}");
}

#[test]
fn expands_empty_elements() {
    let output = canonicalize("<root><empty/></root>");

    assert_eq!(output, "<root><empty></empty></root>");
}

#[test]
fn canonicalizes_real_cleared_ubl_invoice() {
    let invoice = include_str!("fixtures/cleared_ubl_invoice.xml");
    let output = canonicalize(invoice);

    assert!(output.starts_with("<Invoice "));
    assert!(output.contains("<cbc:ProfileID>clearance</cbc:ProfileID>"));
    assert!(output.contains("<ds:SignedInfo>"));
    assert!(output.contains("<xades:SignedProperties Id=\"xadesSignedProperties\">"));
    assert!(output.contains("<cbc:ID>QR</cbc:ID>"));
    assert!(!output.contains("<?xml"));
}

#[test]
fn canonicalizes_signed_info() {
    let output = canonicalize(
        r#"<ds:SignedInfo xmlns:ds="http://www.w3.org/2000/09/xmldsig#"><ds:CanonicalizationMethod Algorithm="http://www.w3.org/2006/12/xml-c14n11#"/><ds:Reference URI=""><ds:DigestValue>abc</ds:DigestValue></ds:Reference></ds:SignedInfo>"#,
    );

    assert_eq!(
        output,
        r#"<ds:SignedInfo xmlns:ds="http://www.w3.org/2000/09/xmldsig#"><ds:CanonicalizationMethod Algorithm="http://www.w3.org/2006/12/xml-c14n11#"></ds:CanonicalizationMethod><ds:Reference URI=""><ds:DigestValue>abc</ds:DigestValue></ds:Reference></ds:SignedInfo>"#
    );
}

#[test]
fn canonicalizes_signed_properties() {
    let output = canonicalize(
        r#"<xades:SignedProperties xmlns:xades="http://uri.etsi.org/01903/v1.3.2#" Id="xadesSignedProperties"><xades:SignedSignatureProperties><xades:SigningTime>2026-05-21T11:39:00Z</xades:SigningTime></xades:SignedSignatureProperties></xades:SignedProperties>"#,
    );

    assert_eq!(
        output,
        r#"<xades:SignedProperties xmlns:xades="http://uri.etsi.org/01903/v1.3.2#" Id="xadesSignedProperties"><xades:SignedSignatureProperties><xades:SigningTime>2026-05-21T11:39:00Z</xades:SigningTime></xades:SignedSignatureProperties></xades:SignedProperties>"#
    );
}

#[test]
fn preserves_utf8_arabic_text() {
    let output = canonicalize("<root><name>فاتورة ضريبية</name></root>");

    assert_eq!(output, "<root><name>فاتورة ضريبية</name></root>");
}

#[test]
fn equivalent_c14n_normalized_xml_produces_same_sha256_hash() {
    let compact = canonicalize(r#"<root b="2" a="1"><child/></root>"#);
    let reordered_with_comment =
        canonicalize(r#"<root a="1" b="2"><!-- ignored by this CLI --><child></child></root>"#);

    assert_eq!(compact, reordered_with_comment);
    assert_eq!(sha256_hex(&compact), sha256_hex(&reordered_with_comment));
}

#[test]
fn invalid_xml_returns_non_zero_exit_code() {
    let (output, output_path) = run_cli("<root><unclosed></root>");
    let _ = fs::remove_file(output_path);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("stc-cli error"));
}
