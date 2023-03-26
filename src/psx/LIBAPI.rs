static mut DWORD_300: [u8; 4] = [0x20, 0xD, 0x0, 0x0];
static mut DWORD_308: [u8; 4] = [0x10, 0x20, 0x40, 0x1];

pub const CTR_RUNNING: u8 = 0;
pub const CTR_STOPPED: u8 = 1;

pub const CTR_MODE_TO_FFFF: u8 = 0;
pub const CTR_MODE_TO_TARG: u8 = 1;

pub const CTR_CLOCK_SYS: u8 = 0;
pub const CTR_CLOCK_PIXEL: u8 = 1;
pub const CTR_HORIZ_RETRACE: u8 = 1;

pub const CTR_CLOCK_SYS_ONE: u8 = 0;
pub const CTR_CLOCK_SYS_ONE_EIGHTH: u8 = 1;

#[derive(Default)]
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

pub fn set_rcnt(counters: &mut [SysCounter; 3], spec: i64, target: u16, mode: i64) -> i64 // (F)
{
    let mut value: u32 = 0x48;
    let mut _spec: usize = spec as usize & 0xFFFF;

    if spec > 2 {
        return 0;
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

    1
}

pub fn get_rcnt(counters: &mut [SysCounter; 3], spec: i64) -> u16 // (F)
{
    let _spec: usize = spec as usize & 0xFFFF;

    if _spec > 2 {
        return 0;
    }

    counters[_spec].cycle
}

pub fn reset_rcnt(counters: &mut [SysCounter; 3], spec: i64) -> i64 // (F)
{
    let _spec: usize = spec as usize & 0xFFFF;

    if _spec > 2 {
        return 0;
    }

    counters[_spec].cycle = 0;

    return 1;
}

pub fn start_rcnt(spec: i64) -> i64 // (F)
{
    let _spec: usize = spec as usize & 0xFFFF;

    unsafe {
        DWORD_300[1] |= DWORD_308[_spec];
    }

    if spec < 3 {
        1
    } else {
        0
    }
}

pub fn stop_rcnt(spec: i64) -> i64 {
    0
}

pub fn open_event(event: u64, unk01: i64, unk02: i64, func: fn() -> i64) -> i64
{
	0
}

pub fn close_event(event: u64) -> i64
{
	0
}

pub fn enable_event(event: u64) -> i64
{
	0
}

pub fn disable_event(event: u64) -> i64
{
	0
}

pub fn wait_event(event: u64) -> i64
{
	0
}

pub fn test_event(event: u64) -> i64
{
	0
}

pub fn deliver_event(ev1: u64, ev2: i64) -> i64
{
    0
}

pub fn undeliver_event(ev1: u64, ev2: i64) -> i64
{
    0
}

pub fn open_th(func: fn() -> i64, unk01: u64, unk02: u64) -> i64
{
	0
}

pub fn close_th(unk00: i64) -> i64
{
	0
}

pub fn change_th(unk00: i64) -> i64
{
	0
}


// pub fn load_test(unk00: &string, struct EXEC* unk01) -> i64
// {
// 	0
	
// }

// pub fn Load(char * unk00, struct EXEC* unk01)
// {
// 	0
	
// }

// pub fn Exec(struct EXEC * unk00, long unk01, char** unk02)
// {
// 	0
	
// }

// pub fn LoadExec(char * unk00, unsigned long unk01, unsigned long unk02)
// {
// 	0
	
// }

// pub fn InitPAD(char * unk00, long unk01, char* unk02, long unk03)
// {
// 	0
	
// }

// pub fn StartPAD()
// {
// 	0
	
// }

// pub fn StopPAD()
// {
// 	0
// }

// pub fn EnablePAD()
// {
// 	0
// }

// pub fn DisablePAD()
// {
// 	0
// }

// pub fn FlushCache()
// {
// 	0
// }

// pub fn ReturnFromException()
// {
// 	0
// }

// pub fn EnterCriticalSection()
// {
// 	0
	
// }

// pub fn ExitCriticalSection()
// {
// 	0
// }

// pub fn Exception()
// {
// 	0
// }

// pub fn SwEnterCriticalSection()
// {
// 	0
// }
// pub fn SwExitCriticalSection()
// {
// 	0
// }

// pub fn SetSp(unsigned long newsp)//(F)
// {
// 	unsigned long old_sp = sp;
// 	sp = newsp;
// 	return old_sp;
// }

// pub fn GetSp()
// {
// 	0
	
// }

// pub fn GetGp()
// {
// 	0
	
// }

// pub fn GetCr()
// {
// 	0
	
// }

// pub fn GetSr()
// {
// 	0
	
// }

// pub fn GetSysSp()
// {
// 	0
	
// }

// pub fn SetConf(unsigned long unk00, unsigned long unk01, unsigned long unk02)
// {
// 	0
	
// }

// pub fn GetConf(unsigned long* unk00, unsigned long* unk01, unsigned long* unk02)
// {
// 	0
// }

// pub fn SystemError(char unk00, long unk01)
// {
// 	0
// }

// pub fn SetMem(long unk00)
// {
// 	0
// }

// pub fn Krom2RawAdd(unsigned long unk00)
// {
// 	0
	
// }

// pub fn Krom2RawAdd2(unsigned short unk00)
// {
// 	0
	
// }

// pub fn _96_init(void)
// {
// 	0
// }

// pub fn _96_remove(void)
// {
// 	0
// }

// pub fn _boot(void)
// {
// 	0
// }

// pub fn ChangeClearPAD(long unk00)
// {
// 	0
// }

// pub fn InitCARD(long val)
// {
// 	0
// }

// pub fn StartCARD()
// {
// 	0
	
// }

// pub fn StopCARD()
// {
// 	0
	
// }

// pub fn _bu_init()
// {
// 	0
// }

// pub fn _card_info(long chan)
// {
// 	0
	
// }

// pub fn _card_clear(long chan)
// {
// 	0
	
// }

// pub fn _card_load(long chan)
// {
// 	0
	
// }

// pub fn _card_auto(long val)
// {
// 	0
	
// }

// pub fn _new_card()
// {
// 	0
// }

// pub fn _card_status(long drv)
// {
// 	0
	
// }

// pub fn _card_wait(long drv)
// {
// 	0
	
// }

// pub fn _card_chan(void)
// {
// 	0
	
// }

// pub fn _card_write(long chan, long block, unsigned char *buf)
// {
// 	0
	
// }

// pub fn _card_read(long chan, long block, unsigned char *buf)
// {
// 	0
	
// }

// pub fn _card_format(long chan)
// {
// 	0
	
// }