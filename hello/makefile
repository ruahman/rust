.PHONY: image
image: 
	docker build --target=development -t rust-dev .
	docker build --target=debug -t rust-debug .
	docker build --target=production -t rust-prod .

.PHONY: rmi-dev
rmi-dev:
	docker image rm rust-dev

.PHONY: rmi-debug
rmi-debug:
	docker image rm rust-debug

.PHONY: rmi-prod
rmi-prod:
	docker image rm rust-prod

.PHONY: rmi-dangling
rmi-dangling:
	docker image rm $(shell docker images -f "dangling=true" -q)

.PHONY: rmi
rmi: rmi-dev rmi-debug rmi-prod rmi-dangling

.PHONY: shell
shell:
	docker run -it -v $(shell pwd):/home/docker/app --rm rust-dev /bin/bash 

.PHONY: debug
debug:
	docker run --privileged -v $(shell pwd):/home/docker/app -p 1234:1234 -p 31200-31300:31200-31300 --rm -d rust-debug