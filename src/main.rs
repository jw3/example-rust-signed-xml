use example_rust_signed_xml::crypto::KeyPair;
use example_rust_signed_xml::error::Result;
use example_rust_signed_xml::xml::XmlDocument;

fn main() -> Result<()> {
    println!("Generating RSA key pair (2048 bits)...");
    let keypair = KeyPair::generate(2048)?;
    println!("Key pair generated successfully\n");

    let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<Document>
    <Header id="h1">
        <Title>Sample Document</Title>
        <Author>John Doe</Author>
    </Header>
    <Body>
        <Section name="intro">
            <Paragraph>This is the first paragraph.</Paragraph>
            <Paragraph>This is the second paragraph.</Paragraph>
        </Section>
        <Section name="conclusion">
            <Paragraph>This is the conclusion.</Paragraph>
        </Section>
    </Body>
</Document>"#;

    let doc = XmlDocument::new(xml_content.to_string());

    let nodes = doc.read_nodes()?;
    for (i, node) in nodes.iter().enumerate() {
        println!("Node {}: <{}>", i + 1, node.name);
        if !node.attributes.is_empty() {
            for (key, value) in &node.attributes {
                println!("  @{} = \"{}\"", key, value);
            }
        }
        if let Some(text) = &node.text_content {
            println!("  Content: {}", text);
        }
    }
    println!("Read {} total unsigned nodes", nodes.len());

    println!("Signing XML document...");
    let signed_doc = doc.sign(&keypair.private_key)?;
    println!("Document signed successfully, signature: {}...", &signed_doc.signature[..20]);

    let signed_nodes = signed_doc.read_nodes()?;
    println!("Read {} total nodes (including signature)", signed_nodes.len());

    if signed_nodes.iter().find(|n| n.name == "Signature").is_some() {
        println!("Signature element found in document");
    }

    let is_valid = signed_doc.verify(&keypair.public_key)?;
    println!("Is signature valid: {}", is_valid);

    println!("Signed XML document:");
    println!("{}", signed_doc.content);

    Ok(())
}
