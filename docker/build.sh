#!/bin/bash

DOCKER_USERNAME="koreadmin"
DOCKER_REPO="kore-http"
TAG_ARRAY=("0.5-sqlite-prometheus" "0.5-leveldb-prometheus" "0.5-sqlite" "0.5-leveldb")
DOCKERFILE_ARRAY=("./docker/Dockerfile.sqlite" "./docker/Dockerfile.leveldb")
FEATURES_ARRAY=("doc sqlite prometheus" "doc leveldb prometheus" "doc sqlite" "doc leveldb")


# Iterar sobre los arrays de características y repositorios
for i in "${!FEATURES_ARRAY[@]}"; do
    FEATURES="${FEATURES_ARRAY[i]}"
    TAG="${TAG_ARRAY[i]}"

    # Seleccionar el archivo Dockerfile basado en las características
    if [[ "$FEATURES" == *"sqlite"* ]]; then
        DOCKERFILE="${DOCKERFILE_ARRAY[0]}"
    else
        DOCKERFILE="${DOCKERFILE_ARRAY[1]}"
    fi

    echo "######################################################################"
    echo "########################## $TAG #########################"
    echo "######################################################################"

    # Construir la imagen para AMD64
    echo ""
    echo "Construyendo la imagen para AMD64 con características: $FEATURES..."
    docker build --platform linux/amd64 --build-arg FEATURES="$FEATURES" -t ${DOCKER_USERNAME}/${DOCKER_REPO}:amd64-${TAG} --target amd64 -f $DOCKERFILE .

    # Construir la imagen para ARM64
    echo ""
    echo "Construyendo la imagen para ARM64 con características: $FEATURES..."
    docker build --platform linux/arm64 --build-arg FEATURES="$FEATURES" -t ${DOCKER_USERNAME}/${DOCKER_REPO}:arm64-${TAG} --target arm64 -f $DOCKERFILE .

    echo ""
    echo "Subiendo las imágenes a Docker Hub..."
    docker push ${DOCKER_USERNAME}/${DOCKER_REPO}:arm64-${TAG}
    docker push ${DOCKER_USERNAME}/${DOCKER_REPO}:amd64-${TAG}

    # Crear una imagen multi-arquitectura usando manifest
    echo ""
    echo "Creando imagen multi-arquitectura..."
    docker manifest create ${DOCKER_USERNAME}/${DOCKER_REPO}:${TAG} \
        --amend ${DOCKER_USERNAME}/${DOCKER_REPO}:amd64-${TAG} \
        --amend ${DOCKER_USERNAME}/${DOCKER_REPO}:arm64-${TAG}

    # Marcar la plataforma para cada arquitectura
    docker manifest annotate ${DOCKER_USERNAME}/${DOCKER_REPO}:${TAG} ${DOCKER_USERNAME}/${DOCKER_REPO}:amd64-${TAG} --os linux --arch amd64
    docker manifest annotate ${DOCKER_USERNAME}/${DOCKER_REPO}:${TAG} ${DOCKER_USERNAME}/${DOCKER_REPO}:arm64-${TAG} --os linux --arch arm64

    # Subir el manifiesto a Docker Hub
    echo ""
    echo "Subiendo el manifiesto a Docker Hub..."
    docker manifest push ${DOCKER_USERNAME}/${DOCKER_REPO}:${TAG}

    echo "Proceso completado para características: $FEATURES. Las imágenes han sido subidas a Docker Hub."
done

echo "######################################################################"
echo "############################## LIMPIANDO #############################"
echo "######################################################################"

for i in "${!TAG_ARRAY[@]}"; do
    TAG="${TAG_ARRAY[i]}"
    docker rmi ${DOCKER_USERNAME}/${DOCKER_REPO}:arm64-${TAG} ${DOCKER_USERNAME}/${DOCKER_REPO}:amd64-${TAG}
done