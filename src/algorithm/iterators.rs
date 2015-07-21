use std::iter::order;

use tree::Tree;

pub fn same_fringe<T: PartialEq>(tree1: &Tree<T>, tree2: &Tree<T>) -> bool
{
    order::eq(tree1.iter(), tree2.iter())
}

// TODO Lazy, copy-pasted from continuations rather than refactoring
// into parameterized tests.
#[cfg(test)]
mod test {
    use super::same_fringe;
    use tree::Tree::{Leaf, Forest};

    #[test]
    fn different_shapes_same_fringe() {
        let tree1 = Forest(vec![Leaf(1isize),
                                Forest(vec![Leaf(2),
                                            Forest(vec![Leaf(3),
                                                        Leaf(4)
                                                        ]
                                                   )
                                            ]
                                       )
                                ]
                           );

        let tree2 = Forest(vec![Forest(vec![Leaf(1isize),
                                            Leaf(2)
                                            ]
                                       ),
                                Leaf(3),
                                Forest(vec![Forest(vec![Leaf(4)
                                                        ]
                                                   )
                                            ]
                                       )
                                ]
                           );

        assert!(same_fringe(&tree1, &tree2));
    }

    #[test]
    fn different_shapes_different_fringe() {
        let tree1 = Forest(vec![Leaf(1isize),
                                Forest(vec![Leaf(2),
                                            Forest(vec![Leaf(3),
                                                        Leaf(4)
                                                        ]
                                                   )
                                            ]
                                       )
                                ]
                           );

        let tree2 = Forest(vec![Forest(vec![Leaf(1isize),
                                            Leaf(2)
                                            ]
                                       ),
                                Leaf(3),
                                Forest(vec![Forest(vec![Leaf(4),
                                                        Leaf(5)
                                                        ]
                                                   )
                                            ]
                                       )
                                ]
                           );

        assert!(!same_fringe(&tree1, &tree2));
    }

}
