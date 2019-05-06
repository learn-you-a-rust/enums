# enums
https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html

A non-option type `T` will always have a valid value (will not be null).
An option type `Option<T>` may or may not be null. 
These two types are not the same and cannot be combined! Ex. you cannot add an `i8` and an `Option<i8>`.
