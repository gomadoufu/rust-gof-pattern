///#Decoratorパターン
///
/// ### わかったこと
/// - トレイトオブジェクトがなぜ使われているかわからなかった。が、わかるようになった！ 確かに、ダックタイピング的だ。
/// - 関数の引数でもdynキーワードで動的ディスパッチできることがわかって感動！ &dynは最初面食らうけど、引数だからそりゃ参照受けとるよね。

pub trait Component {
    fn do_something(&self);
}

pub trait Decorator: Component {
    fn do_something_more(&self);
}

pub struct ConcreteComponent(pub usize);
//具体的なコンポーネントにはコンポーネントトレイトが実装されている
impl Component for ConcreteComponent {
    fn do_something(&self) {
        println!("ConcreteComponent do_something {}", self.0);
    }
}

pub struct ConcreteDecorator {
    //デコレータは、具体的なコンポーネントまたは別のデコレータを、実行時に保持する。これを実現するために、トレイトオブジェクトを用いている(dyn)。これにより、Conponentトレイトと同じインターフェースを持つオブジェクトなら、なんでも保持できるようになる。
    pub component: Box<dyn Component>,
    pub more_value: usize,
}

//デコレータにも、コンポーネントトレイトが実装されている(重要！！)
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

//ここでもdynキーワードが使われている
pub fn process(c: &dyn Component) {
    c.do_something();
}
