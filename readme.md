# これは何？

Rustのチュートリアルを進めたもの。

* [プログラミング言語Rust](https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/README.html)

# Vagrant

`Vagrantfile`にて、IPを好きなものに変更してVagrantを立てる。

```
$ vagrant up
```

# インストール
`$ vagrant ssh`で入ったあと、

```
$ curl https://sh.rustup.rs -sSf | sh
$ export PATH="$HOME/.cargo/bin:$PATH"
```

(インストール時の選択肢は`1`を選択）

バージョン確認して出たらOK。

```
$ rustc --version
rustc 1.26.0 (a77568041 2018-05-07)
```

# 実行
ディレクトリに移動してコンパイル。

```
$ cd /var/www/
$ cd projects/hello_world_2/
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_world_2`
Hello, world! 
```

