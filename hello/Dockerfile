### dev
FROM rust:1.59 as dev

RUN useradd -ms /bin/bash docker
USER docker

WORKDIR /home/docker/app

### debug
FROM dev as debug

USER root
RUN apt-get update; \
    apt-get install gdbserver;

USER docker
EXPOSE 1234