/*
　イテレータ
　・一連の要素に順番に何らかの処理を行える
　・各要素を繰り返し、シーケンスが終わったことを決定するロジックの責任を負う
　・全てのイテレータはIteratorトレイトを実装している

　iter
　・不変参照へのイテレータを生成
　into_iter
　・所有権を奪い、所有された値を返すイテレータを生成
　iter_mut
　・可変参照へのイテレータを生成
*/

fn main() {
    let v1 = vec![1, 2, 3];
    // イテレータ生成をしているが、これだけでは何も起こらない
    let v1_iter = v1.iter();
    // イテレータを消費
    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2 = vec![1, 2, 3];
    let v2_iter = v2.iter();
    // sumはイテレータの所有権を奪う -> この後v2_iterは無効
    let total: i32 = v2_iter.sum();
    assert_eq!(total, 6);

    let v3: Vec<i32> = vec![1, 2, 3];
    // mapは怠惰であるため、別のメソッドを使用し、イテレータを消費する必要がある
    let v4: Vec<_> = v3.iter().map(|x| x + 1).collect();
    assert_eq!(v4, vec![2, 3, 4]);
}
