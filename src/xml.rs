use crate::error::{Result, XmlSignError};
use base64::{engine::general_purpose, Engine as _};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use rsa::pkcs1v15::{SigningKey, VerifyingKey};
use rsa::rand_core::OsRng;
use rsa::signature::{RandomizedSigner, SignatureEncoding, Verifier};
use rsa::RsaPrivateKey;
use rsa::RsaPublicKey;
use sha2::{Digest, Sha512};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct XmlDocument {
    pub content: String,
}

impl XmlDocument {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn read_nodes(&self) -> Result<Vec<XmlNode>> {
        let mut reader = Reader::from_str(&self.content);
        {
            let config = reader.config_mut();
            config.trim_text_start = true;
            config.trim_text_end = true;
        }
        let mut nodes = Vec::new();
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    let mut attributes = Vec::new();

                    for attr in e.attributes() {
                        let attr = attr?;
                        let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                        let value = String::from_utf8_lossy(&attr.value).to_string();
                        attributes.push((key, value));
                    }

                    nodes.push(XmlNode {
                        name,
                        attributes,
                        text_content: None,
                    });
                }
                Ok(Event::Text(e)) => {
                    let text = e.decode()?.to_string();
                    if !text.trim().is_empty() {
                        if let Some(last_node) = nodes.last_mut() {
                            last_node.text_content = Some(text);
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(XmlSignError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }

        Ok(nodes)
    }

    pub fn find_node(&self, node_name: &str) -> Result<Option<XmlNode>> {
        let nodes = self.read_nodes()?;
        Ok(nodes.into_iter().find(|n| n.name == node_name))
    }

    pub fn find_all_nodes(&self, node_name: &str) -> Result<Vec<XmlNode>> {
        let nodes = self.read_nodes()?;
        Ok(nodes.into_iter().filter(|n| n.name == node_name).collect())
    }

    fn canonicalize(&self) -> Result<String> {
        let mut reader = Reader::from_str(&self.content);
        {
            let config = reader.config_mut();
            config.trim_text_start = true;
            config.trim_text_end = true;
        }
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(e @ Event::Start(_) | e@ Event::End(_)) => {
                    writer.write_event(e)?;
                }
                Ok(e @ Event::Empty(_)) => {
                    writer.write_event(e)?;
                }
                Ok(Event::Text(e)) => {
                    let text = e.decode()?;
                    if !text.trim().is_empty() {
                        writer.write_event(Event::Text(BytesText::new(&text)))?;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(XmlSignError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }

        let result = writer.into_inner().into_inner();
        Ok(String::from_utf8_lossy(&result).to_string())
    }

    pub fn sign(&self, private_key: &RsaPrivateKey) -> Result<SignedXmlDocument> {
        let canonical = self.canonicalize()?;

        let mut hasher = Sha512::new();
        hasher.update(canonical.as_bytes());
        let hash = hasher.finalize();

        let signing_key: SigningKey<Sha512> = SigningKey::new(private_key.clone());
        let signature = signing_key.sign_with_rng(&mut OsRng, &hash);

        let signature_b64 = general_purpose::STANDARD.encode(signature.to_bytes());
        let signed_content = self.add_signature_element(&signature_b64)?;

        Ok(SignedXmlDocument {
            content: signed_content,
            signature: signature_b64,
        })
    }

    fn add_signature_element(&self, signature_b64: &str) -> Result<String> {
        let mut reader = Reader::from_str(&self.content);
        {
            let config = reader.config_mut();
            config.trim_text_start = true;
            config.trim_text_end = true;
        }
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let mut buf = Vec::new();
        let mut root_closed = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    writer.write_event(Event::Start(e.clone()))?;
                }
                Ok(Event::End(e)) => {
                    if !root_closed {
                        let sig_start = BytesStart::new("Signature");
                        writer.write_event(Event::Start(sig_start))?;

                        let sig_value_start = BytesStart::new("SignatureValue");
                        writer.write_event(Event::Start(sig_value_start))?;
                        writer.write_event(Event::Text(BytesText::new(signature_b64)))?;
                        writer.write_event(Event::End(BytesEnd::new("SignatureValue")))?;

                        writer.write_event(Event::End(BytesEnd::new("Signature")))?;

                        root_closed = true;
                    }
                    writer.write_event(Event::End(e))?;
                }
                Ok(Event::Text(e)) => {
                    writer.write_event(Event::Text(e))?;
                }
                Ok(Event::Empty(e)) => {
                    writer.write_event(Event::Empty(e))?;
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(XmlSignError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }

        let result = writer.into_inner().into_inner();
        Ok(String::from_utf8_lossy(&result).to_string())
    }
}

#[derive(Debug, Clone)]
pub struct XmlNode {
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub text_content: Option<String>,
}

impl XmlNode {
    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
}

#[derive(Debug)]
pub struct SignedXmlDocument {
    pub content: String,
    pub signature: String,
}

impl SignedXmlDocument {
    pub fn verify(&self, public_key: &RsaPublicKey) -> Result<bool> {
        let original_content = self.remove_signature_element()?;

        let doc = XmlDocument::new(original_content);
        let canonical = doc.canonicalize()?;

        let mut hasher = Sha512::new();
        hasher.update(canonical.as_bytes());
        let hash = hasher.finalize();

        let signature_bytes = general_purpose::STANDARD.decode(&self.signature)?;
        let signature = rsa::pkcs1v15::Signature::try_from(signature_bytes.as_slice())?;

        let verifying_key = VerifyingKey::<Sha512>::new(public_key.clone());
        match verifying_key.verify(&hash, &signature) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn remove_signature_element(&self) -> Result<String> {
        let mut reader = Reader::from_str(&self.content);
        {
            let config = reader.config_mut();
            config.trim_text_start = true;
            config.trim_text_end = true;
        }
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let mut buf = Vec::new();
        let mut in_signature = false;
        let mut depth = 0;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let name = e.name();
                    let name = String::from_utf8_lossy(name.as_ref());
                    if name == "Signature" && depth == 0 {
                        in_signature = true;
                        depth = 1;
                    } else if in_signature {
                        depth += 1;
                    } else {
                        writer.write_event(Event::Start(e))?;
                    }
                }
                Ok(Event::End(e)) => {
                    if in_signature {
                        depth -= 1;
                        if depth == 0 {
                            in_signature = false;
                        }
                    } else {
                        writer.write_event(Event::End(e))?;
                    }
                }
                Ok(Event::Text(e)) => {
                    if !in_signature {
                        writer.write_event(Event::Text(e))?;
                    }
                }
                Ok(Event::Empty(e)) => {
                    if !in_signature {
                        writer.write_event(Event::Empty(e))?;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(XmlSignError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }

        let result = writer.into_inner().into_inner();
        Ok(String::from_utf8_lossy(&result).to_string())
    }

    pub fn read_nodes(&self) -> Result<Vec<XmlNode>> {
        let doc = XmlDocument::new(self.content.clone());
        doc.read_nodes()
    }
}
