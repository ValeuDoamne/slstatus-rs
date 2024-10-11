# slstatus-rs

This is my status bar written in rust, this time, it's very similar with what I've had wrote in C, I now use it in my dwl (yes, wayland, it's 2024 lol) configuration.

## Usage
For debugging to having a look at the status bar string, run it with the `-s` argument:
```
$ slstatus-rs -s
```

By default the program will try to create a named pipe a/k/a FIFO file, at location `/tmp/dwm_bar.pipe` which is going to be used by both `dwl` and `slstatus-rs` as a `Consumer` and `Producer` pattern.
```
$ slstatus-rs
```
**Word of advice**: When opening a FIFO file only with one end (eg. READ or WRITE, only one, not both at the same time) the `open` syscall becomes `blocking` if the `O_NONBLOCK` flag is not specified.

You can also change the location of where the named pipe is stored using the `-p` argument.
```
$ slstatus-rs -p /run/dwl/asdf.pipe
```

## Licensing

Under AGPL3, see [LICENSE.txt](./LICENSE.txt).
