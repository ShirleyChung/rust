pub fn fabonicii(n:i32)->i32{
    if n <=0 {
        0
    }else if n == 1{
        1
    }else{
        fabonicii(n-1) + fabonicii(n-2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(fabonicii(0), 0);
        assert_eq!(fabonicii(1), 1);
        assert_eq!(fabonicii(2), 1);
        assert_eq!(fabonicii(3), 2);
        assert_eq!(fabonicii(4), 3);
        assert_eq!(fabonicii(5), 5);
        assert_eq!(fabonicii(6), 8);
    }
}
