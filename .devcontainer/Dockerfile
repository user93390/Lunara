FROM rust:alpine

ENV USER_ID=1000
ENV GROUP_ID=1000
ENV USER_NAME=lunara-developer
ENV GROUP_NAME=lunara-developer

ENV PROJECT_PATH=/home/lunara-developer/lunara

RUN addgroup -g $GROUP_ID $GROUP_NAME
RUN adduser --shell /bin/sh --disabled-password --uid $USER_ID --ingroup $GROUP_NAME $USER_NAME

RUN mkdir -p $PROJECT_PATH
COPY install.pm $PROJECT_PATH/install.pm

RUN apk add perl

WORKDIR $PROJECT_PATH

RUN perl install.pm
