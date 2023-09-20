# RetroParse

This project is nothing more than a parser for [Retrofit](https://square.github.io/retrofit/) HTTP calls using regex. This program **will not** find all URLs in the Java code, but only the retrofit2 ones, since my goal is only to find API calls made by the application and not everything resembles an URL.

Before writing this program I tried [apkleaks](https://github.com/dwisiswant0/apkleaks) and [Diggy](https://github.com/s0md3v/Diggy), but the first is pretty slow and finds too much stuff, while the second can't find anything useful in my case.

## Installation

Just download the file from the releases page and run it.
If there is no release for your OS, you can build it yourself using the following command:

```bash
cargo build --release
```

## Usage

RetroParse has three input modes:
- Text `-t`: the program will read the input from stdin
- File `-f path/to/file`: the program will read the input from a given file
- Directory `-d path/to/dir`: the program will recursively read the input from all the files in a given directory

The default output is on stdout, but it's possible to write the output to a file using the `-o path/to/file` option.

The default output format is a Markdown table, but it's possible to change it using the `-F` option:
- Markdown: `-F md`: the output will be a Markdown table
- JSON: `-F json`: the output will be a JSON array

## TODO

- [ ] Customizable path blacklist for directory mode
- [ ] Sort by file
- [ ] Parameters parsing
- [ ] **Handle errors!** Crashing programs are the worst...