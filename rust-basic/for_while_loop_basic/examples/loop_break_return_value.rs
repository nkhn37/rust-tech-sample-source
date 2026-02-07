fn main() {
    let mut count = 0;

    // loopのbreak時に値を返却する
    let result = loop {
        if count >= 5 {
            break true;
        }

        println!("count = {}", count);
        count += 1;
    };
    println!("result : {}", result);
}
