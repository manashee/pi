extern crate is_odd;
use is_odd::IsOdd;

fn main() {
    let mut previous_number : f64 = 4.0;
    let mut next_number : f64 ;
    let mut n : u32 = 2;

    loop {
        next_number = next ( n, previous_number );
        // println!("n = {}, previous_number = {} , next_number = {} ", n,previous_number,next_number );
        println!("n = {},  next_number = {} ", n,next_number );

        if  500000 == n  { 
            break;
        } else {
            previous_number = next_number;
            n = n + 1 ;
        }

    }
}



fn next ( n: u32 , previous_number : f64 ) -> f64 { 
    let n1  = n+1;

    if  n.is_odd()  {
        let n1 = n1 as f64;
        let f1 :f64 =  n1 as f64 / n as f64 ; 
        let r = f1 * previous_number;
        // println! (" n = {} , previous_number = {},  n1 = {} , f1 = {} , r = {} " , n , previous_number, n1 , f1, r );
        return r;
    } else {
        let f2 :f64 =  n as f64 / n1 as f64   ; 
        let r = f2 * previous_number;
        // println! (" n = {} , previous_number = {},  n1 = {} , f2 = {} , r = {} " , n , previous_number, n1 , f2, r );
        return r;
    }
}


