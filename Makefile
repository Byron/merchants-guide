help:
	$(info -Targets -----------------------------------------------------------------------------)
	$(info answers                    | produce answers expected by the challenge)
	$(info -Development Targets -----------------------------------------------------------------)
	$(info journey-tests              | run all stateless journey test)
	$(info continuous-journey-tests   | run all stateless journey test whenever something changes)

always:

target/debug/guide: always
	cargo build

journey-tests: target/debug/guide
	./tests/stateless-journey.sh $<

answers: target/debug/guide
	$< tests/fixtures/input.txt

continuous-journey-tests:
	watchexec $(MAKE) journey-tests

