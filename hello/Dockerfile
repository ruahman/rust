### dev
FROM rust:1.59 as dev

RUN useradd -ms /bin/bash docker
USER docker

WORKDIR /home/docker/app

### debug
FROM dev as debug

USER root
RUN apt update
RUN apt install -y build-essential clang lldb

EXPOSE 1234
EXPOSE 31200-31300

USER docker

ENTRYPOINT lldb-server platform --listen "*:1234" --server --min-gdbserver-port 31200 --max-gdbserver-port 31300

#### builder
FROM dev as builder
USER root
COPY ./ . 
RUN cargo build --release

### production
FROM alpine as prod
WORKDIR /app
COPY --from=builder /home/docker/app/target/release/hello ./
CMD ["/app/hello"]