pub const ABOUT: &str = "\
Add strings to a file on their own lines, if they aren't already there.
A write to the file is only done if at least one of the specified strings need to be added.
Final newlines are trimmed in that case.
If all of the specified strings are already in the file, a non-zero exitcode is returned with no error message.";
