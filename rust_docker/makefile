.PHONY: image-dev
image-dev:
ifndef rmi
	docker build --target=development -t rust-dev .
else
	docker image rm rust-dev
endif

.PHONY: image-debug
image-debug:
ifndef rmi
	docker build --target=debug -t rust-debug .
else
	docker image rm rust-debug
endif

.PHONY: image-prod
image-prod:
ifndef rmi
	docker build --target=production -t rust-prod .
else
	docker image rm rust-prod
endif

.PHONY: shell
shell:
	docker run -it -v $(shell pwd):/home/docker/app --rm rust-dev /bin/bash 

.PHONY: debug
debug:
	docker run --privileged -v $(shell pwd):/home/docker/app -p 1234:1234 -p 31200-31300:31200-31300 --rm -d rust-debug