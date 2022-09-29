use primes::is_prime;
use std::vec::Vec;
mod function;

fn main() {
    let vec= function::findprimelist(100);
    let mut prefix:Vec<i32> = Vec::new();
    let mut total = 0;
    for (_ix, n) in vec.iter().enumerate() {
        //println!("Prime {}: {}", ix, n);
        total += n;
        prefix.push(total);
    }
    let mut small = 0;
    let mut big ;
    let mut maxlen = 0;
    let mut maxbig= 0;
    let mut maxsmall = 0;
    let mut periodsum;
    println!("prefix length is {}",prefix.len());
    while small < prefix.len() - 1{
        big = prefix.len() - 1;
        while big > small{
            periodsum = prefix[big] - prefix[small];
            
            //println!("period sum is {}",periodsum);
            if is_prime(periodsum as u64){
                // println!("prime period sum is {}",periodsum);
                if maxlen < big - small{
                    maxlen = big - small;
                    maxbig = big;
                    maxsmall = small;
                    println!("maxlength is {},{},{}",maxlen, big,small);
                    println!("periodsum is {}",periodsum);
                }
            }
            big -= 1;
        }
        small += 1;
    }
    println!("maxlen is {}",maxlen);
    println!("maxsmall , maxbig is {}, {}",maxsmall,maxbig);
    let mut count = 0;
    for i in &vec {
        if (count > maxsmall && count <= maxbig){
            println!("{}", i);
        }
        count += 1;
    }
    
}
