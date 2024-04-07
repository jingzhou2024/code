# 安装
MSVC
rustup-init


# 环境变量
RUSTUP_HOME=安装目录\.rustup
CARGO_HOME=安装目录\.cargo


# cargo 使用

```posh
PS C:\my_code\code> cargo new cargo_learn
     Created binary (application) `cargo_learn` package
PS C:\my_code\code\cargo_learn> ls

    Directory: C:\my_code\code\cargo_learn

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d----            2024/4/7    23:57                src
-a---            2024/4/7    23:57            180 Cargo.toml

PS C:\my_code\code\cargo_learn> cd .\src\
PS C:\my_code\code\cargo_learn\src> ls

    Directory: C:\my_code\code\cargo_learn\src

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a---            2024/4/7    23:57             45 main.rs

PS C:\my_code\code\cargo_learn\src> cd ..
PS C:\my_code\code\cargo_learn> cargo build
   Compiling cargo_learn v0.1.0 (C:\my_code\code\cargo_learn)
    Finished dev [unoptimized + debuginfo] target(s) in 1.06s
PS C:\my_code\code\cargo_learn> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\cargo_learn.exe`
Hello, world!
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
