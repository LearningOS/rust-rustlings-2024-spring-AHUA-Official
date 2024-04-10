// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//完成循环，使得给定的 Vec 中的每个数字都乘以 2。
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

/*
考察 Vec 的迭代和修改：vec_loop 函数展示了如何使用 iter_mut 方法来遍历可变引用的 Vec，
并通过乘以 2 修改每个元素的值。

迭代器和函数式编程：vec_map 函数展示了如何使用 map 方法和闭包对 Vec 中的每个元素进行操作，
而不需要修改原始 Vec。
这种方法更符合函数式编程的风格，能够更简洁地表达意图。*/

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.  解引用运算符（*）来访问每个元素的可变引用
        *element  *=2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}
/*在第二个函数（vec_map）中，我们不需要使用解引用运算符，因为map函数期望闭包返回新值。这允许我们在不直接修改原始向量的情况下对每个元素进行转换。

至于在Rust开发人员中更常用的模式，这取决于具体的用例和个人偏好。

对于直接修改的情况，使用iter_mut更为明确，可能在需要清晰度的情况下更受欢迎。
对于无需修改的转换，使用map更具功能性，可能在需要不可变性和可组合性的情况下更受欢迎。 */
 
fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| element * 2).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
