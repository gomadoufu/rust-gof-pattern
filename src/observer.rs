///#Observerパターン
///
/// ### わかったこと
/// - トレイトオブジェクトにはdynキーワードが必要
/// - タプル構造体のフィールドにも、pubが必要
/// - トレイト関数の引数には、型名だけでなく、変数名も必要

pub trait Subject<T: Clone> {
    fn register_observer(&mut self, observer: Box<dyn Observer<T>>) -> usize;
    fn remove_observer(&mut self, id: usize) -> Option<Box<dyn Observer<T>>>;
    fn notify_observers(&self, event: &T);
}

pub trait Observer<T: Clone> {
    fn update(&self, observer: &T);
}

#[derive(Debug, Clone)]
pub struct EventObject(pub usize);

pub struct ConcreteSubject {
    observers: Vec<Option<Box<dyn Observer<EventObject>>>>,
}

impl ConcreteSubject {
#[allow(dead_code)]
    pub fn new() -> ConcreteSubject {
        ConcreteSubject {
            observers: Vec::new(),
        }
    }
}

impl Subject<EventObject> for ConcreteSubject {
    fn notify_observers(&self, e: &EventObject) {
        for observer in self.observers.iter() {
            match *observer {
                Some(ref observer) => observer.update(&e),
                None => (),
            }
        }
    }

    fn register_observer(&mut self, observer: Box<dyn Observer<EventObject>>) -> usize {
        self.observers.push(Some(observer));
        self.observers.len() - 1
    }

    fn remove_observer(&mut self, index: usize) -> Option<Box<dyn Observer<EventObject>>> {
        self.observers[index].take()
    }
}

pub struct ConcreteObserver(pub usize);
impl Observer<EventObject> for ConcreteObserver {
    fn update(&self, e: &EventObject) {
        println!("Observer {} received event {:?}", self.0, e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observer() {
    let mut subject = ConcreteSubject::new();
    subject.register_observer(Box::new(ConcreteObserver(1)));
    subject.register_observer(Box::new(ConcreteObserver(2)));
    subject.register_observer(Box::new(ConcreteObserver(3)));

    subject.notify_observers(&EventObject(100));
    subject.notify_observers(&EventObject(200));
    }
}
