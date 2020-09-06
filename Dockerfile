FROM rust:1.46.0
LABEL maintainer="abhi"

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .
EXPOSE 8000
CMD ["tauceti"]