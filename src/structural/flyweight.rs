use std::{collections::HashMap, rc::Rc};

pub struct ImageFlyweight {
    pub data: String,
}

impl ImageFlyweight {
    pub fn new(data: impl Into<String>) -> Self {
        Self {
            data: format!("{}-{}", data.into(), rand::random::<u64>()),
        }
    }
}

pub struct ImageViewer {
    image: Rc<ImageFlyweight>,
}

impl ImageViewer {
    pub fn new(image: Rc<ImageFlyweight>) -> Self {
        Self { image }
    }

    pub fn display(&self) -> String {
        format!("Display: {}", self.image.data)
    }
}

pub struct ImageFlyweightFactory {
    map: HashMap<String, Rc<ImageFlyweight>>,
}

impl ImageFlyweightFactory {
    pub fn get(&mut self, filename: &str) -> Rc<ImageFlyweight> {
        if !self.map.contains_key(filename) {
            self.map
                .insert(filename.into(), Rc::new(ImageFlyweight::new(filename)));
        }
        self.map.get(filename).unwrap().clone()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::{ImageFlyweightFactory, ImageViewer};

    #[test]
    fn image_test() {
        let mut factory = ImageFlyweightFactory {
            map: HashMap::new(),
        };

        let v1 = ImageViewer::new(factory.get("test.png"));
        let v2 = ImageViewer::new(factory.get("test.png"));
        assert_eq!(v1.image.data, v2.image.data);
    }
}
