# btsrch
a cross-platform search bar that opens with a keyboard shortcut, lets you type something and shows results which can be selected with arrow keys and executed with enter.
## Features

- Instant search UI (opens via keyboard shortcut)
- Unit-aware calculator (supports mixed units and conversions)
- Link detection and browser launching
- File/folder path opening
- Unicode & emoji search with images
- Custom shortcuts via scripts directory
- Works on Windows, X11, and Wayland
- Extend functionality by adding Rust modules and recompiling

## capabilities
### unit calculator
a calculator that parses the search input as a calculation and outputs the result, ready to copy with enter.
#### example:
`987654321/123456789`
evaluates to:
`8.0000000729`

`1m+3yards as in`
evaluates to:
`147.37008 inches`

`(13mA*4V*1h)/(12u*0.7mmol+2mg)/(9.81m/s²) as mi`
evaluates to:
`1140.13063 miles`

### custom shortcuts
create a directory `scripts` in the root of the cloned repository and add your shortcuts to make their file names searchable. Press enter to open / execute the selected shortcut.
#### supported file formats
##### linux
- `.sh`
- `.url`
##### windows
- `.exe`
- `.ps1`
- `.bat`
- `.lnk`
- `.url`

especially for windows, there are probably more supported formats since everything btsrch doesn't recognize gets passed to the `open` crate.
### links
paste a link in the search bar and it'll offer to open it in your standard browser. Only the most common top level domains are supported. If you want another to also work, like e.g. `.ai`, add it to the list in `src/link_parser.rs`.

If a link with a supported top level domain doesn't work, please create an issue on github.

example working link: `github.com/Johannes-Pabst/btsrch/`
### paths
paste a path to a folder or file to open it in the default application for that file type / the file manager.
### unicode / emoji
search a unicode character's / emoji's name to get offered to copy that character.

Data is taken from the following files:

[https://gist.github.com/bvincent1/cc1b0391c611d8501bad8e2780060d25](https://gist.github.com/bvincent1/cc1b0391c611d8501bad8e2780060d25)

[https://raw.githubusercontent.com/chalda-pnuzig/emojis.json/refs/heads/master/src/list.with.images.with.modifiers.json](https://raw.githubusercontent.com/chalda-pnuzig/emojis.json/refs/heads/master/src/list.with.images.with.modifiers.json)

[https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt](https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt)

Since egui doesn't support colored characters, btsrch also shows an image for each emoji from the same json file.


## Installation
first, clone the github repository where you want the programm installation. (portable btw!)

to do this, run `git clone https://github.com/Johannes-Pabst/btsrch.git` in the right folder.

If you don't have git installed, you can also download the zip from github ([https://github.com/Johannes-Pabst/btsrch/archive/refs/heads/master.zip](https://github.com/Johannes-Pabst/btsrch/archive/refs/heads/master.zip)) and extract it where you want it.

next, you'll want to install rust to compile the repository from source.

To install rust, follow the instructions on [https://rust-lang.org/tools/install/](https://rust-lang.org/tools/install/).

to compile the programm, run `cargo build --release` in the repo's root directory.

Now, the only thing left is to make it auto-run on a specific shortcut. This differs based on the OS.
### Windows
the easiest method I found is using autohotkey. There is already a .exe file in the root directory called `btsrch.exe`. This file was created by compiling `btsrch.ahk` with autohotkey for windows. If you don't have autohotkey installed, you can simply add this file to autostart and every time you press alt + space the search bar should open.

If you don't want to run a random .exe file from the internet, install autohotkey and add btsrch.ahk to autostart instead.
### Linux
Since this is linux, there are lots of different ways to achieve this. Your desktop manager might have an auto-installed app for managing hotkeys like cinnamon's `cinnamon-settings-keyboard` app. If not, You can install and use `sxhkd`. Here's a config file that runs btsrch on alt + space:
```
alt + space
    /path/to/btsrch/repo/btsrch.sh
```
after adding this to your config file at `~/.config/sxhkd/sxhkdrc`, you'll have to make sxhkd start at boot. Then, everything should work. If it doesn't, feel free to post an issue with a detailed error message.

### Troubleshooting
- on cinnamon 6.6+, btsrch might not gain focus when opened. As a workaround, the following command can be used in the cinnamon shortcut manager to start btsrch and then give it focus using xdotool: `bash -c "/path/to/btsrch/repo/btsrch.sh & xdotool search --sync --name "^BTSRCH$" windowactivate"`

## Licenses


### License for list.with.images.with.modifiers.json
ISC License

Copyright (c) 2021, Chalda Pnuzig

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted, provided that the above
copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

### License for UnicodeData.txt

UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 1991-2025 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
Data Files or Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, and/or sell
copies of the Data Files or Software, and to permit persons to whom the
Data Files or Software are furnished to do so, provided that either (a)
this copyright and permission notice appear with all copies of the Data
Files or Software, or (b) this copyright and permission notice appear in
associated Documentation.

THE DATA FILES AND SOFTWARE ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY
KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF
THIRD PARTY RIGHTS.

IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS INCLUDED IN THIS NOTICE
BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES,
OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THE DATA
FILES OR SOFTWARE.

Except as contained in this notice, the name of a copyright holder shall
not be used in advertising or otherwise to promote the sale, use or other
dealings in these Data Files or Software without prior written
authorization of the copyright holder.