# Web-Boilerplate

[![build badge](https://github.com/LJason77/Web-Boilerplate/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/LJason77/Web-Boilerplate/actions/workflows/rust.yml)
![GitHub forks](https://img.shields.io/github/forks/LJason77/Web-Boilerplate?style=social)
![GitHub Repo stars](https://img.shields.io/github/stars/LJason77/Web-Boilerplate?style=social)

这是一个 Web 后端模板。

## 搭建

复制 `.env.example` 为 `.env`，并更新为自己的配置。

### 生成密钥

```shell
# 生成私钥
openssl ecparam -name secp384r1 -genkey -noout -out jwtES384key.pem
# 生成公钥
openssl ec -in jwtES384key.pem -pubout -out src/models/public_ecdsa_key.pem
# 转换为 PKCS8
openssl pkcs8 -topk8 -nocrypt -in jwtES384key.pem -out src/models/private_ecdsa_key.pem
```

### 创建数据库

```shell
docker run -d --name mongo --restart always -e MONGO_INITDB_ROOT_USERNAME=mongo -e MONGO_INITDB_ROOT_PASSWORD=mongo -v $(pw)/mongo-init.js:/docker-entrypoint-initdb.d/mongo-init.js -v ~/.db/mongo:/data/db -p 27017:27017 mongo --wiredTigerCollectionBlockCompressor zstd
```

### 部署到服务器

复制部署脚本 `build/deploy.sh.example` 为 `build/deploy.sh`。脚本里有两种部署方式，使用前需要填写服务器的 IP 地址，然后执行 `./build/deploy.sh` 即可。

### 启动服务

启动时需要添加 `MONGO_URL` 环境变量，例如：

```shell
docker run -d --restart always --name web -e MONGO_URL=mongodb://root:root@127.0.0.1:27017/web -p 8080:8080 web
```

## 测试 & 性能

需安装 [drill](https://github.com/fcsonline/drill) 。

```shell
drill -s -q --benchmark benchmark.yml
```

## 许可

[![996.icu](https://img.shields.io/badge/link-996.icu-red.svg)](https://996.icu)
[![LICENSE](https://img.shields.io/badge/license-Anti%20996-blue.svg)](https://github.com/996icu/996.ICU/blob/master/LICENSE)
![GitHub](https://img.shields.io/github/license/LJason77/Web-Boilerplate)
