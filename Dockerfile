FROM gcr.io/distroless/static:nonroot

ARG TARGETPLATFORM

COPY target/${TARGETPLATFORM}/release/srok /srok

ENV RUST_LOG=debug \
    LEPTOS_OUTPUT_NAME=srok \
    LEPTOS_SITE_ADDR=0.0.0.0:3000

EXPOSE 3000
ENTRYPOINT ["/srok"]