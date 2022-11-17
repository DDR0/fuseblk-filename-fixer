# Fuseblk Filename Fixer

```bash
./fuseblk-filename-fixer --full path/to/pictures
```

Having trouble with invalid filename issues when transfering files from a Linux machine to an external drive? Want an automated tool to fix this? FFF is a quick, opinionated program that substitutes the bad characters for good characters which look similar, so you can copy your files wherever you want.

Also cleans up Twitter file extensions, which were giving me trouble.


## Download
The tool may be downloaded for x86_64 Linux systems from [https://ddr0.ca/files/release/linux/fuseblk-filename-fixer.1.1.1.tar.gz](https://ddr0.ca/files/release/linux/fuseblk-filename-fixer.1.1.1.tar.gz). (SHA1 `ffc76c6f9d2ee65f4d237887b464fd757317e53d`.) For other systems, please refer to Building.

## Usage
`fuseblk-filename-fixer [-f or -m] [-e] [-u] [-r] [path]`

Options:
- `--full` or `-f`: Replace all Microsoft reserved strings:, `<>:\"/\\|?*`.
- `--minimal` or `-m`: Replace a minimal set to make ext4 fuseblk work, `:?|/`.
- `--extentions` or `-e`: Fix Twitter's mangled file extentions.
- `--use-replacement-char` or `-r`: Use `�` instead of a Unicode homoglyph.

Defaults: `--minimal` `--extentions`. Path defaults to the current directory.

## Building

With Rust installed, in the project folder, run: `cargo update` and `cargo build --release`. (Use `cargo run` to build and execute the dev binary.) You can copy or link the stand-alone binary from the build folder, `fuseblk-filename-fixer`, to anywhere you like on your system. Alternatively, you can add the release folder to your PATH variable.


## Running

Invoke the binary with the path you'd like to clean. Defaults to cleaning `~/Pictures/collections`, since that's where my memes are.


## What does it do?

Current file extension substitutions are, by default:

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

Trailing dots are stripped if `--full` or `--minimal` is specified.

Illegal characters are stripped. They are replaced their appropriate unicode homoglyphs, or, if `--use-replacement-char` is specified, by the replacement character: `�`. `--full` replaces all characters specified by the spec as being illegal. `--minimal` replaces a few that cause me trouble.

## Changelog
- v1.3.0
	- Strip trailing dots if not replacing extensions only.
	- Added `"` to the minimal replacement list.
	- Fixed incorrect logic around default file paths; it would always use the CWD.
- v1.2.1
	- Added support for fixing .png_large files & friends.
- v1.2.0
	- Path defaults to `./`, not `~/pictures/collections/`.
- v1.1.1
	- Silenced options echoing.
- v1.1.0
	- Added options as to what to clean, to make fff more generally useful.	
		- `--full` / `-f`
		- `--minimal` / `-m`
		- `--extensions` / `-e`
		- `--use-replacement-char` / `-r`

- v1.0.0
	- Initial release.