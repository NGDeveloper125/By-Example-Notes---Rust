# Rust Traits - Advanced Concepts

## Implementing Traits for Complex Types

You can implement traits for built-in types and complex custom types like iterators:

```rust
// Generic trait definition
pub trait SomeTrait {
    fn some_function(&mut self);
}

// Implementation for a built-in array iterator
impl<T, const N: usize> SomeTrait for core::array::IntoIter<T, N> 
where 
    T: std::fmt::Debug,
{
    fn some_function(&mut self) {
        for item in self {
            println!("Processing item: {:?}", item);
        }
    }
}

// Usage example
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let mut iter = arr.into_iter();
    iter.some_function();
}
```

## Iterator Trait

The Iterator trait allows you to make any type iterable:

```rust
// Custom collection type
struct NumberRange {
    start: i32,
    end: i32,
}

// Custom iterator type
struct NumberRangeIterator {
    current: i32,
    end: i32,
}

// Implementation for creating an iterator
impl NumberRange {
    fn new(start: i32, end: i32) -> Self {
        NumberRange { start, end }
    }

    fn iter(&self) -> NumberRangeIterator {
        NumberRangeIterator {
            current: self.start,
            end: self.end,
        }
    }
}

// Implementation of the Iterator trait
impl Iterator for NumberRangeIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

// Usage example
fn main() {
    let numbers = NumberRange::new(1, 5);
    for num in numbers.iter() {
        println!("Number: {}", num);
    }
}
```

## Index and IndexMut Traits

These traits allow using array-like indexing syntax (`[]`) with custom types:

```rust
use std::ops::{Index, IndexMut};

// Custom collection type
#[derive(Debug)]
struct Collection {
    data: Vec<i32>,
}

impl Collection {
    fn new(data: Vec<i32>) -> Self {
        Collection { data }
    }
}

// Implementing Index trait for immutable access
impl Index<usize> for Collection {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// Implementing IndexMut trait for mutable access
impl IndexMut<usize> for Collection {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// Usage example
fn main() {
    let mut collection = Collection::new(vec![1, 2, 3, 4, 5]);
    
    // Using Index trait (immutable access)
    println!("Third element: {}", collection[2]);
    
    // Using IndexMut trait (mutable access)
    collection[2] = 10;
    println!("Modified third element: {}", collection[2]);
    
    // Iterating over the collection
    for i in 0..5 {
        println!("Element {}: {}", i, collection[i]);
    }
}
```