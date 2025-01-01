[![Python and Rust with Linux Command Line Tools](./resources/banner.svg)](https://www.coursera.org/learn/python-rust-linux "Python and Rust with Linux Command Line Tools")

# LSRS

The `lsrs` program lists information about files (of any type, including directories). Options and file arguments can be intermixed arbitrarily, as usual. Later options override earlier options that are incompatible.

👉 [External lab: build a basic Rust CLI](https://www.coursera.org/learn/python-rust-linux/home/module/1)

## NAME

`lsrs` - primitive `ls` implementation in Rust.

## SYNOPSIS

lsrs [OPTION]... [FILE]...

### DESCRIPTION

List information about the FILEs (the current directory by default).

`-B`, `--ignore-backups`

    do not list implied entries ending with ~

`--hide=PATTERN`

    do not list implied entries matching shell PATTERN

`--show=PATTERN`

    do list implied entries matching shell PATTERN

`-R`, `--recursive`

    list subdirectories recursively

### EXAMPLE

```sh
# Install CLI util to /home/username/.cargo/bin/lsrs.
$ make install

# Check install path.
$ which lsrs

$ lsrs
file1.txt
file1.md
file1.md~
nested
file2.md
file2.txt

$ lsrs -R
file1.txt
file1.md
file1.md~
nested
nested/file1.txt
nested/file1.md
nested/file2.txt~
nested/file2.md
nested/file2.txt
file2.md
file2.txt

$ lsrs -R -B
file1.txt
file1.md
nested
nested/file1.txt
nested/file1.md
nested/file2.md
nested/file2.txt
file2.md
file2.txt

$ lsrs -R -B --show=file2
nested/file2.md
nested/file2.txt
file2.md
file2.txt

$ lsrs -R -B --show=file2 --hide=.txt
nested/file2.md
file2.md

$ lsrs --show="car|file" ~
.cargo
.profile
```

### Exit status

`0` - success

`1` - minor problems  (e.g., failure to access a file or directory not
  specified as a command line argument.  This happens when listing a
  directory in which entries are actively being removed or renamed.)

`2` - serious trouble (e.g., memory exhausted, invalid option, failure
  to access a file or directory specified as a command line argument
  or a directory loop)

### AUTHOR

Written by Yurii Musolov.

### COPYRIGHT

Copyright (c) 2025 Yurii Musolov. License MIT.
