FROM rust:1.58
EXPOSE 7878
COPY ./ ./
ENV RUST_LOG=trace
RUN cargo build --release
CMD ["./target/release/freudeman-blog"]