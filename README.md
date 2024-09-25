[![Crates.io](https://img.shields.io/crates/v/r-idcard?style=flat-square)](https://crates.io/crates/r-idcard)
[![idcard](https://github.com/bujnlc8/idcard/actions/workflows/idcard.yml/badge.svg)](https://github.com/bujnlc8/idcard/actions/workflows/idcard.yml)

# 解析 18 位身份证信息

基于[https://github.com/bujnlc8/region-cn](https://github.com/bujnlc8/region-cn)

## 用法

使用之前会自动下载区域数据到`$HOME/.cache/region_full.dat`，如果由于网络的原因未能下载成功，请手动从[https://github.com/bujnlc8/region-cn/raw/refs/heads/main/data/region_full.dat](https://github.com/bujnlc8/region-cn/raw/refs/heads/main/data/region_full.dat)下载。

```
❯ idcard
USAGE: idcard your-idcard-number
❯ idcard 110101196404071855
生日: 1964-04-07
性别: 男
区号: 110101
地址: 北京市东城区
```

## 安装

从 [release](https://github.com/bujnlc8/idcard/releases) 下载

OR

```
cargo install r-idcard
```
