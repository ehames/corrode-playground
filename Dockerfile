FROM cpbase:latest
COPY corrode /usr/local/bin/
COPY static /webapp
COPY corrode_playground /usr/local/bin

EXPOSE 6767
ENTRYPOINT /usr/local/bin/corrode_playground
