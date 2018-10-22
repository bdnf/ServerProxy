proxy:
	cargo run --bin proxy

server:
	cargo run --bin server

run:
	start_server
	start_proxy
