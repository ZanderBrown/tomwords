# Tom Words

A Rust implementation of [Tom Scott's](https://tomscott.com) 7 segment program demonstrated in ["What's The Longest Word You Can Write With Seven-Segment Displays?"](https://www.youtube.com/watch?v=zp4BMR88260) (Series 2, Episode 1 of "The Basics")

It's worth nothing that a sligtly different regex is used, Tom's orginal `\[gkmqvwxzio]\` only match unsupported *letters* but the list contains quite a few hyphenated words so `\[gkmqvwxzio\W]\` is used instead as `\W` matches all 'non-word' charecters

## Build & Run

The project uses Rust's standard [cargo](https://doc.rust-lang.org/cargo/) build system, once Cargo (& Rust!) is installed you can run the project with

```
$ cargo run --release
```

*Note*: The inital build will be quite slow as regex and it's dependencies will need to be built

## Benchmarks

Original script (as featured in video)

| real     | user     | sys      |
| ---      | ---      | ---      |
| 0m0.579s | 0m0.475s | 0m0.097s |

Release Build (`cargo build --release`)

| real     | user     | sys      |
| ---      | ---      | ---      |
| 0m0.155s | 0m0.142s | 0m0.005s |

Debug Build (`cargo build`)

| real     | user     | sys      |
| ---      | ---      | ---      |
| 0m5.650s | 0m5.553s | 0m0.015s |

Now these benchmarks really aren't great because they are highly dependent on my machine and will be different depending on what else the machine is doing but give us a rough guide.

It seems safe to say an optimised Rust build is a fair bit faster than Node.js but when one considers the fact Node is a interpreted language (and it's fractions of a second) there isn't much in it. It's worth noting that debug builds are actually significantly slower.

I'm quite certain this could run even faster but I decided to show off Rust's syntax and iterators rather than focus on raw speed
