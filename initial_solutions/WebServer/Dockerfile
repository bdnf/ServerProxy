FROM rust:1.23.0

COPY ./web_server ./web_server
WORKDIR web_server
RUN cargo build --release
#RUN cargo run
CMD ["cargo run"]
