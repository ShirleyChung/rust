#![crate_type = "lib"]
#![crate_name = "mymathlib"]

pub fn fabonicii(n:i32)->i32{
    if n <=0 {
        0
    }else if n == 1{
        1
    }else{
        fabonicii(n-1) + fabonicii(n-2)
    }
}

pub fn sum(a1:i32, a2:i32, a3:i32)->i32{
    a1 + a2 + a3
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert_eq!(sum(1,2,3), 6);
        assert_eq!(fabonicii(2), 1);
    }
}
