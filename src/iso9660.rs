/*
 * Sub-header info
 *
 +----------------------+--------------------------------------------------+
 |            | Interleaved file (1 byte)
 |            |   1 if this file is interleaved, or 0 if not
 | byte 1     |
 +--        --+------------------------------------------------------------+
 |            | Channel number (1 byte)
 |            |   The sub-channel in this 'file'. Video, audio and data
 |            |   sectors can be mixed into the same channel or can be
 |            |   on separate channels. Usually used for multiple audio
 |            |   tracks (e.g. 5 different songs in the same 'file', on
 |            |   channels 0, 1, 2, 3 and 4)
 | byte 2     |
 +--        --+------------------------------------------------------------+
 |            | Submode (1 byte)
 |            |   bit 7: eof_marker -- set if this sector is the end
 |            |                        of the 'file'
 |            |   bit 6: real_time  -- always set in PSX STR streams
 |            |   bit 5: form       -- 0 = Form 1 (2048 user data bytes)
 |            |                        1 = Form 2 (2324 user data bytes)
 |            |   bit 4: trigger    -- for use by reader application
 |            |                        (unimportant)
 |            |   bit 3: DATA       -- set to indicate DATA  sector
 |            |   bit 2: AUDIO      -- set to indicate AUDIO sector
 |            |   bit 1: VIDEO      -- set to indicate VIDEO sector
 |            |   bit 0: end_audio  -- end of audio frame
 |            |                        (rarely set in PSX STR streams)
 |            |
 |            |   bits 1, 2 and 3 are mutually exclusive
 | byte 3     |
 +--        --+------------------------------------------------------------+
 |            | Coding info (1 byte)
 |            | If Submode.AUDIO bit is set:
 |            |   bit 7: reserved -- should always be 0
 |            |   bit 6: emphasis -- boost audio volume (ignored by us)
 |            |   bit 5: bitssamp -- must always be 0
 |            |   bit 4: bitssamp -- 0 for mode B/C
 |            |                        (4 bits/sample, 8 sound sectors)
 |            |                      1 for mode A
 |            |                        (8 bits/sample, 4 sound sectors)
 |            |   bit 3: samprate -- must always be 0
 |            |   bit 2: samprate -- 0 for 37.8kHz playback
 |            |                      1 for 18.9kHz playback
 |            |   bit 1: stereo   -- must always be 0
 |            |   bit 0: stereo   -- 0 for mono sound, 1 for stereo sound
 |            |
 |            | If Submode.AUDIO bit is NOT set, this byte can be ignored
 | byte 4     |
 +--        --+------------------------------------------------------------+
 | byte 5-8   | Duplicated
*/

pub const CD_ROOT_DIRECTORY_SECTOR: usize = 22;
pub const CD_SECTOR_SIZE: usize = 2048;
pub const CD_SECTOR_SIZE_MODE2: usize = 2352; // MODE2/2352

pub struct TOC {
    tocEntryLength: u8,
    extEntryLength: u8,
    sectorPosition: [u32; 2],
    fileSize: [u32; 2],
    date: [u8; 7],
    flags: u8,
    fileUnitSize: u8,
    interleaveGapSize: u8,
    volSeqNum: [u16; 2],
    nameLength: u8,
}

pub struct Sector {
    sync: [u8; 12],     // Sync pattern (usually 00 FF FF FF FF FF FF FF FF FF FF 00)
    addr: [u8; 3],      // Sector address (see below for encoding details)
    mode: u8,           // Mode (usually 2 for Mode 2 Form 1/2 sectors)
    subHead: [u8; 8],   // Sub-header (00 00 08 00 00 00 08 00 for Form 1 data sectors). See below for more
    data: [u8; 2048],   // Data (form 1)
    edc: [u8; 4],       // Error-detection code (CRC32 of data area)
    ecc: [u8; 276],     // Error-correction code (uses Reed-Solomon ECC algorithm)
}


pub struct AudioSector {
    sync: [u8; 12],
    addr: [u8; 3],
    mode: u8,
    data: [u8; 2336],
}

