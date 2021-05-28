pub fn raindrops(n: u32) -> String {
    //unimplemented!("what sound does Raindrop #{} make?", n)
    let mut result = String::from("");
    if n%3==0{
        result.push_str("Pling")
    }
    if n%5==0{
        result.push_str("Plang")
    }
    if n%7==0{
        result.push_str("Plong")
    }
    if result.len()==0 {
        result.push_str(&n.to_string())
    }
    return result
}
