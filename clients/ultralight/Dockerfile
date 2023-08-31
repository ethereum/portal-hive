FROM node:18-alpine

RUN apk update && apk add --no-cache bash g++ make git python3 && rm -rf /var/cache/apk/*

RUN git clone --branch hive-interop-updates https://github.com/ethereumjs/ultralight.git 

WORKDIR /ultralight

RUN npm i --omit-dev

COPY . .


WORKDIR /
COPY ultralight.sh /ultralight.sh
RUN chmod +x /ultralight.sh

RUN echo "latest" > /version.txt

# Export the usual networking ports to allow outside access to the node
EXPOSE 8545 9000/udp

ENTRYPOINT ["/ultralight.sh"]
