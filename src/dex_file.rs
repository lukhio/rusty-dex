#![allow(dead_code)]

use std::io::Read;

use crate::error::DexError;
use crate::dex_reader::DexReader;

#[derive(Debug)]
pub struct DexHeader {
    version: [u8; 3],
    checksum: u32,
    signature: [u8; 20],
    pub file_size: u32,
    pub header_size: u32,
    endian_tag: u32,
    pub link_size: u32,
    pub link_off: u32,
    pub map_off: u32,
    pub string_ids_size: u32,
    pub string_ids_off: u32,
    pub type_ids_size: u32,
    pub type_ids_off: u32,
    pub proto_ids_size: u32,
    pub proto_ids_off: u32,
    pub fields_ids_size: u32,
    pub fields_ids_off: u32,
    pub method_ids_size: u32,
    pub method_ids_off: u32,
    pub class_defs_size: u32,
    pub class_defs_off: u32,
    pub data_size: u32,
    pub data_off: u32
}

impl DexHeader {
    pub fn new(dex_cursor: &mut DexReader) -> Result<DexHeader, DexError> {
        /* DEX version */
        let mut magic = [0; 8];
        dex_cursor.bytes.read_exact(&mut magic).unwrap();
        let mut version = [0; 3];
        version[0] = magic[4];
        version[1] = magic[5];
        version[2] = magic[6];

        let checksum = dex_cursor.read_u32().unwrap();
        /* match adler32::verify_from_bytes(&dex_cursor.bytes.bytes(), checksum) {
            Ok(_) => { },
            Err(err) => {panic!("{}", err);},
        } */

        let mut signature = [0; 20];
        dex_cursor.bytes.read_exact(&mut signature).unwrap();

        let file_size = dex_cursor.read_u32().unwrap();
        let header_size = dex_cursor.read_u32().unwrap();
        let endian_tag = dex_cursor.read_u32().unwrap();

        let link_size = dex_cursor.read_u32().unwrap();
        let link_off = dex_cursor.read_u32().unwrap();
        let map_off = dex_cursor.read_u32().unwrap();
        let string_ids_size = dex_cursor.read_u32().unwrap();
        let string_ids_off = dex_cursor.read_u32().unwrap();
        let type_ids_size = dex_cursor.read_u32().unwrap();
        let type_ids_off = dex_cursor.read_u32().unwrap();
        let proto_ids_size = dex_cursor.read_u32().unwrap();
        let proto_ids_off = dex_cursor.read_u32().unwrap();
        let fields_ids_size = dex_cursor.read_u32().unwrap();
        let fields_ids_off = dex_cursor.read_u32().unwrap();
        let method_ids_size = dex_cursor.read_u32().unwrap();
        let method_ids_off = dex_cursor.read_u32().unwrap();
        let class_defs_size = dex_cursor.read_u32().unwrap();
        let class_defs_off = dex_cursor.read_u32().unwrap();
        let data_size = dex_cursor.read_u32().unwrap();
        let data_off = dex_cursor.read_u32().unwrap();

        Ok(DexHeader {
                version,
                checksum,
                signature,
                file_size,
                header_size,
                endian_tag,
                link_size,
                link_off,
                map_off,
                string_ids_size,
                string_ids_off,
                type_ids_size,
                type_ids_off,
                proto_ids_size,
                proto_ids_off,
                fields_ids_size,
                fields_ids_off,
                method_ids_size,
                method_ids_off,
                class_defs_size,
                class_defs_off,
                data_size,
                data_off
        })
    }
}
