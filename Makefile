test:
	rm aias.db; touch aias.db && cargo test -- --nocapture 

run:
	export AIAS_DEBUG=false && cargo run
