# cd ngnix
# docker build -t nck974/where-have-i-been-nginx:1.0.0 .
# docker push nck974/where-have-i-been-nginx:1.0.0
FROM nginx:nginx:1.27.1-alpine
COPY nginx.conf /etc/nginx/nginx.conf
COPY ssl.conf /etc/nginx/conf.d/ssl.conf