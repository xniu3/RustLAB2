use primes::is_prime;

pub(crate) fn findprimelist(n:i32) -> Vec<i32>{
    let mut vec = Vec::new();
    let mut num = 1;
    while num < n{
        //println!("{}", num);
        if is_prime(num as u64){
            vec.push(num);
        }
        num += 1;
    }
    return vec;
}