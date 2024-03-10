FROM php:8.3-fpm

#Install dependencies
RUN docker-php-ext-install mysqli pdo pdo_mysql

ARG USER_ID
ARG GROUP_ID

WORKDIR /var/www/html

RUN if [ ${USER_ID:-0} -ne 0 ] && [ ${GROUP_ID:-0} -ne 0 ]; then \
  userdel -f www-data &&\
  if getent group www-data ; then groupdel www-data; fi &&\
  groupadd -g ${GROUP_ID} www-data &&\
  useradd -l -u ${USER_ID} -g www-data www-data &&\
  install -d -m 0755 -o www-data -g www-data /home/www-data; fi

RUN chown -R www-data:www-data /var/www/html

USER www-data
