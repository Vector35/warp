# TODO

## `downpad.py`

### Purpose

The `downpad.py` script is used to download `.deb` files from launchpad, these `.deb` files include the static libraries (namely `libc.a`) we are interested in.

### Examples

To download all `opensc` packages for arch `arm64`:

```bash
python downpad.py --package opensc --arch arm64
```

To download all `downloaded_packages` for `downloaded_archs`:

```bash
python downpad.py
```

## `downwind.py`

### Purpose

The `downwind.py` script is used to download msvc files from microsoft, these files include the static libraries (namely `msvcrt.lib`) we are interested in.

### Examples

Download all targets (`x86`, `arm`, `arm64`, `x64`) for the latest version:

```bash
python downpad.py --accept-license
```

## `extract_archives.py`

### Purpose

The `extract_archives.py` script is used after running `downpad.py` to extract `.deb` files into their contents, namely the static libraries.

### Examples

To extract all `.deb` (and other formats) in the CWD:

```bash
python extract_archives.py
```