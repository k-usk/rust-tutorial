# Rust
チュートリアル

* [プログラミング言語Rust](https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/README.html)

## インストール

```
$ curl https://sh.rustup.rs -sSf | sh
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust programming
language, and its package manager, Cargo.

It will add the cargo, rustc, rustup and other commands to Cargo's bin
directory, located at:

  /home/vagrant/.cargo/bin

This path will then be added to your PATH environment variable by modifying the
profile files located at:

  /home/vagrant/.profile
  /home/vagrant/.bash_profile

You can uninstall at any time with rustup self uninstall and these changes will
be reverted.

Current installation options:

   default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
```

1を選んでインストールする。

>Rust is installed now. Great!

と出たが、`rustc --version` ではコマンドがない、と出るので、PATHが通ってないよう。

以下でいけた。

```
$ export PATH="$HOME/.cargo/bin:$PATH"
$ rustc --version
rustc 1.26.0 (a77568041 2018-05-07)
```

[Rust入門 - 開発環境構築 - 無理なご乗車はおやめ下さい。](https://tydk27.hatenadiary.com/entry/20160713/1468416790)

## 実行
以下を`main.rs`で保存。

```rust
fn main() {
    println!("Hello, world!");
}
```

コンパイルするとエラーが。

```
$ rustc main.rs
error: linker `cc` not found
  |
  = note: No such file or directory (os error 2)

error: aborting due to previous error
```

どうやらCのコンパイラが入ってない？  

[gcc（GNU Compiler Collection）とは - IT用語辞典](http://e-words.jp/w/gcc.html)

```
$ sudo yum -y install gcc
```

入った。

```
$ rpm -qa gcc
gcc-4.8.5-28.el7.x86_64
```

[［ CentOS ］ gcc をインストールする ( yum -y install rpm-package ) – 行け！偏差値40プログラマー](http://hensa40.cutegirl.jp/archives/2084)

これで無事、コンパイル、実行が出来た。

```
$ rustc main.rs
$ ./main
Hello, world!
```

### コードスタイル

* 関数の`{`は同じ行にスペースを一つあけて置く（改行しない）
	* `fn main() {`
* インデントはスペース４つ（タグでない）

## プログラミング

* `!`がつくものはメタプログラミング機構、マクロの呼び出し
	* `!`がつかないものは関数呼び出し
* 変数は基本イミュータブル（不変）
	* ミュータブル（可変）にしたい場合は、`mut`をつける `let mut hoge`
	* 変数、ではなく、束縛、と呼ぶよう
* `&`がつく束縛（変数）は参照 `&hoge`
	* デフォルトはイミュータブル
	* `&mut hoge`でミュータブル
* exceptionは panic! と呼ばれるよう
* **クレート**とはパッケージのこと
* メソッドは全て？トレイトで定義されており、使用するにはそのメソッドのトレイトが同じスコープにないといけない
	* そのために`use`したりする
	* `use rand::Rng`
		* `gen_range()`を使えるようになる
* 以前に定義した束縛を新しい型で再度定義可能
	* 型変換の時によく使われるよう
	* 前の定義を「隠す」ので、**シャドーイング**というよう
* Rustは式ペースの言語なので関数内でもほとんどが値を返す
	* 関数では最後の式が自動的に戻り値となる。`return`などは不要
* `impl`は構造体に関数定義
	* クラスみたいなもん？
* lockの解法はスコープから抜ける際に自動的に解放される