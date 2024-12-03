# Nombre de la aplicación y de la imagen
APP_NAME=kata-barcode
DOCKER_IMAGE=kcramsolutions/$(APP_NAME)

# Arquitecturas a las que queremos construir la imagen
ARCHS=amd64,linux/arm64v8,linux/armv7

# Comando de docker buildx
DOCKER_BUILDX=docker buildx

# Etiquetas (tags) para la imagen
TAG_LATEST=latest
TAG_VERSION=1.0.0

# Archivo de Dockerfile
DOCKERFILE=Dockerfile

# Construcción multiplataforma
build:
	@echo "Construyendo la imagen para múltiples arquitecturas..."
	$(DOCKER_BUILDX) build --platform linux/$(ARCHS) -t $(DOCKER_IMAGE):$(TAG_LATEST) -t $(DOCKER_IMAGE):$(TAG_VERSION) -f $(DOCKERFILE) .
	@echo "Construcción completada."

# Para mostrar los entornos de arquitectura disponibles
platforms:
	@echo "Arquitecturas disponibles: $(ARCHS)"

# Para usar Docker Buildx en tu máquina si no lo tienes habilitado:
init:
	@echo "Inicializando el builder docker buildx..."
	$(DOCKER_BUILDX) create --use

# Empujar la imagen al repositorio
push:
	@echo "Subiendo imagen a Docker Hub..."
	docker push $(DOCKER_IMAGE):$(TAG_LATEST)
	docker push $(DOCKER_IMAGE):$(TAG_VERSION)
	@echo "Imagen subida exitosamente."

# Limpiar imágenes locales
clean:
	@echo "Eliminando imágenes locales..."
	docker rmi $(DOCKER_IMAGE):$(TAG_LATEST) $(DOCKER_IMAGE):$(TAG_VERSION)
	@echo "Imágenes locales eliminadas."

.PHONY: build push clean platforms init
