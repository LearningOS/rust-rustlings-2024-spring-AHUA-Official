// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//创建一个 Vec（向量），其中包含与数组 a 中完全相同的元素
//VEC是什么  Vec 是 Rust 标准库中的一种动态数组类型，它可以在运行时动态增长和缩减大小。
//与数组不同，Vec 的
//大小是可变的，它可以动态地增加或减少元素数量。因此，Vec 提供了更灵活的内存管理和更方便的数据操作方式。
//le and pass the test!
/*使用 Vec，你可以：

动态地添加和移除元素。
在堆上存储元素，因此可以容纳任意数量的元素（取决于可用内存）。
使用 push() 方法在末尾添加元素，使用 pop() 方法从末尾移除元素。
使用 len() 方法获取 Vec 中元素的数量。
使用索引或迭代器访问 Vec 中的元素。 */
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

// I AM N

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v = Vec::from(a);// TODO: declare your vector here with the macro for vectors

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
