pub mod crypto;
pub mod error;
pub mod xml;

pub use error::Result;

#[cfg(test)]
mod tests {
    use crate::crypto::KeyPair;
    use crate::xml::XmlDocument;

    #[test]
    fn test_read_nodes() {
        let xml = r#"<root><child id="1">Text</child></root>"#;
        let doc = XmlDocument::new(xml.to_string());
        let nodes = doc.read_nodes().unwrap();
        assert!(nodes.len() >= 2);
    }

    #[test]
    fn test_sign_and_verify() {
        let xml = r#"<root><data>test</data></root>"#;
        let doc = XmlDocument::new(xml.to_string());
        let keypair = KeyPair::generate(2048).unwrap();
        let signed = doc.sign(&keypair.private_key).unwrap();
        let is_valid = signed.verify(&keypair.public_key).unwrap();
        assert!(is_valid);
    }
}
