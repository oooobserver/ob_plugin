.PHONY: build test_file install test_dir

build:
	@cargo build -r 
	@cp target/release/ob_plugin ob

test_file:
	@make build  
	@./ob ../../Obsidian/Computer\ Network/HTTP/HTTP2.md

test_dir:
	@make build  
	@./ob ../../Obsidian/Computer\ Network/

install:
	@make build
	@sudo mv ob /usr/local/bin/ob
