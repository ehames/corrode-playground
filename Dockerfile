FROM ubuntu:16.04
RUN apt-get update && \
    apt-get install -y --no-install-recommends libgmp-dev libffi-dev gcc
COPY corrode /usr/local/bin/
COPY static /webapp
COPY corrode_playground /usr/local/bin

EXPOSE 6767
ENTRYPOINT /usr/local/bin/corrode_playground
