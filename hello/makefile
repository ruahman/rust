.PHONY: image
image: 
	docker build --target=dev -t rust-dev .
	docker build --target=debug -t rust-debug . 

.PHONY: rmi-dev
rmi-dev:
	docker image rm rust-dev

.PHONY: rmi-debug
rmi-debug:
	docker image rm rust-debug

.PHONY: rmi-dangling
rmi-dangling:
	docker image rm $(shell docker images -f "dangling=true" -q)

.PHONY: rmi
rmi: rmi-dev rmi-debug


.PHONY: shell
shell:
	docker run -it -v $(shell pwd):/home/docker/app --rm rust-dev /bin/bash 

.PHONY: debug
debug:
	docker run --privileged -v $(shell pwd):/home/docker/app -p 1234:1234 -p 31200-31300:31200-31300 --rm -d rust-debug