pub fn is_armstrong_number(num: u32) -> bool {
    //unimplemented!("true if {} is an armstrong number", num)
    let digits: Vec<_> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    print!("{}",digits.len());
    let len: usize =digits.len();
    let mut squared_sum:u32=0;
    for n in digits{
        squared_sum+=n.pow(len as u32)
    }
    return num==squared_sum;
}
