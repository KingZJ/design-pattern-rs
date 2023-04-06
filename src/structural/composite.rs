/// 游戏场景中地图系统， 包含小地图和传送门，传送门之后的地图中可能还存在传送门和小地图
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

type Link = Rc<RefCell<dyn Map>>;
type WeakLink = Weak<RefCell<dyn Map>>;
pub trait Map {
    fn get_coordinate(&self) -> (f32, f32);
    fn get_route(&self, x: f32, y: f32) -> String;
    fn get_back_portal(&self) -> Option<WeakLink>;
    fn get_link_map(&self) -> Vec<Link>;
    fn add_link_map(&mut self, link_map: Link);
    fn is_contains(&self, x: f32, y: f32) -> bool;
}

pub struct SwordsmanMap {
    pub back_portal: WeakLink,
    origin_x: f32,
    origin_y: f32,
    width: f32,
    height: f32,
}

impl SwordsmanMap {
    pub fn new_link(
        origin_x: f32,
        origin_y: f32,
        width: f32,
        height: f32,
        back_portal: WeakLink,
    ) -> Link {
        Rc::new(RefCell::new(Self {
            back_portal,
            origin_x,
            origin_y,
            width,
            height,
        }))
    }
}

impl Map for SwordsmanMap {
    fn get_coordinate(&self) -> (f32, f32) {
        (self.origin_x, self.origin_y)
    }

    fn get_route(&self, x: f32, y: f32) -> String {
        format!("({}, {}) -> ({x}, {y})", self.origin_x, self.origin_y)
    }

    fn get_back_portal(&self) -> Option<WeakLink> {
        Some(self.back_portal.clone())
    }

    fn get_link_map(&self) -> Vec<Link> {
        vec![]
    }

    fn add_link_map(&mut self, _link_map: Link) {
        panic!("没有子地图")
    }

    fn is_contains(&self, x: f32, y: f32) -> bool {
        x >= self.origin_x
            && y >= self.origin_y
            && x <= self.origin_x + self.width
            && y <= self.origin_y + self.height
    }
}

pub struct Portal {
    back_portal: Option<WeakLink>,
    link_map: Vec<Link>,
    x: f32,
    y: f32,
}

impl Portal {
    pub fn new_link(x: f32, y: f32, back_portal: Option<WeakLink>) -> Link {
        Rc::new(RefCell::new(Self {
            back_portal,
            link_map: vec![],
            x,
            y,
        }))
    }
}

impl Map for Portal {
    fn get_coordinate(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_route(&self, x: f32, y: f32) -> String {
        if self.is_contains(x, y) {
            return format!("({}, {}) -> ({x}, {y})", self.x, self.y);
        }
        // let mut s =
        for link in self.link_map.iter() {
            if link.borrow().is_contains(x, y) || !link.borrow().get_link_map().is_empty() {
                let coordinate = self.get_coordinate();
                let route = link.borrow().get_route(x, y);
                if route != "no route" {
                    return format!("({}, {})-> {route}", coordinate.0, coordinate.1);
                }
            }
        }

        "no route".to_string()
    }

    fn get_back_portal(&self) -> Option<WeakLink> {
        self.back_portal.clone()
    }

    fn get_link_map(&self) -> Vec<Link> {
        self.link_map.clone()
    }

    fn add_link_map(&mut self, link_map: Link) {
        self.link_map.push(link_map)
    }

    fn is_contains(&self, x: f32, y: f32) -> bool {
        self.x == x && self.y == y
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::{Portal, SwordsmanMap};

    #[test]
    fn map_test() {
        let root = Portal::new_link(0.0, 0.0, None);
        let p1 = Portal::new_link(100.0, 0.0, Some(Rc::downgrade(&root)));
        let p2 = Portal::new_link(200.0, 0.0, Some(Rc::downgrade(&root)));
        let s_map1 = SwordsmanMap::new_link(0.0, 0.0, 100.0, 100.0, Rc::downgrade(&root));
        let s_map2 = SwordsmanMap::new_link(100.0, 0.0, 100.0, 100.0, Rc::downgrade(&p1));
        let s_map3 = SwordsmanMap::new_link(200.0, 0.0, 100.0, 100.0, Rc::downgrade(&p2));
        let p3 = Portal::new_link(250.0, 50.0, Some(Rc::downgrade(&p2)));
        let s_map4 = SwordsmanMap::new_link(0.0, 100.0, 300.0, 100.0, Rc::downgrade(&p3));
        p3.borrow_mut().add_link_map(s_map4);

        p1.borrow_mut().add_link_map(s_map2);
        p2.borrow_mut().add_link_map(s_map3);
        p2.borrow_mut().add_link_map(p3);

        println!("{}", p2.borrow().get_route(50.0, 150.0));
        assert_eq!(
            "(200, 0)-> (250, 50)-> (0, 100) -> (50, 150)",
            p2.borrow().get_route(50.0, 150.0)
        );

        root.borrow_mut().add_link_map(p1);
        root.borrow_mut().add_link_map(p2);
        root.borrow_mut().add_link_map(s_map1);

        println!("{}", root.borrow().get_route(150.0, 50.0));
        println!("{}", root.borrow().get_route(50.0, 50.0));
        println!("{}", root.borrow().get_route(250.0, 50.0));
        println!("{}", root.borrow().get_route(350.0, 50.0));
        println!("{}", root.borrow().get_route(0.0, 50.0));
        println!("{}", root.borrow().get_route(50.0, 150.0));

        assert_eq!(
            "(0, 0)-> (100, 0)-> (100, 0) -> (150, 50)",
            root.borrow().get_route(150.0, 50.0)
        );
        assert_eq!(
            "(0, 0)-> (0, 0) -> (50, 50)",
            root.borrow().get_route(50.0, 50.0)
        );
        assert_eq!(
            "(0, 0)-> (200, 0)-> (200, 0) -> (250, 50)",
            root.borrow().get_route(250.0, 50.0)
        );
        assert_eq!("no route", root.borrow().get_route(350.0, 50.0));
        assert_eq!(
            "(0, 0)-> (0, 0) -> (0, 50)",
            root.borrow().get_route(0.0, 50.0)
        );
        assert_eq!(
            "(0, 0)-> (200, 0)-> (250, 50)-> (0, 100) -> (50, 150)",
            root.borrow().get_route(50.0, 150.0)
        );
    }
}
