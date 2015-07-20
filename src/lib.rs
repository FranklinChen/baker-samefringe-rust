//! Reimplement Henry Baker's [no-iterator same-fringe solution](http://home.pipeline.com/~hbaker1/Iterator.html)

/// Recursively defined tree where values are stored only at the leaves.
pub enum Tree<A> {
    Leaf(A),
    Forest(Vec<Tree<A>>),
}

/// Generator and Consumer are basically closures that feed into each other.
///
/// The newtype wrappers are needed to break the cyclic references of
/// function types.
///
/// A Consumer receives a Consumer to communicate to.
struct Generator<'a, A>(&'a Fn(&Consumer<A>) -> bool);

/// A Consumer receives either:
///
/// - end of data
/// - a new item plus the next generator to call
struct Consumer<'a, A>(&'a Fn(Option<(&A, &Generator<A>)>) -> bool);

/// For unwrapping convenience only.
impl <'a, A> Generator<'a, A> {
    #[inline]
    fn run_g(&self, c: &Consumer<A>) -> bool {
        let &Generator(closure) = self;
        closure(c)
    }
}

/// For unwrapping convenience only.
impl <'a, A> Consumer<'a, A> {
    #[inline]
    fn run_c(&self, next: Option<(&A, &Generator<A>)>) -> bool {
        let &Consumer(closure) = self;
        closure(next)
    }
}

/// Generator that tells the consumer that there are no more elements left.
fn eof<A: PartialEq>(c: &Consumer<A>) -> bool {
    c.run_c(None)
}

/// Determine whether tree1 and tree2 have the same fringe.
///
/// Kick off a leaf generator for each tree.
pub fn same_fringe<A: PartialEq>(tree1: &Tree<A>, tree2: &Tree<A>) -> bool
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
fn same_fringe_c<A: PartialEq>(xg: &Generator<A>, yg: &Generator<A>) -> bool
{
    xg.run_g(&Consumer(
        &(|x_next|
          yg.run_g(&Consumer(
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
fn gen_fringe<A: PartialEq>(
    tree: &Tree<A>,
    c: &Consumer<A>,
    g: &Generator<A>)
    -> bool
{
    match tree {
        &Tree::Leaf(ref leaf) => c.run_c(Some((leaf, g))),
        &Tree::Forest(ref forest) => gen_fringe_l(&forest, c, g)
    }
}

/// Walk through a forest recursively through its elements,
/// examining each subtree in sequence while threading through
/// the consumer and generator.
///
/// TODO Use #[feature(slice_patterns)] to get head and tail cleanly.
fn gen_fringe_l<A: PartialEq>(
    forest: &[Tree<A>],
    c: &Consumer<A>,
    g: &Generator<A>)
    -> bool
{
    if forest.is_empty() {
        g.run_g(c)
    } else {
        gen_fringe(&forest.first().unwrap(),
                   c,
                   &Generator(&(|c2| gen_fringe_l(&forest[1..], c2, g))))
    }
}

#[cfg(test)]
mod test {
    use super::same_fringe;
    use super::Tree::{Leaf, Forest};

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
