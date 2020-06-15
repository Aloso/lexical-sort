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

## Characteristics

The comparison functions constitute a [total order](https://en.wikipedia.org/wiki/Total_order).
Two strings are only considered equal if they are consist of exactly the same Unicode code points.

## Performance

The algorithm uses iterators and never allocates memory on the heap. It is optimized for strings
that consist mostly of ASCII characters. Note that the comparison is slower for strings where many
characters at the start are the same (after transliterating them to lowercase ASCII).

In my benchmark, sorting 100 strings is roughly 3 to 6 times as slow as using the sorting algorithms
from the standard library or the [alphanumeric-sort](https://github.com/magiclen/alphanumeric-sort)
crate – which is still impressive, considering the complexity of the algorithm.

### Benchmarks

These benchmarks compare

 - alphanumeric sort (`alphanumeric-sort` crate)
 - lexical sort (`lexical-sort` crate)
 - lexical + natural sort (`lexical-sort` crate)
 - native sort (standard library)

They were executed on an AMD A8-7600 Radeon R7 CPU with 4x 3.1GHz.

The following benchmark sorts 100 randomly generated strings. Each string is 5 to 20 characters long
and can contain ASCII letters, digits, special characters and various alphanumeric, non-ASCII
characters. Several of them need to be transliterated to multiple ASCII characters (e.g. `ß`, `æ`):

![Violin graph](./docs/Random_strings.svg)

The following benchmark sorts 100 randomly generated strings. Each string consists of `"T-"`
followed 1 to 8 decimal digits:

![Violin graph](./docs/Numeric_strings.svg)

## Contributing

Contributions, bug reports and feature requests are welcome!

If support for certain characters is missing, you can contribute them to the
[any_ascii](https://github.com/hunterwb/any-ascii) crate.

Let me know if you want to use this in `no_std`. It's certainly possible to add `no_std` support
to this crate and its dependencies.

## License

This project is dual-licensed under the **MIT** and **Apache 2.0** license.
Use whichever you prefer.
