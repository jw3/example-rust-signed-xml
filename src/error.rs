use thiserror::Error;

#[derive(Error, Debug)]
pub enum XmlSignError {
    #[error("XML parsing error: {0}")]
    XmlError(#[from] quick_xml::Error),
    #[error("RSA error: {0}")]
    RsaError(#[from] rsa::Error),
    #[error("Signature error: {0}")]
    SignatureError(#[from] rsa::signature::Error),
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Signature not found")]
    SignatureNotFound,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Decoding error")]
    EncodingError(#[from] quick_xml::encoding::EncodingError),
    #[error("Attribute error")]
    AttributeError(#[from] quick_xml::events::attributes::AttrError),
}

pub type Result<T> = std::result::Result<T, XmlSignError>;
