/**
#Factoryパターン

### わかったこと
- あるトレイト(今回はProductトレイト)を返すFnトレイトを、関数のトレイト境界に指定することで、受け取ったクロージャのメソッドをその関数の中で呼び出すことができる。
- このメソッド呼び出しは、クロージャが返す型のAPIがトレイトにより判明しているため実現できる(今回はcreate_product().convert(s)の部分。create_product()はProductトレイトを実装した何かを返すことだけはわかっているので、convertメソッドが必ず呼び出せる)。
*/
pub trait Product {
    fn convert(&self, _: String) -> String;
}

#[allow(dead_code)]
pub struct Factory;

impl Factory {
#[allow(dead_code)]
    pub fn convert<P, F>(&self, s: String, create_product: F) -> String
    where
        P: Product,
        F: FnOnce() -> P,
    {
        create_product().convert(s)
    }
}

pub struct ConcreteProduct;
impl Product for ConcreteProduct {
    fn convert(&self, s: String) -> String {
        s.to_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory() {
        let f = Factory;
        assert_eq!("HELLO", f.convert("hello".to_string(), || ConcreteProduct));
    }
}
