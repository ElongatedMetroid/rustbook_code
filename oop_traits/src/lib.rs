// This example will be making a "gui" crate keep that
// in mind.

pub trait Draw {
    fn draw(&self);    
}

pub struct Screen {
    // components holds a vector of trait objects that
    // implement the draw trait.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    /// Draw all components
    pub fn run(&self) {
        for component in self.components.iter() {
            // Since the components field only cares
            // that the component implements a Draw
            // trait
            component.draw();
        }
    }
}

// Fields on other components may differ
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button");
    }
}