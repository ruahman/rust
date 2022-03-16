.PHONY: image
image: 
	docker build --target=dev -t rust-dev .
	docker build --target=debug -t rust-debug . 

.PHONY: rmi
rmi:
	docker image rm rust-dev
	docker image rm rust-debug
# ifdef clear
# 	docker image rm $(shell docker images -f "dangling=true" -q)
# endif


.PHONY: shell
shell:
	docker run -it -v $(shell pwd):/home/docker/app --rm rust-dev /bin/bash 

.PHONY: debug
debug:
	docker run -it -v $(shell pwd):/home/docker/app -p 1234:1234 --rm rust-debug /bin/bash 