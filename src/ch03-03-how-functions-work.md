<!--
## Functions
-->

## 関数

<!--
Functions are pervasive in Rust code. You’ve already seen one of the most
important functions in the language: the `main` function, which is the entry
point of many programs. You’ve also seen the `fn` keyword, which allows you to
declare new functions.
-->

関数は、Rust のコードにおいてよく見かける存在です。既に、言語において最も重要な関数のうちの一つを目撃していますね：
そう、`main`関数です。これは、多くのプログラムのエントリーポイント (`訳注`: プログラム実行時に最初に走る関数のこと) になります。
`fn`キーワードもすでに見かけましたね。これによって新しい関数を宣言することができます。

<!--
Rust code uses *snake case* as the conventional style for function and variable
names. In snake case, all letters are lowercase and underscores separate words.
Here’s a program that contains an example function definition:
-->

Rust の関数と変数の命名規則は、*スネークケース*(`訳注`: some_variable のような命名規則) を使うのが慣例です。
スネークケースとは、全文字を小文字にし、単語区切りにアンダースコアを使うことです。
以下のプログラムで、サンプルの関数定義をご覧ください：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

<!--
Function definitions in Rust start with `fn` and have a set of parentheses
after the function name. The curly brackets tell the compiler where the function
body begins and ends.
-->

Rust において関数定義は、`fn`キーワードで始まり、関数名の後に丸かっこの組が続きます。
波かっこが、コンパイラに関数本体の開始と終了の位置を伝えます。

<!--
We can call any function we’ve defined by entering its name followed by a set
of parentheses. Because `another_function` is defined in the program, it can be
called from inside the `main` function. Note that we defined `another_function`
*after* the `main` function in the source code; we could have defined it before
as well. Rust doesn’t care where you define your functions, only that they’re
defined somewhere.
-->

定義した関数は、名前に丸かっこの組を続けることで呼び出すことができます。
`another_function`関数がプログラム内で定義されているので、`main`関数内から呼び出すことができるわけです。
ソースコード中で`another_function`を`main`関数の*後*に定義していることに注目してください;
勿論、main 関数の前に定義することもできます。コンパイラは、関数がどこで定義されているかは気にしません。
どこかで定義されていることのみ気にします。

<!--
Let’s start a new binary project named *functions* to explore functions
further. Place the `another_function` example in *src/main.rs* and run it. You
should see the following output:
-->

*functions*という名前の新しいバイナリ生成プロジェクトを始めて、関数についてさらに深く探究していきましょう。
`another_function`の例を*src/main.rs*ファイルに配置して、走らせてください。
以下のような出力が得られるはずです：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

<!--
The lines execute in the order in which they appear in the `main` function.
First, the “Hello, world!” message prints, and then `another_function` is
called and its message is printed.
-->

行出力は、`main`関数内に書かれた順序で実行されています。最初に"Hello, world"メッセージが出て、
それから`another_function`が呼ばれて、こちらのメッセージが出力されています。

<!--
### Function Parameters
-->

### 関数の引数

<!--
Functions can also be defined to have *parameters*, which are special variables
that are part of a function's signature. When a function has parameters, you
can provide it with concrete values for those parameters. Technically, the
concrete values are called *arguments*, but in casual conversation people tend
to use the words “parameter” and “argument” interchangeably for either the
variables in a function's definition or the concrete values passed in when you
call a function.
-->

関数は、引数を持つようにも定義できます。引数とは、関数シグニチャの一部になる特別な変数のことです。
関数に引数があると、引数の位置に実際の値を与えることができます。技術的にはこの実際の値は
*実引数*と呼ばれますが、普段の会話では、仮引数 ("parameter") と実引数 ("argument") を関数定義の変数と関数呼び出し時に渡す実際の値、
両方の意味に区別なく使います (`訳注`: 日本語では、特別区別する意図がない限り、どちらも単に引数と呼ぶことが多いでしょう)。

<!--
The following rewritten version of `another_function` shows what parameters
look like in Rust:
-->

以下の書き直した`another_function`では、Rust の仮引数がどのようなものかを示しています：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

<!--
Try running this program; you should get the following output:
-->

このプログラムを走らせてみてください; 以下のような出力が得られるはずです：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

<!--
The declaration of `another_function` has one parameter named `x`. The type of
`x` is specified as `i32`. When `5` is passed to `another_function`, the
`println!` macro puts `5` where the pair of curly brackets were in the format
string.
-->

`another_function`の宣言には、`x`という名前の仮引数があります。`x`の型は、
`i32`と指定されています。値`5`が`another_function`に渡されると、`println!`マクロにより、
フォーマット文字列中の 1 組の波かっこがあった位置に値`5`が出力されます。

<!--
In function signatures, you *must* declare the type of each parameter. This is
a deliberate decision in Rust’s design: requiring type annotations in function
definitions means the compiler almost never needs you to use them elsewhere in
the code to figure out what you mean.
-->

関数シグニチャにおいて、各仮引数の型を宣言しなければ*なりません*。これは、Rust の設計において、
意図的な判断です：関数定義で型注釈が必要不可欠ということは、コンパイラがその意図するところを推し量るのに、
プログラマがコードの他の箇所で使用する必要がないということを意味します。

<!--
When you want a function to have multiple parameters, separate the parameter
declarations with commas, like this:
-->

関数に複数の仮引数を持たせたいときは、仮引数定義をカンマで区切ってください。
こんな感じです：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

<!--
This example creates a function with two parameters, both of which are `i32`
types. The function then prints the values in both of its parameters. Note that
function parameters don't all need to be the same type, they just happen to be
in this example.
-->

この例では、2 引数の関数を生成しています。そして、引数はどちらも`i32`型です。それからこの関数は、
仮引数の値を両方出力します。関数引数は、全てが同じ型である必要はありません。今回は、
偶然同じになっただけです。

<!--
Let’s try running this code. Replace the program currently in your *functions*
project’s *src/main.rs* file with the preceding example and run it using `cargo
run`:
-->

このコードを走らせてみましょう。今、*function*プロジェクトの*src/main.rs*ファイルに記載されているプログラムを先ほどの例と置き換えて、
`cargo run`で走らせてください：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

<!--
Because we called the function with `5` as the value for `x` and `6` is passed
as the value for `y`, the two strings are printed with these values.
-->

`x`に対して値`5`、`y`に対して値`6`を渡して関数を呼び出したので、この二つの文字列は、
この値で出力されました。

<!--
NOTE: doc.rust-lang.org のものではヘッダが変わっている
-->

<!--
### Function Bodies Contain Statements and Expressions
-->

### 関数本体は、文と式を含む

<!--
Function bodies are made up of a series of statements optionally ending in an
expression. So far, we’ve only covered functions without an ending expression,
but we have seen an expression as part of a statement. Because Rust is an
expression-based language, this is an important distinction to understand.
Other languages don’t have the same distinctions, so let’s look at what
statements and expressions are and how their differences affect the bodies of
functions.
-->

関数本体は、文が並び、最後に式を置くか文を置くという形で形成されます。現在までには、
式で終わらない関数だけを見てきたわけですが、式が文の一部になっているものなら見かけましたね。Rust は、式指向言語なので、
これは理解しておくべき重要な差異になります。他の言語にこの差異はありませんので、文と式がなんなのかと、
その違いが関数本体にどんな影響を与えるかを見ていきましょう。

<!--
We’ve actually already used statements and expressions. *Statements* are
instructions that perform some action and do not return a value. *Expressions*
evaluate to a resulting value. Let’s look at some examples.
-->

実のところ、もう文と式は使っています。*文*とは、なんらかの動作をして値を返さない命令です。
*式*は結果値に評価されます。ちょっと例を眺めてみましょう。

<!--
Creating a variable and assigning a value to it with the `let` keyword is a
statement. In Listing 3-1, `let y = 6;` is a statement.
-->

`let`キーワードを使用して変数を生成し、値を代入することは文になります。
リスト 3-1 で`let y = 6;`は文です。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

<!--
<span class="caption">Listing 3-1: A `main` function declaration containing one statement</span>
-->

<span class="caption">リスト 3-1: 1 文を含む`main`関数宣言</span>

<!--
Function definitions are also statements; the entire preceding example is a
statement in itself.
-->

関数定義も文になります。つまり、先の例は全体としても文になるわけです。

<!--
as は前の文にかかるべきだが、大して意味が変わらないので、語順をそのままにして後ろにかかるように訳した
-->

<!--
Statements do not return values. Therefore, you can’t assign a `let` statement
to another variable, as the following code tries to do; you'll get an error:
-->

文は値を返しません。故に、`let`文を他の変数に代入することはできません。
以下のコードではそれを試みていますが、エラーになります：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

<!--
When you run this program, the error you’ll get looks like this:
-->

このプログラムを実行すると、以下のようなエラーが出るでしょう：


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

<!--
The `let y = 6` statement does not return a value, so there isn’t anything for
`x` to bind to. This is different from what happens in other languages, such as
C and Ruby, where the assignment returns the value of the assignment. In those
languages, you can write `x = y = 6` and have both `x` and `y` have the value
`6`; that is not the case in Rust.
-->

この`let y = 6`という文は値を返さないので、`x`に束縛するものがないわけです。これは、
C や Ruby などの言語とは異なる動作です。C や Ruby では、代入は代入値を返します。これらの言語では、
`x = y = 6`と書いて、`x`も`y`も値 6 になるようにできるのですが、Rust においては、
そうは問屋が卸さないわけです。

<!--
Expressions evaluate to something and make up most of the rest of the code that
you’ll write in Rust. Consider a simple math operation, such as `5 + 6`, which
is an expression that evaluates to the value `11`. Expressions can be part of
statements: in Listing 3-1, the `6` in the statement `let y = 6;` is an
expression that evaluates to the value `6`. Calling a function is an
expression. Calling a macro is an expression. The block that we use to create
new scopes, `{}`, is an expression, for example:
-->

式は何かに評価され、これからあなたが書く Rust コードの多くを構成します。
簡単な数学演算 (`5 + 6`など) を思い浮かべましょう。この例は、値`11`に評価される式です。式は文の一部になりえます：
リスト 3-1 において、`let y = 6`という文の`6`は値`6`に評価される式です。関数呼び出しも式です。マクロ呼び出しも式です。
新しいスコープを作る際に使用するブロック (`{}`) も式です：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

<!--
This expression:
-->

以下の式：

```rust,ignore
{
    let x = 3;
    x + 1
}
```

<!--
is a block that, in this case, evaluates to `4`. That value gets bound to `y`
as part of the `let` statement. Note the `x + 1` line without a semicolon at
the end, which is unlike most of the lines you’ve seen so far. Expressions do
not include ending semicolons. If you add a semicolon to the end of an
expression, you turn it into a statement, which will then not return a value.
Keep this in mind as you explore function return values and expressions next.
-->

は今回の場合、`4`に評価されるブロックです。その値が、`let`文の一部として`y`に束縛されます。
今まで見かけてきた行と異なり、文末にセミコロンがついていない`x + 1`の行に気をつけてください。
式は終端にセミコロンを含みません。式の終端にセミコロンを付けたら、文に変えてしまいます。そして、文は値を返しません。
次に関数の戻り値や式を見ていく際にこのことを肝に銘じておいてください。

<!--
### Functions with Return Values
-->

### 戻り値のある関数

<!--
Functions can return values to the code that calls them. We don’t name return
values, but we do declare their type after an arrow (`->`). In Rust, the return
value of the function is synonymous with the value of the final expression in
the block of the body of a function. You can return early from a function by
using the `return` keyword and specifying a value, but most functions return
the last expression implicitly. Here’s an example of a function that
returns a value:
-->

関数は、それを呼び出したコードに値を返すことができます。戻り値に名前を付けはしませんが、
矢印 (`->`) の後に型を書いて確かに宣言します。Rust では、関数の戻り値は、関数本体ブロックの最後の式の値と同義です。
`return`キーワードで関数から早期リターンし、値を指定することもできますが、多くの関数は最後の式を暗黙的に返します。
こちらが、値を返す関数の例です：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

<!--
There are no function calls, macros, or even `let` statements in the `five`
function—just the number `5` by itself. That’s a perfectly valid function in
Rust. Note that the function’s return type is specified, too, as `-> i32`. Try
running this code; the output should look like this:
-->

`five`関数内には、関数呼び出しもマクロ呼び出しも、`let`文でさえ存在しません。数字の 5 が単独であるだけです。
これは、Rust において、完璧に問題ない関数です。関数の戻り値型が`-> i32`と指定されていることにも注目してください。
このコードを実行してみましょう; 出力はこんな感じになるはずです：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

<!--
The `5` in `five` is the function’s return value, which is why the return type
is `i32`. Let’s examine this in more detail. There are two important bits:
first, the line `let x = five();` shows that we’re using the return value of a
function to initialize a variable. Because the function `five` returns a `5`,
that line is the same as the following:
-->

`five`内の`5`が関数の戻り値です。だから、戻り値型が`i32`なのです。これについてもっと深く考察しましょう。
重要な箇所は 2 つあります：まず、`let x = five()`という行は、関数の戻り値を使って変数を初期化していることを示しています。
関数`five`は`5`を返すので、この行は以下のように書くのと同義です：

```rust
let x = 5;
```

<!--
Second, the `five` function has no parameters and defines the type of the
return value, but the body of the function is a lonely `5` with no semicolon
because it’s an expression whose value we want to return.
-->

2 番目に、`five`関数は仮引数をもたず、戻り値型を定義していますが、関数本体はセミコロンなしの`5`単独です。
なぜなら、これが返したい値になる式だからです。

<!--
Let's look at another example:
-->

もう一つ別の例を見ましょう：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

<!--
Running this code will print `The value of x is: 6`. But if we place a
semicolon at the end of the line containing `x + 1`, changing it from an
expression to a statement, we'll get an error:
-->

このコードを走らせると、`The value of x is: 6`と出力されるでしょう。しかし、
`x + 1`を含む行の終端にセミコロンを付けて、式から文に変えたら、エラーになるでしょう：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

<!--
Running this code produces an error, as follows:
-->

このコードを実行すると、以下のようにエラーが出ます：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

<!--
The main error message, “mismatched types,” reveals the core issue with this
code. The definition of the function `plus_one` says that it will return an
`i32`, but statements don’t evaluate to a value, which is expressed by `()`,
the empty tuple. Therefore, nothing is returned, which contradicts the function
definition and results in an error. In this output, Rust provides a message to
possibly help rectify this issue: it suggests removing the semicolon, which
would fix the error.
-->

メインのエラーメッセージである「型が合いません」でこのコードの根本的な問題が明らかになるでしょう。
関数`plus_one`の定義では、`i32`型を返すと言っているのに、文は値に評価されないからです。このことは、
`()`、つまり空のタプルとして表現されています。それゆえに、何も戻り値がなく、これが関数定義と矛盾するので、
結果としてエラーになるわけです。この出力内で、コンパイラは問題を修正する手助けになりそうなメッセージも出していますね：
セミコロンを削除するよう提言しています。そして、そうすれば、エラーは直るわけです。
