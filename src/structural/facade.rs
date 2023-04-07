//! 我们现在有一个家庭影院，其中包含了DVD，投影机，自动屏幕，立体环绕声等等，
//! 每当我们想要看电影的时候，都需要进行一系列操作；而看完电影时，又需要反向再操作一遍，因此提供一个影院门面
//! 我们最初的设计并不是要设计一个影院系统，而是设计播放系统、投影系统、展示系统、音箱系统；
//! 因为我们使用的时候可能会将DVD接到电视机进行播放
pub struct DVD {}

impl DVD {
    pub fn play(&self) -> String {
        String::from("按下DVD开关")
    }
}

pub struct Projector {}

impl Projector {
    pub fn projection(&self) -> String {
        String::from("打开投影仪开关")
    }
}

pub struct Curtain {}

impl Curtain {
    pub fn down(&self) -> String {
        String::from("放下幕布")
    }
}

pub struct Speaker {}

impl Speaker {
    pub fn connect(&self) -> String {
        String::from("连接音箱")
    }
}

pub struct Light {}

impl Light {
    pub fn close(&self) -> String {
        String::from("关灯")
    }
}

pub struct CinemaFacade {}

impl CinemaFacade {
    pub fn play() -> String {
        let speaker = Speaker {};
        let curtain = Curtain {};
        let projector = Projector {};
        let dvd = DVD {};
        let light = Light {};

        format!(
            "{}-{}-{}-{}-{}",
            speaker.connect(),
            curtain.down(),
            projector.projection(),
            dvd.play(),
            light.close()
        )
    }
}

#[cfg(test)]
mod test {
    use super::CinemaFacade;

    #[test]
    fn test() {
        // 注意与桥接模式的区别
        // 假如一开始我们就设计一个电影院系统，则直接使用桥接模式
        let s = CinemaFacade::play();

        assert_eq!("连接音箱-放下幕布-打开投影仪开关-按下DVD开关-关灯", s);
    }
}
