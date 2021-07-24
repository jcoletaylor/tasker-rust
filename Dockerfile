FROM rust:1.53-buster as builder

ARG SVC=tasker
ARG RELEASE_LIBS_PREFIX=tasker
ARG PORT=8080

RUN USER=root cargo new --bin ${SVC}
WORKDIR /rusty_builder
COPY . .
RUN cargo build --release --bin ${SVC}
RUN rm -rf src/*

ADD . ./

RUN rm ./target/release/deps/${RELEASE_LIBS_PREFIX}*
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app
ARG SVC=tasker

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE ${PORT}

ENV TZ=Etc/UTC \
    APP_USER=rusty

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /rusty_builder/target/release/${SVC} ${APP}/${SVC}
RUN chmod 700 ${APP}/${SVC}
RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["/usr/src/app/tasker"]