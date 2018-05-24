## Goal

Solve the merchants guide problem the way I would like it, in Rust.
Try to be as idiomatic as possible, and consider the answer the main user
value to provide. Thus that and only that MUST be tested, anything else 
is 'extra', like actually testing for a few things the user can run into.

## Getting the answers

Run `make answers` if your `rust` installation is at least at v1.26.
If you have no `rust` but `docker`, run `make answers-in-docker`.

In any case, you can run all `make` targets using docker via `make interactive-developer-environment-in-docker`.
Please be warned that initial compilation takes a while.

## Features

* [x] shows correct answers
* [x] fully streaming with minimal state
* [x] support for profiling
* [x] support for benchmarking
* [x] support for linting
* [x] interactive developer environment in docker

## Benchmark Results

Even though the Rust Implementation is the fastest, which is mainly due to its non-regex parsing.
The fastest python3 implementation clocks in at 2.3s, Rust at 1.03s with regex, and 240ms without.
Peak memory is 26MB in python and 480kb in Rust.
Interestingly: as python has plenty of unit-level tests, it clocks in at 311 lines including tests,
and 162 without. However, Rust has 185 only for the implementation, and 33 lines of bash for journey tests.

Another python2 implementation (streaming) clocked in at 4.6s and sported only 3.6MB of heap size!
And this is a fun one! A java implementation (streaming), taking 6.3s and using a whopping 196MB of peak heap!!! It's notable that the user time
is at 11.3s, as multiple threads are busy! The system time is highest, too, clocking in at 0.48s, compared to the 0.4 to 0.6 in the other
implementations.

Yet another implementation found was in Node, which managed to process everything in 2.3s, at a peak memory of
55MB. The implementation was rather massive at 583 and certainly unnecessarily complex.
