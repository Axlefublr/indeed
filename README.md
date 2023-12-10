# Indeed

> Indeed, those strings are in the file

This program lets you abstract away a very simple action: add a string to a file on its own line, if it isn't there already.

For example, if file `asdf` contains this text:
```
one
two
three

```

You can execute `indeed asdf two six` to make the file contain:
```
one
two
three
six
```

Since `two` is already in the file *on its own on a line*, it didn't get added, but `six` did because it wasn't.

The program does an exact match, rather than a substring match, so any extra characters will make your strings considered unmatched, be careful.

Mind that all final newlines will be trimmed, so you won't have to worry about them as you would have to otherwise.

This program exists because of how annoying it is to make sure things are `.gitignore`d

With `indeed`, you can now do `indeed .gitignore target/` to make sure `target/` is git ignored, without possibly introducing a duplicate line in the `.gitignore` file.

# Usage

```
Add strings to a file on their own lines, if they aren't already there.

Usage: indeed <PATH> [STRINGS]...

Arguments:
  <PATH>

  [STRINGS]...


Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

# Install

```
cargo install indeed
```

`cargo-binstall` and `cargo-quickinstall` are also supported

# Uninstall

```
cargo uninstall indeed
```