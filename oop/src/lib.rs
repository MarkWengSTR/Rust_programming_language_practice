pub trait Draw {
    fn draw(&self);
}

// // 泛型 + 特徵界線 寫法
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl <T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button {:?}", self)
        // 實際做出按鈕的code
    }
}

