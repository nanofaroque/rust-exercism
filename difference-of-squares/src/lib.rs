pub fn square_of_sum(n: u32) -> u32 {
    let mut sumOfSqr:u32=0;
    for i in 1..n+1 {
        sumOfSqr+=i
    }
    return sumOfSqr*sumOfSqr
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sumOfSqr:u32=0;
    for i in 1..n+1 {
        sumOfSqr+=i*i
    }
    return sumOfSqr
}

pub fn difference(n: u32) -> u32 {
    unimplemented!(
        "difference between square of sum of 1...{n} and sum of squares of 1...{n}",
        n = n,
    )
}
