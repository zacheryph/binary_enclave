#![allow(incomplete_features)]
#![feature(const_fn)]
#![feature(const_generics)]

mod result;

use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::io::{Seek, SeekFrom, Write};
use std::marker::PhantomData;

pub use bincrypt_macro::bincrypt;
pub use result::{Error, Result};

pub struct Bincrypt<T, const SIZE: usize>([u8; SIZE], PhantomData<T>);

pub trait Locator {
    const SECTION: &'static str;
}

impl<T, const SIZE: usize> Bincrypt<T, SIZE>
where
    T: Default + Locator + Serialize + DeserializeOwned,
{
    pub const fn new() -> Self {
        Self([0; SIZE], PhantomData)
    }

    pub fn decode(&self) -> T {
        match bincode::deserialize(&self.0) {
            Ok(s) => s,
            Err(e) => {
                println!("Deserialization Error. Ignoring. Bad Binary? {}", e);
                Default::default()
            }
        }
    }

    #[cfg(target_os = "macos")]
    pub fn write(&self, payload: &T) -> Result<usize> {
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
    pub fn write(&self, payload: &T) -> Result<usize> {
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
    pub fn write(&self, payload: &T) -> Result<usize> {
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
