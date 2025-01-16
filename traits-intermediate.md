# Rust Traits - Intermediate Concepts

## Auto-implementing Traits

Rust can automatically implement common traits using the `derive` attribute:

```rust
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SomeStruct {}
```

## Traits for Enums
Rust allows you to implement trait for enums to add behavior and grouping them with other types and enums.
### Basic Enum Definition
```rust
pub enum SomeEnum {
    SomeField1,
    SomeField2,
}
```

### Implementing Default Trait
```rust
impl Default for SomeEnum {
    fn default() -> Self {
        SomeEnum::SomeField1
    }
}

// Usage examples:
let some_field1 = SomeEnum::default(); // result in the type SomeEnum::SomeField1
```

## From and Into Traits

The `From` and `Into` traits are complementary - implementing one automatically implements the other.

### Implementing From
```rust
impl From<u8> for SomeEnum {
    fn from(value: u8) -> Self {
        match value {
            1 => SomeEnum::SomeField1,
            2 => SomeEnum::SomeField2,
            _ => panic!("value is not valid"),
        }
    }
}

// Usage examples:
let enum_value = SomeEnum::from(1);  // Using From
let enum_value: SomeEnum = 1.into(); // Using Into
```

### Implementing Into
```rust
impl Into<SomeEnum> for u8 {
    fn into(self) -> SomeEnum {
        match self {
            1 => SomeEnum::SomeField1,
            2 => SomeEnum::SomeField2,
            _ => panic!("value is not valid"),
        }
    }
}

// Usage examples:
let value: u8 = 1;
let enum_value: SomeEnum = value.into();     // Using Into
let enum_value = SomeEnum::from(value);      // Using From
```

## FromStr Trait

Allows parsing strings into your type:

```rust
use std::str::FromStr;

impl FromStr for SomeEnum {
    type Err = String;

    fn from_str(some_string: &str) -> Result<Self, Self::Err> {
        match some_string {
            "SomeField1" => Ok(SomeEnum::SomeField1),
            "SomeField2" => Ok(SomeEnum::SomeField2),
            _ => Err("Invalid string".to_string()),
        }
    }
}

// Usage examples:
let enum_value = SomeEnum::from_str("SomeField1").unwrap();
let enum_value: SomeEnum = "SomeField1".parse().unwrap();
```

## Display Trait

Allows custom formatting when using format! macro or to_string():

```rust
use std::fmt;

impl fmt::Display for SomeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SomeEnum::SomeField1 => write!(f, "Field 1"),
            SomeEnum::SomeField2 => write!(f, "Field 2"),
        }
    }
}

// Usage example:
let enum_value = SomeEnum::SomeField1;
println!("Value: {}", enum_value);
```

## Add and Sub Traits

These traits allow you to define addition and subtraction behavior for your types.

### Add Trait
```rust
use std::ops::Add;

#[derive(Debug)]
struct Counter {
    count: i32,
}

impl Add for Counter {
    type Output = Counter;

    fn add(self, other: Self) -> Self::Output {
        Counter {
            count: self.count + other.count,
        }
    }
}

// Usage example:
let counter1 = Counter { count: 5 };
let counter2 = Counter { count: 3 };
let result = counter1 + counter2;  // result.count will be 8
```

### Sub Trait
```rust
use std::ops::Sub;

impl Sub for Counter {
    type Output = Counter;

    fn sub(self, other: Self) -> Self::Output {
        Counter {
            count: self.count - other.count,
        }
    }
}

// Usage example:
let counter1 = Counter { count: 5 };
let counter2 = Counter { count: 3 };
let result = counter1 - counter2;  // result.count will be 2
```
