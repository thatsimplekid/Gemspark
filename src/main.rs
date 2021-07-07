use nom::*;
use nom::number::complete::*;
use nom::bytes::complete::*;
#[allow(unused_imports)]
use nom::multi::count;

#[derive(Debug, PartialEq)]
struct Map<'a> {
    // File Header
    pub version:    i32,            // The game version that the map is for
    pub magic:      &'a str,        // Always 'relogic'
    pub filetype:   u8,             // Filetype is '2' for worlds
    // Skip 12 bytes for favorite/revision data
    pub ptrcount:   i16,            // How many pointers are held below?
    pub pointers:   Vec<i32>,       // Pointers to specific parts of this file
    pub tmkcount:   i16,            // How many tiles in the tilemask?
    // Skip the tilemask (for now)

    // Map Header
    pub name:       &'a str,        // The name of the map as shown in game
    //pub seed:       str,
    //pub genversion: &[u8],
    //pub guid:       &[u16],
    //pub worldid:    i32,
    //pub left:       i32,
    //pub right:      i32,
    //pub top:        i32,
    //pub bottom:     i32,
    //pub maxY:       i32,
    //pub maxX:       i32,
}

fn parse_data(i: &[u8]) -> IResult<&[u8], Map> {
    let (i, version)        = le_i32(i)?;
    let (i, magic)          = take(7usize)(i)?;
    let (i, filetype)       = le_u8(i)?;
    let (i, _)              = take(12usize)(i)?;
    let (i, ptrcount)       = le_i16(i)?;
    let (i, pointers)       = count(le_i32, ptrcount as usize)(i)?;
    let (i, tmkcount)       = le_i16(i)?;

    let (i, _)              = take((tmkcount + 7 & !7) as usize)(i)?;
    let (i, _)              = take(23usize)(i)?;

    let (i, _namelen)       = le_u8(i)?;
    println!("{}", _namelen);
    let (i, name)           = take(_namelen as usize)(i)?;
    Ok((i, Map {
        version,
        magic: std::str::from_utf8(magic).unwrap(),
        filetype,
        ptrcount,
        pointers,
        tmkcount,
        name: std::str::from_utf8(name).unwrap()
    }))
}

fn parse(path: &str) {
    let file = std::fs::read(path).unwrap();
    let input: &[u8] = &file;
    let (_remaining, result) = parse_data(input).unwrap();
    println!("{:?}", result);
}

fn main() {
    parse("./tsow.wld");
}
