# GET_CHUNK

## About

This utility is used to retrieve the fragment from a file. ( I've created this tool for my personal use, aimed at extracting specific snippets from a generic `CHANGELOG` file :D )

## Example

```bash
# text from CHANGELOG.md
# # Changelog
# ## [2.0.0] - xxxx-xx-xx
# ### Added
# - text text text text from version 2.0.0
# # [1.0.0] - xxxx-xx-xx
# ### Added
# - text text text text
get_chunk --read-from "./CHANGELOG.md" --start-str "## \[\d\.\d\.\d\]" --write-to "temp_changelog.md"

# Output
# ## [2.0.0] - xxxx-xx-xx
# ### Added
# - text text text text from version 2.0.0
```

```bash
Retrieve the fragment from the file

Usage: get_chunk [OPTIONS] --read-from <READ_FROM> --start-str <START_STR>

Options:
  -r, --read-from <READ_FROM>  read from file
  -s, --start-str <START_STR>  start string (Regular Expression is available)
  -e, --end-str <END_STR>      end string (Optional, if not specified || no final match is found, the file is read to the end. Regular Expression is available)
  -w, --write-to <WRITE_TO>    write to file (Optional, if not specified, output to stdout)
  -h, --help                   Print help (see more with '--help')
  -V, --version                Print version
```