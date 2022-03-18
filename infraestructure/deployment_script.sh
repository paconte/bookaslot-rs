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

ENV_VARIABLES_FILE=".env"


echo Proceding to remove running version $SOURCE_IMAGE_TAG and run version $DESTINATION_IMAGE_TAG


echo Updating $ENV_VARIABLE_FILE file
sed -i '/RESERVATIONS_API_VERSION=/c\RESERVATIONS_API_VERSION='"$DESTINATION_TAG"'' $ENV_VARIABLES_FILE
echo $RESERVATIONS_API_VERSION
echo $DESTINATION_TAG
cat $ENV_VARIABLES_FILE


echo Loading environment variables from .env file
load_environment_variables


echo Docker login
docker login


echo Setting up variables
CONTAINER_NAME="rs_reservation_api"
IMAGE_ID=$(docker inspect --format="{{.Image}}" $CONTAINER_NAME)
CONTAINER_ID=$(docker ps -aqf "name=${CONTAINER_NAME}")


echo Purge old software version
purge_old_software_version


echo Printing docker containers
docker ps


echo Starting new container with image $DESTINATION_IMAGE_TAG
#docker run -dp 8000:8000 --restart always --name $CONTAINER_NAME $DESTINATION_IMAGE_TAG
docker-compose down
docker-compose --env-file ./.env up -d


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
