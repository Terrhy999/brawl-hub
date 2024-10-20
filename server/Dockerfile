FROM debian:bookworm-slim

# Install necessary dependencies (if any)
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && apt-get install -y curl \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /server

COPY ./target/release/server .

EXPOSE 3030

CMD [ "./server" ]