pub fn build_proverb(list: &[&str]) -> String {
    //unimplemented!("build a proverb from this list of items: {:?}", list)
    //For want of a nail the shoe was lost
    let mut res=String::from("");
    let len:usize=list.len();
    for i in 0..len-1 {
        let start:String=list.get(i).unwrap().to_string();
        let end:String=list.get(i+1).unwrap().to_string();
        list.get(i+1);
        res.push_str(&*format!("For want of a {} the {} was lost.\n", start, end))
    }
    res.push_str("And all for the want of a nail.");
    return res
}
