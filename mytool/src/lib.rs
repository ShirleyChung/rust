#![crate_type = "lib"]
#![crate_name = "mytool"]

trait Log{
    fn write_log(log: String);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
    }
}
