#[cfg(not(target_os = "windows"))]
use libc::timeval;

#[cfg(target_os = "windows")]
pub struct TimerCtx {
    clock_start: u64,
}

#[cfg(not(target_os = "windows"))]
pub struct TimerCtx {
    time_start: timeval,
}

#[cfg(target_os = "windows")]
pub fn util_init_hpctimer(mut timer: TimerCtx) 
{
    use windows::Win32::System::Performance::QueryPerformanceCounter;
    QueryPerformanceCounter(&mut timer.clock_start);
}

#[cfg(not(target_os = "windows"))]
pub fn util_init_hpctimer(mut timer: TimerCtx) 
{
    use std::ptr;
    use libc::gettimeofday;

    unsafe {
        gettimeofday(&mut timer.time_start, ptr::null_mut());
    }
    
}

#[cfg(not(target_os = "windows"))]
pub unsafe fn util_get_hcptimer(mut timer: TimerCtx, reset: bool) -> f64
{
    use std::ptr;
    use libc::gettimeofday;

    let mut curr: timeval = timeval { tv_sec: 0, tv_usec: 0 };

    unsafe {
        gettimeofday(&mut curr, ptr::null_mut());
    }

    let value: f64 = ((curr.tv_sec - timer.time_start.tv_sec) as f64) + ((curr.tv_usec - timer.time_start.tv_usec) as f64 * 0.000001);

    if reset {
        timer.time_start = curr;
    }

    return value;
}

#[cfg(target_os = "windows")]
pub fn util_get_hcptimer(mut timer: TimerCtx, reset: bool) -> f64
{
    use windows::Win32::System::Performance::QueryPerformanceCounter;
    use windows::Win32::System::Performance::QueryPerformanceFrequency;

    let mut curr: i64;
    let mut performanceFrequency: i64;

    QueryPerformanceCounter(performanceFrequency);
    QueryPerformanceCounter(curr);


    let value: f64 = ((curr.QuadPart - timer.clock_start) as f64) / performanceFrequency as f64;

    if reset {
        timer.clock_start = curr.QuadPart;
    }

    return value;
}
