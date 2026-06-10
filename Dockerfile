FROM alpine:3.24.0@sha256:8ddefa941e689fc29abcdeb8dae3b3c6d139cc08ce9a52633931160701770685

ARG TARGET
COPY ./target/${TARGET}/release/monkey_interpreter /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["monkey_interpreter"]
