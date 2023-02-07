use std::fs::File;
use std::io::{Read, Cursor, Seek, SeekFrom};

pub mod dex_file;
pub mod map_list;
pub mod error;
pub mod endianness;
pub mod adler32;
pub mod constants;
use crate::endianness::DexCursor;
use crate::dex_file::DexHeader;
use crate::map_list::MapList;

fn main() {
    // TODO: CLI arg
    let fpath = "classes.dex";
    println!("[+] loading file: {fpath}");
    let mut file = File::open(fpath)
        .unwrap_or_else(|err| panic!("Could not open input file: {err}"));

    let mut raw_dex = Vec::new();
    file.read_to_end(&mut raw_dex)
        .unwrap_or_else(|err| panic!("Could not read input file: {err}"));

    /* First check endianness */
    let mut bytes = Cursor::new(&raw_dex);
    let bytes_len = bytes.seek(SeekFrom::End(0)).unwrap();
    bytes.rewind().unwrap();
    let endianness = DexCursor::check_endianness(&raw_dex).unwrap();
    let mut dex_cursor = DexCursor {
        bytes,
        bytes_len,
        endianness
    };

    let dex_header = DexHeader::new(&mut dex_cursor).unwrap();
    println!("{dex_header:#?}");

    let map_list = MapList::build(&mut dex_cursor, dex_header.map_off);
    println!("{map_list:#?}");
}