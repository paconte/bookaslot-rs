####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-gnu
RUN apt update
RUN update-ca-certificates

# Create appuser
ENV USER=app
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-gnu --release
####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-slim

RUN apt update && apt install --no-install-recommends -y libpq-dev

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/reservations-api ./

# Use an unprivileged user.
USER app:app

CMD ["/app/reservations-api"]