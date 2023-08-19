# GET_CHUNK

## About

This utility is used to retrieve the fragment from a file. (I made it for myself to just get the snippet I need from a generic CHANGELOG file :D)

## Simple example

```bash
# text from CHANGELOG.md
# # Changelog
# ## [2.0.0] - xxxx-xx-xx
# ### Added
# - text text text text from version 2.0.0
# # [1.0.0] - xxxx-xx-xx
# ### Added
# - text text text text
get_chunk --read-from "./CHANGELOG.md" --start-str "(?m)## \[\d\.\d\.\d\]" --write-to "temp_changelog.md"

# Output
# ## [2.0.0] - xxxx-xx-xx
# ### Added
# - text text text text from version 2.0.0
```