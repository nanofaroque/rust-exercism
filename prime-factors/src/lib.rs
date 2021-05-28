use std::collections::HashSet;

pub fn factors(n: u64) -> Vec<u64> {
    let factor:u64=2;
    let mut not_prime:HashSet<u64>=HashSet::new();
    let mut result:Vec<u64>=Vec::new();
    while factor<n{
      if !not_prime.contains(&factor){
          if n%factor==0 {
              result.push(factor);
              let mut itr:u64=2;
              while itr*factor<n {
                  not_prime.insert(itr*factor);
                  itr+=1;
              }
          }
      }
    }
    return result;
}
