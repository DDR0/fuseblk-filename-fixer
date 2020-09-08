# Fuseblk Filename Fixer

```bash
./fuseblk-filename-fixer --full path/to/pictures
```

Having trouble with invalid filename issues when transfering files from a Linux machine to an external drive? Want an automated tool to fix this? FFF is a quick, opinionated program that substitutes the bad characters for good characters which look similar, so you can copy your files wherever you want.

Also cleans up Twitter file extensions, which were giving me trouble.


## Download
The tool may be downloaded for x86_64 Linux systems from [https://ddr0.ca/files/release/linux/fuseblk-filename-fixer.1.1.0.tar.gz](https://ddr0.ca/files/release/linux/fuseblk-filename-fixer.1.1.0.tar.gz). (SHA1 `9f01f56a06a7fdd37c62c95123fc558716da9ab6`.) For other systems, please refer to Building.

## Usage
`fuseblk-filename-fixer [-f or -m] [-e] [-u] [-r] [path]`

Defaults: `--minimal` `--extentions` and `~/Pictures/collections`

Options:
- `--full` or `-f`: Replace all Microsoft reserved strings:, `<>:\"/\\|?*`.
- `--minimal` or `-m`: Replace a minimal set to make ext4 fuseblk work, `:?|/`.
- `--extentions` or `-e`: Fix Twitter's mangled file extentions.
- `--use-replacement-char` or `-r`: Use `�` instead of a Unicode homoglyph.


## Building

With Rust installed, in the project folder, run: `cargo update` and `cargo build --release`. (Use `cargo run` to build and execute the dev binary.) You can copy the stand-alone binary from the build folder, `fuseblk-filename-fixer`, to anywhere you like on your system.


## Running

Invoke the binary with the path you'd like to clean. Defaults to cleaning `~/Pictures/collections`, since that's where my memes are.


## What does it do?

Current filename substitutions are, by default:

| Input | Output |
| --- | --- |
| `:` | `ː` |
| `?` | `﹖` |
| `|` | `⼁` |
| `/` | `⁄` |
| `.jpg:large` | `.jpg` |
| `.jpg_large` | `.jpg` |
| `.jpg:small` | `.jpg` |
| `.jpg_small` | `.jpg` |
| `.jpeg.jpg` | `.jpg` |
| `.jpg.jpg` | `.jpg` |
| `.jpeg.jpeg` | `.jpeg` |
| `.jpg.jpeg` | `.jpeg` |

For example, `A bird?.jpg:large` is turned into into `A bird﹖.jpg`. (Note the difference in the `?`s.)