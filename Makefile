fixture = tests/fixtures/input.txt
bench_fixture = tests/fixtures/big-input.txt
docker_image = guide_docker_developer_environment

help:
	$(info -Targets -----------------------------------------------------------------------------)
	$(info answers                      | produce answers expected by the challenge)
	$(info answers-in-docker            | as above, but uses docker for those without Rust)
	$(info -- Use docker for all dependencies - run make interactively from there ----------------)
	$(info interactive-developer-environment-in-docker | gives you everything you need to run all targets)
	$(info -Development Targets -----------------------------------------------------------------)
	$(info lint                         | run lints with clippy)
	$(info benchmark                    | just for fun, really)
	$(info journey-tests                | run all stateless journey test)
	$(info continuous-journey-tests     | run all stateless journey test whenever something changes)

always:

interactive-developer-environment-in-docker:
	docker build -t $(docker_image) - < etc/developer.Dockerfile
	docker run -v $$PWD:/volume -w /volume -it $(docker_image)

target/debug/guide: always
	cargo build

target/release/guide: always
	cargo build --release

lint:
	cargo clippy

benchmark: target/release/guide
	hyperfine '$< $(bench_fixture)'

journey-tests: target/debug/guide
	./tests/stateless-journey.sh $<

answers: target/debug/guide
	$< $(fixture)

answers-in-docker:
	docker run -v $$PWD:/volume -w /volume rust make answers

continuous-journey-tests:
	watchexec $(MAKE) journey-tests

