FROM rust:latest
RUN cargo install wasm-pack

# watch for compile
RUN apt-get update
RUN apt-get install inotify-tools -y
COPY ./watch.sh /sbin
RUN chmod 777 /sbin/watch.sh

WORKDIR /usr/src/myapp