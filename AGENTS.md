# Writing bindings

A *binding* exposes one rust-analyzer item to Python with the same behaviour, and does nothing else. Bind an item
only when code that uses the bindings needs it. Start from an existing binding that handles a similar case.

- A returned reference is kept only if no binding can change the value it points into; otherwise the binding returns
  a clone. Each `Held::lent` call has a `// SAFETY:` comment that says why that value cannot change.
- A value with a `'db` lifetime holds the Python object of the database it came from, so that database outlives it.
- A type rust-analyzer does not export is bound only through its trait implementations, from the value that contains
  it (`DepKind`). Its doc comment says what breaks if rust-analyzer changes the type.
- Each binding has a test that checks its result on a fixture. Break the binding and check that a test fails.
- Nothing stops code from passing a value from one database to a call on another.
