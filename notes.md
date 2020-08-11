- `let VARIABLE_NAME` to shadow the same variable name

- run multiple tests, by default they run in parallel using threads

- each file in `tests` directory is compiles as its own separate crate

- `src` only has `main.rs` without `lib.rs` can't create integration tests in the  `tests` directory

- static variables can only store references with the `'static` lifetime

- `static variables` always access to the same data, whereas `constants variables` are allowed to duplicate their data whenever they are used

- `Vec<T>` index type: `usize`

- when calling functions, all arguments and function's local variables are pushed onto stack. After execution, all is popped off the stack

- `Vec<T>` can only store values of the same type

  > `Vec<T>` method `push` refer `self` as mutable reference


# Term

- `double free error`: data pointers pointing to the same location are getting out of scope, they will try to free the same memory multiple times!
- `recursive type`: one type whose size can't be known at compile time


#  Traits

- `trait` tell the compiler about functionality a particular type has and can share with other types, similar to `class method` in other language

- __a trait bound on `?Sized` has to work as reference `&T`, whereas `Sized` trait can work as `T`. Because data without known size has to be wrapped__

- __types with `Copy` trait will be not moved even though they are taken by other functions__. But they still can be `Drop`

   - all integer types, such as `u32`

   - Boolean types `bool`

   - all floating types `f64`

   - character type `char`

   - any group of simple scalar values can be `Copy`

   - nothing that requires allocation or is some form of resource is `Copy`

  ## Ownership
- __when moving the ownership of variables, the real data is not copied__
  ## Closure

  > `fn(T) -> U` function pointers implement all three of the closure traits

  > Closure are represented by traits, which means wen can't return closures directly, must use other types to wrap closures, like __`Box<dyn Fn(T) -> U>`__

  - `Fn(T) -> U`
  - `FnMut(T) -> U`
  - `FnOnce(T) -> U`

  ## Reference 

  - `Deref` trait allows an instance of the smart pointer struct to behave like a reference

  - `Deref coercion` (__with no runtime penalty__) works only on types that implement the `Deref` trait

    > __automatically execute__ when we pass a reference to a particular type's value as an argument to functions or methods that doesn't match the parameter type of function or method definition. 

  ## Smart Pointer

  >  __why need to use smart pointers?__
  >
  > Rust needs to know how much memory to allocate for any value of a particular type, and all values of a type must use the same amount of memory, thereby usually using smart pointers or other types to wrap the real data allocated on the heap

  

  - `Box<T>`: trait `Deref`
    - heap allocation
    - single ownership
    - immutable
    - __automatically dereference to `T` (via `Deref` trait)__
    - never allocate more than `isize::MAX` bytes
    - `Box::leak()` consume and leak Box, returning a mutable reference `&'a mut T`

  - `Rc<T>`: trait `Deref` (VS. `Arc<T>`)
    - reference-counting pointer
    - shared ownership
    - immutable
    - __automatically dereference to `T` (via `Deref` trait)__
    - `Rc::clone(&T)` produce a new pointer to the same allocation in the heap
    - `Rc::downgrade(&T)` create a non-owning `Weak` pointer (which doest not keep values inside the allocation but keep the allocation alive)

  - `RefCell<T>` & `Cell<T>` (VS. `Mutex<T>`)
    - mutable memory location with dynamically checked borrow rules
    - 

  - `Mutex<T>` multiple threads
    - mutual exclusion primitive
    - will block threads for the lock to become available
    - __`lock()` or `try_lock()` prevent other threads to access the the data__
    - __`MuteGuard` has `Drop` trait, when its lifetime ends, the data automatically unlock__
  - `Arc<T>` multiple threads trait `Deref`
    - immutable

## Concurrency
> For multiple threads, create multiple receivers by `Arc::clone(&receiver)`, must guarantee `lock()` released (automatically drop when out of scope or manually `drop()`), like:


    let received = rx.lock().recv().unwrap();
    match received {//do something}
or

    match rx.lock().unwrap().recv() {//do something}
     
# APIs

- `format!()` not take the ownership of its parameters
- `std::iter:Iterator::take()` take the ownership
  - for `Option<T>`, `Some(val)` variant is taken out, and leave `None` in place

# Examples

    let s3 = s1 + &s2;
> `s3` take the ownership of `s1`, then appends a copy of the contents of `s2`

- `while let`

        while let Some(val) = Iterator {
          // exhaust all elements of Iterator until emit 'None'
        } 

     

     ​		

     ​	