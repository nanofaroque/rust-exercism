
pub fn factors(mut n: u64) -> Vec<u64> {

    let mut factor:u64=2;
    let mut result:Vec<u64>=Vec::new();
    if n==1 {
        return result
    }
    while factor<=n{
      if n%factor==0{
          result.push(factor);
          n=n/factor;
      }else {
          factor+=1;
      }

    }
    if n>1 {result.push(n);}
    return result;
}
