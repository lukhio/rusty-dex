use std::io::{Seek, SeekFrom};

use crate::dex_reader::DexReader;
use crate::dex_strings::DexStrings;

#[derive(Debug)]
pub struct DexTypeItem {
    offset: u32,
    pub str_type: String,
}

#[derive(Debug)]
pub struct DexTypes{
    pub items: Vec<String>
}

impl DexTypes {
    pub fn build(dex_reader: &mut DexReader,
                 offset: u32,
                 size: u32,
                 strings_list: &DexStrings) -> Self {
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        let mut types = Vec::new();

        for _ in 0..size {
            let offset = dex_reader.read_u32().unwrap();
            types.push(DexTypeItem {
                offset,
                str_type: strings_list.strings[offset as usize].string.clone()
            });
        }
        types.sort_by(|a, b| a.offset.cmp(&b.offset));

        let mut items = Vec::new();
        for dex_type in types.iter() {
            items.push(dex_type.str_type.clone());
        }
        items.dedup();

        DexTypes { items }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dex_strings::DexStringsItem;

    #[test]
    fn test_build_dex_types_empty() {
        let dex_data = vec![
            0x64, 0x65, 0x78, 0x0a, 0x30, 0x33, 0x35, 0x00, 0x00, 0x00,  // DEX magic
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // endianness tag
        ];

        let mut dex_reader = DexReader::build(dex_data);
        let strings_list = DexStrings { strings: Vec::new() };

        let dex_types = DexTypes::build(&mut dex_reader, 0, 0, &strings_list);

        assert_eq!(dex_types.items.len(), 0);
    }

    #[test]
    fn test_build_dex_types() {
        let dex_data = vec![
            0x64, 0x65, 0x78, 0x0a, 0x30, 0x33, 0x35, 0x00, 0x00, 0x00,  // DEX magic
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // endianness tag
            0x00, 0x00, 0x00, 0x00,     // type 0 offset
            0x01, 0x00, 0x00, 0x00,     // type 1 offset
            0x02, 0x00, 0x00, 0x00,     // type 2 offset
            0x03, 0x00, 0x00, 0x00,     // type 3 offset
        ];

        let mut dex_reader = DexReader::build(dex_data);
        let strings_list = DexStrings { strings: vec![
            DexStringsItem {
                utf16_size: 5,
                offset: 0,
                is_raw: false,
                string: "Type0".to_string()
            },
            DexStringsItem {
                utf16_size: 5,
                offset: 1,
                is_raw: false,
                string: "Type1".to_string()
            },
            DexStringsItem {
                utf16_size: 5,
                offset: 2,
                is_raw: false,
                string: "Type2".to_string()
            },
            DexStringsItem {
                utf16_size: 5,
                offset: 3,
                is_raw: false,
                string: "Type3".to_string()
            }]
        };

        let dex_types = DexTypes::build(&mut dex_reader, 50, 4, &strings_list);

        assert_eq!(dex_types.items.len(), 4);
        assert_eq!(dex_types.items[0], "Type0");
        assert_eq!(dex_types.items[1], "Type1");
        assert_eq!(dex_types.items[2], "Type2");
        assert_eq!(dex_types.items[3], "Type3");
    }

    #[test]
    fn test_build_dex_types_duplicates() {
        let dex_data = vec![
            0x64, 0x65, 0x78, 0x0a, 0x30, 0x33, 0x35, 0x00, 0x00, 0x00,  // DEX magic
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // endianness tag
            0x00, 0x00, 0x00, 0x00,     // type 0 offset
            0x01, 0x00, 0x00, 0x00,     // type 1 offset
            0x02, 0x00, 0x00, 0x00,     // type 1 duplicate offset
        ];

        let mut dex_reader = DexReader::build(dex_data);
        let strings_list = DexStrings { strings: vec![
            DexStringsItem {
                utf16_size: 5,
                offset: 0,
                is_raw: false,
                string: "Type0".to_string()
            },
            DexStringsItem {
                utf16_size: 5,
                offset: 1,
                is_raw: false,
                string: "Type1".to_string()
            },
            DexStringsItem {
                utf16_size: 5,
                offset: 2,
                is_raw: false,
                string: "Type1".to_string()
            }]
        };

        let dex_types = DexTypes::build(&mut dex_reader, 50, 2, &strings_list);

        assert_eq!(dex_types.items.len(), 2);
        assert_eq!(dex_types.items[0], "Type0");
        assert_eq!(dex_types.items[1], "Type1");
    }
}
