# resport

Simple CLI tool that tells you an images dimensions, aspect-ratio or if it is landscape. Supports all image formats the [image](https://crates.io/crates/image) crate supports.

```
$ resport orientation -i "..\wallpaper.jpg"
landscape
$ resport aspect-ratio -i "..\wallpaper.jpg"
16:9
$ resport dimensions -i "..\wallpaper.jpg"
1920x1080

$ resport
CLI tool that tells you an images dimensions, aspect-ratio or if it is landscape

Usage: resport.exe <COMMAND>

Commands:
  dimensions    prints 1920x1080
  width         prints 1920
  height        prints 1080
  aspect-ratio  prints 16:9
  orientation   prints landscape
  is-portrait   prints false
  is-landscape  prints true
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```


## Installation

Via Cargo:

`cargo install resport`

Or [download a release from github](https://github.com/grayfallstown/resport/releases)


## License

MIT or Apache 2
