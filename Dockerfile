FROM alpine:3.23.2@sha256:865b95f46d98cf867a156fe4a135ad3fe50d2056aa3f25ed31662dff6da4eb62

ARG TARGET
COPY ./target/${TARGET}/release/monkey_interpreter /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["monkey_interpreter"]
