pub trait Component {
    fn do_something(&self);
}

pub trait Decorator: Component {
    fn do_something_more(&self);
}

pub struct ConcreteComponent(pub usize);
impl Component for ConcreteComponent {
    fn do_something(&self) {
        println!("ConcreteComponent do_something {}", self.0);
    }
}

pub struct ConcreteDecorator {
    pub component: Box<dyn Component>,
    pub more_value: usize,
}

impl Component for ConcreteDecorator {
    fn do_something(&self) {
        self.component.do_something();
    }
}

impl Decorator for ConcreteDecorator {
    fn do_something_more(&self) {
        println!("ConcreteDecorator do_something_more {}", self.more_value);
    }
}

pub fn process(c: &dyn Component) {
    c.do_something();
}
