FROM rust AS build

WORKDIR /usr/src

COPY . .

RUN cargo build --release

RUN rm -rf src

FROM alpine:3.9.6

COPY --from=build /usr/src/target/release/service-accounts .

EXPOSE 443
EXPOSE 80

STOPSIGNAL SIGTERM

CMD [ "./service-accounts" ]