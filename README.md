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
* [x] support for benchmarking
* [x] support for linting
* [x] interactive developer environment in docker

## Benchmark Results

Even though the Rust Implementation is the fastest, it's not doing so by far.
The fastest python3 implementation clocks in at 2.3s, Rust at 1.03s.
Peak memory is 26MB and 3.9MB respectively.

Interestingly: as python as plenty of unit-level tests, it clocks in at 311 lines including tests,
and 162 without. However, Rust has 286 only for the implementation, and 33 lines of bash for journey tests.
