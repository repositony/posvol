# A posvol file reader (`posvol`)

[![GitHub release](https://img.shields.io/github/v/release/repositony/posvol?include_prereleases)](https://github.com/repositony/posvol/releases/latest)

Command line tool to inspect and convert UKAEA CuV posvol files.

```bash
Usage: posvol <file> [options]

Arguments:
  <file>               Path to posvol binary file

Options:
  -v, --verbose...     Verbose logging (-v, -vv)
  -q, --quiet          Supress all log output (overrules --verbose)
  -h, --help           Print help (see more with '--help')

Conversion options:
  -o, --output <name>  Name of output files [default: posvol]
  -j, --json           Generate a JSON file
  -a, --ascii          Generate an ASCII file
  -p, --pretty         Format the ASCII file for readability

Note: --help shows more information and examples
```

Help is printed with the `-h` flag, and `--help` will show examples, default
values, examples, and any important behaviour.

## Overview

Very simple reader for UKAEA CuV posvol binaries, skipping the need to open a
special viewer or sort it out manually just to check simple properties.

Allows for conversion to:

- 1:1 conversion text file
- Human-readable text file
- JSON data format

The endian is assumed to be the same as the native type of the system this tool
is run on. If needed, an option can be provided in future updates.

## Install

Download and unpack the latest binary executable release [here](https://github.com/repositony/posvol/releases/latest) for running in a terminal/powershell.

### Linux/MacOS

Unpack the relevant executable from the [latest release](https://github.com/repositony/posvol/releases/latest).

```bash
# Linux
tar -xjf posvol-x86_64-unknown-linux-gnu.tar.xz  # Generic linux
tar -xjf posvol-aarch64-unknown-linux-gnu.tar.xz # ARM64 Linux

# MacOS
tar -xjf posvol-x86_64-apple-darwin.tar.xz       # Intel macOS
tar -xjf posvol-aarch64-apple-darwin.tar.xz      # Apple Silicon macOS
```

And either run from there or add the executable to your `$PATH`.

```bash
./posvol -h
```

### Windows

Extract `posvol-x86_64-pc-windows-msvc.zip` from the [latest release](https://github.com/repositony/posvol/releases/latest).

Navigate to this folder and run from powershell.

```bash
.\posvol.exe -h
```

This may be set as an alias for convenience.

```powershell
Set-Alias -Name posvol -Value C:\Path\To\Folder\posvol.exe
```

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
# Output two files named 'myfile.txt' and 'myfile.json'
posvol plot_fmesh_104.bin       \
            --json              \
            --ascii             \
            --output myfile
```
