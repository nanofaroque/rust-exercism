pub fn verse(n: u32) -> String {
    let mut res=String::new();
    if n==0{
        res.push_str(&*format!("No more bottles of beer on the wall, no more bottles of beer.\n"));
        res.push_str(&*format!("Go to the store and buy some more, 99 bottles of beer on the wall.\n"));
        return res;
    }
    if n==1{
        res.push_str(&*format!("1 bottle of beer on the wall, 1 bottle of beer.\n"));
        res.push_str(&*format!("Take it down and pass it around, no more bottles of beer on the wall.\n"));
        return res;
    }
    res.push_str(&*format!("{} bottles of beer on the wall, {} bottles of beer.\n", n, n));
    res.push_str(&*format!("Take one down and pass it around, {} bottles of beer on the wall.\n", n-1));

    return res
}

pub fn sing(mut start: u32, end: u32) -> String {
    let mut res=String::new();
    while start<=end {
        res.push_str(&*verse(start));
        start=start-1;
    }
    return res
}
