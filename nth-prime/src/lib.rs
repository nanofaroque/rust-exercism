pub fn nth(n: u32) -> u32 {
    let mut num: u32 = 1;
    let mut count: u32 = 0;
    while count < n {
        num += 1;
        for i in 2..num {
            if num % i == 0 {
                break;
            }
        }
        if i == num {//if it is a prime number
            count = count + 1;
        }
    }
    return num;
}
