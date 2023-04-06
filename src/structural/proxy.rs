pub trait Subject {
    fn operation(&self) -> String;
}

pub struct RealSubject {}

impl Subject for RealSubject {
    fn operation(&self) -> String {
        "real".to_string()
    }
}

pub struct Proxy {
    real: RealSubject,
}

impl Subject for Proxy {
    fn operation(&self) -> String {
        format!("pre-{}-after", self.real.operation())
    }
}

#[cfg(test)]
mod test {
    use super::{RealSubject, Proxy, Subject};

    #[test]
    fn proxy_test() {
        let proxy = Proxy {
            real: RealSubject {}
        };

        assert_eq!("pre-real-after", proxy.operation())
    }
}