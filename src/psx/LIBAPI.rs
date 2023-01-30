const DWORD_300: [u8; 4] = [0x20, 0xD, 0x0, 0x0];
const DWORD_308: [u8; 4] = [0x10, 0x20, 0x40, 0x1];

const CTR_RUNNING: u8 = 0;
const CTR_STOPPED: u8 = 1;

const CTR_MODE_TO_FFFF: u8 = 0;
const CTR_MODE_TO_TARG: u8 = 1;

const CTR_CLOCK_SYS: u8 = 0;
const CTR_CLOCK_PIXEL: u8 = 1;
const CTR_HORIZ_RETRACE: u8 = 1;

const CTR_CLOCK_SYS_ONE: u8 = 0;
const CTR_CLOCK_SYS_ONE_EIGHTH: u8 = 1;

pub struct SysCounter {
    i_cycle: u32,
    cycle: u16,

    i_value: u32,
    value: u16,

    i_target: u32,
    target: u16,

    padding00: u32,
    padding01: u32,
}

pub fn set_rcnt(counters: &mut [SysCounter; 3], spec: i64, target: u16, mode: i64) -> bool // (F)
{
    let mut value: u32 = 0x48;
    let mut _spec: usize = spec as usize & 0xFFFF;

    if spec > 2 {
        return false;
    }

    counters[_spec].value = 0;
    counters[_spec].target = target;

    if _spec < 2 {
        if (mode & 0x10) > 0 {
            value = 0x49;
        } else if (mode & 0x1) > 0
        // loc_148
        {
            value |= 0x100;
        }
    } else {
        // loc_158
        if _spec == 2 && (mode & 1) < 0 {
            value = 0x248;
        } // loc_174
    }
    // loc_174
    if mode & 0x1000 > 0 {
        value |= 0x10;
    } // loc_180

    counters[_spec].value = value as u16;

    return true;
}

fn get_rcnt(counters: &mut [SysCounter; 3], spec: i64) -> u16 // (F)
{
    let _spec: usize = spec as usize & 0xFFFF;

    if _spec > 2 {
        return 0;
    }

    counters[_spec].cycle
}

fn reset_rcnt(counters: &mut [SysCounter; 3], spec: i64) -> bool // (F)
{
    let _spec = spec as usize & 0xFFFF;

    if _spec > 2 {
        return false;
    }

    counters[_spec].cycle = 0;

    return true;
}

fn start_rcnt(spec: i64) -> bool // (F)
{
    let _spec: usize = spec as usize & 0xFFFF;

    // TODO: Need mutable
    DWORD_300[1] |= DWORD_308[_spec];

    if spec < 3 {
        true
    } else {
        false
    }
}

// TODO: Implement method
fn stop_rcnt(spec: i64) -> bool {
    return false;
}
