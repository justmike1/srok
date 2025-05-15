FROM gcr.io/distroless/cc-debian12

ARG TARGETPLATFORM

WORKDIR /app

COPY target/${TARGETPLATFORM}/release/srok /app/srok

COPY target/site /app/site

ENV RUST_LOG=debug \
    LEPTOS_OUTPUT_NAME=srok \
    LEPTOS_SITE_ADDR=0.0.0.0:3000

EXPOSE 3000
ENTRYPOINT ["/app/srok"]