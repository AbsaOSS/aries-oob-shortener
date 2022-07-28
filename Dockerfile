FROM alpine:3.16.1 AS builder

ARG UID=1000
ARG GID=1000

ENV RUST_LOG=warning

RUN addgroup -g $GID my_group && adduser -u $UID -D -G my_group my_user

RUN apk update && apk upgrade && \
    apk add --no-cache \
        build-base \
        curl

USER my_user
WORKDIR /home/my_user

ARG RUST_VER="1.59.0"
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $RUST_VER --default-host x86_64-unknown-linux-musl
ENV PATH="/home/my_user/.cargo/bin:$PATH" RUSTFLAGS="-C target-feature=-crt-static"

COPY . .
RUN cargo build --release --bin dlt_shortener

FROM alpine:3.16.1 AS runtime

COPY --from=builder /home/my_user/target/release/dlt_shortener dlt_shortener
ENTRYPOINT ["./dlt_shortener"]
