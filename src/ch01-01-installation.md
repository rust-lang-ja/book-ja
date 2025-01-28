<!--
## Installation
-->

## インストール

<!--
The first step is to install Rust. We’ll download Rust through `rustup`, a
command line tool for managing Rust versions and associated tools. You’ll need
an internet connection for the download.
-->

最初の手順は、Rustをインストールすることです。Rustは、Rustのバージョンと関連するツールを管理する、`rustup`というコマンドラインツールを使用してダウンロードします。ダウンロードには、インターネットへの接続が必要になります。

<!--
> Note: If you prefer not to use `rustup` for some reason, please see the
> [Other Rust Installation Methods page][otherinstall] for more options.
-->

> 注釈: なんらかの理由で`rustup`を使用したくない場合、[Other Rust Installation Methods ページ][otherinstall]で、
> 他の選択肢をご覧になってください。

<!--
The following steps install the latest stable version of the Rust compiler.
Rust’s stability guarantees ensure that all the examples in the book that
compile will continue to compile with newer Rust versions. The output might
differ slightly between versions because Rust often improves error messages and
warnings. In other words, any newer, stable version of Rust you install using
these steps should work as expected with the content of this book.
-->

以下の手順で最新の安定版のRustコンパイラをインストールします。
Rustは安定性 (stability) を保証しているので、現在この本の例でコンパイルできるものは、新しいバージョンになってもコンパイルでき続けることが保証されます。
出力は、バージョンによって多少異なる可能性があります。Rustは頻繁にエラーメッセージと警告を改善しているからです。
言い換えると、どんな新しいバージョンでもこの手順に従ってインストールした安定版なら、
この本の内容で想定通りに動くはずです。

<!--
> ### Command Line Notation
>
> In this chapter and throughout the book, we’ll show some commands used in the
> terminal. Lines that you should enter in a terminal all start with `$`. You
> don’t need to type the `$` character; it’s the command line prompt shown to
> indicate the start of each command. Lines that don’t start with `$` typically
> show the output of the previous command. Additionally, PowerShell-specific 
> examples will use `>` rather than `$`.
-->

> ### コマンドラインの記法
>
> この章及び、本を通して、端末で使用するなんらかのコマンドを示すことがあります。読者が入力するべき行は、
> 全て`$`で始まります。ただし、読者が`$`文字を入力する必要はありません;
> これは各コマンドの開始を示すために表示しているコマンドラインプロンプトです。
> `$`で始まらない行は、典型的には直前のコマンドの出力を示します。また、PowerShell限定の例には、
> `$`ではなく、`>`を使用します。

<!--
### Installing `rustup` on Linux or macOS
-->

### LinuxとmacOSに`rustup`をインストールする

<!--
If you’re using Linux or macOS, open a terminal and enter the following command:
-->

LinuxかmacOSを使用しているなら、端末（ターミナル）を開き、以下のコマンドを入力してください:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
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
You will also need a *linker*, which is a program that Rust uses to join its
compiled outputs into one file. It is likely you already have one. If you get 
linker errors, you should install a C compiler, which will typically include a
linker. A C compiler is also useful because some common Rust packages depend on
C code and will need a C compiler.
-->

*リンカ*も必要になるでしょう。
リンカは、コンパイルされた出力をひとつのファイルに合体させるためにRustが使用するプログラムです。
リンカが既にインストールされている可能性は高いでしょう。
リンカエラーが発生したときは、Cコンパイラは典型的にリンカを含んでいるでしょうから、Cコンパイラをインストールすべきです。
一般的なRustパッケージの中には、Cコードに依存し、Cコンパイラが必要になるものもあるので、この理由からもCコンパイラは有用です。

<!--
On macOS, you can get a C compiler by running:
-->

macOSでは、以下を実行することでCコンパイラが手に入ります:

```console
$ xcode-select --install
```

<!--
Linux users should generally install GCC or Clang, according to their
distribution’s documentation. For example, if you use Ubuntu, you can install
the `build-essential` package.
-->

Linuxユーザは、通常はディストリビューションのドキュメントに従って、GCCまたはClangをインストールするべきです。
例えばUbuntuを使用している場合は、`build-essential`パッケージをインストールすれば大丈夫です。

<!--
### Installing `rustup` on Windows
-->

### Windowsで`rustup`をインストールする


<!--
On Windows, go to [https://www.rust-lang.org/tools/install][install] and follow
the instructions for installing Rust. At some point in the installation, you’ll
be prompted to install Visual Studio. This provides a linker and the native
libraries needed to compile programs. If you need more help with this step, see
[https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]
-->

Windowsでは、[https://www.rust-lang.org/tools/install][install]に行き、手順に従ってRustをインストールしてください。
インストールの途中で、Visual Studioをインストールするよう求められるはずです。
Visual Studioは、プログラムのコンパイルに必要となる、リンカーとネイティブライブラリを提供します。
この手順について、さらにヘルプが必要な場合は、[https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]を参照してください。

> 訳注：Windowsの言語を日本語にしている場合は言語パックのところで「日本語」が選択されており、そのままの設定でインストールしても基本的に問題ないはずです。しかし、サードパーティーのツールやライブラリの中には英語の言語パックを必要とするものがあるため、「日本語」に加えて「英語」も選択することをお勧めします。

<!--
The rest of this book uses commands that work in both *cmd.exe* and PowerShell.
If there are specific differences, we’ll explain which to use.
-->

これ以降、*cmd.exe*とPowerShellの両方で動くコマンドを使用します。
特段の違いがあったら、どちらを使用すべきか説明します。

<!--
### Troubleshooting
-->

### トラブルシューティング

<!--
To check whether you have Rust installed correctly, open a shell and enter this
line:
-->

Rustが正常にインストールされているか確かめるには、シェルを開いて以下の行を入力してください:

```console
$ rustc --version
```

<!--
You should see the version number, commit hash, and commit date for the latest
stable version that has been released, in the following format:
-->

リリース済みの最新の安定版のバージョンナンバー、コミットハッシュ、コミット日が以下の形式で表示されるはずです。

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

<!--
If you see this information, you have installed Rust successfully! If you don’t
see this information, check that Rust is in your `%PATH%` system variable as
follows.
-->

この情報が見られたなら、Rustのインストールに成功しています！
この情報が出ない場合は、次のようにしてRustが`%PATH%`システム環境変数にあることを確認してください。

<!--
In Windows CMD, use:
-->

Windows CMDでは:

```console
> echo %PATH%
```

<!--
In PowerShell, use:
-->

PowerShellでは:

```powershell
> echo $env:Path
```

<!--
In Linux and macOS, use:
-->

LinuxおよびmacOSでは:

```console
$ echo $PATH
```

<!--
If that’s all correct and Rust still isn’t working, there are a number of
places you can get help. Find out how to get in touch with other Rustaceans (a
silly nickname we call ourselves) on [the community page][community].
-->

これらが全て正常であるのに、それでもRustがうまく動かないなら、助力を得られる場所はたくさんあります。
他のRustacean（Rustユーザが自分たちのことを呼ぶ、冗談めいたニックネーム）たちと交流する方法を[コミュニティページ][community]で探してください。

> 訳注1：Rustaceanについて、いらないかもしれない補足です。[公式Twitter曰く、Rustaceanはcrustaceans（甲殻類）から来ている][twitter]そうです。
> そのため、Rustのマスコットは（非公式らしいですが）[カニ][mascott]。上の会話でCの欠点を削ぎ落としているからcを省いてるの？みたいなことを聞いていますが、
> 違うそうです。検索したら、堅牢性が高いから甲殻類という意見もありますが、真偽は不明です。
> 明日使えるかもしれないトリビアでした。

> 訳注2：上にあるコミュニティページはどれも英語話者のコミュニティへのリンク集です。日本語話者のためのコミュニティが[Zulip rust-lang-jpにあり][zulip_jp]、こちらでもRustaceanたちが活発に議論をしています。
> 公式Discord同様、初心者向けの#beginnersチャンネルが存在するので、気軽に質問してみてください。

[twitter]: https://mobile.twitter.com/rustlang/status/916284650674323457
[mascott]: https://www.slideshare.net/wolf-dog/ss-64026540
[zulip_jp]: https://rust-lang-jp.zulipchat.com

<!--
### Updating and Uninstalling
-->

### 更新及びアンインストール

<!--
Once Rust is installed via `rustup`, updating to a newly released version is
easy. From your shell, run the following update script:
-->

`rustup`経由でRustがインストールされたなら、新しくリリースされた版へ更新するのは簡単です。
シェルから以下の更新スクリプトを実行してください:

```console
$ rustup update
```

<!--
To uninstall Rust and `rustup`, run the following uninstall script from your
shell:
-->

Rustと`rustup`をアンインストールするには、シェルから以下のアンインストールスクリプトを実行してください:

```console
$ rustup self uninstall
```

<!--
### Local Documentation
-->

### ローカルのドキュメンテーション

<!--
The installation of Rust also includes a local copy of the documentation so
that you can read it offline. Run `rustup doc` to open the local documentation
in your browser.
-->

インストールされたRustには、オフラインでドキュメンテーションを閲覧できるように、ドキュメンテーションのローカルコピーが含まれています。
ブラウザでローカルのドキュメンテーションを開くには、`rustup doc`を実行してください。

<!--
Any time a type or function is provided by the standard library and you’re not
sure what it does or how to use it, use the application programming interface
(API) documentation to find out!
-->

標準ライブラリにより提供される型や関数がなんなのかや、それをどう使えば良いのかがよくわからないときは、いつでもAPIのドキュメンテーションを検索してみてください！


[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
