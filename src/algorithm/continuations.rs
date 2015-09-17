use tree::Tree;

/// Generator and Consumer are basically closures that feed into each other.
///
/// The newtype wrappers are needed to break the cyclic references of
/// function types.
///
/// A Consumer receives a Consumer to communicate to.
struct Generator<'a, T: 'a>(&'a Fn(&Consumer<T>) -> bool);

/// A Consumer receives either:
///
/// - end of data
/// - a new item plus the next generator to call
struct Consumer<'a, T: 'a>(&'a Fn(Option<(&T, &Generator<T>)>) -> bool);

/// Generator that tells the consumer that there are no more elements left.
fn eof<T: PartialEq>(c: &Consumer<T>) -> bool {
    c.0(None)
}

/// Determine whether tree1 and tree2 have the same fringe.
///
/// Kick off a leaf generator for each tree.
pub fn same_fringe<T: PartialEq>(tree1: &Tree<T>, tree2: &Tree<T>) -> bool
{
    same_fringe_c(
        &Generator(
            &(|c| gen_fringe(tree1, c, &Generator(&eof)))),
        &Generator(
            &(|c| gen_fringe(tree2, c, &Generator(&eof))))
            )
}

/// Run one step of each tree's generator and compare the leaves
/// (if any) to decide whether we are done, failed, or need to continue
/// recursively with a new pair of generators.
fn same_fringe_c<T: PartialEq>(xg: &Generator<T>, yg: &Generator<T>) -> bool
{
    xg.0(&Consumer(
        &(|x_next|
          yg.0(&Consumer(
              &(|y_next|
                match (x_next, y_next) {
                    (None, None) => true,
                    (Some((x, xg2)), Some((y, yg2))) =>
                        *x == *y
                        && same_fringe_c(xg2, yg2),
                    (_, _) => false,
                }
                ))))))
}

/// Examining the tree, if we are at a leaf, call the consumer,
/// else enter forest-traversing mode.
fn gen_fringe<T: PartialEq>(
    tree: &Tree<T>,
    c: &Consumer<T>,
    g: &Generator<T>)
    -> bool
{
    match tree {
        &Tree::Leaf(ref leaf) => c.0(Some((leaf, g))),
        &Tree::Forest(ref forest) => gen_fringe_l(&forest, c, g)
    }
}

/// Walk through a forest recursively through its elements,
/// examining each subtree in sequence while threading through
/// the consumer and generator.
fn gen_fringe_l<T: PartialEq>(
    forest: &[Tree<T>],
    c: &Consumer<T>,
    g: &Generator<T>)
    -> bool
{
    match forest.split_first() {
        None =>
            g.0(c),
        Some((head, tail)) =>
            gen_fringe(head,
                       c,
                       &Generator(&(|c2|
                                    gen_fringe_l(tail, c2, g))))
    }
}

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
