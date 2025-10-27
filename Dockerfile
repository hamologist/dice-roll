FROM rust:1.90

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

RUN apt update

RUN apt install tini

ENTRYPOINT [ "tini", "--" ]

CMD [ "dice-roll-api" ]
