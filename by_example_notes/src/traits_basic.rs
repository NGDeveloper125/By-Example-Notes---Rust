
//Traits

//example 1
// simple trait that define a shared behavior (function) between types that implement it, 
// each type implementing this trait needs to implement its own custom implementation of the function.
pub trait TraitName {
    fn function_name(&self) -> String;
}

// using this trait with an object will look like this
pub struct StructName {
    pub struct_field: String,
}

impl TraitName for StructName {
    fn function_name(&self) -> String {
        format!("{}", self.struct_field)
    }
}

//example 2
// similar simple trait that define a shared behavior (function) between types that implement it, 
// the implementation of the function is implemented by the trait.
pub trait TraitName {
    fn function_name(&self) -> String {
        String::from("(Default implementation...)")
    }
}

// using this trait with an object will look like this
pub struct StructName {
    pub struct_field: String,
}

impl TraitName for StructName {}


//Traits as Parameters (trait bound)
// you can use trait as a generic type with the impl keyword to specify that an item can be any type that implement the trait:
pub fn function_a(item: &impl TraitName) {
    println!("This type implement the trait! {}", item.function_name());
}

// or like this:
pub fn function_b<T: TraitName>(item: &T) {
    println!("This type implement the trait! {}", item.function_name());
}

// a key different between these two implementation is if you want to have more then one parameter and have each parameter be a different type that implement the trait:
pub fn function_c(item1: &impl TraitName, item2: &impl TraitName) {
    println!("This types implement the trait! {} {}", item1.function_name(), item2.function_name());
}

// or if you want to force the 2 parameters to have exactly the same type:
pub fn function_d<T: TraitName>(item1: &T, item2: &T) { 
    println!("This types implement the trait! {} {}", item1.function_name(), item2.function_name());
}

// you can also specify multiple traits in this 2 ways:
pub fn function_e(item: &(impl TraitName + OtherTrait)) {

}

pub fn function_f<T: TraitName + OtherTrait>(item: &T) {

}

//other acceptable syntax:
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}

// bounding a type that implement the trait can also be used in the return type:
fn some_function_a() -> impl TraitName {
    StructName
}