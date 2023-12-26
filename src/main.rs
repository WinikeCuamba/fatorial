fn main() {
    let fat5 = fatorial(5);


    println!("O fatorial de 5! = {} ", fat5);
    assert_eq!(fat5, 122);
}

#[warn(dead_code)]
fn my_asset_eq(left: i32, right: i32) -> bool {
    if left == right {
        return true
    }

    return false
}

fn fatorial(x: i128) -> i128{
    if x == 1 {
        return 1
    }
    return x * fatorial(x -1)
}

// Fatorial uma funcao que retorna x! = x(x-1) 