.PHONY: clean distclean run docker_build docker_run stop mock

ROOT=$(shell pwd)
TMP=$(ROOT)/.tmp
DATA=$(TMP)/data
PROJECT=hlsocial
BUILD_OS=$(PROJECT)/debian:bookworm
BUILD_RUST=$(PROJECT)/debian-rust:bookworm
BUILD_PSQL=$(PROJECT)/postgres:15.6-bookworm
VERSION=$(shell git describe --tags --abbrev=10)
RUST_API=$(PROJECT)/api:$(VERSION)

clean: stop
	sudo rm -rf $(TMP)/postgresql

distclean: clean
	rm -rf $(TMP)


$(TMP):
	mkdir -p $(TMP)

$(TMP)/.debian12: $(TMP)
	docker build -f docker/Dockerfile.debian12 --tag $(BUILD_OS) .
	touch $@

$(TMP)/.builder.rust: $(TMP)/.debian12
	docker build -f docker/Dockerfile.build.rust --tag $(BUILD_RUST) --build-arg Version=$(VERSION) .
	touch $@

$(TMP)/.psql: $(TMP)
	docker build -f docker/Dockerfile.build.psql --tag $(BUILD_PSQL) .
	touch $@


docker_build: $(TMP)/.builder.rust $(TMP)/.psql

run:
	docker-compose -f docker/docker-compose.yml up -d db
	cd rust/$(PROJECT) && \
		export DB_HOST=$$(docker inspect  -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' db) && \
		export DB_NAME=$$(docker exec db env | grep POSTGRES_DB | awk -F= '{print $$2}') && \
		export DB_USER=$$(docker exec db env | grep POSTGRES_USER | awk -F= '{print $$2}') && \
		export DB_PASS=$$(docker exec db env | grep POSTGRES_PASSWORD | awk -F= '{print $$2}') && \
		cargo run

mock:


docker_run:
	docker-compose -f docker/docker-compose.yml up -d

stop:
	docker-compose -f docker/docker-compose.yml down

