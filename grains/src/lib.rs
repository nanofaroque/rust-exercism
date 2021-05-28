pub fn square(s: u32) -> u64 {
    return (s * s) as u64;
}

pub fn total() -> u64 {
    let mut sum:u64=0;
    for i in 1..65 {
        sum+=square(i);
    }
    return sum;
}
