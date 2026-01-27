FROM rust:alpine

ENV USER_ID=1000
ENV GROUP_ID=1000
ENV USER_NAME=lunara-developer
ENV GROUP_NAME=dev

ENV PROJECT_PATH=/home/lunara-developer/lunara

RUN addgroup -g $GROUP_ID $GROUP_NAME
RUN adduser --shell /bin/sh --disabled-password --uid $USER_ID --ingroup $GROUP_NAME $USER_NAME



COPY install.be $PROJECT_PATH/install.be

# Set up berry
RUN apk add gcc make readline git
RUN git clone https://github.com/berry-lang/berry.git

RUN cd berry \ RUN mkdir -p $PROJECT_PATH \ make \ make install \ berry install.be

WORKDIR $PROJECT_PATH