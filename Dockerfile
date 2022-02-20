#
# radio-thermostat-exporter
#

# Multi-stage dockerfile

#
# First we build the project statically
FROM ekidd/rust-musl-builder as builder

WORKDIR /home/rust/

COPY . .
RUN cargo build --release

#
# Now we create the runtime container
FROM scratch

COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/radio-thermostat-exporter .
ENTRYPOINT [ "./radio-thermostat-exporter" ]
