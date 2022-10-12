mod decorator;
mod observer;

use decorator::{process, ConcreteComponent, ConcreteDecorator, Decorator};
use observer::{ConcreteObserver, ConcreteSubject, EventObject, Subject};
fn main() {
    println!("Hello, world!");
    observer();
    decorator();
}

fn observer() {
    let mut subject = ConcreteSubject::new();
    subject.register_observer(Box::new(ConcreteObserver(1)));
    subject.register_observer(Box::new(ConcreteObserver(2)));
    subject.register_observer(Box::new(ConcreteObserver(3)));

    subject.notify_observers(&EventObject(100));
    subject.notify_observers(&EventObject(200));
}

fn decorator() {
    let c = ConcreteComponent(100);
    process(&c);

    let d = ConcreteDecorator {
        component: Box::new(c),
        more_value: 999,
    };
    process(&d);
    d.do_something_more();
}
