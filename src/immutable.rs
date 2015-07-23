/// Immutable stuff.

use tree::Tree;
use tree::Tree::{Leaf, Forest};

/// Map one tree into another.
///
/// f is a reference to a closure because of ownership subtleties.
impl<T> Tree<T> {
    /// This cannot be defined given only an external iterator
    /// because the point is to preserve the shape of the output
    /// while changing the type of the leaves.
    pub fn map<U, F>(&self, f: &F) -> Tree<U>
        where F: Fn(&T) -> U
    {
        match self {
            &Leaf(ref leaf) =>
                Leaf(f(leaf)),
            &Forest(ref forest) =>
                Forest(forest.iter()
                       .map(|tree| tree.map(f))
                       .collect())
        }
    }

    pub fn map_into<U, F>(self, f: &F) -> Tree<U>
        where F: Fn(T) -> U
    {
        match self {
            Leaf(leaf) =>
                Leaf(f(leaf)),
            Forest(forest) =>
                Forest(forest.into_iter()
                       .map(|tree| tree.map_into(f))
                       .collect())
        }
    }

    /// This recursive internal iteration is superfluous given an
    /// external iterator, because any iterator automatically supports
    /// a fold by itself simply through looping, with the advantage
    /// that the fold is iterative and does not use the Rust call
    /// stack, but instead the manual stack used in the iterator
    /// itself.
    ///
    /// If you don't need mutation through an external iterator, and
    /// don't worry about the Rust call stack, however, an internal
    /// iteration can be much easier to implement than an external
    /// iterator.
    pub fn fold<B, F>(&self, init: B, f: &F) -> B
        where F: Fn(B, &T) -> B
    {
        match self {
            &Leaf(ref leaf) =>
                f(init, leaf),
            &Forest(ref forest) =>
                forest.iter()
                .fold(init, |b, tree| tree.fold(b, f))
        }
    }
}

#[cfg(test)]
mod test {
    use tree::Tree;
    use tree::Tree::{Leaf, Forest};

    fn make_tree1() -> Tree<isize> {
        Forest(vec![Leaf(1),
                    Forest(vec![Leaf(2),
                                Forest(vec![Leaf(3),
                                            Leaf(4)
                                            ]
                                       )
                                ]
                           )
                    ]
               )
    }

    fn make_tree2() -> Tree<isize> {
        Forest(vec![Leaf(2),
                    Forest(vec![Leaf(3),
                                Forest(vec![Leaf(4),
                                            Leaf(5)
                                            ]
                                       )
                                ]
                           )
                    ]
               )
    }

    #[test]
    fn map_works() {
        let tree = make_tree1();
        let expected = make_tree2();
        let actual = tree.map(&|n| n+1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn map_into_works() {
        let tree = make_tree1();
        let expected = make_tree2();
        let actual = tree.map_into(&|n| n+1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn internal_fold_works() {
        let tree = make_tree1();
        let actual = tree.fold(0, &|x, y| x + y);
        assert_eq!(actual, 10);
    }
}
