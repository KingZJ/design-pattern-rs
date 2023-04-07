/// 古老的游戏武器系统，现想增加皮肤，要求不改变原来的类
pub trait Weapon {
    fn style(&self) -> String;
}

pub struct SwordWeapon {}

impl Weapon for SwordWeapon {
    fn style(&self) -> String {
        String::from("长剑")
    }
}

pub trait SkinDecorator {
    fn style(&self) -> String;

    fn skin(&self) -> String;
}

pub struct SwordSkin {
    component: Box<dyn Weapon>,
}

impl SkinDecorator for SwordSkin {
    fn skin(&self) -> String {
        String::from("紫色皮肤")
    }

    fn style(&self) -> String {
        format!("{} {}", self.skin(), self.component.style())
    }
}

#[cfg(test)]
mod test {
    use super::{SkinDecorator, SwordSkin, SwordWeapon};

    #[test]
    fn decorator_test() {
        // 注意与代理类的区别：装饰器模式客户端使用时，需要关注目标类和装饰器，
        // 客户端清楚装饰器为目标类扩展了何种功能
        let weapon_sword = SwordWeapon {};
        let sword_skin = SwordSkin {
            component: Box::new(weapon_sword),
        };

        assert_eq!("紫色皮肤 长剑", sword_skin.style())
    }
}
