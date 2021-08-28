test:
	export AIAS_ISSUER_PRIVKEY="`openssl genrsa 2024`" && rm aias.db; touch aias.db && cargo test -- --nocapture 

run:
	export AIAS_DEBUG=false && cargo run
