
FROM ekidd/rust-musl-builder:latest as builder
ADD --chown=rust:rust . ./
RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/ridit \
    /usr/local/bin/
CMD /usr/local/bin/ridit
