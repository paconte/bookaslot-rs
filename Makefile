include .env

build-api:
	source .env && cd backend && $(MAKE) build

build-web:
	source .env && cd web && $(MAKE) build

build-web-dev:
	source .env && cd web && $(MAKE) build-dev

build: build-web build-api

build-dev: build-web-dev build-api

tag-images:
	docker tag bookaslot-web paconte/bookaslot-web:$(BOOKASLOT_VERSION)
	docker tag bookaslot-api paconte/bookaslot-api:$(BOOKASLOT_VERSION)

push-images: tag-images
	docker login
	docker push paconte/bookaslot-web:$(BOOKASLOT_VERSION)
	docker push paconte/bookaslot-api:$(BOOKASLOT_VERSION)

start-postgres:
	source .env && docker-compose start postgres

stop-postgres:
	docker-compose stop postgres

start:
	docker-compose --env-file ./.env up -d

start-dev:
	docker-compose -f docker-compose.dev.yaml up --build

psql:
	psql -h 0.0.0.0 -p 7000 -U $(POSTGRES_USER) -d $(POSTGRES_DB)