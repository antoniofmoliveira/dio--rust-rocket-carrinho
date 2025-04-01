FROM rust:1.53.0-alpine as build
WORKDIR /usr/src/app
COPY . /usr/src/app
RUN cargo build --release

FROM alpine:latest
WORKDIR /usr/src/app
COPY --from=build /usr/src/app/target/release/rocket_simples ./
COPY --from=build /usr/src/app/Rocket.toml ./
COPY --from=build /usr/src/app/static ./
COPY --from=build /usr/src/app/templates ./
COPY --from=build /usr/src/app/db ./

EXPOSE 8000
CMD ["./rocket_simples"]
