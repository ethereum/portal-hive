FROM node:18

RUN npm i -g npm@9.8.1

WORKDIR /get-block-by-hash

COPY ./hivesim-ts/ ./
COPY ./hivesim-ts/src ./src
COPY ./hivesim-ts/package*.json ./
COPY ./hivesim-ts/src/simulators ./src/simulators
COPY ./hivesim-ts/src/simulators/get-block-by-hash ./src/simulators/get-block-by-hash
COPY ./hivesim-ts/tsconfig.json ./
COPY ./hivesim-ts/eslint.cjs ./
COPY ./hivesim-ts/prettier.config.js ./
COPY ./hivesim-ts/tsconfig.lint.json ./
COPY ./hivesim-ts ./../../hivesim-ts

RUN npm i
RUN npm run build


COPY . .
RUN pwd
RUN ls -la
CMD node ./dist/simulators/get-block-by-hash/main.js

