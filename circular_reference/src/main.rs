/*
　メモリリーク
　・プログラムが確保したメモリ領域を、使い終わった後も解放せず、不要に保持し続けることで、利用可能な空きメモリが徐々に減る現象
*/

/*
　・Rc<T> RefCell<T> を使用し、要素がお互いに循環して参照する参照を生成可能
　・循環の各要素の参照カウントが絶対に0にならない -> メモリリークを起こし、値は絶対にドロップされない
*/

/*
　Rc::downgrade 
　・RC<T>への参照を渡すことでRc<T>インスタンス内部の値への弱い参照を作れる
　・Weak<T>のスマートポインタが得られる
　・weak_countが1増える
　・Rc<T>型はweak_countを使用してWeak<T>参照が存在しているかを追跡する

　強い参照
　・Rc<T>インスタンスの所有権を共有する方法

　弱い参照
　・所有関係を表現せず、そのカウントはRc<T>インスタンスが片付けられるときに影響を与えない
　・強い参照カウントが0になれば、弱い参照が関わる循環はなんでも破壊される -> 循環参照にならない

　Weak<T>
　・参照する値はドロップされてる可能性があるため、指す値に何かをするには値が存在することを確認する必要がある
　 -> updateメソッドで確認
　・updateメソッドはOption<Rc<T>>を返す
*/

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// #[derive(Debug)]
// struct Node {
//     value: i32,
//     children: RefCell<Vec<Rc<Node>>>,
// }

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    // let leaf = Rc::new(Node {
    //     value: 3,
    //     children: RefCell::new(vec![]),
    // });
    // let branch = Rc::new(Node {
    //     value: 5,
    //     children: RefCell::new(vec![Rc::clone(&leaf)]),
    // });

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let leaf2 = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf2 strong = {}, weak = {}",
        Rc::strong_count(&leaf2),
        Rc::weak_count(&leaf2),
    );
    {
        let branch2 = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf2)]),
        });
        *leaf2.parent.borrow_mut() = Rc::downgrade(&branch2);
        println!(
            "branch2 strong = {}, weak = {}",
            Rc::strong_count(&branch2),
            Rc::weak_count(&branch2),
        );
        println!(
            "leaf2 strong = {}, weak = {}",
            Rc::strong_count(&leaf2),
            Rc::weak_count(&leaf2),
        );
    }
    println!("leaf2 parent = {:?}", leaf2.parent.borrow().upgrade());
    println!(
        "leaf2 strong = {}, weak = {}",
        Rc::strong_count(&leaf2),
        Rc::weak_count(&leaf2),
    );
}
