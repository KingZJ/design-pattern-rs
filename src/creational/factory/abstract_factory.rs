pub trait PersonModel {
    fn ui(&self) -> String;
}

pub trait PrimaryWeapon {
    fn name(&self) -> String;
}

pub trait SpecialSkill {
    fn style(&self) -> String;
}

pub struct SwordsmanPersonModel {}

impl PersonModel for SwordsmanPersonModel {
    fn ui(&self) -> String {
        "swordsman ui".to_string()
    }
}

pub struct SwordsmanPrimaryWeapon {}

impl PrimaryWeapon for SwordsmanPrimaryWeapon {
    fn name(&self) -> String {
        "swordsman".to_string()
    }
}

pub struct SwordsmanSpecialSkill {}

impl SpecialSkill for SwordsmanSpecialSkill {
    fn style(&self) -> String {
        "swordsman skill style".to_string()
    }
}

pub struct AssassinPersonModel {}

impl PersonModel for AssassinPersonModel {
    fn ui(&self) -> String {
        "assassin ui".to_string()
    }
}

pub struct AssassinPrimaryWeapon {}

impl PrimaryWeapon for AssassinPrimaryWeapon {
    fn name(&self) -> String {
        "assassin".to_string()
    }
}

pub struct AssassinSpecialSkill {}

impl SpecialSkill for AssassinSpecialSkill {
    fn style(&self) -> String {
        "assassin skill style".to_string()
    }
}

pub trait Sect {
    fn get_person_model(&self) -> Box<dyn PersonModel>;
    fn get_prinmary_weapon(&self) -> Box<dyn PrimaryWeapon>;
    fn get_special_skill(&self) -> Box<dyn SpecialSkill>;
}

pub struct SwordsmanSect {}

impl Sect for SwordsmanSect {
    fn get_person_model(&self) -> Box<dyn PersonModel> {
        Box::new(SwordsmanPersonModel {})
    }

    fn get_prinmary_weapon(&self) -> Box<dyn PrimaryWeapon> {
        Box::new(SwordsmanPrimaryWeapon {})
    }

    fn get_special_skill(&self) -> Box<dyn SpecialSkill> {
        Box::new(SwordsmanSpecialSkill {})
    }
}

pub struct AssassinSect {}

impl Sect for AssassinSect {
    fn get_person_model(&self) -> Box<dyn PersonModel> {
        Box::new(AssassinPersonModel {})
    }

    fn get_prinmary_weapon(&self) -> Box<dyn PrimaryWeapon> {
        Box::new(AssassinPrimaryWeapon {})
    }

    fn get_special_skill(&self) -> Box<dyn SpecialSkill> {
        Box::new(AssassinSpecialSkill {})
    }
}

#[cfg(test)]
mod test {
    use super::{AssassinSect, Sect, SwordsmanSect};

    #[test]
    fn abstract_factory_test() {
        let swordsman_sect = SwordsmanSect {};
        let assassin_sect = AssassinSect {};

        assert_eq!("swordsman ui", swordsman_sect.get_person_model().ui());
        assert_eq!("swordsman", swordsman_sect.get_prinmary_weapon().name());
        assert_eq!(
            "swordsman skill style",
            swordsman_sect.get_special_skill().style()
        );

        assert_eq!("assassin ui", assassin_sect.get_person_model().ui());
        assert_eq!("assassin", assassin_sect.get_prinmary_weapon().name());
        assert_eq!(
            "assassin skill style",
            assassin_sect.get_special_skill().style()
        );
    }
}
