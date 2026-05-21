use std::{env, fs, process};

use xml_c14n::{CanonicalizationMode, CanonicalizationOptions, canonicalize_xml};

fn main() {
    if let Err(err) = run() {
        eprintln!("stc-cli error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err(format!(
            "Usage: {} <input.xml> <output.xml>",
            args.first().map(String::as_str).unwrap_or("stc-cli")
        )
        .into());
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let xml = fs::read_to_string(input_path)?;
    let canonicalized = canonicalize_xml(
        &xml,
        CanonicalizationOptions {
            mode: CanonicalizationMode::Canonical1_1,
            keep_comments: false,
            inclusive_ns_prefixes: vec![],
        },
    )?;

    fs::write(output_path, canonicalized)?;

    println!("Canonicalized {input_path} to {output_path}");

    Ok(())
}
