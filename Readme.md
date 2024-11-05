# rfc
## 介绍
IpTellDnspod是一款创新性的工具，它巧妙地监听本地IPv4及IPv6地址的任何变动，并据此自动同步更新DNSPOD平台上的域名解析记录。借助DNSPOD最新的3.0版本API，我们确保了与服务的无缝集成与高效通信。

我们的系统专为多用户环境打造，允许每个用户独立管理多个API密钥，而这些API密钥又能分别控制多个域名记录。这种设计不仅提升了管理的灵活性，还极大地满足了不同用户的多样化需求。

IpTellDnspod系统由两部分组成：一个强大的本地服务组件和一个直观的Web管理界面。本地服务组件负责实时监控地址变动，并即时触发更新操作；而Web管理界面则提供了便捷的用户和域名配置管理工具，让用户可以轻松管理自己的域名和账户信息。

值得一提的是，IpTellDnspod基于Rust语言开发，实现了真正的跨平台兼容性。只需一个二进制运行文件和一个数据库文件，用户即可轻松安装并运行我们的系统，无需复杂的配置或依赖项。

##  数据库SQLite

创建数据库文件

```sql
CREATE TABLE "user" (
  "id" integer PRIMARY KEY AUTOINCREMENT,
  "username" varchar(255) NOT NULL,
  "password" varchar(255) NOT NULL,
  "created_at" datetime DEFAULT CURRENT_TIMESTAMP,
  "updated_at" datetime DEFAULT CURRENT_TIMESTAMP,
  "status" tinyint(1) DEFAULT 1
);
CREATE TABLE "user_apps" (
  "id" integer PRIMARY KEY AUTOINCREMENT,
  "uid" integer NOT NULL,
  "title" varchar(255) NOT NULL,
  "appid" varchar(255) NOT NULL,
  "secret" varchar(255) NOT NULL,
  "created_at" datetime DEFAULT CURRENT_TIMESTAMP,
  "updated_at" datetime DEFAULT CURRENT_TIMESTAMP,
  "status" tinyint(1) DEFAULT 1
);

CREATE TABLE "user_domain" (
  "id" integer PRIMARY KEY AUTOINCREMENT,
  "appid" varchar(255) NOT NULL,
  "domain" varchar(255) NOT NULL,
  "ip_type" tinyint(1) DEFAULT 1,
  "ip" varchar(255) NOT NULL,
  "record_id" varchar(255) NOT NULL,
  "weight" integer DEFAULT 1,
  "ttl" integer DEFAULT 600
  "created_at" datetime DEFAULT CURRENT_TIMESTAMP,
  "updated_at" datetime DEFAULT CURRENT_TIMESTAMP,
);

```
