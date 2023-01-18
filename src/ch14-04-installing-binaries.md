<!--
## Installing Binaries from Crates.io with `cargo install`
-->

## `cargo install`で Crates.io からバイナリをインストールする

<!--
The `cargo install` command allows you to install and use binary crates
locally. This isn’t intended to replace system packages; it’s meant to be a
convenient way for Rust developers to install tools that others have shared on
[crates.io](https://crates.io). Note that you can only install
packages that have binary targets. A binary target is the runnable program
that is created if the crate has a *src/main.rs* file or another file specified
as a binary, as opposed to a library target that isn’t runnable on its own but
is suitable for including within other programs. Usually, crates have
information in the *README* file about whether a crate is a library, has a
binary target, or both.
-->

`cargo install`コマンドにより、バイナリクレートをローカルにインストールし、使用することができます。
これは、システムパッケージを置き換えることを意図したものではありません。<ruby>即<rp>(</rp><rt>すなわ</rt><rp>)</rp></ruby>ち、
Rust の開発者が、他人が[crates.io](https://crates.io)に共有したツールをインストールするのに便利な方法を意味するのです。
バイナリターゲットを持つパッケージのみインストールできることに注意してください。バイナリターゲットとは、
クレートが*src/main.rs*ファイルやバイナリとして指定された他のファイルを持つ場合に生成される実行可能なプログラムのことであり、
単独では実行不可能なものの、他のプログラムに含むのには適しているライブラリターゲットとは一線を画します。
通常、クレートには、*README*ファイルに、クレートがライブラリかバイナリターゲットか、両方をもつかという情報があります。

<!--
All binaries installed with `cargo install` are stored in the installation
root’s *bin* folder. If you installed Rust using `rustup` and don’t have any
custom configurations, this directory will be *$HOME/.cargo/bin*. Ensure that
directory is in your `$PATH` to be able to run programs you’ve installed with
`cargo install`.
-->

`cargo install`でインストールされるバイナリは全て、インストールのルートの*bin*フォルダに保持されます。
Rust を`rustup`を使用し、独自の設定を何も行なっていなければ、このディレクトリは、*$HOME/.cargo/bin*になります。
`cargo install`でインストールしたプログラムを実行できるようにするためには、そのディレクトリが`$PATH`に含まれていることを確かめてください。

<!--
For example, in Chapter 12 we mentioned that there’s a Rust implementation of
the `grep` tool called `ripgrep` for searching files. If we want to install
`ripgrep`, we can run the following:
-->

例えば、第 12 章で、ファイルを検索する`ripgrep`という`grep`ツールの Rust 版があることに触れました。
`ripgrep`をインストールしたかったら、以下を実行することができます：

```text
$ cargo install ripgrep
Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading ripgrep v0.3.2
 --snip--
   Compiling ripgrep v0.3.2
    Finished release [optimized + debuginfo] target(s) in 97.91 secs
  Installing ~/.cargo/bin/rg
```

<!--
The last line of the output shows the location and the name of the installed
binary, which in the case of `ripgrep` is `rg`. As long as the installation
directory is in your `$PATH`, as mentioned previously, you can then run `rg
--help` and start using a faster, rustier tool for searching files!
-->

出力の最後の行が、インストールされたバイナリの位置と名前を示していて、`ripgrep`の場合、`rg`です。
インストールディレクトリが`$PATH`に存在する限り、前述したように、`rg --help`を走らせて、
より高速で Rust らしいファイル検索ツールを使用し始めることができます！
