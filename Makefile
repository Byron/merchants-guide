fixture = tests/fixtures/input.txt
bench_fixture = tests/fixtures/big-input.txt

help:
	$(info -Targets -----------------------------------------------------------------------------)
	$(info answers                    | produce answers expected by the challenge)
	$(info -Development Targets -----------------------------------------------------------------)
	$(info benchmark                  | just for fun, really)
	$(info journey-tests              | run all stateless journey test)
	$(info continuous-journey-tests   | run all stateless journey test whenever something changes)

always:

target/debug/guide: always
	cargo build

target/release/guide: always
	cargo build --release

benchmark: target/release/guide
	hyperfine '$< $(bench_fixture)'

journey-tests: target/debug/guide
	./tests/stateless-journey.sh $<

answers: target/debug/guide
	$< $(fixture)

continuous-journey-tests:
	watchexec $(MAKE) journey-tests

