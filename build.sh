#!/usr/bin/env bash

GIT_TAG=${GIT_TAG:=$(git tag -l --contains HEAD)}
GIT_BRANCH=${GIT_BRANCH:=$(git rev-parse --abbrev-ref HEAD)}
GIT_COMMIT=$(git rev-parse HEAD)
GIT_REF=${GIT_TAG:=$GIT_BRANCH}
GIT_REF=${GIT_REF:=$GIT_COMMIT}
DOCKER_REPO=${DOCKER_REPO:-hub.docker.com}
DOCKER_IMAGE=${DOCKER_IMAGE:-"$(basename `git rev-parse --show-toplevel`)"}
DOCKER_TAG=${GIT_REF//[^a-zA-Z0-9]/-}
DOCKER_FULL=${DOCKER_REPO}/${DOCKER_IMAGE}:${DOCKER_TAG}
DOCKER_FILE=${DOCKER_FILE:-"Dockerfile"}
DOCKER_COMPOSE_FILE=${DOCKER_COMPOSE_FILE:-"docker-compose.yaml"}
DOCKER_COMPOSE_WAIT_COMMAND=${DOCKER_COMPOSE_WAIT_COMMAND:-""}
MAVEN_TASKS=${MAVEN_TASKS:-"clean package"}
GRADLE_TASKS=${GRADLE_TASKS:-"build"}

print_values() {
  echo ""
  echo "Git referece used: ${GIT_REF}"
  echo "Docker final image: ${DOCKER_FULL}"
  echo ""
}

build() {
  if [ -f ${DOCKER_COMPOSE_FILE} ]; then
    docker compose down
    while ! docker compose pull; do sleep .1; done
    docker compose up -d
    ${DOCKER_COMPOSE_WAIT_COMMAND}
  fi

  cargo test
  cargo sqlx prepare

  if [ -f ${DOCKER_COMPOSE_FILE} ]; then
    docker compose down
  fi
}

docker_build() {
  if [ -f ${DOCKER_FILE} ]; then
    docker build . -f $DOCKER_FILE --build-arg APP_VERSION=${GIT_TAG:latest} -t $DOCKER_FULL
  fi
}

docker_logged_in() {
  echo $(cat ~/.docker/config.json | grep "${DOCKER_REPO}")
}

docker_login() {
  RES=$(docker_logged_in)
  if [ -z "$RES" ] && [ ! -z "${DOCKER_REPO_USER}" ] && [ ! -z "${DOCKER_REPO_PASS}" ]; then
    docker login -u ${DOCKER_REPO_USER} -p ${DOCKER_REPO_PASS} ${DOCKER_REPO}
  fi
}

docker_push() {
  docker inspect $DOCKER_FULL > /dev/null 2>&1 || return
  docker_login
  RES=$(docker_logged_in)
  if [ ! -z "$RES" ]; then
    docker push $DOCKER_FULL
  fi
}

print_values
build
docker_build
if [ "$1" == "push" ]; then
  docker_push
fi
print_values

exit 0
