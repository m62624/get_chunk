# RETRIEVE_THE_FRAGMENT_FROM

## About

This utility is used to retrieve the fragment from a file. (I made it for myself to just get the snippet I need from a generic CHANGELOG file :D)

## Simple example

```bash
retrieve_the_fragment_from --read-from "./CHANGELOG.md" --start-str "(?m)## \[\d\.\d\.\d\]" --write-to "temp_changelog.md"
```