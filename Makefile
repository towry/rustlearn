
all:
	cargo run --example ${ARG}

.PYHON: test 

test:
	cargo test
