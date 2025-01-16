# Rust Traits - Basic Concepts

Traits in Rust define shared behavior between types. They are similar to interfaces in other programming languages.

## Basic Trait Definition

### 1. Traits with Required Implementation

A trait can define a function that must be implemented by any type that uses this trait:

```rust
pub trait TraitName {
    fn function_name(&self) -> String;
}

// Example Implementation
pub struct StructName {
    pub struct_field: String,
}

impl TraitName for StructName {
    fn function_name(&self) -> String {
        format!("{}", self.struct_field)
    }
}
```

### 2. Traits with Default Implementation

A trait can provide a default implementation that types can use without defining their own:

```rust
pub trait TraitName {
    fn function_name(&self) -> String {
        String::from("(Default implementation...)")
    }
}

pub struct StructName {
    pub struct_field: String,
}

// We can implement the trait without defining the function
impl TraitName for StructName {}
```

## Using Traits as Parameters (Trait Bounds)

There are several ways to use traits as parameters in functions:

### 1. Using `impl` Syntax
```rust
pub fn function_a(item: &impl TraitName) {
    println!("This type implement the trait! {}", item.function_name());
}
```

### 2. Using Generic Type Syntax
```rust
pub fn function_b<T: TraitName>(item: &T) {
    println!("This type implement the trait! {}", item.function_name());
}
```

### Key Differences in Parameter Usage

#### Different Types Implementing Same Trait
```rust
// Allows different types that implement TraitName
pub fn function_c(item1: &impl TraitName, item2: &impl TraitName) {
    println!("This types implement the trait! {} {}", 
             item1.function_name(), 
             item2.function_name());
}
```

#### Enforcing Same Type
```rust
// Forces both parameters to be the same type
pub fn function_d<T: TraitName>(item1: &T, item2: &T) { 
    println!("This types implement the trait! {} {}", 
             item1.function_name(), 
             item2.function_name());
}
```

## Multiple Trait Bounds

You can require a type to implement multiple traits:

### 1. Using `impl` Syntax
```rust
pub fn function_e(item: &(impl TraitName + OtherTrait)) {
    // Function body
}
```

### 2. Using Generic Type Syntax
```rust
pub fn function_f<T: TraitName + OtherTrait>(item: &T) {
    // Function body
}
```

### 3. Using `where` Clause
```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // Function body
}
```

## Returning Types that Implement Traits

You can use traits to specify return types:

```rust
fn some_function_a() -> impl TraitName {
    StructName
}
```
