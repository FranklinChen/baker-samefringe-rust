/// Recursively defined tree where values are stored only at the leaves.
#[derive(PartialEq, Debug)]
pub enum Tree<T> {
    Leaf(T),
    Forest(Vec<Tree<T>>),
}
