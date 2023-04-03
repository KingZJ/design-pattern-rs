use std::{
    mem::MaybeUninit,
    sync::{Mutex, Once},
};

use once_cell::sync::OnceCell;

static INSTANCE: OnceCell<Singleton> = OnceCell::new();

pub struct Singleton;

pub fn get_instance() -> &'static Singleton {
    INSTANCE.get_or_init(|| Singleton)
}

pub fn get_instance2() -> &'static Mutex<Singleton> {
    static mut CONF: MaybeUninit<Mutex<Singleton>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        CONF.as_mut_ptr().write(Mutex::new(Singleton));
    });

    unsafe { &*CONF.as_ptr() }
}

#[cfg(test)]
mod test {
    use super::{get_instance, get_instance2};

    #[test]
    fn singleton_test() {
        let instance1 = get_instance();
        let instance2 = get_instance();
        assert_eq!(true, std::ptr::eq(instance1, instance2));

        let instance1 = get_instance2();
        let instance2 = get_instance2();
        assert_eq!(true, std::ptr::eq(instance1, instance2));
    }
}
