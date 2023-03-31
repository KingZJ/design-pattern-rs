#[derive(PartialEq, Eq, Debug)]
pub enum WeaponType {
    OrcWeapon,
    ElfWeapon,
}

pub trait Weapon {
    fn get_weapon_type(&self) -> WeaponType;
}

pub struct OrcWeapon {}

impl Weapon for OrcWeapon {
    fn get_weapon_type(&self) -> WeaponType {
        WeaponType::OrcWeapon
    }
}

pub struct ElfWeapon {}

impl Weapon for ElfWeapon {
    fn get_weapon_type(&self) -> WeaponType {
        WeaponType::ElfWeapon
    }
}

/// 抽象工厂定义
pub trait Blacksimth {
    fn manufacture_weapon() -> Box<dyn Weapon>;
}

/// 具体工厂子类
pub struct OrcBlacksimth {}

impl Blacksimth for OrcBlacksimth {
    fn manufacture_weapon() -> Box<dyn Weapon> {
        Box::new(OrcWeapon {})
    }
}

pub struct ElfBlacksmith {}

impl Blacksimth for ElfBlacksmith {
    fn manufacture_weapon() -> Box<dyn Weapon> {
        Box::new(ElfWeapon {})
    }
}

#[cfg(test)]
mod test {
    use super::{Blacksimth, ElfBlacksmith, OrcBlacksimth, WeaponType};

    #[test]
    fn manufacture_weapon_test() {
        let orc_weapon = OrcBlacksimth::manufacture_weapon();
        let elf_weapon = ElfBlacksmith::manufacture_weapon();

        assert_eq!(WeaponType::OrcWeapon, orc_weapon.get_weapon_type());
        assert_eq!(WeaponType::ElfWeapon, elf_weapon.get_weapon_type());
    }
}
