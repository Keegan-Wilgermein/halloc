//! # Linked Array
//! 
//! An array where each
//! entry links to another entry
//! 

use core::{ops::{Index, IndexMut}, slice::SliceIndex};

use crate::{PAGE_SIZE, modules::linked_array::node::Node};

/// Iterator over `LinkedArray`
pub(crate) struct NodeIter<'a, const MAX: usize> {
    list: &'a LinkedArray<MAX>,
    index: usize,
}

impl<'a, const MAX: usize> Iterator for NodeIter<'a, MAX> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.list.nodes.get(self.index);
        self.index += 1;
        next
    }
}

/// A linked array for `Page` that uses
/// a max size to form an array of `Node`
/// that link to each other
pub(crate) struct LinkedArray<const MAX: usize> {
    free: usize,
    next_free: Option<usize>,
    nodes: [Node; MAX],
}

impl<const MAX: usize, Idx> Index<Idx> for LinkedArray<MAX>
where
    Idx: SliceIndex<[Node]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<const MAX: usize, Idx> IndexMut<Idx> for LinkedArray<MAX>
where
    Idx: SliceIndex<[Node]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl<'a, const MAX: usize> IntoIterator for &'a LinkedArray<MAX> {
    type Item = &'a Node;
    type IntoIter = NodeIter<'a, MAX>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIter {
            list: self,
            index: 0,
        }
    }
}

impl<const MAX: usize> LinkedArray<MAX> {
    /// Creates a new `LinkedArray`
    pub(crate) const fn new() -> Self {
        let mut nodes = [Node::new(); MAX];

        let mut i = 0;
        while i < MAX {
            let prev = if i > 0 { i - 1 } else { MAX - 1 };
            let next = if i < MAX - 1 { i + 1 } else { 0 };
            nodes[i] = Node::from(
                prev,
                next,
                i * PAGE_SIZE,
            );
            i += 1;
        }

        Self {
            free: MAX,
            next_free: Some(0),
            nodes,
        }
    }

    /// Gets the next free index
    /// into the internal nodes list,
    /// or `None` if there is none
    pub(crate) fn next_free(&self) -> Option<usize> {
        self.next_free
    }

    /// Unlinks a page so it doesn't
    /// get used again
    pub(crate) fn unlink(&mut self, index: usize) {
        let next = *self[index].next();
        let prev = *self[index].prev();

        self[next].set_prev(prev);
        self[prev].set_next(next);

        self.free -= 1;

        if self.free == 0 {
            self.next_free = None;
        } else {
            self.next_free = Some(next);
        }
    }

    /// Links a page so it can be
    /// used again
    pub(crate) fn link(&mut self, index: usize) {
        if let Some(idx) = self.next_free() {
            let next2 = *self[idx].next();
            self[idx].set_next(index);
            self[index].set_next(next2);
            self[index].set_prev(idx);
            self[next2].set_prev(index);
        } else {
            self.next_free = Some(index);
        }

        self.free += 1;
    }
}

/// Checks that each `Node` points
/// to the correct next `Node`
#[test]
fn linked_array_created_with_correct_next_pointers() {
    const MAX: usize = 10;
    let linked_array = LinkedArray::<MAX>::new();

    linked_array.into_iter()
        .enumerate()
        .for_each(|(index, node)| {
            if index == MAX - 1 {
                assert!(*node.next() == 0, "{} | {}", node.next(), index);
            } else {
                assert!(*node.next() == index + 1, "{} | {}", node.next(), index);
            }
        });
}

/// Checks that each `Node` points
/// to the correct prev `Node`
#[test]
fn linked_array_created_with_correct_prev_pointers() {
    const MAX: usize = 10;
    let linked_array = LinkedArray::<MAX>::new();

    linked_array.into_iter()
        .enumerate()
        .for_each(|(index, node)| {
            if index == 0 {
                assert!(*node.prev() == MAX - 1, "{} | {}", node.prev(), index);
            } else {
                assert!(*node.prev() == index - 1, "{} | {}", node.prev(), index);
            }
        });
}

/// Checks that `Node` pages point to
/// exact `PAGE_SIZE` indexes in the heap
#[test]
fn linked_array_created_with_correct_page_pointers() {
    const MAX: usize = 10;
    let linked_array = LinkedArray::<MAX>::new();

    linked_array.into_iter()
        .for_each(|node| {
            assert!(node.page().index() % PAGE_SIZE == 0);
        });
}

/// Checks that every index into the `LinkedArray`
/// is valid
#[test]
fn linked_array_nodes_create_only_inbound_indexes() {
    const MAX: usize = 10;
    let linked_array = LinkedArray::<MAX>::new();

    (0..MAX).into_iter()
        .for_each(|idx| {
            // Checking the index so the index
            // doesn't get compiled away
            let next = if idx == MAX - 1 { 0 } else { idx + 1 };
            assert!(*linked_array[idx].next() == next)
        });
}

/// Checks that `next_free()` returns
/// `None` on the last iteration
/// after going through `unlink()`
#[test]
fn unlink_sets_none_on_correct_node() {
    const MAX: usize = 10;
    let mut linked_array = LinkedArray::<MAX>::new();

    (0..MAX).into_iter()
        .for_each(|idx| {
            linked_array.unlink(idx);
            if idx == MAX - 1 {
                assert!(linked_array.next_free().is_none())
            } else {
                assert!(linked_array.next_free().is_some())
            }
        });
}

/// Checks that different combinations of
/// linking and unlinking doesn't
/// break anything
#[test]
fn link_and_unlink_works() {
    const MAX: usize = 100;
    let mut linked_array = LinkedArray::<MAX>::new();
    let mut linked = [true; MAX];

    let check = |list: &LinkedArray<MAX>, linked: &[bool; MAX]| {
        let count = linked.iter().filter(|&&is_linked| is_linked).count();
        assert_eq!(list.free, count);

        if count == 0 {
            assert!(list.next_free().is_none());
            return;
        }

        let start = list.next_free().expect(
            "linked nodes should always be Some() on the first read",
        );

        assert!(linked[start]);
        let mut current = start;
        for _ in 0..count {
            assert!(linked[current]);
            let next = *list[current].next();
            let prev = *list[current].prev();
            assert!(linked[next]);
            assert!(linked[prev]);
            assert_eq!(*list[next].prev(), current);
            assert_eq!(*list[prev].next(), current);
            current = next;
        }
        assert_eq!(current, start, "linked nodes should loop");
    };

    check(&linked_array, &linked);

    // Fill and empty links

    for index in 0..MAX {
        linked_array.unlink(index);
        linked[index] = false;
        check(&linked_array, &linked);
    }

    for index in (0..MAX).rev() {
        linked_array.link(index);
        linked[index] = true;
        check(&linked_array, &linked);
    }
}
