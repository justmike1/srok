FROM gcr.io/distroless/cc-debian12

ARG TARGETPLATFORM

WORKDIR /app

COPY target/${TARGETPLATFORM}/release/srok /app/srok

COPY target/site/logos /app/logos 
COPY target/site/pkg/*.css /app/pkg/srok.css
COPY target/site/pkg/*.js /app/pkg/srok.js
COPY target/site/pkg/*.wasm /app/pkg/srok_bg.wasm
COPY target/site/favicon.ico /app/favicon.ico

ENV RUST_LOG=debug \
    LEPTOS_OUTPUT_NAME=srok \
    LEPTOS_SITE_ROOT=. \
    LEPTOS_SITE_ADDR=0.0.0.0:3000

EXPOSE 3000
ENTRYPOINT ["/app/srok"]