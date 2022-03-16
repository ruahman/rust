### dev
FROM rust:1.59 as dev

RUN useradd -ms /bin/bash docker
USER docker

WORKDIR /home/docker/app

### debug
FROM dev as debug

USER root
RUN apt update
RUN apt install -y cmake build-essential xz-utils
RUN apt install -y clang lldb

USER docker
EXPOSE 1234