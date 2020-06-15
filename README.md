# lexical-sort

This is a library to sort strings (or file paths) **lexically**. This means that non-ASCII
characters such as `á` or `ß` are treated like their closest ASCII character: `á` is treated
as `a`, `ß` is treated as `ss`.

The sort is case-insensitive. Alphanumeric characters are sorted after all other characters
(punctuation, whitespace, special characters, emojis, ...).

It is possible to enable **natural sorting**, which also handles ASCII numbers. For example,
`50` is sorted before `100` with natural sorting turned on.

If different strings have the same ASCII representation (e.g. `"Foo"` and `"fóò"`), we fall
back to the default implementation, which just compares Unicode code points.

## Usage

To sort strings or paths, use the `LexicalSort` trait:

```rust
use lexical_sort::LexicalSort;

let mut strings = vec!["ß", "é", "100", "hello", "world", "50", ".", "B!"];

strings.lexical_sort(/* enable natural sorting: */ true);
assert_eq!(&strings, &[".", "50", "100", "B!", "é", "hello", "ß", "world"]);
```

To just compare two strings, use the `lexical_cmp` or `lexical_natural_cmp` function.

## Contributing

Contributions, bug reports and feature requests are welcome!

If support for certain characters is missing, you can contribute them to the
[any_ascii](https://github.com/hunterwb/any-ascii) crate.

Let me know if you want to use this in `no_std`. It's certainly possible to add `no_std` support
to this crate and its dependencies.

## License

This project is dual-licensed under the **MIT** and **Apache 2.0** license.
Use whichever you prefer.
