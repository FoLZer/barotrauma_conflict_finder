use std::{io::Read, ops::Deref};

use flate2::read::GzDecoder;
use md5::{Digest, Md5};
use roxmltree::Document;

use crate::shared::{content_files::submarine_file::SubmarineFile, util::XmlContentFile};

#[derive(Debug)]
pub struct ContentFile<T: XmlContentFile + Sync + Send> {
    value: T,
    pub file_path: String,
}

impl<T: XmlContentFile + Sync + Send> ContentFile<T> {
    pub fn load_from_path(file_path: String) -> Result<Self, roxmltree::Error> {
        let s = std::fs::read_to_string(&file_path).unwrap();
        Self::load(&s, file_path)
    }

    pub fn load(s: &str, file_path: String) -> Result<Self, roxmltree::Error> {
        let document = Document::parse(&s).unwrap();
        let root = document.root_element();
        Ok(ContentFile {
            value: T::from_xml(root),
            file_path,
        })
    }
}

impl<T: XmlContentFile + Sync + Send> Deref for ContentFile<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Debug)]
pub struct SubmarineAsset {
    pub sub: SubmarineFile,
    pub hash: [u8; 16],
    pub file_path: String,
}

impl SubmarineAsset {
    pub fn load_from_path(file_path: String) -> Result<Self, SubAssetLoaderError> {
        let s = std::fs::read(&file_path).unwrap();
        Self::load(&s, file_path)
    }

    pub fn load(bytes: &[u8], file_path: String) -> Result<Self, SubAssetLoaderError> {
        let mut decoder = GzDecoder::new(bytes);
        let mut buf = Vec::new();
        decoder.read_to_end(&mut buf)?;

        let raw = String::from_utf8(buf)?;

        let hash = {
            let mut hasher = Md5::new();
            hasher.update(raw.replace(char::is_whitespace, ""));
            hasher.finalize().into()
        };

        let document = Document::parse(&raw)?;

        Ok(SubmarineAsset {
            sub: SubmarineFile::from_xml(document.root_element()),
            hash,
            file_path
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SubAssetLoaderError {
    #[error("failed to parse utf8")]
    Utf8(std::string::FromUtf8Error),
    #[error("failed to parse xml")]
    Xml(roxmltree::Error),
    #[error("failed to read data from file")]
    Io(std::io::Error),
}

impl From<std::io::Error> for SubAssetLoaderError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<std::string::FromUtf8Error> for SubAssetLoaderError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::Utf8(value)
    }
}

impl From<roxmltree::Error> for SubAssetLoaderError {
    fn from(value: roxmltree::Error) -> Self {
        Self::Xml(value)
    }
}
