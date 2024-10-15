.PHONY: watch\:server watch\:test 

watch\:server:
	cargo watch -q -c -w src/ -x run

watch\:test: 
	cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
