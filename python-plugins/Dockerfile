FROM python:3.9

RUN apt-get update -y
RUN pip install shiv

ADD entrypoint.sh /entrypoint.sh
WORKDIR /src

ENTRYPOINT [ "/entrypoint.sh" ]
