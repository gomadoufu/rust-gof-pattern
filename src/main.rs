mod observer;

use observer::{ConcreteObserver, ConcreteSubject, EventObject, Subject};
fn main() {
    println!("Hello, world!");
    observer();
}

fn observer() {
    let mut subject = ConcreteSubject::new();
    subject.register_observer(Box::new(ConcreteObserver(1)));
    subject.register_observer(Box::new(ConcreteObserver(2)));
    subject.register_observer(Box::new(ConcreteObserver(3)));

    subject.notify_observers(&EventObject(100));
    subject.notify_observers(&EventObject(200));
}
