ARG branch=21.11.22

FROM ogenev/fluffy:$branch

ADD fluffy.sh /fluffy.sh
RUN chmod +x /fluffy.sh

RUN echo "21.11.22" > /version.txt

EXPOSE 8545 9009/udp

ENTRYPOINT ["/fluffy.sh"]
