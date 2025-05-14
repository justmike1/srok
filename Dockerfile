FROM gcr.io/distroless/static:nonroot

USER root:root

WORKDIR /app

COPY target/aarch64-unknown-linux-gnu/release/srok /app/srok

ENV RUST_LOG=debug \
    LEPTOS_OUTPUT_NAME=srok \
    LEPTOS_SITE_ADDR=0.0.0.0:3000

RUN chmod 777 /app/srok

USER nonroot:nonroot

EXPOSE 3000
ENTRYPOINT ["/app/srok"]