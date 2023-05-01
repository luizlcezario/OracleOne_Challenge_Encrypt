NAME = docs


all: $(NAME)

$(NAME): deploy
	cp -r ./front_end_simple/* docs
	cp -r ./rust_front_end/dist/* docs/rust
	
deploy:
	cd rust_front_end && trunk build --release --public-url=/OracleOne_Challenge_Encrypt/rust

clean: 
	rm -rf docs

.PHONY: all clean deploy
