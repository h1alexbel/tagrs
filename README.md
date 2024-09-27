# tagrs. Rust test tagging

[![EO principles respected here](https://www.elegantobjects.org/badge.svg)](https://www.elegantobjects.org)
[![DevOps By Rultor.com](http://www.rultor.com/b/h1alexbel/tagrs)](http://www.rultor.com/p/h1alexbel/tagrs)
[![We recommend IntelliJ IDEA](https://www.elegantobjects.org/intellij-idea.svg)](https://www.jetbrains.com/idea/)

[![just](https://github.com/h1alexbel/tagrs/actions/workflows/just.yml/badge.svg)](https://github.com/h1alexbel/tagrs/actions/workflows/just.yml)
[![Crates.io Version](https://img.shields.io/crates/v/tagrs)](https://crates.io/crates/tagrs)
[![codecov](https://codecov.io/github/h1alexbel/tagrs/graph/badge.svg?token=GXcsA2ffuN)](https://codecov.io/github/h1alexbel/tagrs)
[![PDD status](http://www.0pdd.com/svg?name=h1alexbel/tagrs)](http://www.0pdd.com/p?name=h1alexbel/tagrs)
[![Hits-of-Code](https://hitsofcode.com/github/h1alexbel/tagrs)](https://hitsofcode.com/view/github/h1alexbel/tagrs)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/h1alexbel/tagrs/blob/master/LICENSE.txt)

`tagrs` is a tool for [Rust test] tagging.

**Motivation**. There was no such tool for Rust, that can run tests
conditionally, based on tags, similarly to JUnit's [@Tag][JUnit Tag]. 

## How to use

Here is an example:

```rust
#[cfg(test)]
mod tests {
    use anyhow::Result;
    use tagrs::tag;
    use std::thread;
    use std::time::Duration;
    
    #[tag("fast")]
    #[test]
    fn runs_fast() -> Result<()> {
        assert_eq!(1 + 1, 2);
        Ok(())
    }
    
    #[tag("slow")]
    #[test]
    fn runs_slow() -> Result<()> {
        thread::sleep(Duration::from_secs(2));
        assert_eq!(2 + 2, 4);
        Ok(())
    }
}
```

then run:

```bash
TTEST=fast cargo test
```

It should run only `runs_fast` test, while `runs_slow` will be ignored.

## How to contribute?

Make sure that you have [Rust] and [just] installed on your system, then fork
this repository, make changes, send us a [pull request][guidelines]. We will
review your changes and apply them to the `master` branch shortly, provided
they don't violate our quality standards. To avoid frustration, before sending
us your pull request please run full build:

```bash
just full
```

[Rust test]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
[JUnit Tag]: https://junit.org/junit5/docs/5.0.2/api/org/junit/jupiter/api/Tag.html
[guidelines]: https://www.yegor256.com/2014/04/15/github-guidelines.html
[Rust]: https://www.rust-lang.org/tools/install
[just]: https://just.systems/man/en/chapter_4.html
