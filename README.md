# About
ilo li ante e sitelen Lasina tawa sitelen UCSUR

󱤎󱤧󱤆󱤉󱥠󱦐󱤡󱦝󱥞󱦝󱦑󱥩󱥠󱦐󱥯󱥛󱦜󱥖󱦜󱦑

tool that converts sitelen Lasina into sitelen UCSUR

## Features
 - conversion of sitelen UCSUR from/to sitelen Lasina
 - supports many additional special characters
 - can recognize names and construct the shortest possible cartouche using [nasin sitelen kalama](https://sona.pona.la/wiki/nasin_sitelen_kalama) (dots and colons)
 - converts whole numbers to sitelen pona using [nasin nanpa pona](https://sona.pona.la/wiki/nasin_nanpa_pona)
 - replaces quotes with `te`/`to` (󱦞󱦟)
 - blazingly🔥 fast🚀


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
echo "mama o lukin  mi ken sitelen Usisa a" | sitelen-ucsur from
echo "󱤱󱥄󱤮　󱤴󱤘󱥠󱦐󱥯󱥛󱦜󱥖󱦜󱦑󱤀" | sitelen-ucsur to
```


#### Argument Name
it might seem weird at first, but the arguments `from`/`to` are intended to be a non complete sentence:
 - `sitelen-ucsur from` convert from the inserted text to sitelen ucsur
 - `sitelen-ucsur to` convert sitelen-ucsur to normal text


#### Special Characters
| character | description |
| :-------: | :---------- |
| `(` | START OF LONG GLYPH (e.g. long pi) |
| `)` | END OF LONG GLYPH |
| `[` | START OF CARTOUCHE |
| `]` | END OF CARTOUCHE |
| `{` | START OF REVERSE LONG GLYPH (e.g. long la) |
| `}` | END OF REVERSE LONG GLYPH |
| `+` | SCALING JOINER (second word inside first word) |
| `-` | STACKING JOINER (second word above first word) |
| `_` | COMBINING LONG GLYPH EXTENSION |
| `.` | middle dot |
| `:` | colon |
| `^` | alternative symbol |
| `\` | escape (everything inside the pair of backslashes will be presereved as is) |
| `^1` | 1st alternative symbol (same as `^`) |
| `^2` | 2nd alternative symbol |
| `^3` | 3rd alternative symbol |
| `^4` | 4th alternative symbol |
| `^5` | 5th alternative symbol |
| `^6` | 6th alternative symbol |
| `^7` | 7th alternative symbol |
| `^8` | 8th alternative symbol |
| `^9` | 9th alternative symbol |


#### Example
here's a script I use to convert the currently selected text to sitelen UCSUR: \
it does that by using the "primary clipboard", which is a clipboard that contains the currently selected text. \
this script uses `wl-copy`, `wl-paste` (both are from the `wl-clipboard` package), and `wtype` (for pressing ctrl-v).

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

it's also possible to use another small program that just types whatever it gets from stdin: [waytyper](https://github.com/netfri25/waytyper)
```bash
#!/usr/bin/env bash

if [[ "$#" -ne 1 ]]; then
    echo "Usage: $0 <from|to>"
    exit 1
fi

# timeout to prevent hanging on non-supported applications
wl-paste -np | sitelen-ucsur "$1" | timeout 3s waytyper
```
