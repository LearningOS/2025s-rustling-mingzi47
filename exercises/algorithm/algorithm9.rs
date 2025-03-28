/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::{max, Ord};
use std::default::Default;
use std::fmt::Display;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);
        self.count += 1;

        let idx = self.count;
        self.up(idx);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        let l_i = self.left_child_idx(idx);
        let r_i = self.right_child_idx(idx);
        if r_i > self.count {
            return l_i;
        }

        let l_v = &self.items[l_i];
        let r_v = &self.items[r_i];
        if (self.comparator)(l_v, r_v) {
            l_i
        } else {
            r_i
        }
    }

    fn up(&mut self, idx: usize) {
        let parent_i = self.parent_idx(idx);
        if parent_i == 0 {
            return;
        }

        if !(self.comparator)(&self.items[idx], &self.items[parent_i]) {
            return;
        }

        self.items.swap(idx, parent_i);
        self.up(parent_i);
    }

    fn down(&mut self, idx: usize) {
        if !self.children_present(idx) {
            return;
        }

        let v = &self.items[idx];
        let c_i = self.smallest_child_idx(idx);
        // let c_v = self.items.get(c_i - 1).inspect(|x| println!("{}", x));
        if let Some(c_v) = self.items.get(c_i) {
            if (self.comparator)(c_v, v) {
                self.items.swap(idx, c_i);
                self.down(c_i);
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.count == 0 {
            return None;
        }

        self.items.swap(1, self.count);
        self.count -= 1;
        let ret = self.items.pop();

        self.down(1);

        ret
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.len(), 2);
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.len(), 1);
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
