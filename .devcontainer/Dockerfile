FROM rust:alpine

ENV USER_ID=1000
ENV GROUP_ID=1000
ENV USER_NAME=lunara-developer
ENV GROUP_NAME=dev

ENV PROJECT_PATH=/home/lunara-developer/lunara

RUN addgroup -g $GROUP_ID $GROUP_NAME
RUN adduser --shell /bin/sh --disabled-password --uid $USER_ID --ingroup $GROUP_NAME $USER_NAME

RUN mkdir -p $PROJECT_PATH

COPY install_libs.sh $PROJECT_PATH/install_libs.sh

WORKDIR $PROJECT_PATH

RUN chmod +x install_libs.sh
RUN ./install_libs.sh