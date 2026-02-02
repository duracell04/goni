SHELL := /usr/bin/env bash

COMPOSE_FILE := software/docker-compose.yml

.PHONY: up down logs doctor smoke demo bench test lint

up:
	docker compose -f $(COMPOSE_FILE) up -d

down:
	docker compose -f $(COMPOSE_FILE) down

logs:
	docker compose -f $(COMPOSE_FILE) logs -f

doctor:
	bash scripts/doctor.sh

smoke:
	bash scripts/smoke_test.sh

demo:
	bash scripts/demo.sh

bench:
	python goni-lab/goni_lab.py bench --scenario goni-lab/scenarios/mixed.json

test:
	bash scripts/smoke_test.sh

lint:
	@echo "lint not configured"
