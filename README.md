# Indeed

> Indeed, those strings are in the file

This program lets you abstract away a very simple action: append lines to a file.

A more complex action, that this program is initially made for is expressed with the `--unique`/`-u` flag, that only appends each line if it isn't already in the file.

For example, if file `asdf` contains this text:
```
one
two
three

```

You can execute `indeed -u asdf two six` to make the file contain:
```
one
two
three
six
```

Since `two` is already in the file *on its own on a line*, it didn't get added, but `six` did because it wasn't.

The program does an exact match, rather than a substring match. Example:
```
one
twofer
three
```
```
indeed -u asdf two
```
```
one
twofer
three
two
```

A write to the file is only done if at least one string needs to be added. If all specified strings are already in the file, a non-zero exitcode will be returned, with no error message.

There's no error message because even if nothing got added, you still ensured certain strings are in the file, essentially meaning a success. The non-zero exit code is useful with chaining shell commands with `&&`.

Mind that all final newlines get trimmed (if there's a write to the file), so you won't have to worry about them as you would have to otherwise.

This program exists because of how annoying it is to make sure things are `.gitignore`d

With `indeed`, you can now do `indeed -u .gitignore target/` to make sure `target/` is git ignored, without possibly introducing a duplicate line in the `.gitignore` file.

You don't even have to preemtively create said `.gitignore` file, because `indeed` will create the provided path if it doesn't exist.

Hell, you don't even need to be in the same directory as the `.gitignore` file, because you could just `indeed -u my-project/.gitignore target/`

## Usage

```
Appends lines of text to a file on their own lines.
A write to the file is only done if at least one of the specified strings needs to be added.
Final newlines are trimmed in that case.
The file (along with its parent directories) is created if it doesn't exist.

Usage: indeed [OPTIONS] <PATH> [STRINGS]...

Arguments:
  <PATH>
  [STRINGS]...

Options:
  -u, --unique   Only append a line if it's not already in the file.
                 This check happens separately for each line.
                 The matching is exact, not substring.
                 If all of the specified strings are already in the file,
                 a non-zero exitcode is returned with no error message.
  -h, --help     Print help
  -V, --version  Print version
```

## Install

```
cargo install indeed
```

`cargo-binstall` and `cargo-quickinstall` are also supported

## Uninstall

```
cargo uninstall indeed
```