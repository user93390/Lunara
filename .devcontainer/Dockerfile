FROM rust:alpine

ENV USER_ID=1000
ENV GROUP_ID=1000
ENV USER_NAME=lunara-developer
ENV GROUP_NAME=dev

ENV PROJECT_PATH=/home/lunara-developer/lunara

RUN addgroup -g $GROUP_ID $GROUP_NAME
RUN adduser --shell /bin/sh --disabled-password --uid $USER_ID --ingroup $GROUP_NAME $USER_NAME

RUN mkdir -p $PROJECT_PATH
COPY install.be $PROJECT_PATH/install.be

# Set up berry
RUN apk add gcc make readline git
RUN git clone https://github.com/berry-lang/berry.git && \
    cd berry && \
    make && \
    make install && \
    berry $PROJECT_PATH/install.be

WORKDIR $PROJECT_PATH