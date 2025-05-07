# Makefile

# Lance les tests avec affichage stdout/stderr
test:
	cargo test -- --nocapture

build:
	cargo build

run:
	cargo run

clean:
	cargo clean
