#!/bin/bash

if [ -z "$1" ] | [ -z "$2" ]; then
    echo use: $0 SOURCE_TAG DESTINATION_TAG
    echo SOURCE_TAG:      current running TAG
    echo DESTINATION_TAG: future running TAG after running this script
    echo example:         $0 0.0.1 0.0.2
    exit
fi


#####################
##### variables #####
#####################


SOURCE_TAG=$1
DESTINATION_TAG=$2

ROOT_IMAGE="paconte/rs-reservation-api"
SOURCE_IMAGE_TAG="$ROOT_IMAGE:$SOURCE_TAG"
DESTINATION_IMAGE_TAG="$ROOT_IMAGE:$DESTINATION_TAG"

ENV_VARIABLES_FILE=".env"


#####################
##### functions #####
#####################

load_environment_variables () {
    if [ ! -f "$ENV_VARIABLES_FILE" ]; then
        echo "ERROR, $ENV_VARIABLES_FILE does not exist."
        exit 1
    fi
    source $ENV_VARIABLES_FILE
}


purge_old_software_version () {
    if [[ -z "${ $CONTAINER_ID}" ]]; then
        echo Stoping conainer  $CONTAINER_ID
        docker stop $CONTAINER_ID

	echo Removing container $CONTAINER_ID
	docker rm $CONTAINER_ID
    fi

    if [[ -z "${ $IMAGE_ID}" ]]; then
        echo Removing image $IMAGE_ID
        docker image rm $IMAGE_ID
    fi
}


#####################
####### main ########
#####################


echo Proceding to remove running version $SOURCE_IMAGE_TAG and run version $DESTINATION_IMAGE_TAG


echo Updating $ENV_VARIABLE_FILE file
sed -i '/RESERVATIONS_API_VERSION=/c\RESERVATIONS_API_VERSION='"$DESTINATION_TAG"'' $ENV_VARIABLES_FILE
cat $ENV_VARIABLES_FILE


echo Loading environment variables from .env file
load_environment_variables


echo Docker login
docker login


echo Setting up variables
CONTAINER_NAME="reservation-api"
CONTAINER_ID=$(docker ps -aqf "name=${CONTAINER_NAME}" -f status=running)
IMAGE_ID=$(docker inspect $CONTAINER_ID --format="{{.Image}}")
CONTAINER_NETWORK=$(docker inspect $CONTAINER_ID --format="{{ .HostConfig.NetworkMode }}")


echo Purge old software version
purge_old_software_version


echo Printing docker containers
docker ps


echo Running database migrations
docker run --rm -v "$PWD:/postgres-data" -w /postgres-data --network=$CONTAINER_NETWORK -e DATABASE_URL="${DOCKER_DATABASE_URL}" -it clux/diesel-cli diesel migration run


echo Starting new container with image $DESTINATION_IMAGE_TAG
#docker run -dp 8000:8000 --restart always --name $CONTAINER_NAME $DESTINATION_IMAGE_TAG
docker-compose down
docker-compose --env-file ./.env up -d
