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
    use super::{Proxy, RealSubject, Subject};

    #[test]
    fn proxy_test() {
        // 注意与适配器模式的区别，代理模式客户端使用时只关注代理类
        // 代理类中有功能增强也认为是目标类提供的
        let proxy = Proxy {
            real: RealSubject {},
        };

        assert_eq!("pre-real-after", proxy.operation());
    }
}
