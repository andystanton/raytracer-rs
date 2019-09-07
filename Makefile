PHONY: docker-run

docker-run:
	docker run -it --rm \
		-e CARGO_HOME=/usr/src/rt/.cargo \
		-v $(CURDIR):/usr/src/rt -w /usr/src/rt \
		rust:1-slim \
			cargo run --release -- \
			--scene random \
			--nx 300 \
			--ny 200 \
			--samples-per-pixel 50 \
			-o ./out.png
