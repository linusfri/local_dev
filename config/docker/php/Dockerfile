FROM php:8.3-fpm

#Install dependencies
RUN docker-php-ext-install mysqli pdo pdo_mysql && \
	apt update && apt install -y nginx

ARG USER_ID
ARG GROUP_ID

COPY . /var/www/html

RUN if [ ${USER_ID:-0} -ne 0 ] && [ ${GROUP_ID:-0} -ne 0 ]; then \
  userdel -f www-data &&\
  if getent group www-data ; then groupdel www-data; fi &&\
  groupadd -g ${GROUP_ID} www-data &&\
  useradd -l -u ${USER_ID} -g www-data www-data &&\
  install -d -m 0755 -o www-data -g www-data /home/www-data; fi

RUN chown -R www-data:www-data /var/www/html && \
	rm /etc/nginx/sites-available/default && \
	rm /etc/nginx/sites-enabled/default

COPY ./default.conf /etc/nginx/conf.d/default.conf
COPY ./entrypoint-prod.sh /
COPY ./php.ini $PHP_INI_DIR/php.ini

RUN chmod +x /entrypoint-prod.sh

ENTRYPOINT [ "/entrypoint-prod.sh" ]
