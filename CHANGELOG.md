# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---
## [unreleased]

### Bug Fixes

- hgetall should return array - ([a98e0b2](https://github.com/lawff/simple-redis/commit/a98e0b2be4cb5683aa56d9046ab6f66cf4b7f38f)) - lawff

### Features

- support get/set, hget/hset/hgetall command - ([525115a](https://github.com/lawff/simple-redis/commit/525115a33b02f951f211e3e99cce9dbbbd6d1c7c)) - lawff
- support redis-server network layer - ([26b64ee](https://github.com/lawff/simple-redis/commit/26b64eecd20458543c9298de50b270ab9e407b2a)) - lawff
- add echo/hmget/sadd/smembers - ([1eddbfd](https://github.com/lawff/simple-redis/commit/1eddbfdeb4fd88d54f1056f3ced40936fd1f2706)) - lawff
- add unit test - ([d35a653](https://github.com/lawff/simple-redis/commit/d35a6532054ec3fedaaf6d6904ee8e75d644ca64)) - lawff

### Refactoring

- move each frame type to its own file - ([82698e2](https://github.com/lawff/simple-redis/commit/82698e203778a42239d7f2560e6dbb9b6bcaccc5)) - lawff
- del RespNullBulkString / RespNullArray - ([19135d6](https://github.com/lawff/simple-redis/commit/19135d6b1d3e8670c53b0443d323965953c10bbd)) - lawff

---
## [0.1] - 2024-05-07

### Features

- support frame encoding - ([108fd9c](https://github.com/lawff/simple-redis/commit/108fd9c7f97f98934329bb51801601fcd8479ff1)) - lawff
- support RespFrame decode - ([6ed7736](https://github.com/lawff/simple-redis/commit/6ed7736e1b65b03b6d4d1ea894e6a66f4198bc36)) - lawff

### Miscellaneous Chores

- init commit - ([7210873](https://github.com/lawff/simple-redis/commit/72108737ce494c0802806d4a0d1b0a45b45eeeb1)) - lawff

<!-- generated by git-cliff -->
