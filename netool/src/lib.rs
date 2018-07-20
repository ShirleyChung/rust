use std::net;

trait Mes{
    fn write(&self)->u64;
    fn read(&self)->u64;
}

pub struct TCPMes{
    hostname: String,
}

impl Mes for TCPMes{
    fn write(&self)->u64{
        0
    }
    fn read(&self)->u64{
        0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
