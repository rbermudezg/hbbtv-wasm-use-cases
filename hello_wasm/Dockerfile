FROM rust:latest

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install wasm-pack

ENTRYPOINT [ "/bin/bash -c" ]
CMD ["wasm-pack build --target web"]