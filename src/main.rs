use byteorder::{ByteOrder, LittleEndian};

struct Map {
    // File Header
    pub version:    i32;
    pub magic:      &[u8];
    pub filetype:   u8;
    pub revision:   u32;
    pub favorite:   bool;
    pub pointers_c: i16;
    pub pointers:   &[i32];
    pub importants: i16;

    // Map Header
    pub name:       str;
    pub seed:       str;
    pub genversion: &[u8];
    pub guid:       &[u16];
    pub worldid:    i32;
    pub left:       i32;
    pub right:      i32;
    pub top:        i32;
    pub bottom:     i32;
    pub maxY:       i32;
    pub maxX:       i32;
}

impl Map {
    fn from_bytes(buffer: &[u8]) -> Map {
        Map {
            version:    LittleEndian::read_i32(&buffer[0..4]),
            magic: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            filetype:   LittleEndian::read_u8(&buffer[11..12]),
            revision:   LittleEndian::read_u32(&buffer[12..16]),

        }
    }
}
