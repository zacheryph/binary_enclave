#![allow(incomplete_features)]
#![feature(const_fn)]
#![feature(const_generics)]

//! `binary_enclave` allows storing configuration data in a binary directly. You
//! will probably never find a good reason for doing this. This is primarily an
//! exercise for learning rust and something I found interesting.
//!
//! > _nightly warning_
//! > this currently requires nightly for `const_fn` and `const_generics`
//!
//! ### Caveats
//!
//! * Written payload is only visible upon next execution.
//!
//! ### Basic Usage
//!
//! ```edition2018
//! use serde::{Serialize, Deserialize};
//! use binary_enclave::{enclave, Enclave};
//!
//! #[derive(Default, Serialize, Deserialize)]
//! struct Config { some: u32, values: String };
//!
//! #[enclave(appconfig)]
//! pub static CONFIG: Enclave<Config, 128> = Enclave::new();
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let conf = CONFIG.decode()?;
//!     let res = CONFIG.write(&Config{ some: 43, values: "see".to_string() })?;
//!     Ok(())
//! }
//! ```

#[doc(hidden)]
mod error;

use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::io::{Seek, SeekFrom, Write};
use std::marker::PhantomData;

pub use crate::error::{Error, Result};
pub use binary_enclave_macro::enclave;

#[doc(hidden)]
pub trait EnclaveLocator {
    const SECTION: &'static str;
}

/// Our enclave that will store the serialized value within our binary
///
/// The Enclave defines the type we are serializing into the binary
/// and also must be given a size for making room in the binary. If
/// the serialized type is larger than this, the write will fail.
///
/// The size given will increase the size of the binary linearly.
/// Setting this to an extremely large size will give you an extremely
/// large binary.
pub struct Enclave<T, const SIZE: usize>([u8; SIZE], PhantomData<T>);

impl<T, const SIZE: usize> Enclave<T, SIZE>
where
    T: Default + Serialize + DeserializeOwned + EnclaveLocator,
{
    /// Gives us a new Enclave with the size specified.
    pub const fn new() -> Self {
        Self([0; SIZE], PhantomData)
    }

    /// Deserialize the embedded Enclave into an instance of our specified type.
    pub fn decode(&self) -> T {
        match bincode::deserialize(&self.0) {
            Ok(s) => s,
            Err(e) => {
                println!("Deserialization Error. Ignoring. Bad Binary? {}", e);
                Default::default()
            }
        }
    }

    /// Write a new payload into the binary. This takes place
    /// by copying the binary, writing our payload into it,
    /// and moving the new binary overtop the current. This
    /// is required due to restrictions on some OS of modifying
    /// a binary currently being executing.
    pub fn write(&self, payload: &T) -> Result<usize> {
        self._write(payload)
    }

    #[cfg(target_os = "macos")]
    #[doc(hidden)]
    pub fn _write(&self, payload: &T) -> Result<usize> {
        use goblin::mach;

        let mut data = read_binary()?;
        let mach = mach::MachO::parse(&data, 0)?;
        let segment = mach
            .segments
            .iter()
            .find(|s| s.name().unwrap() == "__DATA")
            .ok_or_else(|| Error::SectionNotFound("__DATA Segment not found".into()))?;
        let (offset, size) = segment
            .sections()?
            .iter()
            .find(|sec| sec.0.name().unwrap() == T::SECTION)
            .map(|x| (x.0.offset, x.0.size))
            .ok_or_else(|| Error::SectionNotFound("Binary Section not found".into()))?;

        write_binary(&mut data, &payload, offset as usize, size as usize)
    }

    #[cfg(target_os = "linux")]
    #[doc(hidden)]
    pub fn _write(&self, payload: &T) -> Result<usize> {
        use goblin::elf::Elf;

        let mut data = read_binary()?;
        let elf = Elf::parse(&data)?;
        let section = elf
            .section_headers
            .iter()
            .find(|sec| &elf.shdr_strtab[sec.sh_name] == ".bincrypt")
            .ok_or_else(|| Error::SectionNotFound("Binary Section not found".into()))?;

        write_binary(&mut data, &payload, section.sh_offset, section.sh_size)
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    pub fn _write(&self, payload: &T) -> Result<usize> {
        panic!("Not Supported")
    }
}

fn read_binary() -> Result<Vec<u8>> {
    let args: Vec<String> = std::env::args().collect();
    let bytes = fs::read(&args[0])?;
    Ok(bytes)
}

fn write_binary<T: Serialize>(
    data: &mut Vec<u8>,
    payload: &T,
    offset: usize,
    size: usize,
) -> Result<usize> {
    let payload = bincode::serialize(payload)?;
    if payload.len() > size {
        return Err(Error::SectionSizeExceeded {
            payload: payload.len(),
            section: size,
        });
    }

    let mut data = std::io::Cursor::new(data);
    data.seek(SeekFrom::Start(offset as u64))?;
    data.write_all(&payload)?;
    let data = data.into_inner();

    let args: Vec<String> = std::env::args().collect();
    let file = &args[0];
    let tmpfile = format!("{}.new", file);

    let perms = fs::metadata(&file)?.permissions();
    fs::write(&tmpfile, &data)?;
    fs::rename(&tmpfile, &file)?;
    fs::set_permissions(&file, perms)?;

    Ok(payload.len())
}
