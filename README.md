# text-blob [<img src="https://travis-ci.org/nathansizemore/text-blob.svg?branch=master">][travis-badge]

Simple CLI tool for generating a blob of text of a certain size.

---

## Usage

```
$ tb 10
0000000000

$ tb --help
tb - generates a text blob of given size. Size is in bytes by default.

Usage:
    tb [-k] <size>
    tb -h | --help
    tb -v | --version

Options:
    -k, --kilobytes    Size is in kilobytes.
    -m, --megabytes    Size is in megabytes.
    -h, --help         Show this screen.
    -v, --version      Show version.
```

---

## Install

```
cargo build --release
cargo install
```

---

### Author

Nathan Sizemore, nathanrsizemore@gmail.com

### License

text-blob is available under the MPL-2.0 license. See the LICENSE file for more info.



[travis-badge]: https://travis-ci.org/nathansizemore/text-blob
