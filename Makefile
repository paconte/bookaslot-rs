include .env

build-api-prod:
	source .env && cd backend && $(MAKE) build

build-web-prod:
	source .env && cd web && $(MAKE) build

build-web-dev:
	source .env && cd web && $(MAKE) build-dev

build-prod: build-web-prod build-api-prod

build-dev: build-web-dev build-api-prod

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

start-prod:
	docker-compose --env-file ./.env -f docker-compose.prod.yaml up -d

start-dev:
	docker-compose up --build

psql:
	psql -h 0.0.0.0 -p 7000 -U $(POSTGRES_USER) -d $(POSTGRES_DB)