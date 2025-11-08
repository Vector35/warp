FROM rust:1.77 AS builder
# The binaryninja-api commit to use, remember sigem is pulled from here.
ARG COMMIT=30a0fbb5839b08dc58efe602a5cd19c90fbd4c50
# Copy the binja install. This won't be included in the final image.
COPY ./binaryninja /binaryninja
ENV BINARYNINJADIR=/binaryninja
# Clone the binaryninja-api repo where `sigem` lives.
RUN apt-get update && apt-get install -y git libclang-dev
RUN git clone https://github.com/Vector35/binaryninja-api /binaryninja-api \
    && cd /binaryninja-api \
    && git checkout $COMMIT
# Install sigem
RUN cargo install --path /binaryninja-api/plugins/warp/ --bin sigem

FROM ubuntu:24.10 AS runner
# Copy sigem binary over
COPY --from=builder /usr/local/cargo/bin/sigem /usr/local/bin/sigem
# Copy in the entrypoint (what sets up the environment)
COPY ./entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]