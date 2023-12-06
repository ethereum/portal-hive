FROM node:18

RUN npm i -g npm@9.8.1

WORKDIR /ts-sim-test

COPY ./hivesim-ts/ ./
COPY ./hivesim-ts/src ./src
COPY ./hivesim-ts/package*.json ./
COPY ./hivesim-ts/src/simulators ./src/simulators
COPY ./hivesim-ts/src/simulators/ts-sim-test ./src/simulators/ts-sim-test
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
CMD node ./dist/simulators/ts-sim-test/main.js

