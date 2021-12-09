.env: init_env.sh .env.tmpl
	./init_env.sh

run: .env
	export $$(cat ./.env) && \
	cargo run

test: .env
	export $$(cat ./.env) && \
	cargo test
