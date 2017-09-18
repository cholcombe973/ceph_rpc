extern crate byteorder;

use std::io::{Result, Error};

use self::byteorder::{LittleEndian, WriteBytesExt};

pub trait Serialize {
    /// Transform rust to ceph wire format
    fn serialize(&mut self) -> Result<Vec<u8>>;
}

/// Ceph utime
pub struct Utime {
    pub tv_sec: u32, // Seconds since epoch.
    pub tv_nsec: u32, // Nanoseconds since the last second.
}

impl Serialize for Utime {
    fn serialize(&mut self) -> Result<Vec<u8>> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.write_u32::<LittleEndian>(self.tv_sec)?;
        bytes.write_u32::<LittleEndian>(self.tv_nsec)?;

        Ok(bytes)
    }
}

pub struct EntityName {
    pub ceph_type: u8, // CEPH_ENTITY_TYPE_*
    pub num: u64,
}

impl Serialize for EntityName {
    fn serialize(&mut self) -> Result<Vec<u8>> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.write_u8(self.ceph_type)?;
        bytes.write_u64::<LittleEndian>(self.num)?;

        Ok(bytes)
    }
}

impl<A, B> Serialize for (A, B)
where
    A: Serialize,
    B: Serialize,
{
    fn serialize(&mut self) -> Result<Vec<u8>> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.0.serialize()?);
        bytes.extend(self.1.serialize()?);

        Ok(bytes)
    }
}

impl<A, B, C> Serialize for (A, B, C)
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
{
    fn serialize(&mut self) -> Result<Vec<u8>> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.0.serialize()?);
        bytes.extend(self.1.serialize()?);
        bytes.extend(self.2.serialize()?);

        Ok(bytes)
    }
}
