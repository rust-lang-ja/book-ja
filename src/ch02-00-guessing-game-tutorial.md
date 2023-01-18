<!--
# Programming a Guessing Game
-->

# 数当てゲームのプログラミング

<!--
Let’s jump into Rust by working through a hands-on project together! This
chapter introduces you to a few common Rust concepts by showing you how to use
them in a real program. You’ll learn about `let`, `match`, methods, associated
functions, using external crates, and more! In the following chapters, we’ll
explore these ideas in more detail. In this chapter, you’ll practice the
fundamentals.
-->

ハンズオン形式のプロジェクトに一緒に取り組むことで、Rust の世界に飛び込んでみましょう！&nbsp;
この章では Rust の一般的な概念を、実際のプログラムでの使い方を示しながら紹介します。
`let`、`match`、メソッド、関連関数、外部クレートの使いかたなどについて学びます！&nbsp;
これらについての詳細は後続の章で取り上げますので、この章では基本的なところを練習します。

<!--
We’ll implement a classic beginner programming problem: a guessing game. Here’s
how it works: the program will generate a random integer between 1 and 100. It
will then prompt the player to enter a guess. After a guess is entered, the
program will indicate whether the guess is too low or too high. If the guess is
correct, the game will print a congratulatory message and exit.
-->

プログラミング初心者向けの定番問題である「数当てゲーム」を実装してみましょう。
これは次のように動作します。
プログラムは 1 から 100 までのランダムな整数を生成します。
そして、プレーヤーに予想（した数字）を入力するように促します。
予想が入力されると、プログラムはその予想が小さすぎるか大きすぎるかを表示します。
予想が当たっているなら、お祝いのメッセージを表示し、ゲームを終了します。

<!--
## Setting Up a New Project
-->

## 新規プロジェクトの立ち上げ

<!--
To set up a new project, go to the *projects* directory that you created in
Chapter 1 and make a new project using Cargo, like so:
-->

新しいプロジェクトを立ち上げましょう。
第 1 章で作成した*projects*ディレクトリに移動し、以下のように Cargo を使って新規プロジェクトを作成します。

```console
$ cargo new guessing_game
$ cd guessing_game
```

<!--
The first command, `cargo new`, takes the name of the project (`guessing_game`)
as the first argument. The second command changes to the new project’s
directory.
-->

最初のコマンド`cargo new`は、第 1 引数としてプロジェクト名 (`guessing_game`) を取ります。
2 番目のコマンドは新規プロジェクトのディレクトリに移動します。

<!--
Look at the generated *Cargo.toml* file:
-->

生成された*Cargo.toml*ファイルを見てみましょう。

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">ファイル名：Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

<!--
As you saw in Chapter 1, `cargo new` generates a “Hello, world!” program for
you. Check out the *src/main.rs* file:
-->

第 1 章で見たように`cargo new`は「Hello, world!」プログラムを生成してくれます。
*src/main.rs*ファイルをチェックしてみましょう。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

<!--
Now let’s compile this “Hello, world!” program and run it in the same step
using the `cargo run` command:
-->

さて、`cargo run`コマンドを使って、この「Hello, world!」プログラムのコンパイルと実行を一気に行いましょう。

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

<!--
The `run` command comes in handy when you need to rapidly iterate on a project,
as we’ll do in this game, quickly testing each iteration before moving on to
the next one.
-->

このゲーム（の開発）では各イテレーションを素早くテストしてから、次のイテレーションに移ります。
`run`コマンドは、今回のようにプロジェクトのイテレーションを素早く回したいときに便利です。

> 訳注：ここでのイテレーションは、アジャイルな開発手法で用いられている用語にあたります。
>
> イテレーションとは開発工程の「一回のサイクル」のことで、サイクルには、設計、実装、テスト、改善（リリース後の振り返り）が含まれます。
> アジャイル開発ではイテレーションを数週間の短いスパンで一通り回し、それを繰り返すことで開発を進めていきます。
> 
> この章では「実装」→「テスト」のごく短いサイクルを繰り返すことで、プログラムに少しずつ機能を追加していきます。

<!--
Reopen the *src/main.rs* file. You’ll be writing all the code in this file.
-->

*src/main.rs*ファイルを開き直しましょう。
このファイルにすべてのコードを書いていきます。

<!--
## Processing a Guess
-->

## 予想を処理する

<!--
The first part of the guessing game program will ask for user input, process
that input, and check that the input is in the expected form. To start, we’ll
allow the player to input a guess. Enter the code in Listing 2-1 into
*src/main.rs*.
-->

数当てゲームプログラムの最初の部分は、ユーザに入力を求め、その入力を処理し、期待した形式になっていることを確認することです。
手始めに、プレーヤーが予想を入力できるようにしましょう。
リスト 2-1 のコードを*src/main.rs*に入力してください。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

<!--
<span class="caption">Listing 2-1: Code that gets a guess from the user and
prints it</span>
-->

<span class="caption">リスト 2-1：ユーザに予想を入力してもらい、それを出力するコード</span>

<!--
This code contains a lot of information, so let’s go over it line by line. To
obtain user input and then print the result as output, we need to bring the
`io` input/output library into scope. The `io` library comes from the
standard library, known as `std`:
-->

このコードには多くの情報が詰め込まれています。
行ごとに見ていきましょう。
ユーザ入力を受け付け、結果を出力するためには`io`（入出力）ライブラリをスコープに入れる必要があります。
`io`ライブラリは、`std`と呼ばれる標準ライブラリに含まれています。

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

<!--
By default, Rust has a few items defined in the standard library that it brings
into the scope of every program. This set is called the *prelude*, and you can
see everything in it [in the standard library documentation][prelude].
-->

Rust はデフォルトで、標準ライブラリで定義されているアイテムの中のいくつかを、すべてのプログラムのスコープに取り込みます。
このセットは*prelude*（プレリュード）と呼ばれ、[標準ライブラリのドキュメント][prelude]でその中のすべてを見ることができます。

<!--
If a type you want to use isn’t in the prelude, you have to bring that type
into scope explicitly with a `use` statement. Using the `std::io` library
provides you with a number of useful features, including the ability to accept
user input.
-->

使いたい型が prelude にない場合は、その型を`use`文で明示的にスコープに入れる必要があります。
`std::io`ライブラリを`use`すると、ユーザ入力を受け付ける機能など（入出力に関する）多くの便利な機能が利用できるようになります。

[prelude]: https://doc.rust-lang.org/std/prelude/index.html

<!--
As you saw in Chapter 1, the `main` function is the entry point into the
program:
-->

第 1 章で見た通り、`main`関数がプログラムへのエントリーポイント（訳注：スタート地点）になります。

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

<!--
The `fn` syntax declares a new function, the parentheses, `()`, indicate there
are no parameters, and the curly bracket, `{`, starts the body of the function.
-->

`fn`構文は関数を新しく宣言し、かっこの`()`は引数がないことを示し、波括弧の`{`は関数の本体を開始します。

<!--
As you also learned in Chapter 1, `println!` is a macro that prints a string to
the screen:
-->

また、第 1 章で学んだように、`println!`は画面に文字列を表示するマクロです。


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

<!--
This code is printing a prompt stating what the game is and requesting input
from the user.
-->

このコードはゲームの内容などを示すプロンプトを表示し、ユーザに入力を求めています。

<!--
### Storing Values with Variables
-->

### 値を変数に保持する

<!--
Next, we’ll create a *variable* to store the user input, like this:
-->

次に、ユーザの入力を格納するための*変数*を作りましょう。
こんな感じです。

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

<!--
Now the program is getting interesting! There’s a lot going on in this little
line. We use the `let` statement to create the variable. Here’s another example:
-->

プログラムが少し興味深いものになってきました。
この小さな行の中でいろいろなことが起きています。
`let`文を使って変数を作っています。
別の例も見てみましょう。

```rust,ignore
let apples = 5;
```

<!--
This line creates a new variable named `apples` and binds it to the value 5. In
Rust, variables are immutable by default. We’ll be discussing this concept in
detail in the [“Variables and Mutability”][variables-and-mutability] section in Chapter 3. To make a variable mutable, we add `mut` before the
variable name:
-->

この行では`apples`という名前の新しい変数を作成し`5`という値に束縛しています。
Rust では変数はデフォルトで不変（immutable）になります。
この概念については第 3 章の[「変数と可変性」][variables-and-mutability]の節で詳しく説明します。
変数を可変（mutable）にするには、変数名の前に`mut`をつけます。

```rust,ignore
let apples = 5; // immutable
                // 不変
let mut bananas = 5; // mutable
                     // 可変
```

<!--
> Note: The `//` syntax starts a comment that continues until the end of the
> line. Rust ignores everything in comments. We’ll discuss comments in more
> detail in [Chapter 3][comments].
-->

> 注：`//`構文は行末まで続くコメントを開始し、Rust はコメント内のすべて無視します。
> コメントについては[第 3 章][comments]で詳しく説明します。

<!--
Returning to the guessing game program, you now know that `let mut guess` will
introduce a mutable variable named `guess`. The equal sign (`=`) tells Rust we
want to bind something to the variable now. On the right of the equals sign is
the value that `guess` is bound to, which is the result of calling
`String::new`, a function that returns a new instance of a `String`.
[`String`][string] is a string type provided by the standard
library that is a growable, UTF-8 encoded bit of text.
-->

数当てゲームのプログラムに戻りましょう。
ここまでの話で`let mut guess`が`guess`という名前の可変変数を導入することがわかったと思います。
等号記号（`=`）は Rust に、いまこの変数を何かに束縛したいことを伝えます。
等号記号の右側には`guess`が束縛される値があります。
これは`String::new`関数を呼び出すことで得られた値で、この関数は`String`型の新しいインスタンスを返します。
[`String`][string]は標準ライブラリによって提供される文字列型で、サイズが拡張可能な、UTF-8でエンコードされたテキスト片になります。

[string]: https://doc.rust-lang.org/std/string/struct.String.html

<!--
The `::` syntax in the `::new` line indicates that `new` is an associated
function of the `String` type. An *associated function* is a function that’s
implemented on a type, in this case `String`. This `new` function creates a
new, empty string. You’ll find a `new` function on many types, because it’s a
common name for a function that makes a new value of some kind.
-->

`::new`の行にある`::`構文は`new`が`String`型の関連関数であることを示しています。
*関連関数*とは、ある型（ここでは`String`）に対して実装される関数のことです。
この`new`関数は新しい空の文字列を作成します。
`new`関数は多くの型に見られます。
なぜなら、何らかの新しい値を作成する関数によくある名前だからです。

<!--
In full, the `let mut guess = String::new();` line has created a mutable
variable that is currently bound to a new, empty instance of a `String`. Whew!
-->

つまり`let mut guess = String::new();`という行は可変変数を作成し、その変数は現時点では新しい空の`String`のインスタンスに束縛されているわけです。
ふう！


<!--
### Receiving User Input
-->

### ユーザの入力を受け取る

<!--
Recall that we included the input/output functionality from the standard
library with `use std::io;` on the first line of the program. Now we’ll call
the `stdin` function from the `io` module, which will allow us to handle user
input:
-->

プログラムの最初の行に`use std::io`と書いて、標準ライブラリの入出力機能を取り込んだことを思い出してください。
ここで`io`モジュールの`stdin`関数を呼び出して、ユーザ入力を処理できるようにしましょう。

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

<!--
If we hadn’t imported the `io` library with `use std::io` at the beginning of
the program, we could still use the function by writing this function call as
`std::io::stdin`. The `stdin` function returns an instance of
[`std::io::Stdin`][iostdin], which is a type that represents a
handle to the standard input for your terminal.
-->

もし、プログラムの最初に`use std::io`と書いて`io`ライブラリをインポートしていなかったとしても、`std::io::stdin`のように呼び出せば、この関数を利用できます。
`stdin`関数はターミナルの標準入力へのハンドルを表す型である[`std::io::Stdin`][iostdin]のインスタンスを返します。

[iostdin]: https://doc.rust-lang.org/std/io/struct.Stdin.html

<!--
Next, the line `.read_line(&mut guess)` calls the [`read_line`][read_line] method on the standard input handle to get input from the user.
We’re also passing `&mut guess` as the argument to `read_line` to tell it what
string to store the user input in. The full job of `read_line` is to take
whatever the user types into standard input and append that into a string
(without overwriting its contents), so we therefore pass that string as an
argument. The string argument needs to be mutable so the method can change the
string’s content.
-->

次の`.read_line(&mut guess)`行は、標準入力ハンドルの[`read_line`][read_line]メソッドを呼び出し、ユーザからの入力を得ています。
また、`read_line`の引数として`&mut guess`を渡し、ユーザ入力をどの文字列に格納するかを指示しています。
`read_line`メソッドの仕事は、ユーザが標準入力に入力したものを文字列に（いまの内容を上書きせずに）追加することですので、文字列を引数として渡しているわけです。
引数の文字列は、その内容をメソッドが変更できるように、可変である必要があります。

[read_line]: https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line

<!--
The `&` indicates that this argument is a *reference*, which gives you a way to
let multiple parts of your code access one piece of data without needing to
copy that data into memory multiple times. References are a complex feature,
and one of Rust’s major advantages is how safe and easy it is to use
references. You don’t need to know a lot of those details to finish this
program. For now, all you need to know is that like variables, references are
immutable by default. Hence, you need to write `&mut guess` rather than
`&guess` to make it mutable. (Chapter 4 will explain references more
thoroughly.)
-->

この`&`は、この引数が*参照*であることを示し、これによりコードの複数の部分が同じデータにアクセスしても、そのデータを何度もメモリにコピーしなくて済みます。
参照は複雑な機能（訳注：一部のプログラム言語では正しく使うのが難しい機能）ですが、Rust の大きな利点の一つは参照を安全かつ簡単に使用できることです。
このプログラムを完成させるのに、そのような詳細を知る必要はないでしょう。
とりあえず知っておいてほしいのは、変数のように参照もデフォルトで不変であることです。
したがって、`&guess`ではなく`&mut guess`と書いて可変にする必要があります。
（参照については第 4 章でより詳しく説明します）

<!--
### Handling Potential Failure with the `Result` Type
-->

### `Result`型で失敗の可能性を扱う

<!--
We’re still working on this line of code. Although we’re now discussing a third
line of text, it’s still part of a single logical line of code. The next part
is this method:
-->

まだ、このコードの行は終わってません。
これから説明するのはテキスト上は 3 行目になりますが、まだ一つの論理的な行の一部分に過ぎません。
次の部分はこのメソッドです。

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

<!--
We could have written this code as:
-->

このコードは、こう書くこともできました。

```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

<!--
However, one long line is difficult to read, so it’s best to divide it. It’s
often wise to introduce a newline and other whitespace to help break up long
lines when you call a method with the `.method_name()` syntax. Now let’s
discuss what this line does.
-->

しかし、長い行は読みづらいので分割したほうがよいでしょう。
`.method_name()`構文でメソッドを呼び出すとき、長い行を改行と空白で分割するのが賢明なことがよくあります。
それでは、この行（`expect()`メソッド）が何をするのか説明します。

<!--
As mentioned earlier, `read_line` puts whatever the user enters into the string
we pass to it, but it also returns a value—in this case, an
[`io::Result`][ioresult]. Rust has a number of types named
`Result` in its standard library: a generic [`Result`][result]
as well as specific versions for submodules, such as `io::Result`. The `Result`
types are [*enumerations*][enums], often referred to as *enums*,
which can have a fixed set of possibilities known as *variants*. Enums are
often used with `match`, a conditional that makes it convenient to execute
different code based on which variant an enum value is when the conditional is
evaluated.
-->

前述したように、`read_line`メソッドは渡された文字列にユーザが入力したものを入れます。
しかし、同時に値（この場合は[`io::Result`][ioresult]）も返します。
Rust の標準ライブラリには`Result`という名前の型がいくつかあります。
汎用の[`Result`][result]と、`io::Result`といったサブモジュール用の特殊な型などです。
これらの`Result`型は[*列挙型*][enums]になります。
列挙型は*enum*とも呼ばれ、取りうる値として決まった数の*列挙子*（variant）を持ちます。
列挙型はよく`match`と一緒に使われます。
これは条件式の一種で、評価時に、列挙型の値がどの列挙子であるかに基づいて異なるコードを実行できるという便利なものです。

[ioresult]: https://doc.rust-lang.org/std/io/type.Result.html
[result]: https://doc.rust-lang.org/std/result/enum.Result.html

<!--
Chapter 6 will cover enums in more detail. The purpose of these `Result` types
is to encode error-handling information.
-->

enum については第 6 章で詳しく説明します。
これらの`Result`型の目的は、エラー処理に関わる情報を符号化（エンコード）することです。

[enums]: ch06-00-enums.html

<!--
`Result`’s variants are `Ok` and `Err`. The `Ok` variant indicates the operation
was successful, and inside `Ok` is the successfully generated value. The `Err`
variant means the operation failed, and `Err` contains information about how or
why the operation failed.
-->

`Result`の列挙子は`Ok`か`Err`です。
`Ok`列挙子は処理が成功したことを示し、`Ok`の中には正常に生成された値が入っています。
`Err`列挙子は処理が失敗したことを意味し、`Err`には処理が失敗した過程や理由についての情報が含まれています。

<!--
Values of the `Result` type, like values of any type, have methods defined on
them. An instance of `io::Result` has an [`expect` method][expect] that you can call. If this instance of `io::Result` is an `Err` value,
`expect` will cause the program to crash and display the message that you
passed as an argument to `expect`. If the `read_line` method returns an `Err`,
it would likely be the result of an error coming from the underlying operating
system. If this instance of `io::Result` is an `Ok` value, `expect` will take
the return value that `Ok` is holding and return just that value to you so you
can use it. In this case, that value is the number of bytes in the user’s input.
-->

`Result`型の値にも、他の型と同様にメソッドが定義されています。
`io::Result`のインスタンスには[`expect`メソッド][expect]がありますので、これを呼び出せます。
この`io::Result`インスタンスが`Err`の値の場合、`expect`メソッドはプログラムをクラッシュさせ、引数として渡されたメッセージを表示します。
`read_line`メソッドが`Err`を返したら、それはおそらく基礎となるオペレーティング・システムに起因するものでしょう。
もしこの`io::Result`オブジェクトが`Ok`値の場合、`expect`メソッドは`Ok`列挙子が保持する戻り値を取り出して、その値だけを返してくれます。
こうして私たちはその値を使うことができるわけです。
今回の場合、その値はユーザ入力のバイト数になります。

[expect]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect

<!--
If you don’t call `expect`, the program will compile, but you’ll get a warning:
-->

もし`expect`メソッドを呼び出さなかったら、コンパイルはできるものの警告が出るでしょう。

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

<!--
Rust warns that you haven’t used the `Result` value returned from `read_line`,
indicating that the program hasn’t handled a possible error.
-->

Rust は私たちが`read_line`から返された`Result`値を使用していないことを警告し、これはプログラムがエラーの可能性に対処していないことを示します。

<!--
The right way to suppress the warning is to actually write error handling, but
in our case we just want to crash this program when a problem occurs, so we can
use `expect`. You’ll learn about recovering from errors in [Chapter
9][recover].
-->

警告を抑制する正しい方法は実際にエラー処理を書くことです。
しかし、現時点では問題が起きたときにこのプログラムをクラッシュさせたいだけなので、`expect`が使えるわけです。
エラーからの回復については第 9 章で学びます。

<!--
### Printing Values with `println!` Placeholders
-->

### `println!`マクロのプレースホルダーで値を表示する

<!--
Aside from the closing curly bracket, there’s only one more line to discuss in
the code so far:
-->

閉じ波かっこを除けば、ここまでのコードで説明するのは残り 1 行だけです。

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

<!--
This line prints the string that now contains the user’s input. The `{}` set of
curly brackets is a placeholder: think of `{}` as little crab pincers that hold
a value in place. You can print more than one value using curly brackets: the
first set of curly brackets holds the first value listed after the format
string, the second set holds the second value, and so on. Printing multiple
values in one call to `println!` would look like this:
-->

この行はユーザの入力を現在保持している文字列を表示します。
一組の波括弧の`{}`はプレースホルダーです。
`{}`は値を所定の場所に保持する小さなカニのはさみだと考えてください。
波括弧をいくつか使えば複数の値を表示できます。
最初の波括弧の組はフォーマット文字列のあとに並んだ最初の値に対応し、2 組目は 2 番目の値、というように続いていきます。
一回の`println!`の呼び出しで複数の値を表示するなら次のようになります。

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

<!--
This code would print `x = 5 and y = 10`.
-->

このコードは`x = 5 and y = 10`と表示するでしょう。

<!--
### Testing the First Part
-->

### 最初の部分をテストする

<!--
Let’s test the first part of the guessing game. Run it using `cargo run`:
-->

数当てゲームの最初の部分をテストしてみましょう。
`cargo run`で走らせてください。


```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

<!--
At this point, the first part of the game is done: we’re getting input from the
keyboard and then printing it.
-->

これで、キーボードからの入力を得て、それを表示するという、ゲームの最初の部分は完成になります。

<!--
## Generating a Secret Number
-->

## 秘密の数字を生成する

<!--
Next, we need to generate a secret number that the user will try to guess. The
secret number should be different every time so the game is fun to play more
than once. We’ll use a random number between 1 and 100 so the game isn’t too
difficult. Rust doesn’t yet include random number functionality in its standard
library. However, the Rust team does provide a [`rand` crate][randcrate] with
said functionality.
-->

次にユーザが数当てに挑戦する秘密の数字を生成する必要があります。
この数字を毎回変えることで何度やっても楽しいゲームになります。
ゲームが難しくなりすぎないように 1 から 100 までの乱数を使用しましょう。
Rust の標準ライブラリには、まだ乱数の機能は含まれていません。
ですが、Rust の開発チームがこの機能を持つ[`rand`クレート][randcrate]を提供してくれています。

[randcrate]: https://crates.io/crates/rand

<!--
### Using a Crate to Get More Functionality
-->

### クレートを使用して機能を追加する

<!--
Remember that a crate is a collection of Rust source code files. The project
we’ve been building is a *binary crate*, which is an executable. The `rand`
crate is a *library crate*, which contains code intended to be used in other
programs, and can’t be executed on its own.
-->

クレートは Rust ソースコードを集めたものであることを思い出してください。
私たちがここまで作ってきたプロジェクトは*バイナリクレート*であり、これは実行可能ファイルになります。
`rand`クレートは*ライブラリクレート*です。
他のプログラムで使用するためのコードが含まれており、単独で実行することはできません。

<!--
Cargo’s coordination of external crates is where Cargo really shines. Before we
can write code that uses `rand`, we need to modify the *Cargo.toml* file to
include the `rand` crate as a dependency. Open that file now and add the
following line to the bottom beneath the `[dependencies]` section header that
Cargo created for you. Be sure to specify `rand` exactly as we have here, with
this version number, or the code examples in this tutorial may not work.
-->

Cargo がその力を発揮するのは外部クレートと連携するときです。
`rand`を使ったコードを書く前に、*Cargo.toml*ファイルを編集して`rand`クレートを依存関係に含める必要があります。
そのファイルを開いて、Cargo が作ってくれた`[dependencies]`セクションヘッダの下に次の行を追加してください。
バージョンナンバーを含め、ここに書かれている通り正確に`rand`を指定してください。
そうしないと、このチュートリアルのコード例が動作しないかもしれません。


<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">ファイル名：Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

<!--
In the *Cargo.toml* file, everything that follows a header is part of that
section that continues until another section starts. In `[dependencies]` you
tell Cargo which external crates your project depends on and which versions of
those crates you require. In this case, we specify the `rand` crate with the
semantic version specifier `0.8.3`. Cargo understands [Semantic
Versioning][semver] (sometimes called *SemVer*), which is a
standard for writing version numbers. The number `0.8.3` is actually shorthand
for `^0.8.3`, which means any version that is at least `0.8.3` but below
`0.9.0`. Cargo considers these versions to have public APIs compatible with
version `0.8.3`, and this specification ensures you’ll get the latest patch
release that will still compile with the code in this chapter. Any version
`0.9.0` or greater is not guaranteed to have the same API as what the following
examples use.
-->

*Cargo.toml*ファイルでは、ヘッダに続くものはすべて、他のセクションが始まるまで続くセクションの一部になります。
（訳注：Cargo.toml ファイル内には複数のセクションがあり、各セクションは`[ ]`で囲まれたヘッダ行から始まります）

`[dependecies]`はプロジェクトが依存する外部クレートと必要とするバージョンを Cargo に伝えます。
今回は`rand`クレートを`0.8.3`というセマンティックバージョン指定子で指定します。
Cargo は[セマンティックバージョニング][semver]（*SemVer*と呼ばれることもあります）を理解しており、これはバージョンナンバーを記述するための標準です。
`0.8.3`という数字は実際には`^0.8.3`の省略記法で、`0.8.3`以上`0.9.0`未満の任意のバージョンを意味します。
Cargo はこれらのバージョンを、バージョン`0.8.3`と互換性のある公開 API を持つものとみなします。
この仕様により、この章のコードが引き続きコンパイルできるようにしつつ、最新のパッチリリースを取得できるようになります。
0.9.0 以降のバージョンは、以下の例で使用しているものと同じ API を持つことを保証しません。

[semver]: http://semver.org

<!--
Now, without changing any of the code, let’s build the project, as shown in
Listing 2-2.
-->

さて、コードを一切変えずに、次のリスト 2-2 のようにプロジェクトをビルドしてみましょう。

```console
$ cargo build
    Updating crates.io index
    (crates.ioインデックスを更新しています)
  Downloaded rand v0.8.3
  (rand v0.8.3をダウンロードしています)
  Downloaded libc v0.2.86
  Downloaded getrandom v0.2.2
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.10
  Downloaded rand_chacha v0.3.0
  Downloaded rand_core v0.6.2
   Compiling rand_core v0.6.2
   (rand_core v0.6.2をコンパイルしています)
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
   (guessing_game v0.1.0をコンパイルしています)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

<!--
<span class="caption">Listing 2-2: The output from running `cargo build` after
adding the rand crate as a dependency</span>
-->

<span class="caption">リスト 2-2：rand クレートを依存として追加した後の`cargo build`コマンドの出力</span>

<!--
You may see different version numbers (but they will all be compatible with the
code, thanks to SemVer!), different lines (depending on the operating system),
and the lines may be in a different order.
-->

もしかしたら異なるバージョンナンバー（とはいえ、SemVer のおかげですべてのコードに互換性があります）や、
異なる行（オペレーティングシステムに依存します）が表示されるかもしれません。
また、行の順序も違うかもしれません。

<!--
When we include an external dependency, Cargo fetches the latest versions of
everything that dependency needs from the *registry*, which is a copy of data
from [Crates.io][cratesio]. Crates.io is where people in the Rust ecosystem
post their open source Rust projects for others to use.
-->

外部依存を持つようになると、Cargo はその依存関係が必要とするすべてについて最新のバージョンを*レジストリ*から取得します。
レジストリとは[Crates.io][cratesio]のデータのコピーです。
Crates.io は、Rust のエコシステムにいる人たちがオープンソースの Rust プロジェクトを投稿し、他の人が使えるようにする場所です。

[cratesio]: https://crates.io

<!--
After updating the registry, Cargo checks the `[dependencies]` section and
downloads any crates listed that aren’t already downloaded. In this case,
although we only listed `rand` as a dependency, Cargo also grabbed other crates
that `rand` depends on to work. After downloading the crates, Rust compiles
them and then compiles the project with the dependencies available.
-->

レジストリの更新後、Cargo は`[dependencies]`セクションにリストアップされているクレートをチェックし、まだ取得していないものがあればダウンロードします。
ここでは依存関係として`rand`だけを書きましたが、`rand`が動作するために依存している他のクレートも取り込まれています。
クレートをダウンロードしたあと、Rust はそれらをコンパイルし、依存関係が利用できる状態でプロジェクトをコンパイルします。

<!--
If you immediately run `cargo build` again without making any changes, you
won’t get any output aside from the `Finished` line. Cargo knows it has already
downloaded and compiled the dependencies, and you haven’t changed anything
about them in your *Cargo.toml* file. Cargo also knows that you haven’t changed
anything about your code, so it doesn’t recompile that either. With nothing to
do, it simply exits.
-->

何も変更せずにすぐに`cargo build`コマンドを再度実行すると、`Finished`の行以外は何も出力されないでしょう。
Cargo はすでに依存関係をダウンロードしてコンパイル済みであることを認識しており、また、あなたが*Cargo.toml*ファイルを変更していないことも知っているからです。
さらに、Cargo はあなたがコードを何も変更していないことも知っているので、再コンパイルもしません。
何もすることがないので単に終了します。

<!--
If you open up the *src/main.rs* file, make a trivial change, and then save it
and build again, you’ll only see two lines of output:
-->

*src/main.rs*ファイルを開いて些細な変更を加え、それを保存して再度ビルドすると2行しか表示されません。

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

<!--
These lines show Cargo only updates the build with your tiny change to the
*src/main.rs* file. Your dependencies haven’t changed, so Cargo knows it can
reuse what it has already downloaded and compiled for those.
-->

これらの行は Cargo が*src/main.rs*ファイルへの小さな変更に対して、ビルドを更新していることを示しています。
依存関係は変わっていないので、Cargo は既にダウンロードしてコンパイルしたものが再利用できることを知っています。

<!--
#### Ensuring Reproducible Builds with the *Cargo.lock* File
-->

#### *Cargo.lock*ファイルで再現可能なビルドを確保する

<!--
Cargo has a mechanism that ensures you can rebuild the same artifact every time
you or anyone else builds your code: Cargo will use only the versions of the
dependencies you specified until you indicate otherwise. For example, say that
next week version 0.8.4 of the `rand` crate comes out, and that version
contains an important bug fix, but it also contains a regression that will
break your code. To handle this, Rust creates the *Cargo.lock* file the first
time you run `cargo build`, so we now have this in the *guessing_game*
directory.
-->

Cargo はあなたや他の人があなたのコードをビルドするたびに、同じ生成物をリビルドできるようにするしくみを備えています。
Cargo は何も指示されない限り、指定したバージョンの依存のみを使用します。
たとえば来週`rand`クレートのバージョン 0.8.4 が出て、そのバージョンには重要なバグ修正が含まれていますが、同時にあなたのコードを破壊するリグレッションも含まれているとします。
これに対応するため、Rust は`cargo build`を最初に実行したときに*Cargo.lock*ファイルを作成します。
（いまの*guessing_game*ディレクトリにもあるはずです）

<!--
When you build a project for the first time, Cargo figures out all the
versions of the dependencies that fit the criteria and then writes them to
the *Cargo.lock* file. When you build your project in the future, Cargo will
see that the *Cargo.lock* file exists and use the versions specified there
rather than doing all the work of figuring out versions again. This lets you
have a reproducible build automatically. In other words, your project will
remain at `0.8.3` until you explicitly upgrade, thanks to the *Cargo.lock*
file.
-->

プロジェクトを初めてビルドするとき、Cargo は条件に合うすべての依存関係のバージョンを計算し*Cargo.lock*ファイルに書き込みます。
次にプロジェクトをビルドすると、Cargo は*Cargo.lock*ファイルが存在することを確認し、バージョンを把握するすべての作業を再び行う代わりに、そこで指定されているバージョンを使います。
これにより再現性のあるビルドを自動的に行えます。
言い換えれば、*Cargo.lock*ファイルのおかげで、あなたが明示的にアップグレードするまで、プロジェクトは`0.8.3`を使い続けます。

<!--
#### Updating a Crate to Get a New Version
-->

#### クレートを更新して新バージョンを取得する

<!--
When you *do* want to update a crate, Cargo provides the command `update`,
which will ignore the *Cargo.lock* file and figure out all the latest versions
that fit your specifications in *Cargo.toml*. Cargo will then write those
versions to the *Cargo.lock* file. Otherwise, by default, Cargo will only look
for versions greater than `0.8.3` and less than `0.9.0`. If the `rand` crate
has released the two new versions `0.8.4` and `0.9.0` you would see the
following if you ran `cargo update`:
-->

クレートを*本当に*アップグレードしたくなったときのために、Cargo は`update`コマンドを提供します。
このコマンドは*Cargo.lock*ファイルを無視して、*Cargo.toml*ファイル内の全ての指定に適合する最新バージョンを算出します。
成功したら Cargo はそれらのバージョンを*Cargo.lock*ファイルに記録します。
ただし、デフォルトで Cargo は`0.8.3`以上、`0.9.0`未満のバージョンのみを検索します。
もし`rand`クレートの新しいバージョンとして`0.8.4`と`0.9.0`の二つがリリースされていたなら、`cargo update`を実行したときに以下のようなメッセージが表示されるでしょう。

```console
$ cargo update
    Updating crates.io index
    (crates.ioインデックスを更新しています)
    Updating rand v0.8.3 -> v0.8.4
    (randクレートをv0.8.3 -> v0.8.4に更新しています)
```

<!--
Cargo ignores the `0.9.0` release. At this point, you would also notice a
change in your *Cargo.lock* file noting that the version of the `rand` crate
you are now using is `0.8.4`. To use `rand` version `0.9.0` or any version in
the `0.9.x` series, you’d have to update the *Cargo.toml* file to look like
this instead:
-->

Cargo は`0.9.0`リリースを無視します。
またそのとき、*Cargo.lock*ファイルが変更され、`rand`クレートの現在使用中のバージョンが`0.8.4`になったことにも気づくでしょう。
そうではなく、`rand`のバージョン`0.9.0`か、`0.9.x`系のどれかを使用するには、*Cargo.toml*ファイルを以下のように変更する必要があります。

```toml
[dependencies]

rand = "0.9.0"
```

<!--
The next time you run `cargo build`, Cargo will update the registry of crates
available and reevaluate your `rand` requirements according to the new version
you have specified.
-->

次に`cargo build`コマンドを実行したとき、Cargo は利用可能なクレートのレジストリを更新し、あなたが指定した新しいバージョンに従って`rand`の要件を再評価します。

<!--
There’s a lot more to say about [Cargo][doccargo] and [its
ecosystem][doccratesio] which we’ll discuss in Chapter 14, but
for now, that’s all you need to know. Cargo makes it very easy to reuse
libraries, so Rustaceans are able to write smaller projects that are assembled
from a number of packages.
-->

[Cargo][doccargo]と[そのエコシステム][doccratesio]については、まだ伝えたいことが山ほどありますが、それらについては第14章で説明します。
いまのところは、これだけ知っていれば十分です。
Cargo はライブラリの再利用をとても簡単にしてくれるので、Rustacean が数多くのパッケージから構成された小さなプロジェクトを書くことが可能になっています。

[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html

<!--
### Generating a Random Number
-->

### 乱数を生成する

<!--
Let’s start using `rand` to generate a number to guess. The next step is to
update *src/main.rs*, as shown in Listing 2-3.
-->

`rand`クレートを使って予想する数字を生成しましょう。
次のステップは*src/main.rs*ファイルをリスト 2-3 のように更新することです。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

<!--
<span class="caption">Listing 2-3: Adding code to generate a random
number</span>
-->

<span class="caption">リスト 2-3：乱数を生成するコードの追加</span>

<!--
First, we add the line `use rand::Rng`. The `Rng` trait defines methods that
random number generators implement, and this trait must be in scope for us to
use those methods. Chapter 10 will cover traits in detail.
-->

まず`use rand::Rng`という行を追加します。
`Rng`トレイトは乱数生成器が実装すべきメソッドを定義しており、それらのメソッドを使用するには、このトレイトがスコープ内になければなりません。
トレイトについて詳しくは第 10 章で解説します。

<!--
Next, we’re adding two lines in the middle. In the first line, we call the
`rand::thread_rng` function that gives us the particular random number
generator that we’re going to use: one that is local to the current thread of
execution and seeded by the operating system. Then we call the `gen_range`
method on the random number generator. This method is defined by the `Rng`
trait that we brought into scope with the `use rand::Rng` statement. The
`gen_range` method takes a range expression as an argument and generates a
random number in the range. The kind of range expression we’re using here takes
the form `start..end` and is inclusive on the lower bound but exclusive on the
upper bound, so we need to specify `1..101` to request a number between 1 and
100. Alternatively, we could pass the range `1..=100`, which is equivalent.
-->

次に、途中に 2 行を追加しています。
最初の行では`rand::thread_rng`関数を呼び出して、これから使う、ある特定の乱数生成器を取得しています。
なお、この乱数生成器は現在のスレッドに固有で、オペレーティングシステムからシード値を得ています。
そして、この乱数生成器の`gen_range`メソッドを呼び出しています。
このメソッドは`use rand::Rng`文でスコープに導入した`Rng`トレイトで定義されています。
`gen_range`メソッドは範囲式を引数にとり、その範囲内の乱数を生成してくれます。
ここで使っている範囲式の種類は`開始..終了`という形式で、下限値は含みますが上限値は含みません。
そのため、1 から 100 までの数をリクエストするには`1..101`と指定する必要があります。
あるいは、これと同等の`1..=100`という範囲を渡すこともできます。

<!--
> Note: You won’t just know which traits to use and which methods and functions
> to call from a crate, so each crate has documentation with instructions for
> using it. Another neat feature of Cargo is that running the `cargo doc
> --open` command will build documentation provided by all of your dependencies
> locally and open it in your browser. If you’re interested in other
> functionality in the `rand` crate, for example, run `cargo doc --open` and
> click `rand` in the sidebar on the left.
-->

> 注：クレートのどのトレイトを`use`するかや、どのメソッドや関数を呼び出すかを知るために、各クレートにはその使い方を説明したドキュメントが用意されています。
> Cargo のもう一つの素晴らしい機能は、`cargo doc --open`コマンドを走らせると、すべての依存クレートが提供するドキュメントをローカルでビルドして、ブラウザで開いてくれることです。
> たとえば`rand`クレートの他の機能に興味があるなら、`cargo doc --open`コマンドを実行して、左側のサイドバーにある`rand`をクリックしてください。

<!--
The second new line prints the secret number. This is useful while we’re
developing the program to be able to test it, but we’ll delete it from the
final version. It’s not much of a game if the program prints the answer as soon
as it starts!
-->

コードに追加した 2 行目は秘密の数字を表示します。
これはプログラムを開発している間のテストに便利ですが、最終版からは削除する予定です。
プログラムが始まってすぐに答えが表示されたらゲームになりませんからね！

<!--
Try running the program a few times:
-->

試しにプログラムを何回か走らせてみてください。

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

<!--
You should get different random numbers, and they should all be numbers between
1 and 100. Great job!
-->

毎回異なる乱数を取得し、それらはすべて 1 から 100 の範囲内の数字になるはずです。
よくやりました！

<!--
## Comparing the Guess to the Secret Number
-->

## 予想と秘密の数字を比較する

<!--
Now that we have user input and a random number, we can compare them. That step
is shown in Listing 2-4. Note that this code won’t compile quite yet, as we
will explain.
-->

さて、ユーザ入力と乱数が揃ったので両者を比較してみましょう。
このステップをリスト 2-4 に示します。
これから説明するように、このコードはまだコンパイルできないことに注意してください。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

<!--
<span class="caption">Listing 2-4: Handling the possible return values of
comparing two numbers</span>
-->

<span class="caption">リスト 2-4：二つの数値を比較したときに返される可能性のある値を処理する</span>

<!--
First we add another `use` statement, bringing a type called
`std::cmp::Ordering` into scope from the standard library. The `Ordering` type
is another enum and has the variants `Less`, `Greater`, and `Equal`. These are
the three outcomes that are possible when you compare two values.
-->

まず`use`文を追加して標準ライブラリから`std::cmp::Ordering`という型をスコープに導入しています。
`Ordering`も enum の一つで`Less`、`Greater`、`Equal`という列挙子を持っています。
これらは二つの値を比較したときに得られる 3 種類の結果です。

```rust,ignore
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

<!--
Then we add five new lines at the bottom that use the `Ordering` type. The
`cmp` method compares two values and can be called on anything that can be
compared. It takes a reference to whatever you want to compare with: here it’s
comparing the `guess` to the `secret_number`. Then it returns a variant of the
`Ordering` enum we brought into scope with the `use` statement. We use a
[`match`][match] expression to decide what to do next based on
which variant of `Ordering` was returned from the call to `cmp` with the values
in `guess` and `secret_number`.
-->

それから`Ordering`型を使用する新しい 5 行をいちばん下に追加してしています。
`cmp`メソッドは二つの値の比較を行い、比較できるものになら何に対しても呼び出せます。
比較対象への参照をとり、ここでは`guess`と`secret_number`を比較しています。
そして`use`文でスコープに導入した`Ordering`列挙型の列挙子を返します。
ここでは[`match`][match]式を使用しており、`guess`と`secret_number`の値に対して`cmp`を呼んだ結果返された`Ordering`の列挙子に基づき、次の動作を決定しています。

[match]: ch06-02-match.html

<!--
A `match` expression is made up of *arms*. An arm consists of a *pattern* to
match against, and the code that should be run if the value given to `match`
fits that arm’s pattern. Rust takes the value given to `match` and looks
through each arm’s pattern in turn. Patterns and the `match` construct are
powerful Rust features that let you express a variety of situations your code
might encounter and make sure that you handle them all. These features will be
covered in detail in Chapter 6 and Chapter 18, respectively.
-->

`match`式は複数の*アーム*（腕）で構成されます。
各アームはマッチさせる*パターン*と、`match`に与えられた値がそのアームのパターンにマッチしたときに実行されるコードで構成されます。
Rust は`match`に与えられた値を受け取って、各アームのパターンを順に照合していきます。
パターンと`match`式は Rust の強力な機能で、コードか遭遇する可能性のあるさまざまな状況を表現し、それらすべてを確実に処理できるようにします。
これらの機能については、それぞれ第 6 章と第 18 章で詳しく説明します。

<!--
Let’s walk through an example with the `match` expression we use here. Say that
the user has guessed 50 and the randomly generated secret number this time is
38. When the code compares 50 to 38, the `cmp` method will return
`Ordering::Greater`, because 50 is greater than 38. The `match` expression gets
the `Ordering::Greater` value and starts checking each arm’s pattern. It looks
at the first arm’s pattern, `Ordering::Less`, and sees that the value
`Ordering::Greater` does not match `Ordering::Less`, so it ignores the code in
that arm and moves to the next arm. The next arm’s pattern is
`Ordering::Greater`, which *does* match `Ordering::Greater`! The associated
code in that arm will execute and print `Too big!` to the screen. The `match`
expression ends because it has no need to look at the last arm in this scenario.
-->

ここで使われている`match`式に対して、例を通して順に見ていきましょう。
たとえばユーザが 50 と予想し、今回ランダムに生成された秘密の数字は 38 だったとしましょう。
コードが 50 と 38 を比較すると、50 は 38 よりも大きいので`cmp`メソッドは`Ordering::Greater`を返します。
`match`式は`Ordering::Greater`の値を取得し、各アームのパターンを吟味し始めます。
まず最初のアームのパターンである`Ordering::Less`を見て、`Ordering::Greater`の値と`Ordering::Less`がマッチしないことがわかります。
そのため、このアームのコードは無視して、次のアームに移ります。
次のアームのパターンは`Ordering::Greater`で、これは`Ordering::Greater`と*マッチ*します！&nbsp;
このアームに関連するコードが実行され、画面に`Too big!`と表示されます。
このシナリオでは最後のアームと照合する必要がないため`match`式（の評価）は終了します。

<!--
However, the code in Listing 2-4 won’t compile yet. Let’s try it:
-->

ところがリスト 2-4 のコードはまだコンパイルできません。
試してみましょう。

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

<!--
The core of the error states that there are *mismatched types*. Rust has a
strong, static type system. However, it also has type inference. When we wrote
`let mut guess = String::new()`, Rust was able to infer that `guess` should be
a `String` and didn’t make us write the type. The `secret_number`, on the other
hand, is a number type. A few of Rust’s number types can have a value between 1
and 100: `i32`, a 32-bit number; `u32`, an unsigned 32-bit number; `i64`, a
64-bit number; as well as others. Unless otherwise specified, Rust defaults to
an `i32`, which is the type of `secret_number` unless you add type information
elsewhere that would cause Rust to infer a different numerical type. The reason
for the error is that Rust cannot compare a string and a number type.
-->

このエラーの核心は*型の不一致*があると述べていることです。
Rust は強い静的型システムを持ちますが、型推論も備えています。
`let guess = String::new()`と書いたとき、Rust は`guess`が`String`型であるべきと推論したので、私たちはその型を書かずに済みました。
一方で`secret_number`は数値型です。
Rust のいくつかの数値型は 1 から 100 までの値を表現でき、それらの型には 32 ビット数値の`i32`、符号なしの 32 ビット数値の`u32`、64 ビット数値の`i64`などがあります。
Rust のデフォルトは`i32`型で、型情報をどこかに追加して Rust に異なる数値型だと推論させない限り`secret_number`の型はこれになります。
エラーの原因は Rust が文字列と数値型を比較できないためです。

<!--
Ultimately, we want to convert the `String` the program reads as input into a
real number type so we can compare it numerically to the secret number. We do so
by adding this line to the `main` function body:
-->

最終的にはプログラムが入力として読み込んだ`String`を実数型に変換し、秘密の数字と数値として比較できるようにしたいわけです。
そのためには`main`関数の本体に次の行を追加します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

<!--
The line is:
-->

その行とはこれのことです。

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

<!--
We create a variable named `guess`. But wait, doesn’t the program already have
a variable named `guess`? It does, but helpfully Rust allows us to *shadow* the
previous value of `guess` with a new one. Shadowing lets us reuse the `guess`
variable name rather than forcing us to create two unique variables, such as
`guess_str` and `guess` for example. We’ll cover this in more detail in Chapter
3, but for now know that this feature is often used when you want to convert a
value from one type to another type.
-->

`guess`という名前の変数を作成しています。
しかし待ってください、このプログラムには既に`guess`という名前の変数がありませんでしたか？&nbsp;
たしかにありますが、Rust では`guess`の前の値を新しい値で*覆い隠す*（shadow する）ことが許されているのです。
シャドーイング（shadowing）は、`guess_str`と`guess`のような重複しない変数を二つ作る代わりに、`guess`という変数名を再利用させてくれるのです。
これについては第 3 章で詳しく説明しますが、今のところ、この機能はある型から別の型に値を変換するときによく使われることを知っておいてください。

<!--
We bind this new variable to the expression `guess.trim().parse()`. The `guess`
in the expression refers to the original `guess` variable that contained the
input as a string. The `trim` method on a `String` instance will eliminate any
whitespace at the beginning and end, which we must do to be able to compare the
string to the `u32`, which can only contain numerical data. The user must press
<span class="keystroke">enter</span> to satisfy `read_line` and input their
guess, which adds a newline character to the string. For example, if the user
types <span class="keystroke">5</span> and presses <span
class="keystroke">enter</span>, `guess` looks like this: `5\n`. The `\n`
represents “newline”. (On Windows, pressing <span
class="keystroke">enter</span> results in a carriage return and a newline,
`\r\n`). The `trim` method eliminates `\n` or `\r\n`, resulting in just `5`.
-->

この新しい変数を`guess.trim().parse()`という式に束縛しています。
式の中にある`guess`は、入力が文字列として格納されたオリジナルの`guess`変数を指しています。
`String`インスタンスの`trim`メソッドは文字列の先頭と末尾の空白をすべて削除します。
これは数値データのみを表現できる`u32`型とこの文字列を比較するために（準備として）行う必要があります。
ユーザは予想を入力したあと`read_line`の処理を終えるために<span class="keystroke">Enter キー</span>を押す必要がありますが、これにより文字列に改行文字が追加されます。
たとえばユーザが<span class="keystroke">5</span>と入力して<span class="keystroke">Enter キー</span>を押すと、`guess`は`5\n`になります。
この`\n`は「改行」を表しています。（Windows では Enter キーを押すとキャリッジリターンと改行が入り`\r\n`となります）
`trim`メソッドは`\n`や`\r\n`を削除するので、その結果`5`だけになります。

<!--
The [`parse` method on strings][parse] parses a string into some
kind of number. Because this method can parse a variety of number types, we
need to tell Rust the exact number type we want by using `let guess: u32`. The
colon (`:`) after `guess` tells Rust we’ll annotate the variable’s type. Rust
has a few built-in number types; the `u32` seen here is an unsigned, 32-bit
integer. It’s a good default choice for a small positive number. You’ll learn
about other number types in Chapter 3. Additionally, the `u32` annotation in
this example program and the comparison with `secret_number` means that Rust
will infer that `secret_number` should be a `u32` as well. So now the
comparison will be between two values of the same type!
-->

[文字列の`parse`メソッド][parse]は文字列をパース（解析）して何らかの数値にします。
このメソッドは（文字列を）さまざまな数値型へとパースできるので、`let guess: u32`として Rust に正確な数値型を伝える必要があります。
`guess`の後にコロン（`:`）を付けることで変数の型に注釈をつけることを Rust に伝えています。
Rust には組み込みの数値型がいくつかあります。
ここにある`u32`は符号なし 32 ビット整数で、小さな正の数を表すデフォルトの型に適しています。
他の数値型については第 3 章で学びます。
さらに、このサンプルプログラムでは、`u32`という注釈と`secret_number`変数との比較していることから、Rust は`secret_number`変数も`u32`型であるべきだと推論しています。
つまり、いまでは二つの同じ型の値を比較することになるわけです！

[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse

<!--
The `parse` method will only work on characters that can logically be converted
into numbers and so can easily cause errors. If, for example, the string
contained `A👍%`, there would be no way to convert that to a number. Because it
might fail, the `parse` method returns a `Result` type, much as the `read_line`
method does (discussed earlier in [“Handling Potential Failure with the
`Result` Type”](#handling-potential-failure-with-the-result-type)). We’ll treat this `Result` the same way by using the `expect` method
again. If `parse` returns an `Err` `Result` variant because it couldn’t create
a number from the string, the `expect` call will crash the game and print the
message we give it. If `parse` can successfully convert the string to a number,
it will return the `Ok` variant of `Result`, and `expect` will return the
number that we want from the `Ok` value.
-->

`parse`メソッドは論理的に数値に変換できる文字にしか使えないので、よくエラーになります。
たとえば文字列に`A👍%`が含まれていたら数値に変換する術はありません。
解析に失敗する可能性があるため、`parse`メソッドは`read_line`メソッドと同様に`Result`型を返します
（[「`Result`型で失敗の可能性を扱う」](#result型で失敗の可能性を扱う)で説明しました）&nbsp;
今回も`expect`メソッドを使用して`Result`型を同じように扱います。
`parse`メソッドが文字列から数値を作成できなかったために`Result`型の`Err`列挙子を返したら、`expect`の呼び出しはゲームをクラッシュさせ、私たちが与えたメッセージを表示します。
`parse`が文字列をうまく数値へ変換できたときは`Result`型の`Ok`列挙子を返し、`expect`は`Ok`値から欲しい数値を返してくれます。

<!--
Let’s run the program now!
-->

さあ、プログラムを走らせましょう！

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

<!--
Nice! Even though spaces were added before the guess, the program still figured
out that the user guessed 76. Run the program a few times to verify the
different behavior with different kinds of input: guess the number correctly,
guess a number that is too high, and guess a number that is too low.
-->

いい感じです！&nbsp;
予想の前にスペースを追加したにもかかわらず、プログラムはちゃんとユーザが 76 と予想したことを理解しました。
このプログラムを何回か走らせ、数字を正しく言い当てたり、大きすぎる数字や小さすぎる数字を予想したりといった、異なる種類の入力に対する動作の違いを検証してください。

<!--
We have most of the game working now, but the user can make only one guess.
Let’s change that by adding a loop!
-->

現在、ゲームの大半は動作していますが、まだユーザは 1 回しか予想できません。
ループを追加して、その部分を変更しましょう！

<!--
## Allowing Multiple Guesses with Looping
-->

## ループで複数回の予想を可能にする

<!--
The `loop` keyword creates an infinite loop. We’ll add a loop to give users
more chances at guessing the number:
-->

`loop`キーワードは無限ループを作成します。
ループを追加してユーザが数字を予想する機会を増やします。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

<!--
As you can see, we’ve moved everything from the guess input prompt onward into
a loop. Be sure to indent the lines inside the loop another four spaces each
and run the program again. The program will now ask for another guess forever,
which actually introduces a new problem. It doesn’t seem like the user can quit!
-->

見ての通り予想入力のプロンプト以降をすべてループ内に移動しました。
ループ内の行をさらに 4 つのスペースでインデントして、もう一度プログラムを実行してください。
プログラムはいつまでも推測を求めるようになりましたが、実はこれが新たな問題を引き起こしています。
これではユーザが（ゲームを）終了できません！

<!--
The user could always interrupt the program by using the keyboard shortcut
<span class="keystroke">ctrl-c</span>. But there’s another way to escape this
insatiable monster, as mentioned in the `parse` discussion in [“Comparing the
Guess to the Secret Number”](#comparing-the-guess-to-the-secret-number): if the user enters a non-number answer, the program will crash. We
can take advantage of that to allow the user to quit, as shown here:
-->

ユーザはキーボードショートカットの<span class="keystroke">ctrl-c</span>を使えば、いつでもプログラムを中断させられます。
しかし「[予想と秘密の数字を比較する](#予想と秘密の数字を比較する)」の`parse`で述べたように、この飽くなきモンスターから逃れる方法はもう一つあります。
ユーザが数字以外の答えを入力すればプログラムはクラッシュします。
それを利用して以下のようにすれば終了できます。

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
(スレッド'main'は'数字を入力してください！：ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:785でパニックしました)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
(注：`RUST_BACKTRACE=1`で走らせるとバックトレースを見れます)
```

<!--
Typing `quit` will quit the game, but as you’ll notice so will entering any
other non-number input. This is suboptimal to say the least; we want the game
to also stop when the correct number is guessed.
-->

`quit`と入力すればゲームが終了しますが、数字以外の入力でもそうなります。
これは控えめに言っても最適ではありません。
私たちは正しい数字が予想されたときにゲームが停止するようにしたいのです。

<!--
### Quitting After a Correct Guess
-->

### 正しい予想をした後に終了する

<!--
Let’s program the game to quit when the user wins by adding a `break` statement:
-->

`break`文を追加して、ユーザが勝ったらゲームが終了するようにプログラムしましょう。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

<!--
Adding the `break` line after `You win!` makes the program exit the loop when
the user guesses the secret number correctly. Exiting the loop also means
exiting the program, because the loop is the last part of `main`.
-->

`You win!`の後に`break`の行を追記することで、ユーザが秘密の数字を正確に予想したときにプログラムがループを抜けるようになりました。
ループは`main`関数の最後の部分なので、ループを抜けることはプログラムを抜けることを意味します。

<!--
### Handling Invalid Input
-->

### 不正な入力を処理する

<!--
To further refine the game’s behavior, rather than crashing the program when
the user inputs a non-number, let’s make the game ignore a non-number so the
user can continue guessing. We can do that by altering the line where `guess`
is converted from a `String` to a `u32`, as shown in Listing 2-5.
-->

このゲームの動作をさらに洗練させるために、ユーザが数値以外を入力したときにプログラムをクラッシュさせるのではなく、数値以外を無視してユーザが数当てを続けられるようにしましょう。
これはリスト 2-5 のように、`String`から`u32`に`guess`を変換する行を変えることで実現できます。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

<!--
<span class="caption">Listing 2-5: Ignoring a non-number guess and asking for
another guess instead of crashing the program</span>
-->

<span class="caption">リスト 2-5：数値以外の予想を無視し、プログラムをクラッシュさせるのではなく、もう 1 回予想してもらう</span>

<!--
We switch from an `expect` call to a `match` expression to move from crashing
on an error to handling the error. Remember that `parse` returns a `Result`
type and `Result` is an enum that has the variants `Ok` and `Err`. We’re using a
`match` expression here, as we did with the `Ordering` result of the `cmp`
method.
-->

`expect`の呼び出しから`match`式に切り替えて、エラーによるクラッシュからエラー処理へと移行します。
`parse`が`Result`型を返すことと、`Result`が`Ok`と`Err`の列挙子を持つ列挙型であることを思い出してください。
ここでは`match`式を、`cmp`メソッドから返される`Ordering`を処理したときと同じように使っています。

<!--
If `parse` is able to successfully turn the string into a number, it will
return an `Ok` value that contains the resulting number. That `Ok` value will
match the first arm’s pattern, and the `match` expression will just return the
`num` value that `parse` produced and put inside the `Ok` value. That number
will end up right where we want it in the new `guess` variable we’re creating.
-->

もし`parse`メソッドが文字列から数値への変換に成功したなら、結果の数値を保持する`Ok`値を返します。
この`Ok`値は最初のアームのパターンにマッチします。
`match`式は`parse`メソッドが生成して`Ok`値に格納した`num`の値を返します。
その数値は私たちが望んだように、これから作成する新しい`guess`変数に収まります。

<!--
If `parse` is *not* able to turn the string into a number, it will return an
`Err` value that contains more information about the error. The `Err` value
does not match the `Ok(num)` pattern in the first `match` arm, but it does
match the `Err(_)` pattern in the second arm. The underscore, `_`, is a
catchall value; in this example, we’re saying we want to match all `Err`
values, no matter what information they have inside them. So the program will
execute the second arm’s code, `continue`, which tells the program to go to the
next iteration of the `loop` and ask for another guess. So, effectively, the
program ignores all errors that `parse` might encounter!
-->

もし`parse`メソッドが文字列から数値への変換に*失敗*したなら、エラーに関する詳細な情報を含む`Err`値を返します。
この`Err`値は最初の`match`アームの`Ok(num)`パターンにはマッチしませんが、2 番目のアームの`Err(_)`パターンにはマッチします。
アンダースコアの`_`はすべての値を受け付けます。
この例ではすべての`Err`値に対して、その中にどんな情報があってもマッチさせたいと言っているのです。
したがってプログラムは 2 番目のアームのコードである`continue`を実行します。
これは`loop`の次の繰り返しに移り、別の予想を求めるようプログラムに指示します。
つまり実質的にプログラムは`parse`メソッドが遭遇し得るエラーをすべて無視するようになります！

<!--
Now everything in the program should work as expected. Let’s try it:
-->

これでプログラム内のすべてが期待通りに動作するはずです。
試してみましょう。

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

<!--
Awesome! With one tiny final tweak, we will finish the guessing game. Recall
that the program is still printing the secret number. That worked well for
testing, but it ruins the game. Let’s delete the `println!` that outputs the
secret number. Listing 2-6 shows the final code.
-->

素晴らしい！&nbsp;
最後にほんの少し手を加えれば数当てゲームは完成です。
このプログラムはまだ秘密の数字を表示していることを思い出してください。
テストには便利でしたが、これではゲームが台無しです。
秘密の数字を表示している`println!`を削除しましょう。
最終的なコードをリスト 2-6 に示します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

<!--
<span class="caption">Listing 2-6: Complete guessing game code</span>
-->

<span class="caption">リスト 2-6：数当てゲームの完全なコード</span>

<!--
## Summary
-->

## まとめ

<!--
At this point, you’ve successfully built the guessing game. Congratulations!
-->

数当てゲームを無事に作り上げることができました。
おめでとうございます！

<!--
This project was a hands-on way to introduce you to many new Rust concepts:
`let`, `match`, functions, the use of external crates, and more. In the next
few chapters, you’ll learn about these concepts in more detail. Chapter 3
covers concepts that most programming languages have, such as variables, data
types, and functions, and shows how to use them in Rust. Chapter 4 explores
ownership, a feature that makes Rust different from other languages. Chapter 5
discusses structs and method syntax, and Chapter 6 explains how enums work.
-->

このプロジェクトではハンズオンを通して、`let`、`match`、メソッド、関連関数、外部クレートの使いかたなど、多くの新しい Rust の概念に触れました。
以降の章では、これらの概念についてより詳しく学びます。
第 3 章では変数、データ型、関数など多くのプログラミング言語が持つ概念を取り上げ、Rust での使い方を説明します。
第 4 章では Rust を他の言語とは異なるものに特徴づける、所有権について説明します。
第 5 章では構造体とメソッドの構文について説明し、第 6 章では列挙型がどのように動くのかについて説明します。

<!--
[prelude]: ../std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: ../std/string/struct.String.html
[iostdin]: ../std/io/struct.Stdin.html
[read_line]: ../std/io/struct.Stdin.html#method.read_line
[ioresult]: ../std/io/type.Result.html
[result]: ../std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: ../std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html
[match]: ch06-02-match.html
[parse]: ../std/primitive.str.html#method.parse
-->

[prelude]: https://doc.rust-lang.org/stable/std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#変数と可変性
[comments]: ch03-04-comments.html
[string]: https://doc.rust-lang.org/stable/std/string/struct.String.html
[iostdin]: https://doc.rust-lang.org/stable/std/io/struct.Stdin.html
[read_line]: https://doc.rust-lang.org/stable/std/io/struct.Stdin.html#method.read_line
[ioresult]: https://doc.rust-lang.org/stable/std/io/type.Result.html
[result]: https://doc.rust-lang.org/stable/std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html
[match]: ch06-02-match.html
[parse]: https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse
