use crate::data_types::Pixlzr;
use anyhow::Result;
use image::EncodableLayout;
// use qoi::Error as QOIError;
use std::{
    // error::Error,
    // fmt::{Display, Error as FMTError, Formatter},
    fs,
    // io::Error as IOError,
    path::Path,
};

// #[derive(Debug)]
// pub enum PixlzrError {
//     IO(IOError),
//     QOI(QOIError),
// }
// impl PixlzrError {
//     pub fn is_io(&self) -> bool {
//         match self {
//             PixlzrError::IO(_) => true,
//             _ => false,
//         }
//     }
//     pub fn is_qoi(&self) -> bool {
//         match self {
//             PixlzrError::QOI(_) => true,
//             _ => false,
//         }
//     }
//     pub fn unwrap_io(&self) -> &IOError {
//         if let PixlzrError::IO(io) = self {
//             io
//         } else {
//             panic!()
//         }
//     }
//     pub fn unwrap_qoi(&self) -> &QOIError {
//         if let PixlzrError::QOI(qoi) = self {
//             qoi
//         } else {
//             panic!()
//         }
//     }
// }

// impl From<QOIError> for PixlzrError {
//     fn from(value: QOIError) -> Self {
//         Self::QOI(value)
//     }
// }
// impl From<IOError> for PixlzrError {
//     fn from(value: IOError) -> Self {
//         Self::IO(value)
//     }
// }
// impl Display for PixlzrError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
//         if self.is_io() {
//             write!(f, "{:?}", self.unwrap_io())
//         } else {
//             write!(f, "{:?}", self.unwrap_qoi())
//         }
//     }
// }
// impl Error for PixlzrError {
//     fn description(&self) -> &str {
//         self.source().unwrap().description()
//     }
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         if self.is_io() {
//             Some(self.unwrap_io())
//         } else {
//             Some(self.unwrap_qoi())
//         }
//     }
// }

impl Pixlzr {
    pub fn open<P>(path: P) -> Result<Pixlzr>
    where
        P: AsRef<Path>,
    {
        let data = &fs::read(path)?;
        let pix = Pixlzr::decode_from_vec(data)?;
        Ok(pix)
    }
    pub fn save<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let data = self.encode_to_vec_vec()?;
        fs::write(path, data.as_bytes())?;
        Ok(())
    }
}
