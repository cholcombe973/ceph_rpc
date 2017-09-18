extern crate byteorder;

use std::collections::HashMap;
use std::hash::Hash;
use std::io::{Cursor, Result};

use self::byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

static CEPH_AES_IV: &'static str = "cephsageyudagreg";
static AUTH_ENC_MAGIC: u64 = 0xff009cad8826aa55;

pub trait Serialize {
    /// Transform rust to ceph wire format
    fn serialize(&self, buff: &mut Vec<u8>) -> Result<()>;
}

/// Ceph utime
pub struct Utime {
    pub tv_sec: u32, // Seconds since epoch.
    pub tv_nsec: u32, // Nanoseconds since the last second.
}

impl Serialize for Utime {
    fn serialize(&self, buff: &mut Vec<u8>) -> Result<()> {
        buff.write_u32::<LittleEndian>(self.tv_sec)?;
        buff.write_u32::<LittleEndian>(self.tv_nsec)?;
        Ok(())
    }
}

pub struct EntityName {
    pub ceph_type: u8, // CEPH_ENTITY_TYPE_*
    pub num: u64,
}

impl Serialize for EntityName {
    fn serialize(&self, buff: &mut Vec<u8>) -> Result<()> {
        buff.write_u8(self.ceph_type)?;
        buff.write_u64::<LittleEndian>(self.num)?;
        Ok(())
    }
}

pub struct CephPair<A, B>
where
    A: Serialize,
    B: Serialize,
{
    pub a: A,
    pub b: B,
}

impl<A, B> Serialize for CephPair<A, B>
where
    A: Serialize,
    B: Serialize,
{
    fn serialize(&self, buff: &mut Vec<u8>) -> Result<()> {
        self.a.serialize(buff)?;
        self.b.serialize(buff)?;
        Ok(())
    }
}

pub struct CephTriple<A, B, C>
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
{
    pub a: A,
    pub b: B,
    pub c: C,
}

impl<A, B, C> Serialize for CephTriple<A, B, C>
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
{
    fn serialize(&self, buff: &mut Vec<u8>) -> Result<()> {
        self.a.serialize(buff)?;
        self.b.serialize(buff)?;
        self.c.serialize(buff)?;
        Ok(())
    }
}

impl<A> Serialize for Vec<A>
where
    A: Serialize,
{
    fn serialize(&self, buff: &mut Vec<u8>) -> Result<()> {
        buff.write_u32::<LittleEndian>(self.len() as u32)?;
        for item in self {
            item.serialize(buff)?;
        }
        Ok(())
    }
}

impl Serialize for str {
    fn serialize(&self, buff: &mut Vec<u8>) -> Result<()> {
        buff.write_u32::<LittleEndian>(self.len() as u32)?;
        buff.extend(self.as_bytes());
        Ok(())
    }
}

impl Serialize for String {
    fn serialize(&self, buff: &mut Vec<u8>) -> Result<()> {
        buff.write_u32::<LittleEndian>(self.len() as u32)?;
        buff.extend(self.as_bytes());
        Ok(())
    }
}

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize(&self, buff: &mut Vec<u8>) -> Result<()> {
        //struct ceph_optional<T> {
        //u8 present;
        //T  element[present? 1 : 0]; // Only if present is non-zero.
        match self {
            &Some(ref t) => {
                buff.write_u8(1)?;
                t.serialize(buff)?;
            }
            &None => {
                buff.write_u8(0)?;
            }
        };
        Ok(())
    }
}

impl<K, V> Serialize for HashMap<K, V>
where
    K: Eq + Hash + Serialize,
    V: Serialize,
{
    fn serialize(&self, buff: &mut Vec<u8>) -> Result<()> {
        buff.write_u32::<LittleEndian>(self.len() as u32)?;
        for (k, v) in self {
            // TODO: Why do i have to clone this?
            // let mut key = k.clone();
            k.serialize(buff)?;
            v.serialize(buff)?;
        }
        Ok(())
    }
}
