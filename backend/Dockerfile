FROM rust:alpine

WORKDIR /usr/src/app

RUN apk add --no-cache socat

# TODO: when the Rust source code is ready, replace this command for the Rust binary execution
# For now, this just echoes back the received request
CMD socat -v tcp-l:9000,fork exec:'/bin/cat'
