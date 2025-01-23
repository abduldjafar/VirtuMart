# Define variables
APP_NAME := virtumart
BUILD_DIR := .
DOCKER_IMAGE := virtumart-backend
DOCKER_TAG := latest
DOCKERFILE_DIR := ./infrastructure
DOCKER_COMPOSE_YML := ./infrastructure/docker-compose.yml
DOCKER_BUILDKIT=1
PLATFORM := linux/amd64

# Build the Docker image
build:
	@echo "Building Docker image for amd64 architecture..."
	docker build --platform $(PLATFORM) -f $(DOCKERFILE_DIR)/Dockerfile -t $(DOCKER_IMAGE):$(DOCKER_TAG) $(BUILD_DIR)

# Start Docker containers using docker-compose
up:
	@echo "Starting Docker containers with docker-compose..."
	docker-compose -f $(DOCKER_COMPOSE_YML) up --build

# Start Docker containers in detached mode
up-detached:
	@echo "Starting Docker containers with docker-compose (detached)..."
	docker-compose -f $(DOCKER_COMPOSE_YML) up -d --build

# Stop and remove Docker containers
down:
	@echo "Stopping and removing Docker containers..."
	docker-compose -f $(DOCKER_COMPOSE_YML) down

# Clean up unused images
clean:
	@echo "Removing old Docker images..."
	docker image prune -f

# Show logs from all containers
logs:
	@echo "Showing logs from all containers..."
	docker-compose -f $(DOCKER_COMPOSE_YML) logs -f

# Rebuild Docker image
rebuild: clean build

# Show status all containers
show:
	@echo "Showing status all containers..."
	docker ps -a | grep virtumart

# Show help
help:
	@echo "Makefile for Rust Application"
	@echo "Commands:"
	@echo "  make build        - Build the Docker image"
	@echo "  make run          - Run the Docker container"
	@echo "  make clean        - Clean up unused Docker images"
	@echo "  make rebuild      - Clean and rebuild the Docker image"
	@echo "  make up           - Start Docker containers using docker-compose"
	@echo "  make up-detached  - Start Docker containers using docker-compose in detached mode"
	@echo "  make down         - Stop and remove Docker containers"
	@echo "  make logs         - Show logs from all containers"
	@echo "  make help         - Show this help message"
