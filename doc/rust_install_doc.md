# wsl
**wsl文件系统位置**
`\\wsl$`

**查看已安装的linux 系统**
wsl --list --verbose
wsl --set-version Ubuntu-22.04 2

**卸载**
wsl --unregister Ubuntu-20.04


**进入ubuntu**
wsl -d Ubuntu-22.04

# rustup 的镜像地址
# 配置文件`.bash_profile`
```
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```

**下载**
```shell
curl https://sh.rustup.rs -sSf | sh
```
```shell
# 安装工具链
rustup install nightly
# 切换工具链, 切换后cargo, rustc都变成这个版本
rustup default nightly 

# 查看工具链
rustup toolchain list
```

**设置cargo 软件包镜像地址**
> 配置文件 `~/.cargo/config.toml`
```shell
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

**安装相关的包, 工具链更换版本了这些都要重新安装**
```shell
rustup target add riscv64gc-unknown-none-elf
rustup component add llvm-tools-preview
rustup component add rust-src
```

```shell
# 命令用于更新软件包列表，即更新本地软件包索引数据库。 
# 当我们在系统中安装新软件或者更新现有软件之前，需要先更新软件包列表，以确保获取最新的软件包信息。

apt-get update

# 命令用于升级已经安装的软件包到最新版本。
apt-get upgrade
```

```shell
sudo apt install cmake gcc clang gdb build-essential

```
## clion配置
```shell
wget https://raw.githubusercontent.com/JetBrains/clion-wsl/master/ubuntu_setup_env.sh
sudo chmod +x ubuntu_setup_env.sh
./ubuntu_setup_env.sh
重启
```
```shell
ssh idea@localhost -p2222
```

配置Clion 内的 Terminal
转到 Settings | Tools | Terminal ，修改“application settings”内的“shell path”为：
C:\Users\ASUS\AppData\Local\Microsoft\WindowsApps\ubuntu2204.exe run


## cargo
**创建cargo项目**
```shell
PS C:\my_code\code> cargo new cargo_learn
     Created binary (application) `cargo_learn` package
PS C:\my_code\code\cargo_learn> ls

    Directory: C:\my_code\code\cargo_learn

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d----            2024/4/7    23:57                src
-a---            2024/4/7    23:57            180 Cargo.toml
```

**构建项目**
```
PS C:\my_code\code\cargo_learn> cargo build
   Compiling cargo_learn v0.1.0 (C:\my_code\code\cargo_learn)
    Finished dev [unoptimized + debuginfo] target(s) in 1.06s
```

**运行项目**
```shell
PS C:\my_code\code\cargo_learn> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\cargo_learn.exe`
Hello, world!
```
**清除target目录**
```shell
PS C:\my_code\code\cargo_learn> cargo clean
     Removed 23 files, 3.0MiB total
PS C:\my_code\code\cargo_learn> ls

    Directory: C:\my_code\code\cargo_learn

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d----            2024/4/7    23:57                src
-a---            2024/4/7    23:57            155 Cargo.lock
-a---            2024/4/7    23:57            180 Cargo.toml
```


# linux 安装
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

source ~/.profile
source ~/.cargo/env


# 验证
rustc --version

# 卸载 rustup
rustup self uninstall

# 查看工具链可用
rustup show

# 查看编译器完整路径
[idea@hadoop100 ~]$ rustup which rustc
/home/idea/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc

# 查看包管理器完整路径
rustup which cargo

```