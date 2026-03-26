pub trait Draw {
    fn draw(&self);
}


// トレイトオブジェクト
// 複数の具体型で埋められる
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

// ジェネリクスとトレイト境界を使用
// 一度に1つの具体型にしか置き換えられない
// -> あるインスタンスではTに指定できるのは1つの型のみ
// 同種の型しか持たないのであればこちらが有用
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {

    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {

    }
}