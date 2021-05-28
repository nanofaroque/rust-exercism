pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut final_result =0;
    for (i,num) in factors.iter().enumerate() {
        print!("{}",i);
        print!("{}",num);
        let mut mul=1;
        let mut res=num*mul;
        while res<limit {
            res=mul*num;
            if res<limit{
                final_result +=res;
            }
            mul+=1;
        }
    }
    return final_result;

}
