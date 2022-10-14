trait Thingy {
    fn hallo(&self);
}

trait Factory {
    type Output: Thingy;
    fn make(&self) -> Self::Output;
}
struct AThingy {}

impl Thingy for AThingy {
    fn hallo(&self) {
        println!("i'm A thingy");
    }
}

struct AFactory {}

impl Factory for AFactory {
    type Output = AThingy;
    fn make(&self) -> AThingy {
        AThingy {}
    }
}

struct BThingy {}

impl Thingy for BThingy {
    fn hallo(&self) {
        println!("i'm B thingy");
    }
}

struct BFactory {}

impl Factory for BFactory {
    type Output = BThingy;
    fn make(&self) -> BThingy {
        BThingy {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abstract_factory() {
        //トレイトオブジェクト&関連型指定
        let a_factory: Box<dyn Factory<Output = AThingy>> = Box::new(AFactory {});
        let b_factory: Box<dyn Factory<Output = BThingy>> = Box::new(BFactory {});

        a_factory.make().hallo();
        b_factory.make().hallo();
    }
}
