## Explanation

Finally, getting my hands on some Rust code again. I was quite satisfied
with the way that I've handled errors and propagated them through
numerous layers during the recursive parsing. Two great articles helped
me with some orientation on how to set up things [1] without too much boilerplate code [2].

The actual parsing was suprisingly trivial, I'd have expected this day's exercise to demand
at least the implementation of a rudimentary language which would require a lexer, parser and
actual unit tests. Alas, this was still a pleasure to do.

There are some weird things going on in Rust but little by little I gain some confidence about
its usefulness as a language. The feature of not having to care about NPEs is very unique
and the type-safe streaming allows for much faster processing in contrast to pragmatic languages like Go. Additionally, the way in which type assertions can be combined with enums that **carry values** is a big benefit.
Nevertheless, with templates, traits, lifetime parameters and other quirks of Rust to go, it's not at all
clear in what direction it'll pull me next.

[1] https://adriann.github.io/rust_parser.html
[2] https://fettblog.eu/rust-enums-wrapping-errors/
