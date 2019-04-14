# mvr
[![Build Status](https://travis-ci.org/jeremystucki/mvr.svg?branch=master)](https://travis-ci.org/jeremystucki/mvr)
[![dependency status](https://deps.rs/repo/github/jeremystucki/mvr/status.svg)](https://deps.rs/repo/github/jeremystucki/mvr)

A replacement for [zsh's zmv](http://zsh.sourceforge.net/Doc/Release/User-Contributions.html#index-zmv).

## Usage
```
USAGE:
    mvr <old pattern> <new pattern>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <old pattern>    Use braces to indicate groups
                     Use question marks to match a single character
                     Use asterisks to match any amount of characters
    <new pattern>    Use $n to insert a matched group (0-based)
```

## Example

Input:  
```
 ├── foo_01.md
 ├── foo_01.txt
 └── foo_02.txt
```

Command:  
`mvr '*_(??).(*)' '$0.$1'`

Output:  
```
 ├── 01.md
 ├── 01.txt
 └── 02.txt
```
