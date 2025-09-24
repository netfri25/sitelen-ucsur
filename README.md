# About
ilo li ante e sitelen Lasina tawa sitelen UCSUR

󱤎󱤧󱤆󱤉󱥠Lasina 󱥩󱥠UCSUR

tool that converts sitelen Lasina into sitelen UCSUR

## Getting started
### Prerequisite
 - [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Installation
```shell
cargo install --git https://github.com/netfri25/sitelen-ucsur
```
this should install the binary `sitelen-ucsur` to your cargo bin path, which by default is `$HOME/.cargo/bin`

## Usage
you can run this program simply by typing `sitelen-ucsur` in your terminal, and then use it kind of like a REPL.

another way to use it is by piping to stdin:
```shell
echo "mi kama sona e toki pona o" | sitelen-ucsur
```

#### Example
here's a script I use to convert the currently selected text to sitelen UCSUR:
```bash
#!/usr/bin/env bash

if [[ "$#" -ne 1 ]]; then
    echo "Usage: $0 <from|to>"
    exit 1
fi

# keep a copy of the clipboard
prev=$(wl-paste -n)

wl-paste -np | sitelen-ucsur "$1" | wl-copy -n

wtype -M ctrl -k v

# wait for the paste to register
sleep 0.1s

# restore clipboard
wl-copy -n "$prev"
```
