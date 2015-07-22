use tree::Tree;

use std::iter::Iterator;
use std::iter::IntoIterator;

/// Three kinds of iterators for Tree
/// TODO there is massive code duplication that should be factored out.
impl<T> Tree<T> {
    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            stack: vec![self]
        }
    }

    #[inline]
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            stack: vec![self]
        }
    }
}

impl<'a, T> IntoIterator for &'a Tree<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Tree<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

impl<T> IntoIterator for Tree<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            stack: vec![self]
        }
    }
}


/// Walk through subtrees left to right.
pub struct Iter<'a, T: 'a> {
    stack: Vec<&'a Tree<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        loop {
            match self.stack.pop() {
                None => return None,
                Some(tree) => match tree {
                    &Tree::Leaf(ref leaf) =>
                        return Some(leaf),
                    &Tree::Forest(ref forest) => {
                        // Push the subtrees right to left.
                        for subtree in forest.iter().rev() {
                            self.stack.push(subtree);
                        }
                        // Continue looping.
                    }

                }
            }
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut Tree<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        loop {
            match self.stack.pop() {
                None => return None,
                Some(tree) => match tree {
                    &mut Tree::Leaf(ref mut leaf) =>
                        return Some(leaf),
                    &mut Tree::Forest(ref mut forest) => {
                        // Push the subtrees right to left.
                        for subtree in forest.iter_mut().rev() {
                            self.stack.push(subtree);
                        }
                        // Continue looping.
                    }

                }
            }
        }
    }
}

pub struct IntoIter<T> {
    stack: Vec<Tree<T>>
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            match self.stack.pop() {
                None => return None,
                Some(tree) => match tree {
                    Tree::Leaf(leaf) =>
                        return Some(leaf),
                    Tree::Forest(forest) => {
                        // Push the subtrees right to left.
                        for subtree in forest.into_iter().rev() {
                            self.stack.push(subtree);
                        }
                        // Continue looping.
                    }

                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use tree::Tree::{Leaf, Forest};

    #[test]
    fn iteration_is_in_order() {
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

        let expected = vec![1isize, 2, 3, 4];
        let actual = tree1.into_iter()
            .collect::<Vec<_>>();

        assert_eq!(actual, expected);
    }

    #[test]
    fn iteration_with_mutation() {
        let mut tree1 = Forest(vec![Leaf(1isize),
                                    Forest(vec![Leaf(2),
                                                Forest(vec![Leaf(3),
                                                            Leaf(4)
                                                            ]
                                                       )
                                                ]
                                           )
                                    ]
                               );

        let tree2 = Forest(vec![Leaf(1isize),
                                Forest(vec![Leaf(2),
                                            Forest(vec![Leaf(33),
                                                        Leaf(4)
                                                        ]
                                                   )
                                            ]
                                       )
                                ]
                           );

        // Modify element of value 3 to 33.
        *tree1.iter_mut().nth(2).unwrap() = 33;

        assert_eq!(tree1, tree2);
    }
}
