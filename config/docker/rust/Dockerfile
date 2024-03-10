FROM rust as builder

ARG USER_ID
ARG GROUP_ID

RUN if [ ${USER_ID:-0} -ne 0 ] && [ ${GROUP_ID:-0} -ne 0 ]; then \
    userdel -f www-data &&\
    if getent group www-data ; then groupdel www-data; fi &&\
    groupadd -g ${GROUP_ID} www-data &&\
    useradd -l -u ${USER_ID} -g www-data www-data &&\
    install -d -m 0755 -o www-data -g www-data /home/www-data; fi

WORKDIR /app

RUN chown -R www-data:www-data /app \
    && mkdir /target \
    && chown -R www-data:www-data /target

USER www-data

RUN cargo install cargo-watch

EXPOSE 8080

CMD ["cargo", "watch", "-x", "run"]
