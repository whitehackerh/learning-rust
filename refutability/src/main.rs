/*
　パターンの2つの形態
　・論駁不可能
　　・渡される可能性のあるあらゆる値に合致するパターン
　　　ex) let x = 5; の x -> xは何にでも合致し、合致に失敗することがあり得ない
　　・関数の引数、let文、forループ
　・論駁可能
　　・何らかの可能性のある値に対して合致しないことがあるパターン
　　　ex) if let Some(x) = a_value の Some(x) -> 右辺がNoneなら合致しない
　　・if let, while let
*/

fn main() {
    println!("Hello, world!");
}
