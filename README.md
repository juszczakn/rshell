rshell
=========

A basic shell written and tested with rust-0.9pre. Not even close to guaranteed to compile with later versions, as rust is still changing very quickly.

As my first program, this was just to get a feeling for the language, and I'm sure is very buggy.

Compile
=========

No dependencies on anything outside of the stdlib, simply compile with

```rustc main.rs```

Usage
=========

Very basic. Allows for changing directories and calling executables found in your $PATH with parameters which only have output (not tested with executables that require input).

Problems
=========

- Currently fails when an invalid command is given.
- Does not support input

License
=========

MIT
