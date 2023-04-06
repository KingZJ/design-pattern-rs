pub trait Weapon: Clone {
    fn quality(&self) -> String;
    fn weapon_type(&self) -> String;
    fn weapon_model(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Dagger {}

impl Weapon for Dagger {
    fn quality(&self) -> String {
        "epic quality".to_string()
    }

    fn weapon_model(&self) -> String {
        "scimitar dagger".to_string()
    }

    fn weapon_type(&self) -> String {
        "dagger".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::{Dagger, Weapon};

    #[test]
    fn prototype_test() {
        let weapon_dagger1 = Dagger {};
        let weapon_dagger2 = weapon_dagger1.clone();

        assert_eq!(false, std::ptr::eq(&weapon_dagger1, &weapon_dagger2));
        assert_eq!(weapon_dagger1.quality(), weapon_dagger2.quality())
    }
}
