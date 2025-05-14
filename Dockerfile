FROM gcr.io/distroless/cc-debian12

ARG TARGETPLATFORM

COPY target/${TARGETPLATFORM}/release/srok /srok

COPY target/site/pkg /pkg
COPY target/site/logos /logos
COPY target/site/favicon.ico /favicon.ico

ENV RUST_LOG=debug \
    LEPTOS_OUTPUT_NAME=srok \
    LEPTOS_SITE_ADDR=0.0.0.0:3000

EXPOSE 3000
ENTRYPOINT ["/srok"]