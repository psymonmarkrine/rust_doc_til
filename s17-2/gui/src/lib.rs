
pub trait Draw {
    fn draw(&self);
}

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

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
//     where T: Draw {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    struct Objct1{}
    impl Draw for Objct1 {
        fn draw(&self) {
            println!("This is Objct1!")
        }
    }

    enum Objct2{TEST,}
    impl Draw for Objct2 {
        fn draw(&self) {
            println!("This is Objct2!")
        }
    }

    #[test]
    fn screen_run() {
        let screen = Screen {
            components: vec![
                Box::new(Objct1{}), 
                Box::new(Objct2::TEST),
                ]
        };
        screen.run();
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw (&self) {
        println!("process to draw a Button");
    }
}
