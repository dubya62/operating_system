We don't need to have a separate error submodule inside of error, that just makes the imports be use error::error::Error.  The Error type should just be in error/mod.rs
The Error should be an enum.  You're basically already doing an enum, just without using the feature.
If you want to use specific numbers, you can use #[repr(u8)] or some other type and just do EPERM = 1
With this, you also wouldn't need the default branch in the match in the perror method.
For converting into a string, I'd recommend implementing the Display trait, which will allow you to just do println!("{}", error)

Rust enums work like C's if you use the #[repr(..)] annotation or if they're all unit structs, otherwise they're pretty different.
