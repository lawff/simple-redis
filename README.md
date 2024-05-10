# simple-redis

一个简单的 redis server 实现。

## 开启服务

```bash
RUST_LOG=info cargo run
```

### 命令

```bash
redis-cli -p 6379
```

### 实现功能

- get // get key
- set // set key value
- hget // hget key field
- hmget // hmget key field1 field2 ...
- hgetall // hgetall key
- hset // hset key field value
- echo // echo message
- sadd // sadd key member1 member2 ...
- smembers // smembers key
