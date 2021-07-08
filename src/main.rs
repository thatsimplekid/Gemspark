use nom::*;
use nom::number::complete::*;
use nom::bytes::complete::*;
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
    pub seed:       &'a str,        // Numerical RNG seed
    pub genversion: i64,            // Generator version - idk what this is tbh
    pub guid:       &'a [u8],       // I want this to be a hex string but memory safety
                                    // and strings are making me sad :(
    pub worldid:    i32,            // ?
    pub left:       i32,            // Map bounds L
    pub right:      i32,            // Map bounds R
    pub top:        i32,            // Map bounds T
    pub bottom:     i32,            // Map bounds B
    pub sizey:      i32,            // Map size vertical
    pub sizex:      i32,            // Map size horizontal
    pub gamemode:   i32,            // World's gamemode (209+)
    pub isdrunk:    bool,           // Is this a 'Drunk' world?
    pub isgetgood:  bool,           // Is this a 'Worthy' world?
    pub created:    u64,            // Timestamp of the world creation
    pub moontype:   u8,             // Moon type
    pub treetypex:  Vec<i32>,
    pub treestyles: Vec<i32>,
    pub cavebackx:  Vec<i32>,
    pub cavestyles: Vec<i32>,
    pub iceback:    i32,
    pub jungleback: i32,
    pub hellback:   i32,
    pub spawnx:     i32,
    pub spawny:     i32,
    pub surfacey:   &'a [u8],
    pub rocklayery: &'a [u8],
    pub gametime:   &'a [u8]
}

fn parse_data<'a>(i: &[u8]) -> IResult<&[u8], Map> {
    //let versions: HashMap<i32, &str> = HashMap::new();
    let (i, version)        = le_i32(i)?;
    let (i, magic)          = take(7usize)(i)?;
    if magic != b"\x72\x65\x6C\x6F\x67\x69\x63" {
        panic!("Invalid world file")
    }
    let (i, filetype)       = le_u8(i)?;
    let (i, _)              = take(12usize)(i)?;
    let (i, ptrcount)       = le_i16(i)?; //26
    let (i, pointers)       = count(le_i32, ptrcount as usize)(i)?;
    //let (i, tmkcount)       = le_i16(i)?;
    //let (i, _)              = take((tmkcount + 7 & !7) as usize)(i)?;
    let headerpos: i32 = pointers.clone().into_iter().nth(0).unwrap(); // The offset of the world header
    let currentpos: i32 = 4 + 7 + 1 + 12 + 2 + (4 * (ptrcount as i32)); // The offset we're at so far
    let difference: usize = (headerpos as i32 - currentpos) as usize;
    let (i, _)              = take(difference)(i)?; // Skip ahead to the world header - nothing important.
    let (i, _namelen)       = le_u8(i)?;
    let (i, name)           = take(_namelen as usize)(i)?;
    let (i, _seedlen)       = le_u8(i)?;
    let (i, seed)           = take(_seedlen as usize)(i)?;
    let (i, genversion)     = le_i64(i)?;
    let (i, guid)           = take(16usize)(i)?;
    let (i, worldid)        = le_i32(i)?;
    let (i, left)           = le_i32(i)?;
    let (i, right)          = le_i32(i)?;
    let (i, top)            = le_i32(i)?;
    let (i, bottom)         = le_i32(i)?;
    let (i, sizey)          = le_i32(i)?;
    let (i, sizex)          = le_i32(i)?;
    let (i, gamemode)       = le_i32(i)?;
    let (i, isdrunk)        = le_u8(i)?;
    let (i, isgetgood)      = le_u8(i)?;
    let (i, created)        = le_u64(i)?;
    let (i, moontype)       = le_u8(i)?;
    let (i, treetypex)      = count(le_i32, 3usize)(i)?;
    let (i, treestyles)     = count(le_i32, 4usize)(i)?;
    let (i, cavebackx)      = count(le_i32, 3usize)(i)?;
    let (i, cavestyles)     = count(le_i32, 4usize)(i)?;
    let (i, iceback)        = le_i32(i)?;
    let (i, jungleback)     = le_i32(i)?;
    let (i, hellback)       = le_i32(i)?;
    let (i, spawnx)         = le_i32(i)?;
    let (i, spawny)         = le_i32(i)?;
    let (i, surfacey)       = take(8usize)(i)?;
    let (i, rocklayery)     = take(8usize)(i)?;
    let (i, gametime)       = take(8usize)(i)?;
    Ok((i, Map {
        version,
        magic: std::str::from_utf8(magic).unwrap(),
        filetype,
        ptrcount,
        pointers,
        tmkcount: 10,
        name: std::str::from_utf8(name).unwrap(),
        seed: std::str::from_utf8(seed).unwrap(),
        genversion,
        guid,
        worldid,
        left,
        right,
        top,
        bottom,
        sizey,
        sizex,
        gamemode,
        isdrunk: isdrunk == 1,
        isgetgood: isgetgood == 1,
        created,
        moontype,
        treetypex,
        treestyles,
        cavebackx,
        cavestyles,
        iceback,
        jungleback,
        hellback,
        spawnx,
        spawny,
        surfacey,
        rocklayery,
        gametime
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
