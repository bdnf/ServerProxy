FROM rust:1.23.0

COPY ./web_proxy ./web_proxy
WORKDIR web_proxy
RUN cargo build --release
#RUN cargo run
CMD ["cargo run"]
