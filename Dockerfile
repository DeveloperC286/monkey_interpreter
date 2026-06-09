FROM alpine:3.24.0@sha256:660e0827bd401543d81323d4886abbd08fda0fe3ba84337837d0b11a67251283

ARG TARGET
COPY ./target/${TARGET}/release/monkey_interpreter /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["monkey_interpreter"]
