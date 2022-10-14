mod decorator;
mod factory;
mod observer;

use decorator::{process, ConcreteComponent, ConcreteDecorator, Decorator};
use factory::{ConcreteProduct, Factory};
use observer::{ConcreteObserver, ConcreteSubject, EventObject, Subject};
fn main() {
    observer();
    decorator();
    factory();
}

fn observer() {
    println!("observer");
    let mut subject = ConcreteSubject::new();
    subject.register_observer(Box::new(ConcreteObserver(1)));
    subject.register_observer(Box::new(ConcreteObserver(2)));
    subject.register_observer(Box::new(ConcreteObserver(3)));

    subject.notify_observers(&EventObject(100));
    subject.notify_observers(&EventObject(200));
}

fn decorator() {
    println!("decorator");
    let c = ConcreteComponent(100);
    process(&c);

    let d = ConcreteDecorator {
        component: Box::new(c),
        more_value: 999,
    };
    process(&d);
    d.do_something_more();
}

fn factory() {
    println!("factory");
    let f = Factory;
    println!(
        "convert: {}",
        f.convert("hello".to_string(), || ConcreteProduct)
    );
}
