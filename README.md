rshell
=========

A basic shell written and tested with rust-0.9pre. Not even close to guaranteed to compile with later versions, as rust is still changing very quickly.

As my first program, this was just to get a feeling for the language, and I'm sure is very buggy.

Compile
=========

No dependencies on anything outside of the stdlib, simply compile with

```rustc src/rshell.rs```

Usage
=========

Very basic. Allows for changing directories and calling executables found in your $PATH with parameters which only have output (not tested with executables that require input).

Problems
=========

- Does not support input

License
=========

MIT
