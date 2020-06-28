// 将数组（array）拆分成多个 size 长度的区块，并将这些区块组成一个新数组。 如果array 无法被分割成全部等长的区块，那么最后剩余的元素将组成一个区块。
pub fn chunk<T>(v: Vec<T>, size: usize) -> Vec<Vec<T>>
where
    T: Copy + Clone,
{
    assert!(size > 0);

    let length = v.len();
    assert!(length >= size);

    let mut n = vec![];

    let t = v.len() / size;
    for i in 0..t + 1 {
        let mut tmp = vec![];

        for j in i * size..((i + 1) * size) {
            if j > length - 1 {
                continue;
            }
            tmp.push(v[j]);
        }
        if tmp.len() != 0 {
            n.push(tmp);
        }
    }
    n
}

// fn compact() {}

// fn concat() {}

// fn difference() {}

// fn drop() {}

// fn drop_right() {}

//
// fn drop_while() {}


// 使用 value 值来填充（替换） array，从start位置开始, 到end位置结束（但不包含end位置）
// fn file() {}

// 获取数组 array 的第一个元素。
// fn head() {}

// 减少一级array嵌套深度。
// fn flatten() {}

// 根据 depth 递归减少 array 的嵌套层级
// fn flatten_depth() {}

// 将array递归为一维数组。
// fn flatten_deep() {}

// 这个方法返回一个由键值对pairs构成的对象。
// fn from_pairs() {}

// 返回首次 value 在数组array中被找到的 索引值， 如果 fromIndex 为负值，将从数组array尾端索引进行匹配。
// fn index_of() {}

// 获取数组array中除了最后一个元素之外的所有元素（注：去除数组array中的最后一个元素）。
// fn initial() {}

// 创建唯一值的数组，这个数组包含所有给定数组都包含的元素
// fn intersection() {}

// 将 array 中的所有元素转换为由 separator 分隔的字符串。
// fn join() {}

// 获取array中的最后一个元素。
// fn last() {}

//
// fn last_index_of() {}


// 获取array数组的第n个元素。如果n为负数，则返回从数组结尾开始的第n个元素。
// fn nth() {}

// 移除数组array中所有和给定值相等的元素
// fn pull() {}

// 这个方法类似pull，区别是这个方法接收一个要移除值的数组。
// fn pull_all() {}

// 根据索引 indexes，移除array中对应的元素，并返回被移除元素的数组。
// fn pull_at() {}

// 反转array，使得第一个元素变为最后一个元素，第二个元素变为倒数第二个元素，依次类推。
// fn reverse() {}

// 裁剪数组array，从 start 位置开始到end结束，但不包括 end 本身的位置。
// fn slice() {}

// 使用二进制的方式检索来决定 value值 应该插入到数组中 尽可能小的索引位置，以保证array的排序。
// fn sorted_index() {}

// 获取除了array数组第一个元素以外的全部元素。
// fn tail() {}

// 创建一个数组切片，从array数组的起始元素开始提取n个元素。
// fn take() {}

// 创建一个数组切片，从array数组的最后一个元素开始提取n个元素。
// fn take_right() {}

// 创建一个按顺序排列的唯一值的数组。
// fn union() {}

// 创建一个去重后的array数组副本。
// fn uniq() {}

// 这个方法类似于_.zip，除了它接收分组元素的数组，并且创建一个数组，分组元素到打包前的结构。
// fn unzip() {}

// 创建一个剔除所有给定值的新数组，剔除值的时候
// fn without() {}

// 创建一个给定数组唯一值的数组
// fn xor() {}

// 创建一个分组元素的数组，数组的第一个元素包含所有给定数组的第一个元素，数组的第二个元素包含所有给定数组的第二个元素，以此类推。
// fn zip() {}

// 这个方法类似 _.fromPairs，除了它接受2个数组，第一个数组中的值作为属性标识符（属性名），第二个数组中的值作为相应的属性值。
// fn zip_object() {}



#[cfg(test)]
mod tests {
    use super::*;

    fn test_chunk_data1() -> Vec<(usize, Vec<u8>, Vec<Vec<u8>>)> {
        vec![
            (
                2,
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]],
            ),
            (
                2,
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
                vec![
                    vec![1, 2],
                    vec![3, 4],
                    vec![5, 6],
                    vec![7, 8],
                    vec![9, 10],
                    vec![11],
                ],
            ),
            (
                3,
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10]],
            ),
        ]
    }

    fn test_chunk_data2() -> Vec<(usize, Vec<f32>, Vec<Vec<f32>>)> {
        vec![(
            2,
            vec![1.23, 3.23, 234.23, 54.223],
            vec![vec![1.23, 3.23], vec![234.23, 54.223]],
        )]
    }

    #[test]
    fn test_chunk() {
        for e in test_chunk_data1() {
            assert_eq!(chunk(e.1, e.0), e.2);
        }
        for e in test_chunk_data2() {
            assert_eq!(chunk(e.1, e.0), e.2);
        }
    }
}
