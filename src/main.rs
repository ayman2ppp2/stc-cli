use std::fs;
use xml_c14n::{CanonicalizationMode, CanonicalizationOptions, canonicalize_xml};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "input.xml";
    let output = "output.xml";

    let xml = fs::read_to_string(input)?;
    let canonicalized = canonicalize_xml(
        &xml,
        CanonicalizationOptions {
            mode: CanonicalizationMode::Canonical1_1,
            keep_comments: false,
            inclusive_ns_prefixes: vec![],
        },
    )?;

    fs::write(output, canonicalized)?;
    println!("Canonicalized {} to {}", input, output);
    Ok(())
}