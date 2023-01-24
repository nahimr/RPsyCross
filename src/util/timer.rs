use libc::timeval;

#[cfg(target_os = "windows")]
struct TimerCtxT {
    clock_start: u64,
}

#[cfg(not(target_os = "windows"))]
struct TimerCtxT {
    clock_start: timeval,
}

#[cfg(target_os = "windows")]
fn util_init_hpctimer(mut timer: TimerCtxT) 
{
    use windows::Win32::System::Performance::QueryPerformanceCounter;
    QueryPerformanceCounter(&mut timer.clock_start);
}

#[cfg(not(target_os = "windows"))]
fn util_init_hpctimer(mut timer: TimerCtxT) 
{
    use std::ptr;
    use libc::gettimeofday;

    unsafe {
        gettimeofday(&mut timer.clock_start, ptr::null_mut());
    }
    
}
