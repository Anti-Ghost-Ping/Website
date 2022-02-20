FROM rustlang/rust:nightly as builder
COPY . .
RUN cargo install --path .


FROM debian as runner
RUN apt-get update
RUN apt-get install libssl-dev
COPY --from=builder /usr/local/cargo/bin/agp_site /usr/local/bin/agp_site
ENV ROCKET_ADDRESS=0.0.0.0
ENV API_KEY=KeyHere
EXPOSE 8000
CMD ["agp_site"]