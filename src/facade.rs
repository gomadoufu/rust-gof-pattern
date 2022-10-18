/*
# Facadeパターン
### わかったこと
- たいしたことしてない。
- ただ、Adapterパターンとの目的の違いには注意したい
 */
fn worker_a() {
    println!("hoge");
}

fn worker_b() {
    println!("huga");
}

fn worker_c() {
    println!("piyo");
}

struct Facade;
impl Facade {
    fn facade_method(&self) {
        worker_a();
        worker_b();
        worker_c();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_facade() {
        let f = Facade;
        f.facade_method();
    }
}
