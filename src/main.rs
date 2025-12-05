use std::fs;
use std::path::PathBuf;
use clap::Parser;
use example_rust_signed_xml::crypto::KeyPair;
use example_rust_signed_xml::error::Result;
use example_rust_signed_xml::xml::XmlDocument;

#[derive(Debug, Clone, Parser)]
struct Opts {
    source: PathBuf,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    println!("Generating 4096 bit RSA key pair...");
    let keypair = KeyPair::generate(4096)?;

    let xml_content = fs::read_to_string(&opts.source)?;
    let doc = XmlDocument::new(xml_content.to_string());
    let nodes = doc.read_nodes()?;
    println!("Original: {} nodes", nodes.len());

    println!("Signing XML document...");
    let signed_doc = doc.sign(&keypair.private_key)?;
    println!("Document signature: {}...", &signed_doc.signature[..20]);

    let signed_nodes = signed_doc.read_nodes()?;
    println!("Updated: {} nodes", signed_nodes.len());

    if signed_nodes.iter().find(|n| n.name == "Signature").is_some() {
        println!("Signature element is present");
    }

    println!("Signed XML document:");
    println!("{}", signed_doc.content);

    let is_valid = signed_doc.verify(&keypair.public_key)?;
    println!("Is signature valid: {}", is_valid);

    Ok(())
}
