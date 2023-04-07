FROM httpd:2.4.55-bullseye

ARG PROJECT_ROOT

RUN chown -R www-data:www-data /usr/local/apache2

USER www-data:www-data

CMD ["httpd-foreground"]
