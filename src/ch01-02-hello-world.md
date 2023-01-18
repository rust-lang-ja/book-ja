<!--
## Hello, World!
-->

## Hello, World!

<!--
Now that you’ve installed Rust, let’s write your first Rust program. It’s
traditional when learning a new language to write a little program that prints
the text `Hello, world!` to the screen, so we’ll do the same here!
-->

Rust をインストールしたので、最初の Rust プログラムを書きましょう。新しい言語を学ぶ際に、
`Hello, world!`というテキストを画面に出力する小さなプログラムを書くことは伝統的なことなので、
ここでも同じようにしましょう！

<!--
> Note: This book assumes basic familiarity with the command line. Rust makes
> no specific demands about your editing or tooling or where your code lives, so
> if you prefer to use an integrated development environment (IDE) instead of
> the command line, feel free to use your favorite IDE. Many IDEs now have some
> degree of Rust support; check the IDE’s documentation for details. Recently,
> the Rust team has been focusing on enabling great IDE support, and progress
> has been made rapidly on that front!
-->

> 注釈：この本は、コマンドラインに基礎的な馴染みがあることを前提にしています。Rust は、編集やツール、
> どこにコードがあるかについて特定の要求をしないので、コマンドラインではなく IDE を使用することを好むのなら、
> どうぞご自由にお気に入りの IDE を使用してください。今では、多くの IDE がなんらかの形で Rust をサポートしています;
> 詳しくは、IDE のドキュメンテーションをご覧ください。最近、Rust チームは優れた IDE サポートを有効にすることに注力し、
> その前線で急激に成果があがっています！

<!--
### Creating a Project Directory
-->

### プロジェクトのディレクトリを作成する

<!--
You’ll start by making a directory to store your Rust code. It doesn’t matter
to Rust where your code lives, but for the exercises and projects in this book,
we suggest making a *projects* directory in your home directory and keeping all
your projects there.
-->

Rust コードを格納するディレクトリを作ることから始めましょう。Rust にとって、コードがどこにあるかは問題ではありませんが、
この本の練習とプロジェクトのために、ホームディレクトリに*projects*ディレクトリを作成してプロジェクトを全てそこに保管することを推奨します。

<!--
Open a terminal and enter the following commands to make a *projects* directory
and a directory for the Hello, world! project within the *projects* directory.
-->

端末を開いて以下のコマンドを入力し、*projects*ディレクトリと、
*projects*ディレクトリ内に Hello, world! プロジェクトのディレクトリを作成してください。

<!--
For Linux and macOS, enter this:
-->

Linux と macOS なら、こう入力してください：

```text
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

<!--
For Windows CMD, enter this:
-->

Windows の cmd なら、こう：

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

<!--
For Windows PowerShell, enter this:
-->

Windows の PowerShell なら、こう：

```powershell
> mkdir $env:USERPROFILE\projects
> cd $env:USERPROFILE\projects
> mkdir hello_world
> cd hello_world
```

<!--
### Writing and Running a Rust Program
-->

### Rust プログラムを書いて走らせる

<!--
Next, make a new source file and call it *main.rs*. Rust files always end with
the *.rs* extension. If you’re using more than one word in your filename, use
an underscore to separate them. For example, use *hello_world.rs* rather than
*helloworld.rs*.
-->

次にソースファイルを作り、*main.rs*というファイル名にしてください。Rust のファイルは常に *.rs*という拡張子で終わります。
ファイル名に 2 単語以上使っているなら、アンダースコアで区切ってください。例えば、*helloworld.rs*ではなく、
*hello_world.rs*を使用してください。

<!--
Now open the *main.rs* file you just created and enter the code in Listing 1-1.
-->

さて、作ったばかりの*main.rs*ファイルを開き、リスト 1-1 のコードを入力してください。

<!--
<span class="filename">Filename: main.rs</span>
-->

<span class="filename">ファイル名：main.rs</span>

```rust
fn main() {
	// 世界よ、こんにちは
    println!("Hello, world!");
}
```

<!--
<span class="caption">Listing 1-1: A program that prints `Hello, world!`</span>
-->

<span class="caption">リスト 1-1: `Hello, world!`と出力するプログラム</span>

<!--
Save the file and go back to your terminal window. On Linux or macOS, enter
the following commands to compile and run the file:
-->

ファイルを保存し、端末ウィンドウに戻ってください。Linux か macOS なら、以下のコマンドを打ってファイルをコンパイルし、
実行してください：

```text
$ rustc main.rs
$ ./main
Hello, world!
```

<!--
On Windows, enter the command `.\main.exe` instead of `./main`:
-->

Windows なら、`./main`の代わりに`.\main.exe`と打ちます：

```powershell
> rustc main.rs
> .\main.exe
Hello, world!
```

<!--
Regardless of your operating system, the string `Hello, world!` should print to
the terminal. If you don’t see this output, refer back to the “Troubleshooting”
section for ways to get help.
-->

OS に関わらず、`Hello, world!`という文字列が端末に出力されるはずです。この出力が見れないなら、
「トラブルシューティング」節に立ち戻って、助けを得る方法を参照してください。

<!--
If `Hello, world!` did print, congratulations! You’ve officially written a Rust
program. That makes you a Rust programmer-welcome!
-->

`Hello, world!`が確かに出力されたら、おめでとうございます！正式に Rust プログラムを書きました。
Rust プログラマになったのです！ようこそ！

<!--
### Anatomy of a Rust Program
-->

### Rust プログラムの解剖

<!--
Let’s review in detail what just happened in your Hello, world! program.
Here’s the first piece of the puzzle:
-->

Hello, world! プログラムでいま何が起こったのか詳しく確認しましょう。
こちらがパズルの最初のピースです：

```rust
fn main() {

}
```

<!--
These lines define a function in Rust. The `main` function is special: it is
always the first code that runs in every executable Rust program. The first
line declares a function named `main` that has no parameters and returns
nothing. If there were parameters, they would go inside the parentheses, `()`.
-->

これらの行で Rust で関数を定義しています。`main`関数は特別です：常に全ての実行可能な Rust プログラムで走る最初のコードになります。
1 行目は、引数がなく、何も返さない`main`という関数を宣言しています。引数があるなら、かっこ (`()`) の内部に入ります。

<!--
Also, note that the function body is wrapped in curly brackets, `{}`. Rust
requires these around all function bodies. It’s good style to place the opening
curly bracket on the same line as the function declaration, adding one space in
between.
-->

また、関数の本体が波括弧 (`{}`) に包まれていることにも注目してください。Rust では、全ての関数本体の周りにこれらが必要になります。
スペースを 1 つあけて、開き波括弧を関数宣言と同じ行に配置するのがいいスタイルです。

<!--
If you want to stick to a standard style across Rust projects, you can use an
automatic formatter tool called `rustfmt` to format your code in a particular
style. The Rust team has included this tool with the standard Rust distribution,
like `rustc`, so it should already be installed on your computer! Check the
online documentation for more details.
-->

複数の Rust プロジェクトに渡って標準的なスタイルにこだわりたいなら、`rustfmt`を使うことでコードを決まったスタイルに整形できるでしょう。
Rust チームは、`rustc`のように標準的な Rust の配布にこのツールを含んでいるため、既にコンピューターにインストールされているはずです！
詳細は、オンラインのドキュメンテーションを確認してください。

<!--
Inside the `main` function is the following code:
-->

`main`関数内には、こんなコードがあります：

```rust
    println!("Hello, world!");
```

<!--
This line does all the work in this little program: it prints text to the
screen. There are four important details to notice here. First, Rust style is
to indent with four spaces, not a tab.
-->

この行が、この小さなプログラムの全作業をしています：テキストを画面に出力するのです。
ここで気付くべき重要な詳細が 4 つあります。まず、Rust のスタイルは、タブではなく、4 スペースでインデントするということです。

<!--
Second, `println!` calls a Rust macro. If it called a function instead, it
would be entered as `println` (without the `!`). We’ll discuss Rust macros in
more detail in Appendix D. For now, you just need to know that using a `!`
means that you’re calling a macro instead of a normal function.
-->

2 番目に`println!`は Rust のマクロを呼び出すということです。代わりに関数を呼んでいたら、
`println`(`!`なし) と入力されているでしょう。Rust のマクロについて詳しくは、第 19 章で議論します。
とりあえず、`!`を使用すると、普通の関数ではなくマクロを呼んでいるのだということを知っておくだけでいいでしょう。

<!--
Third, you see the `"Hello, world!"` string. We pass this string as an argument
to `println!`, and the string is printed to the screen.
-->

3 番目に、`"Hello, world!"`文字列が見えます。この文字列を引数として`println!`に渡し、
この文字列が画面に表示されているのです。

<!--
Fourth, we end the line with a semicolon (`;`), which indicates that this
expression is over and the next one is ready to begin. Most lines of Rust code
end with a semicolon.
-->

4 番目にこの行をセミコロン (`;`) で終え、この式が終わり、次の式の準備ができていると示唆していることです。
Rust コードのほとんどの行は、セミコロンで終わります。

<!--
### Compiling and Running Are Separate Steps
-->

### コンパイルと実行は個別のステップ

<!--
You’ve just run a newly created program, so let’s examine each step in the
process.
-->

新しく作成したプログラムをちょうど実行したので、その途中の手順を調査しましょう。

<!--
Before running a Rust program, you must compile it using the Rust compiler by
entering the `rustc` command and passing it the name of your source file, like
this:
-->

Rust プログラムを実行する前に、以下のように、`rustc`コマンドを入力し、ソースファイルの名前を渡すことで、
Rust コンパイラを使用してコンパイルしなければなりません。

```text
$ rustc main.rs
```

<!--
If you have a C or C++ background, you’ll notice that this is similar to `gcc`
or `clang`. After compiling successfully, Rust outputs a binary executable.
-->

あなたに C や C++の背景があるなら、これは`gcc`や`clang`と似ていると気付くでしょう。コンパイルに成功後、
Rust はバイナリの実行可能ファイルを出力します。

<!--
On Linux, macOS, and PowerShell on Windows, you can see the executable by
entering the `ls` command in your shell as follows:
-->

Linux、macOS、Windows の PowerShell なら、シェルで以下のように`ls`コマンドを入力することで実行可能ファイルを見られます：

```text
$ ls
main  main.rs
```

<!--
With CMD on Windows, you would enter the following:
-->

Windows の CMD なら、以下のように入力するでしょう：

```cmd
> dir /B %= the /B option says to only show the file names =%
         %= /Bオプションは、ファイル名だけを表示することを宣言する =%
main.exe
main.pdb
main.rs
```

<!--
This shows the source code file with the *.rs* extension, the executable file
(*main.exe* on Windows, but *main* on all other platforms), and, when using
CMD, a file containing debugging information with the *.pdb* extension. From
here, you run the *main* or *main.exe* file, like this:
-->

これは、*.rs*拡張子のソースコードファイル、実行可能ファイル (Windows なら*main.exe*、他のプラットフォームでは、*main*)、
そして、CMD を使用しているなら、*.pdb*拡張子のデバッグ情報を含むファイルを表示します。ここから、
*main*か*main.exe*を走らせます。このように：

```text
$ ./main # or .\main.exe on Windows
         # または、Widnowsなら.\main.exe
```

<!--
If *main.rs* was your Hello, world! program, this line would print `Hello,
world!` to your terminal.
-->

*main.rs*が Hello, world! プログラムなら、この行は`Hello, world!`と端末に出力するでしょう。

<!--
*.rb*がなぜかイタリックにならない
-->

<!--
If you’re more familiar with a dynamic language, such as Ruby, Python, or
JavaScript, you might not be used to compiling and running a program as
separate steps. Rust is an *ahead-of-time compiled* language, meaning you can
compile a program and give the executable to someone else, and they can run it
even without having Rust installed. If you give someone a *.rb*, *.py*, or
*.js* file, they need to have a Ruby, Python, or JavaScript implementation
installed (respectively). But in those languages, you only need one command to
compile and run your program. Everything is a trade-off in language design.
-->

Ruby や Python、JavaScript などの動的言語により造詣が深いなら、プログラムのコンパイルと実行を個別の手順で行うことに慣れていない可能性があります。
Rust は*AOT コンパイル*(ahead-of-time; `訳注`: 予め) 言語です。つまり、プログラムをコンパイルし、
実行可能ファイルを誰かにあげ、あげた人が Rust をインストールしていなくても実行できるわけです。
誰かに *.rb*、*.py*、*.js*ファイルをあげたら、それぞれ Ruby、Python、JavaScript の処理系がインストールされている必要があります。
ですが、そのような言語では、プログラムをコンパイルし実行するには、1 コマンドしか必要ないのです。
全ては言語設計においてトレードオフなのです。

<!--
Just compiling with `rustc` is fine for simple programs, but as your project
grows, you’ll want to manage all the options and make it easy to share your
code. Next, we’ll introduce you to the Cargo tool, which will help you write
real-world Rust programs.
-->

簡単なプログラムなら`rustc`でコンパイルするだけでも十分ですが、プロジェクトが肥大化してくると、
オプションを全て管理し、自分のコードを簡単に共有したくなるでしょう。次は、Cargo ツールを紹介します。
これは、現実世界の Rust プログラムを書く手助けをしてくれるでしょう。
