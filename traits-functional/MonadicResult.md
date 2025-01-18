# Rust Traits - Functional Programming Concepts

## Custom Monadic Operations with Result

The MonadicResult trait is a custom implementation (not built into Rust) that enables function chaining with Result types. While Rust provides similar functionality with the built-in and_then for Option type, this custom trait demonstrates monadic operations inspired by Railway Oriented Programming principles, where success and failure paths are treated as parallel tracks, allowing for clean and predictable error flow through the entire operation chain.

### Custom Trait Definition and Implementation
```rust
// This is a custom trait that needs to be defined in your codebase
pub trait MonadicResult<T, E> {
    fn and_then<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(T) -> Result<T, E>;
}

impl<T, E> MonadicResult<T, E> for Result<T, E> {
    fn and_then<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(T) -> Result<T, E>,
    {
        match self {
            Ok(value) => f(value),
            Err(_) => self,  // Early return on error
        }
    }
}
```

### Error Handling Behavior

The key feature of this implementation is its early error return behavior:
1. If an operation returns `Err`, the chain immediately stops
2. No further operations in the chain are executed
3. The original error is propagated through the chain
4. This prevents unnecessary computation and maintains error context

Here's an example demonstrating the error handling:

```rust
fn validate_positive(x: i32) -> Result<i32, String> {
    if x > 0 {
        Ok(x)
    } else {
        Err("Number must be positive".to_string())
    }
}

fn double(x: i32) -> Result<i32, String> {
    Ok(x * 2)
}

fn main() {
    // Example with early error return
    let result = Ok(-5)
        .and_then(validate_positive)  // Returns Err immediately
        .and_then(double);           // This is never executed
        
    // Prints: "Error: Number must be positive"
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
    
    // Example with successful chain
    let result = Ok(5)
        .and_then(validate_positive)  // Ok(5)
        .and_then(double);           // Ok(10)
        
    // Prints: "Result: 10"
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Proper Usage Pattern with Monoid Structure

The key to effective use of this trait is maintaining a monoid structure where:
1. All functions in the chain operate on and return the same types
2. Functions are passed directly as operations, not wrapped in closures
3. Each operation preserves the Result wrapper throughout the chain

Here's a proper example:

```rust
#[derive(Debug)]
struct EncryptedData {
    content: String,
    key: i32,
}

// All functions follow the same signature: fn(EncryptedData) -> Result<EncryptedData, ProcessError>
type ProcessResult = Result<EncryptedData, ProcessError>;

#[derive(Debug)]
enum ProcessError {
    InvalidInput,
    ProcessingFailed,
    ValidationFailed,
}

fn validate_data(data: EncryptedData) -> ProcessResult {
    if data.content.is_empty() {
        Err(ProcessError::InvalidInput)
    } else {
        Ok(data)
    }
}

fn apply_encryption(data: EncryptedData) -> ProcessResult {
    Ok(EncryptedData {
        content: format!("encrypted_{}", data.content),
        key: data.key,
    })
}

fn add_checksum(data: EncryptedData) -> ProcessResult {
    Ok(EncryptedData {
        content: format!("{}_checksum", data.content),
        key: data.key + 1,
    })
}

fn finalize_process(data: EncryptedData) -> ProcessResult {
    Ok(EncryptedData {
        content: format!("final_{}", data.content),
        key: data.key,
    })
}

fn main() {
    let initial_data = EncryptedData {
        content: "secret".to_string(),
        key: 123,
    };

    // Proper chaining pattern - passing functions directly
    let result = Ok(initial_data)
        .and_then(validate_data)
        .and_then(apply_encryption)
        .and_then(add_checksum)
        .and_then(finalize_process);

    match result {
        Ok(data) => println!("Process completed: {:?}", data),
        Err(e) => println!("Error occurred: {:?}", e),
    }
}
```

### Key Points About This Pattern

1. **Type Consistency**: All operations use the same types throughout the chain:
   - Input: `EncryptedData`
   - Output: `Result<EncryptedData, ProcessError>`

2. **Direct Function Passing**: Notice how functions are passed directly:
   ```rust
   .and_then(validate_data)  // Correct
   // Not like this:
   // .and_then(|data| validate_data(data))  // Unnecessary closure
   ```

3. **Error Type Consistency**: All operations use the same error type (`ProcessError`), making error handling uniform throughout the chain

4. **Monoid Structure**: The chain preserves these properties:
   - All operations have the same input and output types
   - Operations can be chained in any order
   - Each operation maintains the Result wrapper

This pattern is particularly useful for:
- Data processing pipelines
- Multi-step validation processes
- Complex transformations that need to maintain error context
- Operations that need to be composed in different orders
