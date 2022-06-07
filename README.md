# Tom Words

A Rust implementation of [Tom Scott's](https://tomscott.com) 7 segment program demonstrated in ["What's The Longest Word You Can Write With Seven-Segment Displays?"](https://www.youtube.com/watch?v=zp4BMR88260) (Series 2, Episode 1 of "The Basics")

## Build & Run

The project uses Rust's standard [cargo](https://doc.rust-lang.org/cargo/) build system, once Cargo (& Rust!) is installed you can run the project with

```
$ cargo run --release
```

*Note*: The inital build will be quite slow as regex and it's dependencies will need to be built

## Benchmarks

node.js: Original script (as featured in video)

```
Benchmark 1: node node.js
  Time (mean ± σ):     210.3 ms ±   5.9 ms    [User: 225.5 ms, System: 30.9 ms]
  Range (min … max):   202.1 ms … 230.3 ms    20 runs
 
Benchmark 2: deno run --allow-read deno.js
  Time (mean ± σ):      92.6 ms ±   3.8 ms    [User: 81.6 ms, System: 25.5 ms]
  Range (min … max):    89.3 ms … 110.6 ms    30 runs
 
Benchmark 3: target/debug/tomwords
  Time (mean ± σ):     579.3 ms ±   7.3 ms    [User: 572.8 ms, System: 2.2 ms]
  Range (min … max):   560.7 ms … 593.2 ms    20 runs
 
Benchmark 4: target/release/tomwords
  Time (mean ± σ):      29.6 ms ±   0.8 ms    [User: 28.2 ms, System: 1.2 ms]
  Range (min … max):    28.6 ms …  32.7 ms    88 runs
 
Summary
  'target/release/tomwords' ran
    3.13 ± 0.15 times faster than 'deno run --allow-read deno.js'
    7.10 ± 0.28 times faster than 'node node.js'
   19.55 ± 0.58 times faster than 'target/debug/tomwords'
```

Now these benchmarks really aren't great because they are highly dependent on my machine and will be different depending on what else the machine is doing but give us a rough guide.

I'm quite certain this could run even faster but I decided to show off Rust's syntax and iterators rather than focus on raw speed
