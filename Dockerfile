FROM rust:1.51

RUN apt-get update && apt-get install -y \
	vim \
	git 

WORKDIR /snarky-dev

COPY . .
