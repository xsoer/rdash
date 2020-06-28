mod vector;


fn main() {
    // let a = vec![1,2,45,23,12,54,23,54,23];
    let a = vec![1.23, 3.23, 234.23, 54.223];
    let b = vector::chunk(a, 2);
    println!("{:?}", b);
}