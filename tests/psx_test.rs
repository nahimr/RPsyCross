use psycross::psx::LIBAPI::*;

#[test]
fn counter_test() {
    let mut counters: [SysCounter; 3] = [SysCounter::default(), SysCounter::default(), SysCounter::default()];
    set_rcnt(&mut counters, 1, 1, 1);
}
