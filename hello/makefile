.PHONY: image
image: 
	docker build --target=dev -t rust-dev .
	docker build --target=debug -t rust-debug . 

.PHONY: shell
shell:
	docker run -it -v $(shell pwd):/home/docker/app --rm rust-dev /bin/bash 

.PHONY: debug
debug:
	docker run -it -v $(shell pwd):/home/docker/app --rm rust-debug /bin/bash 