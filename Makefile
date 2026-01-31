# Variables
DOCKER_COMPOSE = docker-compose

.PHONY: run-env stop-env clean-env

## run-env: Launching ClickHouse in Docker
run-env:
	@echo "🚀 Launching the ClickHouse environment..."
	$(DOCKER_COMPOSE) up -d
	@echo "✅ The environment is running. ClickHouse is available on ports 9000 (TCP) and 8123 (HTTP)."

## stop-env: Stop containers
stop-env:
	@echo "🛑 Stopping the environment..."
	$(DOCKER_COMPOSE) stop

## clean-env: Complete cleaning (removal of containers and data)
clean-env:
	@echo "🧹 Cleaning up data and containers..."
	$(DOCKER_COMPOSE) down -v
