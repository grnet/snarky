#!/bin/bash

usage_string="usage: ./$(basename "$0") [OPTIONS]

Run snarky-ceremonies-dev container with installed prerequisites (Debian)

Options:
--build   	(Re-)Build image before running the container
 -h, --help	Display help message and exit

Examples:
 ./$(basename "$0")
 ./$(basename "$0") --build
"

usage() { echo -n "$usage_string" 1>&2; }

BUILD=false
NAME=snarky-dev
IMAGE="${NAME}"
CONTAINER="${NAME}"
WORKDIR="${NAME}"	# See Dockerfile

while [[ $# -gt 0 ]]
do
	arg="$1"
	case $arg in
		--build)
			BUILD=true
			shift
			;;
		-h|--help)
			usage
			exit 0
			;;
		*)
			echo "[-] Invalid option: $arg"
			echo
			usage
			exit 1
			;;
	esac
done

docker container rm "$CONTAINER"
if [ ${BUILD} = true ]; then
	docker image build -t "$IMAGE" .
fi
docker run --name "$CONTAINER" -v "$PWD":/"$WORKDIR" -it "$IMAGE":latest
