#!/bin/bash

if [ -z "$1" ] | [ -z "$2" ]; then
    echo use: $0 SOURCE_TAG DESTINATION_TAG
    echo SOURCE_TAG:      current running TAG
    echo DESTINATION_TAG: future running TAG after running this script
    echo example:         $0 0.0.1 0.0.2
    exit
fi

SOURCE_TAG=$1
DESTINATION_TAG=$2

ROOT_IMAGE="paconte/rs-reservation-api"
SOURCE_IMAGE_TAG="$ROOT_IMAGE:$SOURCE_TAG"
DESTINATION_IMAGE_TAG="$ROOT_IMAGE:$DESTINATION_TAG"

echo Proceding to remove running version $SOURCE_IMAGE_TAG and run version $DESTINATION_IMAGE_TAG

docker login
CONTAINER_NAME="rs_reservation_api"
IMAGE_ID=$(docker inspect --format="{{.Image}}" $CONTAINER_NAME)
CONTAINER_ID=$(docker ps -aqf "name=${CONTAINER_NAME}")

echo Stoping container $CONTAINER_ID
docker stop $CONTAINER_ID

echo Removing container $CONTAINER_ID
docker rm $CONTAINER_ID

echo Removing image $IMAGE_ID
docker image rm $IMAGE_ID

echo Starting new container with image $DESTINATION_IMAGE_TAG
docker run -dp 8000:8000 --restart always --name $CONTAINER_NAME $DESTINATION_IMAGE_TAG