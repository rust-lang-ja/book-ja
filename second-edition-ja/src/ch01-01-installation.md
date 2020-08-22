<!--
## Installation
-->

## インストール

<!--
The first step is to install Rust. We’ll download Rust through `rustup`, a
command line tool for managing Rust versions and associated tools. You’ll need
an internet connection for the download.
-->

最初の手順は、Rustをインストールすることです。Rustは、`rustup`というRustのバージョンと関連するツールを管理するコマンドラインツールを使用して、
ダウンロードします。ダウンロードするには、インターネット接続が必要でしょう。

<!--
> Note: If you prefer not to use `rustup` for some reason, please see [the Rust
> installation page](https://www.rust-lang.org/install.html) for other options.
-->

> 注釈: なんらかの理由で`rustup`を使用しないことを好むのなら、[Rustインストールページ](https://www.rust-lang.org/install.html)で、
> 他の選択肢をご覧になってください。

<!--
The following steps install the latest stable version of the Rust compiler. All
the examples and output in this book use stable Rust 1.21.0. Rust’s stability
guarantees ensure that all the examples in the book that compile will continue
to compile with newer Rust versions. The output might differ slightly between
versions, because Rust often improves error messages and warnings. In other
words, any newer, stable version of Rust you install using these steps should
work as expected with the content of this book.
-->

以下の手順で最新の安定版のRustコンパイラをインストールします。この本の例と出力は全て、安定版のRust1.21.0を使用しています。
Rustの安定性保証により、現在この本の例でコンパイルできるものは、新しいバージョンになってもコンパイルでき続けることを保証します。
出力は、バージョンによって多少異なる可能性があります。Rustは頻繁にエラーメッセージと警告を改善しているからです。
言い換えると、どんな新しいバージョンでもこの手順に従ってインストールした安定版なら、
この本の内容で想定通りに動くはずです。

<!--
> ### Command Line Notation
>
> In this chapter and throughout the book, we’ll show some commands used in the
> terminal. Lines that you should enter in a terminal all start with `$`. You
> don’t need to type in the `$` character; it indicates the start of each
> command. Lines that don't start with `$` typically show the output of the
> previous command. Additionally, PowerShell-specific examples will use `>`
> rather than `$`.
-->

> ### コマンドライン表記
>
> この章及び、本を通して、端末で使用するなんらかのコマンドを示すことがあります。読者が入力するべき行は、
> 全て`$`で始まります。`$`文字を入れる必要はありません; 各コマンドの開始を示しているだけです。
> `$`で始まらない行は、典型的には直前のコマンドの出力を示します。また、PowerShell限定の例は、
> `$`ではなく、`>`を使用します。

<!--
### Installing `rustup` on Linux or macOS
-->

### LinuxとmacOSに`rustup`をインストールする

<!--
If you’re using Linux or macOS, open a terminal and enter the following command:
-->

LinuxかmacOSを使用しているなら、端末を開き、以下のコマンドを入力してください:

```text
$ curl https://sh.rustup.rs -sSf | sh
```

<!--
The command downloads a script and starts the installation of the `rustup`
tool, which installs the latest stable version of Rust. You might be prompted
for your password. If the install is successful, the following line will appear:
-->

このコマンドはスクリプトをダウンロードし、`rustup`ツールのインストールを開始し、Rustの最新の安定版をインストールします。
パスワードを求められる可能性があります。インストールがうまく行けば、以下の行が出現するでしょう:

```text
Rust is installed now. Great!
```

<!--
If you prefer, feel free to download the script and inspect it before running
it.
-->

お好みでご自由にスクリプトをダウンロードし、実行前に調査することもできます。

<!--
The installation script automatically adds Rust to your system PATH after your
next login. If you want to start using Rust right away instead of restarting
your terminal, run the following command in your shell to add Rust to your
system PATH manually:
-->

インストールスクリプトは、次回のログイン後にRustをシステムのPATHに自動的に追加します。端末を再起動するのではなく、
いますぐにRustを使用し始めたいのなら、シェルで以下のコマンドを実行してRustをシステムのPATHに手動で追加します:

```text
$ source $HOME/.cargo/env
```

<!--
Alternatively, you can add the following line to your *~/.bash_profile*:
-->

また、以下の行を *~/.bash_profile*に追加することもできます:

```text
$ export PATH="$HOME/.cargo/bin:$PATH"
```

<!--
Additionally, you’ll need a linker of some kind. It’s likely one is already
installed, but when you try to compile a Rust program and get errors indicating
that a linker could not execute, that means a linker isn’t installed on your
system and you’ll need to install one manually. C compilers usually come with
the correct linker. Check your platform’s documentation for how to install a C
compiler. Also, some common Rust packages depend on C code and will need a C
compiler. Therefore, it might be worth installing one now.
-->

さらに、なんらかの類のリンカが必要になるでしょう。既にインストールされている可能性が高いものの、
Rustプログラムのコンパイルを試みて、リンカが実行できないというエラーが出たら、
システムにリンカがインストールされていないということなので、手動でインストールする必要があるでしょう。
Cコンパイラは通常正しいリンカとセットになっています。
自分のプラットフォームのドキュメンテーションを見てCコンパイラのインストール方法を確認してください。
一般的なRustパッケージの中には、Cコードに依存し、Cコンパイラが必要になるものもあります。
故に今インストールする価値はあるかもしれません。

<!--
### Installing `rustup` on Windows
-->

### Windowsで`rustup`をインストールする

<!--
On Windows, go to [https://www.rust-lang.org/install.html][install] and follow
the instructions for installing Rust. At some point in the installation, you’ll
receive a message explaining that you’ll also need the C++ build tools for
Visual Studio 2013 or later. The easiest way to acquire the build tools is to
install [Build Tools for Visual Studio 2017][visualstudio]. The tools are in
the Other Tools and Frameworks section.
-->

Windowsでは、[https://www.rust-lang.org/install.html][install]に行き、手順に従ってRustをインストールしてください。
インストールの途中で、Visual Studio2013以降用のC++ビルドツールも必要になるという旨のメッセージが出るでしょう。
ビルドツールを取得する最も簡単な方法は、[Visual Studio 2017用のビルドツール][visualstudio]をインストールすることです。
ツールは、他のツール及びフレームワークのセクションにあります。

[install]: https://www.rust-lang.org/install.html
[visualstudio]: https://www.visualstudio.com/downloads/

<!--
The rest of this book uses commands that work in both *cmd.exe* and PowerShell.
If there are specific differences, we’ll explain which to use.
-->

これ以降、*cmd.exe*とPowerShellの両方で動くコマンドを使用します。
特定の違いがあったら、どちらを使用すべきか説明します。

<!--
### Updating and Uninstalling
-->

### 更新及びアンインストール

<!--
After you’ve installed Rust via `rustup`, updating to the latest version is
easy. From your shell, run the following update script:
-->

`rustup`経由でRustをインストールしたら、最新版への更新は、簡単になります。シェルから、
以下の更新スクリプトを実行してください:

```text
$ rustup update
```

<!--
To uninstall Rust and `rustup`, run the following uninstall script from your
shell:
-->

Rustと`rustup`をアンインストールするには、シェルから以下のアンインストールスクリプトを実行してください:

```text
$ rustup self uninstall
```

<!--
### Troubleshooting
-->

### トラブルシューティング

<!--
To check whether you have Rust installed correctly, open a shell and enter this
line:
-->

Rustが正常にインストールされているか確かめるには、シェルを開いて以下の行を入力してください:

```text
$ rustc --version
```

<!--
You should see the version number, commit hash, and commit date for the latest
stable version that has been released in the following format:
-->

バージョンナンバー、コミットハッシュ、最新の安定版がリリースされたコミット日時が以下のフォーマットで表示されるのを目撃するはずです。

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

<!--
If you see this information, you have installed Rust successfully! If you don’t
see this information and you’re on Windows, check that Rust is in your `%PATH%`
system variable. If that’s all correct and Rust still isn’t working, there are
a number of places you can get help. The easiest is [the #rust IRC channel on
irc.mozilla.org][irc], which you can access through
[Mibbit][mibbit]. At that address you can chat with other Rustaceans (a silly
nickname we call ourselves) who can help you out. Other great resources include
[the Users forum][users] and [Stack Overflow][stackoverflow].
-->

この情報が見れたら、Rustのインストールに成功しました！この情報が出ず、Windowsを使っているなら、
Rustが`%PATH%`システム環境変数にあることを確認してください。全て正常で、それでもRustが動かないなら、
助力を得られる場所はたくさんあります。最も簡単なのが[irc.mozilla.orgの#rust IRCチャンネル][irc]で、
[Mibbit][mibbit]を通してアクセスできます。そのアドレスで、助けてくれる他のRustacean(自分たちを呼ぶバカなニックネーム)とチャットできます。
他の素晴らしいリソースには、[ユーザ・フォーラム][users]と[Stack Overflow][stackoverflow]が含まれます。

> Rustacean: いらないかもしれない補足です。Rustaceanは公式にcrustaceans(甲殻類)から[来て][twitter]いるそうです。
> そのため、Rustのマスコットは非公式らしいですが、[カニ][mascott]。上の会話でCの欠点を削ぎ落としているからcを省いてるの？みたいなことを聞いていますが、
> 違うそうです。検索したら、堅牢性が高いから甲殻類という意見もありますが、真偽は不明です。
> 明日使えるかもしれないトリビアでした。

[irc]: irc://irc.mozilla.org/#rust
[mibbit]: http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust
[users]: https://users.rust-lang.org/
[stackoverflow]: http://stackoverflow.com/questions/tagged/rust
[twitter]: https://mobile.twitter.com/rustlang/status/916284650674323457
[mascott]: https://www.slideshare.net/wolf-dog/ss-64026540

<!--
### Local Documentation
-->

### ローカルのドキュメンテーション

<!--
The installer also includes a copy of the documentation locally, so you can
read it offline. Run `rustup doc` to open the local documentation in your
browser.
-->

インストーラは、ドキュメンテーションの複製もローカルに含んでいるので、オフラインで閲覧することができます。
ブラウザでローカルのドキュメンテーションを開くには、`rustup doc`を実行してください。

<!--
Any time a type or function is provided by the standard library and you’re not
sure what it does or how to use it, use the application programming interface
(API) documentation to find out!
-->

標準ライブラリにより型や関数が提供され、それがなんなのかや使用方法に確信が持てない度に、APIドキュメンテーションを使用して探してください！
