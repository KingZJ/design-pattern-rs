/// 游戏场景中武器系统， 包含武器品质模块、强化材料模块、镶嵌宝石模块
pub trait Weapon {
    fn get_quality(&self) -> Quality;
    fn get_hardening_material(&self) -> HardeningMaterial;
    fn get_gem_set(&self) -> Vec<Gemstone>;
}

/// 武器品质 出现的概率，出现的地方，品质种类
#[derive(Debug, Clone, Default)]
pub struct Quality {
    pub producing_place: String,
    pub quality_type: String,
    pub probability: f32,
}

/// 强化材料 名称、所需数量、获取途径
#[derive(Debug, Clone, Default)]
pub struct HardeningMaterial {
    pub name: String,
    pub number: i32,
    pub acquire_way: String,
}

/// 镶嵌宝石 宝石类型、宝石附带的能力
#[derive(Debug, Clone, Default)]
pub struct Gemstone {
    pub gem_type: String,
    pub ability: String,
}

pub struct SwordWeapon {
    quality: Quality,
    hardening_material: HardeningMaterial,
    gem_set: Vec<Gemstone>,
}

impl SwordWeapon {
    pub fn new(quality: Quality, hardening_material: HardeningMaterial) -> Self {
        Self {
            quality,
            hardening_material,
            gem_set: Vec::new(),
        }
    }
}

impl Weapon for SwordWeapon {
    fn get_quality(&self) -> Quality {
        self.quality.clone()
    }

    fn get_hardening_material(&self) -> HardeningMaterial {
        self.hardening_material.clone()
    }

    fn get_gem_set(&self) -> Vec<Gemstone> {
        self.gem_set.clone()
    }
}

#[cfg(test)]
mod test {
    use super::{HardeningMaterial, Quality, SwordWeapon, Weapon};

    #[test]
    fn sword_test() {
        let quality = Quality {
            producing_place: "暴风峡谷".to_string(),
            quality_type: "传说".to_string(),
            probability: 0.1,
        };
        let hardening_material = HardeningMaterial::default();
        let sword_weapon = SwordWeapon::new(quality, hardening_material);

        assert_eq!("传说", sword_weapon.get_quality().quality_type)
    }
}
