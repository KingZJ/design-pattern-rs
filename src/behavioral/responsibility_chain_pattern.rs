//! 注意与管道模式 pipeline 的区别
//! pipeline 侧重在链上的处理对象依次执行
//! 职责链模式 侧重匹配链上的其中一个对象进行执行
pub trait Manager {
    fn have_right(&self, money: usize) -> bool;
    fn handle_fee_req(&self, role: &str, money: usize) -> bool;
}

#[derive(Default)]
pub struct ProjectManager {}

impl Manager for ProjectManager {
    fn have_right(&self, money: usize) -> bool {
        money < 500
    }

    fn handle_fee_req(&self, role: &str, money: usize) -> bool {
        if role.to_lowercase() == "project manager" {
            println!("{role} is permited to approve {money} fee request");
            true
        } else {
            println!("{role} isn't permited to approve {money} fee request");
            false
        }
    }
}

#[derive(Default)]
pub struct DepManager {}

impl Manager for DepManager {
    fn have_right(&self, money: usize) -> bool {
        money < 5000
    }

    fn handle_fee_req(&self, role: &str, money: usize) -> bool {
        if role.to_lowercase() == "department manager" {
            println!("{role} is permited to approve {money} fee request");
            true
        } else {
            println!("{role} isn't permited to approve {money} fee request");
            false
        }
    }
}

#[derive(Default)]
pub struct TrusteeManager {}

impl Manager for TrusteeManager {
    fn have_right(&self, _money: usize) -> bool {
        true
    }

    fn handle_fee_req(&self, role: &str, money: usize) -> bool {
        if role.to_lowercase() == "trustee" {
            println!("{role} is permited to approve {money} fee request");
            true
        } else {
            println!("{role} isn't permited to approve {money} fee request");
            false
        }
    }
}

pub struct RequestChain {
    manager: Box<dyn Manager>,
    successor: Option<Box<RequestChain>>,
}

impl RequestChain {
    pub fn with_manager(manager: Box<dyn Manager>) -> Self {
        Self {
            manager,
            successor: None,
        }
    }

    pub fn handle_fee_req(&self, role: &str, money: usize) -> bool {
        if self.manager.have_right(money) {
            self.manager.handle_fee_req(role, money)
        } else {
            self.successor
                .as_ref()
                .map(|f| f.handle_fee_req(role, money))
                .unwrap_or(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut c1 = RequestChain::with_manager(Box::new(ProjectManager::default()));
        let mut c2 = RequestChain::with_manager(Box::new(DepManager::default()));
        let c3 = RequestChain::with_manager(Box::new(TrusteeManager::default()));
        c2.successor = Some(Box::new(c3));
        c1.successor = Some(Box::new(c2));

        let c = Box::new(c1);
        assert_eq!(true, c.handle_fee_req("project manager", 100));
        assert_eq!(true, c.handle_fee_req("department manager", 2500));
        assert_eq!(true, c.handle_fee_req("trustee", 12000));
        assert_eq!(false, c.handle_fee_req("captain", 400));
        // Output:
        // project manager is permited to approve 100 fee request
        // department manager is permited to approve 2500 fee request
        // trustee is permited to approve 12000 fee request
        // captain isn't permited to approve 400 fee request
    }
}
