################################################################################
## Builder
################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=webmocket
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /webmocket

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

################################################################################
## Final image
################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /webmocket

# Copy our build
COPY --from=builder /webmocket/target/x86_64-unknown-linux-musl/release/webmocket ./

# Use an unprivileged user.
USER webmocket:webmocket

EXPOSE 3000/tcp

CMD ["/webmocket/webmocket"]
