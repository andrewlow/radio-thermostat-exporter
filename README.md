# radio-thermostat-exporter
Prometheus exporter for CT30, CT80, and the Filtrete 3M50

## Getting started

I've used a Makefile based approach. Read my [blog](https://lowtek.ca/roo/2021/managing-docker-containers-with-makefiles/) for details.

- Clone the repo.
- Create a `config.mk` file, a template is provided
- `make build`
- `make start`

To pull the latest git changes and rebuild your image
- `make update`

## Port

The exporter is on `localhost:9864/metrics` - but the code is permissive and will answer any web request with the metric payload.

## References

Written in Rust as a learning exercise.

- https://doc.rust-lang.org/book/ch20-01-single-threaded.html
- https://rust-lang-nursery.github.io/rust-cookbook/web/clients/requests.html
- https://blog.sedrik.se/posts/my-docker-setup-for-rust/

## [MIT License](LICENSE)

The scope of the license is for the contents of this repository, there are dependencies which are covered by the licenses specified by those projects.
