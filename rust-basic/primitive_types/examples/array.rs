fn main() {
    let scores = [1, 2, 3]; // 推論される型は [i32; 3] (i32の要素を3つ持つ配列)
    let zeros: [i32; 5] = [0; 5]; // 要素0を5つ繰り返す配列 [0, 0, 0, 0, 0]

    // 配列へのアクセス
    println!("scoresの値: {:?}", scores);
    println!("zerosの値: {:?}", zeros);
    // 配列の要素へのアクセス
    println!("scores[0]: {}", scores[0]);
    println!("scores[1]: {}", scores[1]);
    println!("scores[2]: {}", scores[2]);
    // println!("エラーになる: {}", scores[3]);
}
