// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(88);
    vec
}

fn main() {
    // move semantics 移动语义

    // 可变引用
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s)
}

fn change(some_string: &mut String) {
    some_string.push_str(",world")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
