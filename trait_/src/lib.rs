/*
　トレイト
　・他の言語のinterfaceに近い機能
　・特定の型に存在し、他の型と共有できる機能を定義
　・共通の振る舞いを抽象的に定義
　・トレイト境界を使用すると、あるジェネリック型が特定の振る舞いを持つあらゆる型になり得ることを指定できる
　・トレイトを実装する型はメソッドに独自の振る舞いが必要
*/

pub trait Summary {
    // メソッドシグニチャは行ごとに並べられ、各行はセミコロンで終わる
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/*
　デフォルト実装
　・全ての型の全メソッドに対して実装を要求するのではなく、トレイトの全てあるいは一部メソッドに対してデフォルトの振る舞いを用意できる
　・特定の型にトレイトを実装する際、デフォルト実装を保持するかオーバーライドするか選べる
　・デフォルト実装をそのメソッドをオーバーライドしている実装から呼び出すことはできない
*/
pub trait Summary2 {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle2 {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet2 {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary2 for NewsArticle2 {
    // デフォルト実装を利用する場合はそのメソッドを実装する必要はない
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Summary2 for Tweet2 {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// トレイトを実装する何らかの型を持つ引数を持つ
// 引数のトレイトを実装している型のインスタンスであればなんでも渡せる
// impl Trait構文
pub fn notify(item: &impl Summary2) {
    println!("Breaking news! {}", item.summarize());
}

// トレイト境界構文
pub fn notify2<T: Summary2>(item: &T) {
    // 速報！ {}
    println!("Breaking news! {}", item.summarize());
}

// 複数引数が必要なメソッドで両方の引数が同じ型であることを強制したい場合
// pub fn notify2<T: Summary2>(item1: &T, item2: &T) {

// 複数の実装をしていなければならないと指定する場合
// pub fn notify2(item: &(impl Summary2 + Display)) {
// pub fn notify2<T: Summary2 + Display>(item: &T) {

// where句を使う
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 内部の型Tが比較を可能にするPartialOrdトレイトと出力を可能にするDisplayトレイトを実装している時のみ、 cmp_displayメソッドを実装
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/*
　ブランケット実装
　・トレイト境界を満たすあらゆる型にトレイトを実装すること
*/