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


#### Special Characters
| character | description |
| :-------: | :---------- |
| `(` | START OF LONG GLYPH (e.g. long pi) |
| `)` | END OF LONG GLYPH |
| `[` | START OF CARTOUCHE |
| `]` | END OF CARTOUCHE |
| `{` | START OF REVERSE LONG GLYPH (e.g. long la) |
| `}` | END OF REVERSE LONG GLYPH |
| `+` | SCALING JOINER (second words inside first word) |
| `-` | STACKING JOINER (second word above first word) |
| `_` | COMBINING LONG GLYPH EXTENSION |
| `.` | middle dot |
| `:` | colon |
| `te` | start of toki pona quotes |
| `to` | end of toki pona quotes |



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
