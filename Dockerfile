FROM alpine:3.24.1@sha256:bec4ccd3817e7c824eb0388971a0b83fab111d586285511ba0266b77e8dc65a9

ARG TARGET
COPY ./target/${TARGET}/release/monkey_interpreter /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["monkey_interpreter"]
