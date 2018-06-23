#[cfg(mytools)]

mod mytools {
    #[mytools]
    fn it_works() {
    }

    fn fabonicii(n:i32)->i32{
        if n <= 0 {
            0;
        }
        else if n == 1{
            1;
        }else{
            fabonicii(n-1) + fabonicii(n-2);
        }
    }
}
