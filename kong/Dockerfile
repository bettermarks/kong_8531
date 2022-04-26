FROM kong:2.7.1

USER root
RUN apk add --update --no-cache nodejs npm python2 make g++ python3 strace
RUN npm install --unsafe -g kong-pdk@0.5.3

USER kong
