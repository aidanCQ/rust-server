build: 
	./tailwindcss -i ./src/static/input.css -o ./src/static/output.css && cargo build --release
dev: 
	./tailwindcss -i ./src/static/input.css -o ./src/static/output.css --watch &
	cargo watch -x run
run: 
	./target/release/rust-server



