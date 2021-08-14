pub fn main() {

    // The Stack and the Heap -> part of memory management -> to be used at runtime

    // The Stack 
    // `last in, first out`
    // `pushing onto the stack` and `popping off the stack`
    // must have a known, fixed size
    // unknown data size at compile time or a size that might change -> stored at the Heap (less organized` )
    // 
    // The Heap
    // memory allocator finds an empty spot in the heap that big enough, marks it as being in use, 
    // and return a `pointer` (address of that location) -> `allocating on the heap`
    // 
    // Ownership Rules:
    // - Each value in Rust has a variable that's called `owner`.
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.
    //
    // Variable Scope
    // ----------------------------------------------------------------

    let _s1 = "hello";

    {
        let _s2 = "world";
        // do stuff with s2
    }

    // println!("{}, {}", s1, s2);

    // The String Type
    let mut s3 = String::from("hello world");
    s3.push_str(", from string type");

    println!("{}", s3);

    // Memory and Allocation
    // - The memory must be requested from the memory allocator at runtime.
    // - We need a way of returning this memory to the allocator when we're done with our `String`.

    // Shallow copy and deep copy
    // Move and Clone

    let s4 = String::from("Hello");
    let s5 = s4.clone();

    println!("s4 = {}, s5 = {}", s4, s5);

    // Some of the type that implement `Copy`
    // - All the integer types, such as `u32`
    // - The boolean type, `bool`, with values `true` and `false`
    // - All the floating point types, such as `f64`
    // - The character type, `char`
    // - Tuples, if they only contain types that also implement `Copy`. For example, `(i32, i32)`
    //   implements `Copy`, but `(i32, String)` does not
}
