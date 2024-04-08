# posvol tool

Command line tool to inspect and convert UKAEA CuV posvol files.

```bash
Inspect and convert UKAEA CuV posvol binaries

Usage: posvol <file> [options]

Arguments:
  <file>  Path to posvol binary file

Options:
  -v, --verbose...  Verbose logging (-v, -vv)
  -q, --quiet       Supress all log output (overrules --verbose)
  -h, --help        Print help (see more with '--help')

Conversion options:
  -o, --output <name>  Name of output files [default: posvol]
  -j, --json           Generate a JSON file
  -a, --ascii          Generate an ASCII file
  -p, --pretty         Format the ASCII file for readability

Note: --help shows more information and examples
```

## Description

Very simple reader for UKAEA CuV posvol binaries, skipping the need to open a
special viewer or sort it out manually just to check simple properties.

Allows for 1:1 conversion to a text file, a more human-readable text file, and
to the JSON format.

The endian is assumed to be the same as the native type of the system this tool
is run on. If needed, an option can be provided in future updates.

Help is printed with the `-h` flag, and `--help` will show examples, default
values, examples, and any important behaviour.

## Examples

### Print a summary

By default, a simple summary of the posvol dimensions is logged when no
arguments are provided.

```bash
# Print a summary of dimension properties
posvol plot_fmesh_104.bin
```

### Convert 1:1 to ASCII integers

It can be useful just to have something that can be open and read, so `--ascii`
converts the binary to a text file.

```bash
# Output a file named 'posvol.txt'
posvol plot_fmesh_104.bin --ascii
```

### Convert to readable text format

For somthing a bit more human-friendly, the dimensions are split up and cells
grouped into single voxels separated by blank lines.

```bash
# Output a file named 'posvol.txt'
posvol plot_fmesh_104.bin --ascii --pretty
```

### Convert to JSON file

For lovers of python and other languages there is a JSON output option to make
the underlying data more portable.

```bash
# Output a file named 'posvol.json'
posvol plot_fmesh_104.bin --json
```

### Change the output file names

By default the file names are 'posvol.txt' for ascii file formats, and
'posvol.json' for a json format.

This can be changed by providing `--output` with a name.

```bash
# Output a files named 'myfile.txt' and 'myfile.json'
posvol plot_fmesh_104.bin       \
            --json              \
            --ascii             \
            --output myfile
```

## Install

Direct from github:

```shell
cargo install --git https://github.com/repositony/posvol.git
```

All executables are under `~/.cargo/bin/`, which should already be in your path
after installing Rust.

<details>
  <summary>Click here if you have never used Rust</summary>

If you have never used the Rust programming language, the toolchain is easily
installed from the [official website](https://www.rust-lang.org/tools/install)

```shell
curl https://sh.rustup.rs -sSf | sh
```

This should have added `source $HOME/.cargo/env` to the bash profile, so update
your environment with `source ~/.bashrc`.

</details>
