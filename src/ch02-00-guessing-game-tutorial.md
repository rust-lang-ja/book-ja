<!--
# Programming a Guessing Game
-->

# 数当てゲームをプログラムする

<!--
Let’s jump into Rust by working through a hands-on project together! This
chapter introduces you to a few common Rust concepts by showing you how to use
them in a real program. You’ll learn about `let`, `match`, methods, associated
functions, external crates, and more! The following chapters will explore these
ideas in more detail. In this chapter, you’ll practice the fundamentals.
-->

実物のプロジェクトに一緒に取り組むことで、Rustの世界へ飛び込みましょう！
この章では、実際のプログラム内で使用しながらいくつかの一般的なRustの概念に触れます。
`let`、`match`、メソッド、関連関数、外部クレートの使用などについて学ぶでしょう！
後ほどの章でこれらの概念について深く知ることになります。この章では、基礎部分だけにしましょう。

<!--
We’ll implement a classic beginner programming problem: a guessing game. Here’s
how it works: the program will generate a random integer between 1 and 100. It
will then prompt the player to enter a guess. After a guess is entered, the
program will indicate whether the guess is too low or too high. If the guess is
correct, the game will print a congratulatory message and exit.
-->

古典的な初心者向けのプログラミング問題を実装してみましょう: 数当てゲームです。 
これは以下のように動作します: プログラムは1から100までの乱数整数を生成します。
そしてプレーヤーに予想を入力するよう促します。予想を入力したら、プログラムは、
その予想が小さすぎたか大きすぎたかを出力します。予想が当たっていれば、ゲームは祝福メッセージを表示し、
終了します。

<!--
## Setting Up a New Project
-->

## 新規プロジェクトの立ち上げ

<!--
To set up a new project, go to the *projects* directory that you created in
Chapter 1 and make a new project using Cargo, like so:
-->

新規プロジェクトを立ち上げるには、第1章で作成した*projects*ディレクトリに行き、
Cargoを使って新規プロジェクトを作成します。以下のように:

```text
$ cargo new guessing_game --bin
$ cd guessing_game
```

<!--
The first command, `cargo new`, takes the name of the project (`guessing_game`)
as the first argument. The `--bin` flag tells Cargo to make a binary project,
like to the one in Chapter 1. The second command changes to the new
project’s directory.
-->

最初のコマンド`cargo new`は、プロジェクト名を第1引数に取ります(`guessing_game`ですね)。
`--bin`というフラグは、Cargoにバイナリ生成プロジェクトを作成させます。第1章のものと似ていますね。
2番目のコマンドで新規プロジェクトのディレクトリに移動します。

<!--
Look at the generated *Cargo.toml* file:
-->

生成された*Cargo.toml*ファイルを見てください:

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">ファイル名: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["名前 <you@example.com>"]

[dependencies]
```

<!--
If the author information that Cargo obtained from your environment is not
correct, fix that in the file and save it again.
-->

もし、Cargoがあなたの環境から取得した作者情報が間違っていたら、
ファイルを編集して保存し直してください。

<!--
As you saw in Chapter 1, `cargo new` generates a “Hello, world!” program for
you. Check out the *src/main.rs* file:
-->

第1章でも見かけたように、`cargo new`コマンドは、"Hello, world!"プログラムを生成してくれます。
*src/main.rs*ファイルをチェックしてみましょう:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

<!--
Now let’s compile this “Hello, world!” program and run it in the same step
using the `cargo run` command:
-->

さて、この"Hello, world!"プログラムをコンパイルし、`cargo run`コマンドを使用して、
以前と同じように動かしてみましょう:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
     Running `target/debug/guessing_game`
Hello, world!
```

<!--
The `run` command comes in handy when you need to rapidly iterate on a project,
as we'll do in this game, quickly testing each iteration before moving on to
the next one.
-->

`run`コマンドは、プロジェクトに迅速に段階を踏んで取り掛かる必要がある場合に有用であり、
次のステップに進む前に各段階を急速にテストして、このゲームではそれを行います。

<!--
Reopen the *src/main.rs* file. You’ll be writing all the code in this file.
-->

再度*src/main.rs*ファイルを開きましょう。ここにすべてのコードを書いていきます。

<!--
## Processing a Guess
-->

## 予想を処理する

<!--
The first part of the guessing game program will ask for user input, process
that input, and check that the input is in the expected form. To start, we’ll
allow the player to input a guess. Enter the code in Listing 2-1 into
*src/main.rs*
-->

数当てプログラムの最初の部分は、ユーザに入力を求め、その入力を処理し、予期した形式になっていることを確認します。
手始めに、プレーヤーが予想を入力できるようにしましょう。
リスト2-1のコードを*src/main.rs*に入力してください。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
use std::io;

fn main() {
    println!("Guess the number!");          // 数を当ててごらん

    println!("Please input your guess.");   // ほら、予想を入力してね

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");     // 行の読み込みに失敗しました

    println!("You guessed: {}", guess);     // 次のように予想しました: {}
}
```

<!--
<span class="caption">Listing 2-1: Code that gets a guess from the user and prints
it</span>
-->

<span class="caption">リスト2-1: ユーザに予想を入力してもらい、それを出力するコード</span>

> 注釈: The programming language Rust第1版の翻訳者によると、
> ソースコードのコメント中以外に日本語文字があるとコンパイルに失敗することがあるそうなので、文字列の英語は、コメントに和訳を載せます。
> また、重複する内容の場合には、最初の1回だけ掲載するようにします。

<!--
This code contains a lot of information, so let’s go over it line by line. To
obtain user input and then print the result as output, we need to bring the
`io` (input/output) library into scope. The `io` library comes from the
standard library (which is known as `std`):
-->

このコードには、たくさんの情報が詰め込まれていますね。なので、行ごとに見ていきましょう。
ユーザ入力を受け付け、結果を出力するためには、`io`(入/出力)ライブラリをスコープに導入する必要があります。
`io`ライブラリは、標準ライブラリ(`std`として知られています)に存在します:

```rust,ignore
use std::io;
```

<!--
By default, Rust brings only a few types into the scope of every program in
[the *prelude*][prelude]. If a type you want to use isn’t in the
prelude, you have to bring that type into scope explicitly with a `use`
statement. Using the `std::io` library provides you with a number of useful
features, including the ability to accept user input.
-->

デフォルトでは、[*prelude*][prelude]<!-- ignored -->に存在するいくつかの型のみ使えます。
もし、使用したい型がpreludeにない場合は、`use`文で明示的にその型をスコープに導入する必要があります。
`std::io`ライブラリを使用することで、ユーザ入力を受け付ける能力などの実用的な機能の多くを使用することができます。

[prelude]: ../../std/prelude/index.html

<!--
As you saw in Chapter 1, the `main` function is the entry point into the
program:
-->

第1章で見た通り、`main`関数がプログラムへのエントリーポイント(`脚注`: スタート地点)になります:

```rust,ignore
fn main() {
```

<!--
The `fn` syntax declares a new function, the parentheses, `()`,  indicate there
are no parameters, and the curly bracket, `{`, starts the body of the function.
-->

`fn`構文が関数を新しく宣言し、かっこの`()`は引数がないことを示し、波括弧の`{`が関数本体のスタート地点になります。

<!--
As you also learned in Chapter 1, `println!` is a macro that prints a string to
the screen:
-->

また、第1章で学んだように、`println!`は、文字列を画面に表示するマクロになります:

```rust,ignore
println!("Guess the number!");

println!("Please input your guess.");
```

<!--
This code is printing a prompt stating what the game is and requesting
input from the user.
-->

このコードは、このゲームが何かを出力し、ユーザに入力を求めています。

<!--
### Storing Values with Variables
-->

### 値を変数に保持する

<!--
Next, we’ll create a place to store the user input, like this:
-->

次に、ユーザ入力を保持する場所を作りましょう。こんな感じに:

```rust,ignore
let mut guess = String::new();
```

<!--
Now the program is getting interesting! There’s a lot going on in this little
line. Notice that this is a `let` statement, which is used to create a
*variable*. Here’s another example:
-->

さあ、プログラムが面白くなってきましたね。このたった1行でいろんなことが起きています。
これが`let`文であることに注目してください。これを使用して*変数*を生成しています。
こちらは、別の例です:

```rust,ignore
let foo = bar;
```

<!--
This line will create a new variable named `foo` and binds it to the value `bar`.
In Rust, variables are immutable by default. We'll discuss this concept in
detail in the "Variables and Mutability" section in Chapter 3. The following
example shows how to use `mut` before the variable name to make a variable
mutable:
-->

この行では、`foo`という名前の新しい変数を作成し、`bar`の値に束縛しています。
Rustでは、変数は標準で不変(immutable)です。この概念について詳しくは、
第3章の「変数と可変性」節で議論します。以下の例には、
変数名の前に`mut`をつけて変数を可変にする方法が示されています:

```rust,ignore
let foo = 5; // immutable
let mut bar = 5; // mutable
```

<!--
コメント中にコメント終了記号があると、パースに失敗するので、省いています。
> Note: The `//` syntax starts a comment that continues until the end of the
> line. Rust ignores everything in comments, which are discussed in more detail
> in Chapter 3.
-->

> 注釈: `//`という記法は、行末まで続くコメントを記述します。
> コンパイラは、コメントを一切無視し、これについても第3章で詳しく議論します。

<!--
Let’s return to the guessing game program. You now know that `let mut guess`
will introduce a mutable variable named `guess`. On the other side of the equal
sign (`=`) is the value that `guess` is bound to, which is the result of
calling `String::new`, a function that returns a new instance of a `String`.
[`String`][string] is a string type provided by the standard
library that is a growable, UTF-8 encoded bit of text.
-->

数当てゲームのプログラムに戻りましょう。さあ、`let mut guess`が`guess`という名前の可変変数を導入するとわかりましたね。
イコール記号(`=`)の反対側には、変数`guess`が束縛される値があります。この値は、
`String::new`関数の呼び出し結果であり、この関数は、`String`型のオブジェクトを返します。
[`String`][string]<!-- ignore -->型は、標準ライブラリによって提供される文字列型で、
サイズ可変、UTF-8エンコードされたテキスト破片になります。

[string]: ../../std/string/struct.String.html

<!--
The `::` syntax in the `::new` line indicates that `new` is an *associated
function* of the `String` type. An associated function is implemented on a type,
in this case `String`, rather than on a particular instance of a `String`. Some
languages call this a *static method*.
-->

`::new`行にある`::`という記法は、`new`が`String`型の*関連関数*であることを表しています。
関連関数とは、`String`型の特定のオブジェクトよりも型(この場合は`String`)に対して
実装された関数のことであり、*静的(スタティック)メソッド*と呼ばれる言語もあります。

<!--
This `new` function creates a new, empty string. You’ll find a `new` function
on many types, because it’s a common name for a function that makes a new value
of some kind.
-->

この`new`関数は、新しく空の文字列を生成します。`new`関数は、いろんな型に見られます。
なぜなら、何らかの新規値を生成する関数にとってありふれた名前だからです。

<!--
To summarize, the `let mut guess = String::new();` line has created a mutable
variable that is currently bound to a new, empty instance of a `String`. Whew!
-->

まとめると、`let mut guess = String::new();`という行は、現在、新たに空の`String`オブジェクトに束縛されている
可変変数を作っているわけです。ふう！

<!--
Recall that we included the input/output functionality from the standard
library with `use std::io;` on the first line of the program. Now we’ll call an
associated function, `stdin`, on `io`:
-->

プログラムの1行目で、`use std::io`として、標準ライブラリから入/出力機能を取り込んだことを思い出してください。
今度は、`io`型の`stdin`関連関数を呼び出しましょう:

```rust,ignore
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

<!--
If we hadn't the `use std::io` line at the beginning of the program, we
could have written this function call as `std::io::stdin`. The `stdin` function
returns an instance of [`std::io::Stdin`][iostdin], which is a
type that represents a handle to the standard input for your terminal.
-->

仮に、プログラムの冒頭で`use std::io`としていなければ、この関数呼び出しは、`std::io::stdin`と記述していたでしょう。
この`stdin`関数は、 [`std::io::Stdin`][iostdin]<!-- ignore -->オブジェクトを返し、この型は、
ターミナルの標準入力へのハンドルを表す型になります。

[iostdin]: ../../std/io/struct.Stdin.html

<!--
The next part of the code, `.read_line(&mut guess)`, calls the
[`read_line`][read_line] method on the standard input handle to
get input from the user. We’re also passing one argument to `read_line`: `&mut
guess`.
-->

その次のコード片、`.read_line(&mut guess)`は、標準入力ハンドルの[`read_line`][read_line]<!-- ignore -->
メソッドを呼び出して、ユーザから入力を受け付けます。また、`read_line`メソッドに対して、`&mut guess`という引数を一つ渡していますね。

[read_line]: ../../std/io/struct.Stdin.html#method.read_line

<!--
The job of `read_line` is to take whatever the user types into standard input
and place that into a string, so it takes that string as an argument. The
string argument needs to be mutable so the method can change the string’s
content by adding the user input.
-->

`read_line`メソッドの仕事は、ユーザが標準入力したものすべてを取り出し、文字列に格納することなので、
格納する文字列を引数として取ります。この文字列引数は、可変である必要があります。
メソッドがユーザ入力を追記して、文字列の中身を変えられるようにということですね。

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

`&`という記号は、この引数が*参照*であることを表し、これのおかげで、データを複数回メモリにコピーせずとも、
コードの複数箇所で同じデータにアクセスできるようになるわけです。参照は複雑な機能であり、
とても安全かつ簡単に参照を使うことができることは、Rustの主要な利点の一つでもあります。
そのような詳細を知らなくても、このプログラムを完成させることはできます。
現時点では、変数のように、参照も標準で不変であることを知っておけばいいでしょう。
故に、`&guess`と書くのではなく、`&mut guess`と書いて、可変にする必要があるのです。
(第4章で参照についてより詳細に説明します)

<!--
### Handling Potential Failure with the `Result` Type
-->

### `Result`型で失敗の可能性を扱う

<!--
We’re not quite done with this line of code. Although what we've discussed so
far is a single line of text, it’s only the first part of the single logical
line of code. The second part is this method:
-->

まだ、この行は終わりではありませんよ。ここまでに議論したのはテキストでは1行ですが、コードとしての論理行としては、
まだ所詮最初の部分でしかないのです。2番目の部分はこのメソッドです:

```rust,ignore
.expect("Failed to read line");
```

<!--
When you call a method with the `.foo()` syntax, it’s often wise to introduce a
newline and other whitespace to help break up long lines. We could have
written this code as:
-->

`.foo()`という記法で、メソッドを呼び出す時、改行と空白で長い行を分割するのがしばしば賢明です。
今回の場合、こう書くこともできますよね:

```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

<!--
However, one long line is difficult to read, so it’s best to divide it: two
lines for two method calls. Now let’s discuss what this line does.
-->

しかし、長い行は読みづらいものです。なので、分割しましょう: 2回のメソッド呼び出しに、2行です。
さて、この行が何をしているのかについて議論しましょうか。

<!--
As mentioned earlier, `read_line` puts what the user types into the string
we're passing it, but it also returns a value—in this case, an
[`io::Result`][ioresult]. Rust has a number of types named
`Result` in its standard library: a generic [`Result`][result] as
well as specific versions for submodules, such as `io::Result`.
-->

以前にも述べたように、`read_line`メソッドは、渡された文字列にユーザが入力したものを入れ込むだけでなく、
値も返します(今回は[`io::Result`][ioresult]<!-- ignore -->です)。 Rustには`Result`と名のついた型が、
標準ライブラリにたくさんあります: 汎用の[`Result`][result]<!-- ignore -->の他、
`io::Result`などのサブモジュール用に特化したものまで。

[ioresult]: ../../std/io/type.Result.html
[result]: ../../std/result/enum.Result.html

<!--
The `Result` types are [*enumerations*][enums], often referred
to as *enums*. An enumeration is a type that can have a fixed set of values,
and those values are called the enum’s *variants*. Chapter 6 will cover enums
in more detail.
-->

この`Result`型は、[*列挙型*][enums]<!-- ignore -->であり、普通、*enum*(イーナム)と呼ばれます。
列挙型とは、固定された種類の値を持つ型のことであり、それらの値は、enumの*列挙子*(variant)と呼ばれます。
enumについては、第6章で詳しく解説します。

[enums]: ch06-00-enums.html

<!--
For `Result`, the variants are `Ok` or `Err`. The `Ok` variant indicates
the operation was successful, and inside `Ok` is the successfully generated value.
The `Err` variant means the operation failed, and `Err` contains information
about how and why the operation failed.
-->

`Result`型に関しては、列挙子は`Ok`か`Err`です。`Ok`列挙子は、処理が成功したことを表し、
中に生成された値を保持します。`Err`列挙子は、処理が失敗したことを意味し、`Err`は、処理が失敗した過程や、
理由などの情報を保有します。

<!--
The purpose of these `Result` types is to encode error handling information.
Values of the `Result` type, like values of any type, have methods defined on them. An
instance of `io::Result` has an [`expect` method][expect] that
you can call. If this instance of `io::Result` is an `Err` value, `expect` will
cause the program to crash and display the message that you passed as an
argument to `expect`. If the `read_line` method returns an `Err`, it would
likely be the result of an error coming from the underlying operating system.
If this instance of `io::Result` is an `Ok` value, `expect` will take the
return value that `Ok` is holding and return just that value to you so you
could use it. In this case, that value is the number of bytes in what the user
entered into standard input.
-->

これら`Result`型の目的は、エラー処理の情報をコード化することです。`Result`型の値も、他の型同様、
メソッドが定義されています。`io::Result`オブジェクトには、呼び出し可能な[`expect`メソッド][expect]<!-- ignore -->があります。
この`io::Result`オブジェクトが`Err`値の場合、`expect`メソッドはプログラムをクラッシュさせ、
引数として渡されたメッセージを表示します。`read_line`メソッドが`Err`を返したら、
恐らく根底にあるOSによるエラーに起因するのでしょう。
この`io::Result`オブジェクトが`Ok`値の場合、`expect`メソッドは、`Ok`列挙子が保持する
返り値を取り出して、ただその値を返すので、これを使用することができるでしょう。
今回の場合、その返り値とは、ユーザが標準入力に入力したデータのバイト数になります。

[expect]: ../../std/result/enum.Result.html#method.expect

<!--
If you don’t call `expect`, the program will compile, but we’ll get a warning:
-->

もし、`expect`メソッドを呼び出さなかったら、コンパイルは通るものの、警告が出るでしょう:

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `std::result::Result` which must be used
(警告: 使用されなければならない`std::result::Result`が使用されていません)
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
```

<!--
Rust warns that we haven’t used the `Result` value returned from `read_line`,
indicating that the program hasn’t handled a possible error.
-->

コンパイラは、私たちが`read_line`メソッドから返ってきた`Result`値を使用していないと警告してきており、
これは、プログラムがエラーの可能性に対処していないことを示します。

<!--
The right way to suppress the warning is to actually write error handling, but
because you just want to crash this program when a problem occurs, you can use
`expect`. You'll learn about recovering from errors in Chapter 9.
-->

警告を抑制する正しい手段は、実際にエラー対処コードを書くことですが、今は、
問題が起きた時にプロラグムをクラッシュさせたいので、`expect`を使用できるわけです。
エラーから復旧する方法については、第9章で学ぶでしょう。

<!--
### Printing Values with `println!` Placeholders
-->

### `println!`マクロのプレースホルダーで値を出力する

<!--
Aside from the closing curly brackets, there’s only one more line to discuss in
the code added so far, which is the following:
-->

閉じ波かっこを除けば、ここまでに追加されたコードのうち議論すべきものは、残り1行であり、それは以下の通りです:

```rust,ignore
println!("You guessed: {}", guess);
```

<!--
This line prints the string we saved the user’s input in. The set of curly
brackets, `{}`, is a placeholder: think of `{}` as little crab pincers that
hold a value in place. You can print more than one value using curly brackets:
the first set of curly brackets holds the first value listed after the format
string, the second set holds the second value, and so on. Printing multiple
values in one call to `println!` would look like this:
-->

この行は、ユーザ入力を保存した文字列の中身を出力します。1組の波括弧の`{}`は、プレースホルダーの役目を果たします:
`{}`は値を所定の場所に保持する小さなカニのはさみと考えてください。波括弧を使って一つ以上の値を出力できます:
最初の波括弧の組は、フォーマット文字列の後に列挙された最初の値に対応し、
2組目は、2つ目の値、とそんな感じで続いていきます。1回の`println!`の呼び出しで複数の値を出力するコードは、
以下のような感じになります:

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

<!--
This code would print `x = 5 and y = 10`.
-->

このコードは、`x = 5 and y = 10`と出力するでしょう.

<!--
### Testing the First Part
-->

### 最初の部分をテストする

<!--
Let’s test the first part of the guessing game. Run it using `cargo run`:
-->

数当てゲームの最初の部分をテストしてみましょう。`cargo run`でプログラムを走らせてください:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
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

ここまでで、ゲームの最初の部分は完成になります: キーボードからの入力を受け付け、出力できています。

<!--
## Generating a Secret Number
-->

## 秘密の数字を生成する

<!--
Next, we need to generate a secret number that the user will try to guess. The
secret number should be different every time so the game is fun to play more
than once. Let’s use a random number between 1 and 100 so the game isn’t too
difficult. Rust doesn’t yet include random number functionality in its standard
library. However, the Rust team does provide a [`rand` crate][randcrate].
-->

次に、ユーザが数当てに挑戦する秘密の数字を生成する必要があります。毎回この秘密の数字は、変わるべきです。
ゲームが何回も楽しめるようにですね。ゲームが難しくなりすぎないように、1から100までの乱数を使用しましょう。
Rustの標準ライブラリには、乱数機能はまだ含まれていません。ですが、実は、
Rustの開発チームが[`rand`クレート][randcrate]を用意してくれています。

[randcrate]: https://crates.io/crates/rand

<!--
### Using a Crate to Get More Functionality
-->

### クレートを使用して機能を追加する

<!--
Remember that a crate is a package of Rust code. The project we’ve been
building is a *binary crate*, which is an executable. The `rand` crate is a
*library crate*, which contains code intended to be used in other programs.
-->

クレートはRustコードのパッケージであることを思い出してください。私たちがここまで作ってきたプロジェクトは、
*バイナリクレート*であり、これは実行可能形式になります。`rand`クレートは*ライブラリクレート*であり、
他のプログラムで使用するためのコードが含まれています。

<!--
Cargo’s use of external crates is where it really shines. Before we can write
code that uses `rand`, we need to modify the *Cargo.toml* file to include the
`rand` crate as a dependency. Open that file now and add the following line to
the bottom beneath the `[dependencies]` section header that Cargo created for
you:
-->

外部クレートを使用する部分は、Cargoがとても輝くところです。`rand`を使ったコードを書ける前に、
*Cargo.toml*ファイルを編集して、`rand`クレートを依存ファイルとして取り込む必要があります。
今このファイルを開いて、以下の行をCargoが自動生成した`[dependencies]`セクションヘッダの一番下に追記しましょう:

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">ファイル名: Cargo.toml</span>

```toml
[dependencies]

rand = "0.3.14"
```

<!--
In the *Cargo.toml* file, everything that follows a header is part of a section
that continues until another section starts. The `[dependencies]` section is
where you tell Cargo which external crates your project depends on and which
versions of those crates you require. In this case, we’ll specify the `rand`
crate with the semantic version specifier `0.3.14`. Cargo understands [Semantic
Versioning][semver] (sometimes called *SemVer*), which is a
standard for writing version numbers. The number `0.3.14` is actually shorthand
for `^0.3.14`, which means “any version that has a public API compatible with
version 0.3.14.”
-->

*Cargo.toml*ファイルにおいて、ヘッダに続くものは全て、他のセクションが始まるまで続くセクションの一部になります。
`[dependecies]`セクションは、プロジェクトが依存する外部クレートと必要とするバージョンを記述するところです。
ここでは、`rand`クレートで、セマンティックバージョン指定子には`0.3.14`を指定します。Cargoは、
バージョンナンバー記述の標準規格である[セマンティックバージョニング][semver]<!-- ignore --> (時に*SemVer*と呼ばれる)を理解します。
`0.3.14`という数字は、実際には`^0.3.14`の省略記法で、これは、「バージョン0.3.14と互換性のある公開APIを持つ任意のバージョン」を意味します。

[semver]: http://semver.org

<!--
Now, without changing any of the code, let’s build the project, as shown in
Listing 2-2.
-->

さて、コードは一切変えずに、リスト2-2のようにプロジェクトをビルドしましょう。

```text
$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index` (レジストリを更新しています)
 Downloading rand v0.3.14                                            (rand v0.3.14をダウンロードしています)
 Downloading libc v0.2.14                                            (libc v0.2.14をダウンロードしています)
   Compiling libc v0.2.14                                            (libc v0.2.14をコンパイルしています)
   Compiling rand v0.3.14                                            (rand v0.3.14をコンパイルしています)
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)   (guessing_game v0.1.0をコンパイルしています)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs    
```

<!--
<span class="caption">Listing 2-2: The output from running `cargo build` after
adding the rand crate as a dependency</span>
-->

<span class="caption">リスト2-2: randクレートを依存として追加した後の`cargo build`コマンドの出力</span>

<!--
You may see different version numbers (but they will all be compatible with
the code, thanks to SemVer!), and the lines may be in a different order.
-->

もしかしたら、バージョンナンバーは違うかもしれません(でも、互換性はあります、SemVerのおかげでね！)。
そして、行の出力順序も違うかもしれません。

<!--
Now that we have an external dependency, Cargo fetches the latest versions of
everything from the *registry*, which is a copy of data from
[Crates.io][cratesio]. Crates.io is where people in the Rust ecosystem post
their open source Rust projects for others to use.
-->

今や、外部依存を持つようになったので、Cargoは*レジストリ*(registry、登録所)から最新バージョンを拾ってきます。
*レジストリ*とは、[Crates.io][cratesio]のデータのコピーです。Crates.ioとは、Rustのエコシステムにいる人間が、
他の人が使えるように自分のオープンソースのRustプロジェクトを投稿する場所です。

[cratesio]: https://crates.io

<!--
After updating the registry, Cargo checks the `[dependencies]` section and
downloads any crates you don’t have yet. In this case, although we only listed
`rand` as a dependency, Cargo also grabbed a copy of `libc`, because `rand`
depends on `libc` to work. After downloading the crates, Rust compiles them and
then compiles the project with the dependencies available.
-->

レジストリの更新後、Cargoは`[dependencies]`セクションをチェックし、まだ取得していないクレートを全部ダウンロードします。
今回の場合、`rand`しか依存ファイルには列挙していませんが、Cargoは`libc`のコピーも拾ってきます。
`rand`クレートが`libc`に依存しているからですね。クレートのダウンロード完了後、コンパイラは依存ファイルをコンパイルし、
依存が利用可能な状態でプロジェクトをコンパイルします。

<!--
If you immediately run `cargo build` again without making any changes, you
won't get any output aside from the `Finished` line. Cargo knows it has already
downloaded and compiled the dependencies, and you haven't changed anything
about them in your *Cargo.toml* file. Cargo also knows that you haven't changed
anything about your code, so it doesn't recompile that either. With nothing to
do, it simply exits.
-->

何も変更せず即座に`cargo build`コマンドを走らせたら、`Finished`行を除いて何も出力されないでしょう。
Cargoは、既に全ての依存をダウンロードしてコンパイル済みであることも、
あなたが*Cargo.toml*ファイルを弄ってないことも知っているからです。さらに、Cargoはプログラマがコードを変更していないことも検知するので、
再度コンパイルすることもありません。することがないので、ただ単に終了します。

<!--
If you open up the *src/main.rs* file, make a trivial change, and then save it
and build again, you’ll only see two line of output:
-->

*src/main.rs*ファイルを開き、些細な変更をし、保存して再度ビルドを行えば、2行だけ出力があるでしょう:

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

<!--
These lines show Cargo only updates the build with your tiny change to the
*src/main.rs* file. Your dependencies haven't changed, so Cargo knows it can
reuse what it has already downloaded and compiled for those. It just rebuilds
your part of the code.
-->

これらの行は、Cargoが*src/main.rs*ファイルへの取るに足らない変更に対して、ビルドを更新していることを示しています。
依存は変更していないので、Cargoは、既にダウンロードしてコンパイルまで済ませてある依存を使用できると検知します。
自分で書いたコードのみ再ビルドをかけるわけです。

<!--
#### Ensuring Reproducible Builds with the *Cargo.lock* File
-->

#### *Cargo.lock*ファイルで再現可能なビルドを保証する

<!--
Cargo has a mechanism that ensures you can rebuild the same artifact every time
you or anyone else builds your code: Cargo will use only the versions of the
dependencies you specified until you indicate otherwise. For example, what
happens if next week version 0.3.15 of the `rand` crate comes out and contains
an important bug fix but also contains a regression that will break your code?
-->

Cargoは、プログラマが自分のコードを更新するたびに同じ生成物を再構成することを保証してくれるメカニズムを備えています: Cargoは、プログラマが示唆するまで、指定したバージョンの依存のみを使用します。
例として、`rand`クレートの次週のバージョン0.3.15が登場し、重要なバグ修正がなされているけれども、
自分のコードを破壊してしまう互換性破壊があった場合はどうなるでしょう？

<!--
The answer to this problem is the *Cargo.lock* file, which was created the
first time you ran `cargo build` and is now in your *guessing_game* directory.
When you build a project for the first time, Cargo figures out all the
versions of the dependencies that fit the criteria and then writes them to
the *Cargo.lock* file. When you build your project in the future, Cargo will
see that the *Cargo.lock* file exists and use the versions specified there
rather than doing all the work of figuring out versions again. This lets you
have a reproducible build automatically. In other words, your project will
remain at `0.3.14` until you explicitly upgrade, thanks to the *Cargo.lock*
file.
-->

この問題に対する回答は、*Cargo.lock*ファイルであり、このファイルは、初めて`cargo build`コマンドを
走らせた時に生成され、現在*guessing_game*ディレクトリに存在しています。プロジェクトを初めてビルドする際に、
Cargoは判断基準(criteria)に合致するよう全ての依存のバージョンを計算し、*Cargo.lock*ファイルに記述します。
次にプロジェクトをビルドする際には、Cargoは*Cargo.lock*ファイルが存在することを確かめ、
再度バージョンの計算の作業を行うのではなく、そこに指定されているバージョンを使用します。
このことにより、自動的に再現可能なビルドを構成できるのです。つまり、明示的にアップグレードしない限り、
プロジェクトが使用するバージョンは`0.3.14`に保たれるのです。*Cargo.lock*ファイルのおかげでね。

<!--
#### Updating a Crate to Get a New Version
-->

#### クレートを更新して新バージョンを取得する

<!--
When you *do* want to update a crate, Cargo provides another command, `update`,
which will ignore the *Cargo.lock* file and figure out all the latest versions
that fit your specifications in *Cargo.toml*. If that works, Cargo will write
those versions to the *Cargo.lock* file
-->

クレートを*本当に*アップグレードする必要が出てきたら、Cargoは別のコマンド(`update`)を提供します。
これは、*Cargo.lock*ファイルを無視して、*Cargo.toml*ファイル内の全ての指定に合致する最新バージョンを計算します。
それがうまくいったら、Cargoはそれらのバージョンを*Cargo.lock*ファイルに記述します。

<!--
But by default, Cargo will only look for versions larger than `0.3.0` and
smaller than `0.4.0`. If the `rand` crate has released two new versions,
`0.3.15` and `0.4.0`, you would see the following if you ran `cargo update`:
-->

しかし標準でCargoは、`0.3.0`より大きく、`0.4.0`未満のバージョンのみを検索します。
`rand`クレートの新バージョンが2つリリースされていたら(`0.3.15`と`0.4.0`だとします)、
`cargo update`コマンドを走らせた時に以下のようなメッセージを目の当たりにするでしょう:

```text
$ cargo update
    Updating registry `https://github.com/rust-lang/crates.io-index`
    (レジストリ`https://github.com/rust-lang/crates-io-index`を更新しています)
    Updating rand v0.3.14 -> v0.3.15
    (randクレートをv0.3.14 -> v0.3.15に更新しています)
```

<!--
At this point, you would also notice a change in your *Cargo.lock* file noting
that the version of the `rand` crate you are now using is `0.3.15`.
-->

<!--
ちょっとこなれた日本語にしづらい英文
-->

この時点で、*Cargo.lock*ファイルに書かれている現在使用している`rand`クレートのバージョンが、
`0.3.15`になっていることにも気付くでしょう。

<!--
If you wanted to use `rand` version `0.4.0` or any version in the `0.4.x`
series, you’d have to update the *Cargo.toml* file to look like this instead:
-->

`rand`のバージョン`0.4.0`または、`0.4.x`シリーズのどれかを使用したかったら、
代わりに*Cargo.toml*ファイルを以下のように更新しなければならないでしょう:

```toml
[dependencies]

rand = "0.4.0"
```

<!--
The next time you run `cargo build`, Cargo will update the registry of crates
available and reevaluate your `rand` requirements according to the new version
you have specified.
-->

次回、`cargo build`コマンドを走らせたら、Cargoは利用可能なクレートのレジストリを更新し、
`rand`クレートの必要条件を指定した新しいバージョンに従って再評価します。

<!--
There’s a lot more to say about [Cargo][doccargo] and [its
ecosystem][doccratesio] which we'll discuss in Chapter 14, but
for now, that’s all you need to know. Cargo makes it very easy to reuse
libraries, so Rustaceans are able to write smaller projects that are assembled
from a number of packages.
-->

まだ第14章で議論する[Cargo][doccargo]<!-- ignore -->と[そのエコシステム][doccratesio]<!-- ignore -->については述べたいことが山ほどありますが、
とりあえずは、これで知っておくべきことは全てです。
Cargoのおかげでライブラリはとても簡単に再利用ができるので、
Rustaceanは数多くのパッケージから構成された小規模のプロジェクトを書くことができるのです。

[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html

<!--
### Generating a Random Number
-->

### 乱数を生成する

<!--
Now that you've added the `rand` crate to *Cargo.toml*, let's start using
`rand`. The next step is to update *src/main.rs*, as shown in Listing 2-3.
-->

*Cargo.toml*に`rand`クレートを追加したので、`rand`クレートを使用開始しましょう。
次のステップは、リスト2-3のように*src/main.rs*ファイルを更新することです。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);    //秘密の数字は次の通り: {}

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

<!--
<span class="caption">Listing 2-3: Adding code to generate a
random number</span>
-->

<span class="caption">リスト2-3: 乱数を生成するコードの追加</span>

<!--
First, we add a line that lets Rust know we’ll be using the `rand` crate as an
external dependency. This also does the equivalent of calling `use rand`, so
now we can call anything in the `rand` crate by placing `rand::` before it.
-->

まず、コンパイラに`rand`クレートを外部依存として使用することを知らせる行を追加しています。
これにより、`use rand`を呼ぶのと同じ効果が得られるので、`rand`クレートのものを`rand::`という接頭辞をつけて呼び出せるようになりました。

<!--
Next, we add another `use` line: `use rand::Rng`. The `Rng` trait defines
methods that random number generators implement, and this trait must be in
scope for us to use those methods. Chapter 10 will cover traits in detail.
-->

次に、別の`use`行を追加しています: `use rand::Rng`ですね。`Rng`トレイトは乱数生成器が実装するメソッドを定義していて、
このトレイトがスコープにないと、メソッドを使用できないのです。トレイトについて詳しくは、
第10章で解説します。

<!--
Also, we’re adding two more lines in the middle. The `rand::thread_rng` function
will give us the particular random number generator that we’re going to use:
one that is local to the current thread of execution and seeded by the
operating system. Next, we call the `gen_range` method on the random number
generator. This method is defined by the `Rng` trait that we brought into
scope with the `use rand::Rng` statement. The `gen_range` method takes two
numbers as arguments and generates a random number between them. It’s inclusive
on the lower bound but exclusive on the upper bound, so we need to specify `1`
and `101` to request a number between 1 and 100.
-->

また、途中に2行追加もしています。`rand::thread_rng`関数は、これから使う特定の乱数生成器を返してくれます: この乱数生成器は、実行スレッドに固有で、OSにより、シード値を与えられています。
次に、この乱数生成器の`gen_range`メソッドを呼び出しています。このメソッドは、
`use rand::Rng`文でスコープに導入した`Rng`トレイトで定義されています。`gen_range`メソッドは二つの数字を引数に取り、
それらの間の乱数を生成してくれます。範囲は下限値を含み、上限値を含まないため、`1`と`101`と指定しないと1から100の範囲の数字は得られません。

<!--
Note: You won’t just know which traits to use and which methods and functions
to call from a crate. Instructions for using a crate are in each crate’s
documentation. Another neat feature of Cargo is that you can run the `cargo
doc --open` command, which will build documentation provided by all of your
dependencies locally and open it in your browser. If you’re interested in
other functionality in the `rand` crate, for example, run `cargo doc --open`
and click `rand` in the sidebar on the left.
-->

> 注釈: 単純に使用すべきトレイトと、クレートからどのメソッドと関数を呼び出すか知っているわけではないでしょう。
> クレートの使用方法は、各クレートのドキュメントにあります。Cargoの別の素晴らしい機能は、
> `cargo doc --open`コマンドを走らせてローカルに存在する依存すべてのドキュメントをビルドし、ブラウザで閲覧できる機能です。
> 例えば、`rand`クレートの他の機能に興味があるなら、`cargo doc --open`コマンドを走らせて、
> 左側のサイドバーから`rand`をクリックしてください。

<!--
The second line that we added to the code prints the secret number. This is
useful while we’re developing the program to be able to test it, but we’ll
delete it from the final version. It’s not much of a game if the program prints
the answer as soon as it starts!
-->

コードに追加した2行目は、秘密の数字を出力してくれます。これは、プログラムを開発中にはテストするのに役立ちますが、
最終版からは削除する予定です。プログラムがスタートと同時に答えを出力しちゃったら、ゲームになりませんからね！

<!--
Try running the program a few times:
-->

試しに何回かプログラムを走らせてみてください:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
     Running `target/debug/guessing_game`
Guess the number!                         (何回も出ているので、ここでは和訳は省略します)
The secret number is: 7
Please input your guess.
4
You guessed: 4
$ cargo run
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

毎回異なる乱数が出て、その数字はすべて1から100の範囲になるはずです。よくやりました！

<!--
## Comparing the Guess to the Secret Number
-->

## 予想と秘密の数字を比較する

<!--
Now that we have user input and a random number, we can compare them. That step
is shown in Listing 2-4. Note that this code won't compile quite yet, as we
will explain.
-->

今や、ユーザ入力と乱数生成ができるようになったので、比較することができますね。
このステップはリスト2-4に示されています。これから説明するように、このコードは現状ではコンパイルできないことに注意してください。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // ---snip---

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),       //小さすぎ！
        Ordering::Greater => println!("Too big!"),      //大きすぎ！
        Ordering::Equal => println!("You win!"),        //やったね！
    }
}
```

<!--
<span class="caption">Listing 2-4: Handling the possible return values of
comparing two numbers</span>
-->

<span class="caption">リスト2-4: 2値比較の可能性のある返り値を処理する</span>

<!--
The first new bit here is another `use` statement, bringing a type called
`std::cmp::Ordering` into scope from the standard library. Like `Result`,
`Ordering` is another enum, but the variants for `Ordering` are `Less`,
`Greater`, and `Equal`. These are the three outcomes that are possible when you
compare two values.
-->

最初の新しい点は、別の`use`文です。これで、`std::cmp::Ordering`という型を標準ライブラリからスコープに導入しています。
`Result`と同じく`Ordering`もenumです。ただ、`Ordering`の列挙子は、
`Less`、`Greater`、`Equal`です。これらは、2値比較した時に発生しうる3種類の結果です。

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

それから、一番下に新しく5行追加して`Ordering`型を使用しています。`cmp`メソッドは、
2値を比較し、比較できるものに対してなら何に対しても呼び出せます。このメソッドは、
比較したいものへの参照を取ります: ここでは、`guess`変数と`secret_number`変数を比較しています。
それからこのメソッドは`use`文でスコープに導入した`Ordering`列挙型の値を返します。 
[`match`][match]<!-- ignore -->式を使用して、`guess`変数と`secret_number`を`cmp`に渡して返ってきた`Ordering`の列挙子に基づき、
次の動作を決定しています。

[match]: ch06-02-match.html

<!--
A `match` expression is made up of *arms*. An arm consists of a *pattern* and
the code that should be run if the value given to the beginning of the `match`
expression fits that arm’s pattern. Rust takes the value given to `match` and
looks through each arm’s pattern in turn. The `match` construct and patterns
are powerful features in Rust that let you express a variety of situations your
code might encounter and helps ensure that you handle them all. These features
will be covered in detail in Chapter 6 and Chapter 18, respectively.
-->

`match`式は、複数の*アーム*(腕)からできています。一つのアームは、
パターンとそのパターンに`match`式の冒頭で与えた値がマッチした時に走るコードから構成されています。Rustは、
`match`に与えられた値を取り、各アームのパターンを順番に照合していきます。`match`式とパターンは、
コードを書く際に出くわす様々なシチュエーションを表現させてくれ、
すべてのシチュエーションに対処していることを保証するのを手助けしてくれるRustの強力な機能です。
これらの機能は、それぞれ、第6章と第18章で詳しく講義することにします。

<!--
Let’s walk through an example of what would happen with the `match` expression
used here. Say that the user has guessed 50 and the randomly generated secret
number this time is 38. When the code compares 50 to 38, the `cmp` method will
return `Ordering::Greater`, because 50 is greater than 38. The `match`
expression gets the `Ordering::Greater value and starts checking each arm's
pattern. It looks at the first arm's pattern, `Ordering::Less`, and sees that
the value `Ordering::Greater` does not match `Ordering::Less`, so it ignores
the code in that arm and moves to the next arm. The next arm’s pattern,
`Ordering::Greater`, *does* match `Ordering::Greater`! The associated code in
that arm will execute and print `Too big!` to the screen. The `match`
expression ends because it has no need to look at the last arm in this scenario.
-->

ここで使われている`match`式でどんなことが起こるかの例をじっくり観察してみましょう！例えば、
ユーザは50と予想し、ランダム生成された秘密の数字は今回、38だったとしましょう。コードが50と38を比較すると、
`cmp`メソッドは`Ordering::Greater`を返します。50は38よりも大きいからですね。
`match`式に`Ordering::Greater`が与えられ、各アームのパターンを吟味し始めます。まず、
最初のアームのパターンと照合します(`Ordering::Less`ですね)。しかし、
値の`Ordering::Greater`と`Ordering::Less`はマッチしないため、このアームのコードは無視され、
次のアームに移ります。次のアームのパターン、`Ordering::Greater`は*見事に*`Ordering::Greater`とマッチします！
このアームに紐づけられたコードが実行され、画面に`Too big!`が表示されます。
これで`match`式の実行は終わりになります。この筋書きでは、最後のアームと照合する必要はもうないからですね。

<!--
However, the code in Listing 2-4 won’t compile yet. Let’s try it:
-->

ところが、リスト2-4のコードは、まだコンパイルが通りません。試してみましょう:

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types          (型が合いません)
  --> src/main.rs:23:21
   |
23 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `std::string::String`, found integral variable
   |                                    (構造体`std::string::String`を予期したけど、整数型変数が見つかりました)
   |
   = note: expected type `&std::string::String`
   = note:    found type `&{integer}`

error: aborting due to previous error   (先のエラーのため、処理を中断します)
Could not compile `guessing_game`.      (`guessing_game`をコンパイルできませんでした)
```

<!--
The core of the error states that there are *mismatched types*. Rust has a
strong, static type system. However, it also has type inference. When we wrote
`let guess = String::new()`, Rust was able to infer that `guess` should be a
`String` and didn’t make us write the type. The `secret_number`, on the other
hand, is a number type. A few number types can have a value between 1 and 100:
`i32`, a 32-bit number; `u32`, an unsigned 32-bit number; `i64`, a 64-bit
number; as well as others. Rust defaults to an `i32`, which is the type of
`secret_number` unless we add type information elsewhere that would cause Rust
to infer a different numerical type. The reason for the error is that Rust
cannot compare a string and a number type.
-->

このエラーの核は、*型の不一致*があると言っています。Rustには、強い静的型システムがあります。
しかし、型推論にも対応しています。`let guess = String::new()`と書いた時、コンパイラは、
`guess`が`String`型であるはずと推論してくれ、その型を明示させられることはありませんでした。
一方で、`secret_number`変数は、数値型です。1から100を表すことができる数値型はいくつかあります:
`i32`は32ビットの数字; `u32`は32ビットの非負数字; `i64`は64ビットの数字などです。
Rustでの標準は、`i32`型であり、型情報をどこかに追加して、コンパイラに異なる数値型だと推論させない限り、
`secret_number`の型はこれになります。エラーの原因は、Rustでは、文字列と数値型を比較できないことです。

<!--
Ultimately, we want to convert the `String` the program reads as input into a
real number type so we can compare it numerically to the guess. We can do that
by adding the following two lines to the `main` function body:
-->

究極的には、プログラムが入力として読み込む`String`型を現実の数値型に変換し、
予想と数値として比較できるようにしたいわけです。これは、以下の2行を`main`関数の本体に追記することでできます:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
// --snip--

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");                 //数値を入力してください！

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

<!--
The two new lines are:
-->

その2行とは:

```rust,ignore
let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```

<!--
We create a variable named `guess`. But wait, doesn’t the program already have
a variable named `guess`? It does, but Rust allows us to *shadow* the previous
value of `guess` with a new one. This feature is often used in sutuations in
which you want to convert a value from one type to another type. Shadowing lets
us reuse the `guess` variable name rather than forcing us to create two unique
variables, such as `guess_str` and `guess` for example. (Chapter 3 covers
shadowing in more detail.)
-->

`guess`という名前の変数を生成しています。あれ、でも待って。もうプログラムには`guess`という名前の変数がありませんでしたっけ？
確かにありますが、Rustでは、新しい値で`guess`の値を*覆い隠す*(shadow)ことが許されているのです。
この機能は、値を別の型に変換したいシチュエーションでよく使われます。
シャドーイング(shadowing)のおかげで別々の変数を2つ作らされることなく、`guess`という変数名を再利用することができるのです。
`guess_str`と`guess`みたいなね(シャドーイングについては、第3章でもっと掘り下げます)。

<!--
We bind `guess` to the expression `guess.trim().parse()`. The `guess` in the
expression refers to the original `guess` that was a `String` with the input in
it. The `trim` method on a `String` instance will eliminate any whitespace at
the beginning and end. Although `u32` can contain only numerical characters,
the user must press the <span class="keystroke">enter</span> to satisfy
`read_line`. When the user presses <span class="keystroke">enter</span>, a
newline character is added to the string. For example, if the user types <span
class="keystroke">5</span> and presses <span class="keystroke">enter</span>,
`guess` looks like this: `5\n`. The `\n` represents “newline,” the result of
pressing <span class="keystroke">enter</span>. The `trim` method eliminates
`\n`, resulting in just `5`.
-->

`guess`を`guess.trim().parse()`という式に束縛しています。この式中の`guess`は、
入力が入った`String`型の元々の`guess`を指しています。`String`オブジェクトの`trim`メソッドは、
両端の空白をすべて除去します。`u32`型は、数字しか含むことができませんが、ユーザは、
`read_line`の処理を終えるために<span class="keystroke">エンター</span>を押さなければなりません。
ユーザが<span class="keystroke">エンター</span>を押したら、改行文字が文字列に追加されます。
具体例として、ユーザが<span class="keystroke">5</span>を入力して、
<span class="keystroke">エンター</span>を押せば、`guess`は次のようになります: `5\n`。
この`\n`が「改行」、つまりエンターキーを押した結果を表しているわけです。
`trim`メソッドは、`\n`を削除するので、ただの`5`になります。

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

[文字列の`parse`メソッド][parse]<!-- ignore -->は、文字列を解析して何らかの数値にします。
このメソッドは、いろんな数値型を解析できるので、`let guess: u32`としてコンパイラに私たちが求めている型をズバリ示唆する必要があるのです。
`guess`の後のコロン(`:`)がコンパイラに変数の型を注釈する合図になります。
Rustには、組み込みの数値型がいくつかあります; ここの`u32`型は、32ビットの非負整数です。
`u32`型は小さな非負整数のデフォルトの選択肢として丁度良いです。他の数値型については、第3章で学ぶでしょう。
付け加えると、このサンプルプログラムの`u32`という注釈と`secret_number`変数との比較は、
`secret_number`変数も`u32`型であるとコンパイラが推論することを意味します。
従って、今では比較が同じ型の2つの値で行われることになるわけです！

[parse]: ../../std/primitive.str.html#method.parse

<!--
The call to `parse` could easily cause an error. If, for example, the string
contained `A👍%`, there would be no way to convert that to a number. Because it
might fail, the `parse` method returns a `Result` type, much as the `read_line`
method does (discussed earlier in “Handling Potential Failure with the Result
Type”). We’ll treat this `Result` the same way by using the `expect` method
again. If `parse` returns an `Err` `Result` variant because it couldn’t create
a number from the string, the `expect` call will crash the game and print the
message we give it. If `parse` can successfully convert the string to a number,
it will return the `Ok` variant of `Result`, and `expect` will return the
number that we want from the `Ok` value.
-->

`parse`メソッドの呼び出しは、エラーになりやすいです。例としては、文字列が`A👍%`を含んでいたら、
数値に変換できるわけがありません。失敗する可能性があるので、`parse`メソッドは、
`Result`型を返すわけです。ちょうど、(「Result型で失敗する可能性に対処する」節で先ほど議論した)`read_line`メソッドのようにというわけですね。
今回も、`expect`メソッドを使用して`Result`型を同じように扱います。この`Result`を`expect`メソッドを再度使用して、
同じように扱います。もし、文字列から数値を生成できなかったために、`parse`メソッドが`Result`型の`Err`列挙子を返したら、
`expect`メソッドの呼び出しは、ゲームをクラッシュさせ、与えたメッセージを表示します。
もし、`parse`メソッドが文字列の数値への変換に成功したら、`Result`型の`Ok`列挙子を返し、
`expect`メソッドは、`Ok`値から必要な数値を返してくれます。

<!--
Let’s run the program now!
-->

さあ、プログラムを走らせましょう！

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43 secs
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

いいですね！予想の前にスペースを追加したにもかかわらず、プログラムはちゃんとユーザが76と予想したことを導き出しました。
プログラムを何回か走らせて、異なる入力の色々な振る舞いを確認してください: つまり、
数字を正しく言い当てたり、大きすぎる値を予想したり、小さすぎる数字を入力したりということです。

<!--
We have most of the game working now, but the user can make only one guess.
Let’s change that by adding a loop!
-->

ここまでで大方ゲームはうまく動くようになりましたが、まだユーザは1回しか予想できません。
ループを追加して、その部分を変更しましょう！

<!--
## Allowing Multiple Guesses with Looping
-->

## ループで複数回の予想を可能にする

<!--
The `loop` keyword creates an infinite loop. We'll add that now to give users
more chances at guessing the number:
-->

`loop`キーワードは、無限ループを作り出します。これを追加して、ユーザが何回も予想できるようにしましょう:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
// --snip--

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

<!--
As you can see, we’ve moved everything into a loop from the guess input prompt
onward. Be sure to indent the lines inside the loop another four spaces each
and run the program again. Notice that there is a new problem because the
program is doing exactly what we told it to do: ask for another guess forever!
It doesn't seem like the user can quit!
-->

見てわかる通り、予想入力部分以降をループに入れ込みました。ループ内の行にインデントを追加するのを忘れないようにして、
またプログラムを走らせてみましょう。新たな問題が発生したことに注目してください。
プログラムが教えた通りに動作しているからですね: 永遠に予想入力を求めるわけです！
これでは、ユーザが終了できないようです！

<!--
The user could always halt the program by using the keyboard shortcut <span
class="keystroke">ctrl-c</span>. But there’s another way to escape this
insatiable monster as mentioned in the `parse` discussion in “Comparing the
Guess to the Secret Number”: if the user enters a non-number answer, the
program will crash. The user can take advantage of that in order to quit, as
shown here:
-->

ユーザは、<span class="keystroke">ctrl-c</span>というキーボードショートカットを使って、いつでもプログラムを強制終了させられます。
しかし、「予想と秘密の数字を比較する」節の`parse`メソッドに関する議論で触れたように、
この貪欲なモンスターを回避する別の方法があります: ユーザが数字以外の答えを入力すれば、プログラムはクラッシュするのです。
ユーザは、その利点を活かして、終了することができます。以下のようにですね:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
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
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:785
(スレッド'main'は'数字を入力してください！: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:785でパニックしました)
note: Run with `RUST_BACKTRACE=1` for a backtrace.
(注釈: `RUST_BACKTRACE=1`で走らせるとバックトレースを見れます)
error: Process didn't exit successfully: `target/debug/guess` (exit code: 101)
(エラー: プロセスは予期なく終了しました)
```

<!--
Typing `quit` actually quits the game, but so will any other non-number input.
However, this is suboptimal to say the least. We want the game to automatically
stop when the correct number is guessed.
-->

`quit`と入力すれば、実際にゲームを終了できるわけですが、別に他の数字以外の入力でもそうなります。
しかしながら、これは最低限度と言えるでしょう。正しい数字が予想されたら、自動的にゲームが停止してほしいわけです。

<!--
### Quitting After a Correct Guess
-->

### 正しい予想をした後に終了する

<!--
Let’s program the game to quit when the user wins by adding a `break` statement:
-->

`break`文を追加して、ユーザが勝った時にゲームが終了するようにプログラムしましょう:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
// --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

<!--
Adding the `break` line after `You win!` makes the program exit the loop when
the user guesses the secret number correctly. Exiting the loop also means
exiting the program, because the loop is the last part of `main`.
-->

`break`文の1行を`You win!`の後に追記することで、ユーザが秘密の数字を正確に予想した時に、
プログラムはループを抜けるようになりました。ついでに、ループを抜けることは、プログラムを終了することを意味します。
ループが`main`関数の最後の部分だからですね。

<!--
### Handling Invalid Input
-->

### 不正な入力を処理する

<!--
To further refine the game’s behavior, rather than crashing the program when
the user inputs a non-number, let’s make the game ignore a non-number so the
user can continue guessing. We can do that by altering the line where `guess`
is converted from a `String` to a `u32`, as showin in Listing 2-5.
-->

さらにゲームの振る舞いを改善するために、ユーザが数値以外を入力した時にプログラムをクラッシュさせるのではなく、
非数値を無視してユーザが数当てを続けられるようにしましょう！これは、
`guess`が`String`型から`u32`型に変換される行を改変することで達成できます。リスト2-5のようにですね。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
// --snip--

io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

println!("You guessed: {}", guess);

// --snip--
```

<!--
<span class="caption">Listing 2-5: Ignoring a non-number guess and asking for
another guess instead of crashing the program</span>
-->

<span class="caption">リスト2-5: 非数値の予想を無視し、プログラムをクラッシュさせるのではなく、もう1回予想してもらう</span>

<!--
Switching from an `expect` call to a `match` expression is how you generally
move from crash on an error to handling the error. Remember that `parse`
returns a `Result` type and `Result` is an enum that has the variants `Ok` or
`Err`. We’re using a `match` expression here, as we did with the `Ordering`
result of the `cmp` method.
-->

`expect`メソッドの呼び出しから`match`式に切り替えることは、
エラーでクラッシュする動作からエラー処理を行う処理に変更する一般的な手段になります。`parse`メソッドは、
`Result`型を返し、`Result`は`Ok`か`Err`の列挙子を取りうる列挙型であることを思い出してください。
ここでは`match`式を使っています。`cmp`メソッドの`Ordering`という結果のような感じですね。

<!--
最後の行が日本語にしづらい
-->

<!--
If `parse` is able to successfully turn the string into a number, it will
return an `Ok` value that contains the resulting number. That `Ok` value will
match the first arm’s pattern, and the `match` expression will just return the
`num` value that `parse` produced and put inside the `Ok` value. That number
will end up right where we want it in the new `guess` variable we’re creating.
-->

`parse`メソッドは、文字列から数値への変換に成功したら、結果の数値を保持する`Ok`値を返します。
この`Ok`値は、最初のアームのパターンにマッチし、この`match`式は`parse`メソッドが生成し、
`Ok`値に格納した`num`の値を返すだけです。その数値が最終的に、生成している新しい`guess`変数として欲しい場所に存在します。

<!--
If `parse` is *not* able to turn the string into a number, it will return an
`Err` value that contains more information about the error. The `Err` value
does not match the `Ok(num)` pattern in the first `match` arm, but it does match
the `Err(_)` pattern in the second arm. The underscore, `_`, is a
catchall value; in this example, we’re saying we want to match all `Err`
values, no matter what information they have inside them. So the program will
execute the second arm's code, `continue`, which tells the program to go to the
next iteration of the `loop` and ask for another guess. So effectively, the
program ignores all errors that `parse` might encounter!
-->

`parse`メソッドは、文字列から数値への変換に*失敗*したら、エラーに関する情報を多く含む`Err`値を返します。
この`Err`値は、最初の`match`アームの`Ok(num)`というパターンにはマッチしないものの、
2番目のアームの`Err(_)`というパターンにはマッチするわけです。この`_`は、包括値です; この例では、
保持している情報がどんなものでもいいから全ての`Err`値にマッチさせたいと宣言しています。
従って、プログラムは2番目のアームのコードを実行し(`continue`ですね)、これは、
`loop`の次のステップに移り、再度予想入力を求めるようプログラムに指示します。故に実質的には、
プログラムは`parse`メソッドが遭遇しうる全てのエラーを無視するようになります！

<!--
Now everything in the program should work as expected. Let’s try it:
-->

さて、プログラムの全てがうまく予想通りに動くはずです。試しましょう:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
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
secret number. Listing 2-6 shows the final code:
-->

素晴らしい！最後にひとつまみ変更を加えて、数当てゲームを完了にしましょう。
プログラムが未だに秘密の数字を出力していることを思い出してください。テスト中はうまく動くけど、
ゲームを台無しにしてしまいます。秘密の数字を出力する`println!`を削除しましょう。
リスト2-6が成果物のコードです:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

<!--
<span class="caption">Listing 2-6: Complete guessing game code</span>
-->

<span class="caption">リスト2-6: 数当てゲームの完全なコード</span>

<!--
## Summary
-->

## まとめ

<!--
At this point, you’ve successfully built the guessing game. Congratulations!
-->

ここまでで、数当てゲームの構築に成功しました。おめでとうございます！

<!--
This project was a hands-on way to introduce you to many new Rust concepts:
`let`, `match`, methods, associated functions, external crates, and more. In
the next few chapters, you’ll learn about these concepts in more detail.
Chapter 3 covers concepts that most programming languages have, such as
variables, data types, and functions, and shows how to use them in Rust.
Chapter 4 explores ownership, a feature that makes Rust different from other
languages. Chapter 5 discusses structs and method syntax, and Chapter 6
explains how enums work.
-->

このプロジェクトは、たくさんの新しいRustの概念に触れる実践的な方法でした:
`let`、`match`、メソッド、関連関数、外部クレートの使用などなど。
以降の数章で、これらの概念についてより深く学ぶことになるでしょう。
第3章では、ほとんどのプログラミング言語に存在する、変数、データ型、関数などの概念について講義し、
それらのRustでの使用方法について示します。
第4章では、所有権について見ます。これにより、Rustは他の言語とかけ離れた存在になっています。
第5章では、構造体とメソッド記法について議論し、第6章ではenumの動作法を説明します。
