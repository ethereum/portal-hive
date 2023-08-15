FROM node:18

RUN npm i -g npm@9.8.1

WORKDIR /rpc-compat-ts

COPY ./hivesim-ts/ ./
COPY ./hivesim-ts/package*.json ./
COPY ./hivesim-ts/simulators/rpc-compat-ts ./simulators/rpc-compat-ts
COPY ./hivesim-ts/tsconfig.json ./
COPY ./hivesim-ts ./../../hivesim-ts

RUN npm i
RUN npm run build

COPY . .
RUN pwd
RUN ls -la
CMD node ./dist/simulators/rpc-compat-ts/main.js

