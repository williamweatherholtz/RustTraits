/*
Example Code

fn consec_kprimes(k: i32, arr: Vec<i32>) -> i32 {
    println!("{}, {:?}", k, arr);
    let k = k as u32; // gotta correct people's code these days
    let mut k_primes = Vec::new();
    
    let mut last_was_kprime = false;
    
    for val in arr.iter() {
        let num_primes = val.get_factors().iter()
                            .filter(|f| f.is_prime())
                            .count() as u32;
        println!("{}: {} primes", val, num_primes);
        if num_primes == k  && last_was_kprime {
            k_primes.push(val);
            //was_kprime = false;
        }
        
        if num_primes == k {
            println!("{} is kprime", val);
            last_was_kprime = true;
        } else {
            last_was_kprime = false;
        }
        println!("{}, {:?}", val, k_primes);
    }
    println!();
    k_primes.len() as i32
}

*/

trait Factors {
    fn get_factors(&self) -> Vec<i32>;
}

impl Factors for i32 {
    fn get_factors(&self) -> Vec<i32> {
        let mut value = self.clone();
        let mut factor_list = Vec::new();
        
        for factor in 2..value+1 {
            while value % factor == 0 {
                factor_list.push(factor);
                value /= factor;
            }
        }
        factor_list
    }
}
trait IsPrime {
    fn is_prime(&self) -> bool;
}

impl IsPrime for i32 {
    fn is_prime(&self) -> bool {
        if *self < 2 {
            return false;
        } else if *self == 2 {
            return true;
        } else if *self % 2 == 0 {
            return false;
        }
            
        for divisor in 3..*self { //.step_by(2)
            if *self % divisor == 0 { return false; }
        }
        return true
    }
}

