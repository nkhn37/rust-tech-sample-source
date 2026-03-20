fn main() {
    let mut v = vec![1, 2, 3];

    // iter メソッドで不変参照のイテレータを返却
    for x in v.iter() {
        println!("{}", x);

        // 不変参照なので、以下のように値を変更することはできない
        // *x += 1;
    }

    // iter_mut メソッドで可変参照のイテレータを返却
    for x in v.iter_mut() {
        *x += 1;
        println!("{}", x);
    }
}
