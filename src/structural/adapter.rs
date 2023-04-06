pub trait Adaptee {
    fn specific_request(&self) -> String;
}

pub struct AdapteeImpl {}

impl AdapteeImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Adaptee for AdapteeImpl {
    fn specific_request(&self) -> String {
        String::from("adaptee method")
    }
}

pub trait Target {
    fn request(&self) -> String;
}

pub struct Adapter {
    adaptee: Box<dyn Adaptee>,
}

impl Adapter {
    pub fn new(adaptee: Box<dyn Adaptee>) -> Self {
        Self { adaptee }
    }
}

impl Target for Adapter {
    fn request(&self) -> String {
        self.adaptee.specific_request()
    }
}

#[cfg(test)]
mod test {
    use super::{AdapteeImpl, Adapter, Target};

    #[test]
    fn adapter_test() {
        let adaptee = Box::new(AdapteeImpl::new());
        let adapter = Adapter::new(adaptee);
        assert_eq!("adaptee method", adapter.request())
    }
}
