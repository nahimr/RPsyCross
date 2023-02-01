use psycross::psx::LIBAPI::*;

#[test]
fn set_rcnt_spec1() {
    let mut counters: [SysCounter; 3] = [SysCounter::default(), SysCounter::default(), SysCounter::default()];
    assert_eq!(true, set_rcnt(&mut counters, 1, 0, 0));    
}

#[test]
fn set_rcnt_spec2() {
    let mut counters: [SysCounter; 3] = [SysCounter::default(), SysCounter::default(), SysCounter::default()];
    assert_eq!(true, set_rcnt(&mut counters, 2, 0, 0));    
}

#[test]
fn set_rcnt_spec3() {
    let mut counters: [SysCounter; 3] = [SysCounter::default(), SysCounter::default(), SysCounter::default()];
    assert_eq!(false, set_rcnt(&mut counters, 3, 0, 0));    
}
