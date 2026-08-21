<!--
## Variables and Mutability
-->

## 変数と可変性

<!--
As mentioned in the [“Storing Values with
Variables”][storing-values-with-variables] section, by default,
variables are immutable. This is one of many nudges Rust gives you to write
your code in a way that takes advantage of the safety and easy concurrency that
Rust offers. However, you still have the option to make your variables mutable.
Let’s explore how and why Rust encourages you to favor immutability and why
sometimes you might want to opt out.
-->

[「値を変数に保持する」][storing-values-with-variables]の節で触れた通り、変数は標準で不変になります。これは、
Rustが提供する安全性や簡便な並行性の利点を享受する形でコードを書くための選択の1つです。
ところが、まだ変数を可変にするという選択肢も残されています。
どのように、そしてなぜRustは不変性を推奨するのか、さらには、なぜそれとは違う道を選びたくなることがあるのか見ていきましょう。

<!--
When a variable is immutable, once a value is bound to a name, you can’t change
that value. To illustrate this, generate a new project called *variables* in
your *projects* directory by using `cargo new variables`.
-->

変数が不変であると、値が一旦名前に束縛されたら、その値を変えることができません。
これを具体的に説明するために、*projects*ディレクトリに`cargo new variables`コマンドを使って、
*variables*という名前のプロジェクトを生成してください。

<!--
Then, in your new *variables* directory, open *src/main.rs* and replace its
code with the following code, which won’t compile just yet:
-->

それから、新規作成した*variables*ディレクトリで、*src/main.rs*ファイルを開き、
その中身を以下のコードに置き換えましょう。このコードはまだコンパイルできません:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

<!--
Save and run the program using `cargo run`. You should receive an error message
regarding an immutability error, as shown in this output:
-->

これを保存し、`cargo run`コマンドでプログラムを走らせてください。
次の出力に示されているような、不変性に関するエラーメッセージを受け取るはずです:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

<!--
This example shows how the compiler helps you find errors in your programs.
Compiler errors can be frustrating, but really they only mean your program
isn’t safely doing what you want it to do yet; they do *not* mean that you’re
not a good programmer! Experienced Rustaceans still get compiler errors.
-->

この例では、コンパイラがプログラムに潜むエラーを見つけ出す手助けをしてくれることが示されています。
コンパイルエラーは、イライラすることもあるものですが、本当はまだプログラムにしてほしいことを安全に行えていないだけということなのです。
エラーが出るからといって、あなたがいいプログラマではないという意味ではあり*ません*！
経験豊富なRustaceanでも、コンパイルエラーを出すことはあります。

<!--
You received the error message `` cannot assign twice to immutable variable `x`
`` because you tried to assign a second value to the immutable `x` variable.
-->

``不変変数`x`に2回代入できません``というエラーメッセージを受け取りました。
不変な`x`という変数に別の値を代入しようとしたからです。

<!--
It’s important that we get compile-time errors when we attempt to change a
value that’s designated as immutable because this very situation can lead to
bugs. If one part of our code operates on the assumption that a value will
never change and another part of our code changes that value, it’s possible
that the first part of the code won’t do what it was designed to do. The cause
of this kind of bug can be difficult to track down after the fact, especially
when the second piece of code changes the value only *sometimes*. The Rust
compiler guarantees that when you state that a value won’t change, it really
won’t change, so you don’t have to keep track of it yourself. Your code is thus
easier to reason through.

-->

不変と指定された値を変えようとした時に、コンパイルエラーが出るのは重要なことです。
なぜなら、この状況はまさしく、バグに繋がるからです。コードのある部分は、
値が変わることはないという前提のもとに処理を行い、別の部分がその値を変更していたら、
最初の部分が目論見通りに動いていない可能性があるのです。このようなバグは、発生してしまってからでは原因が追いかけづらいものです。
特に第2のコード片が、値を*時々*しか変えない場合、尚更です。
Rustコンパイラは、値が不変であると宣言したら、本当に変わらないことを担保してくれるので、変更を自分で追いかける必要がなくなります。
故にコードを通して正しいことを確認するのが簡単になるのです。

<!--
But mutability can be very useful, and can make code more convenient to write.
Although variables are immutable by default, you can make them mutable by
adding `mut` in front of the variable name as you did in [Chapter
2][storing-values-with-variables]. Adding `mut` also conveys
intent to future readers of the code by indicating that other parts of the code
will be changing this variable’s value.
-->

しかし可変性は非常に有用で、よりコードを書きやすくしてくれることもあります。
変数は標準では不変ですが、[第2章][storing-values-with-variables]でやったように、
変数名の前に`mut`キーワードを付けることで、可変にできるわけです。
また`mut`を付けることで、コードの別の部分がこの変数の値を変えるだろうと示すことによって、
未来の読者に対してその意図を汲ませることができるのです。

<!--
For example, let’s change *src/main.rs* to the following:
-->

例として、*src/main.rs*ファイルを以下のように書き換えましょう:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

<!--
When we run the program now, we get this:
-->

今、このプログラムを走らせると、以下のような出力が得られます:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

<!--
We’re allowed to change the value bound to `x` from `5` to `6` when `mut` is
used. Ultimately, deciding whether to use mutability or not is up to you and
depends on what you think is clearest in that particular situation.
-->

`mut`キーワードが使われると、`x`に束縛されている値を`5`から`6`に変更できます。
可変性を使うかどうかは最終的にはプログラマに任せられており、どちらがより明白と思えるかは個別の状況によるでしょう。

<!--
### Constants
-->

### 定数

<!--
Like immutable variables, *constants* are values that are bound to a name and
are not allowed to change, but there are a few differences between constants
and variables.
-->

不変変数のように、*定数*(constants)は名前に束縛され、変更することが叶わない値のことですが、
定数と変数の間にはいくつかの違いがあります。

<!--
First, you aren’t allowed to use `mut` with constants. Constants aren’t just
immutable by default—they’re always immutable. You declare constants using the
`const` keyword instead of the `let` keyword, and the type of the value *must*
be annotated. We’ll cover types and type annotations in the next section,
[“Data Types”][data-types] so don’t worry about the details
right now. Just know that you must always annotate the type.
-->

まず、定数には`mut`キーワードは使えません: 定数は標準で不変であるだけでなく、常に不変なのです。
定数は`let`キーワードの代わりに、`const`キーワードで宣言し、値の型は*必ず*注釈しなければなりません。
型と型注釈については次のセクション、[「データ型」][data-types]で講義しますので、その詳細について気にする必要はありません。
ただ単に型は常に注釈しなければならないのだと思っていてください。

<!--
Constants can be declared in any scope, including the global scope, which makes
them useful for values that many parts of code need to know about.
-->

定数はどんなスコープでも定義できます。グローバルスコープも含めてです。なので、
いろんなところで使用される可能性のある値を定義するのに役に立ちます。

<!--
The last difference is that constants may be set only to a constant expression,
not the result of a value that could only be computed at runtime.
-->

最後の違いは、定数は定数式にしかセットできないことです。実行時に評価される値にはセットできません。

<!--
Here’s an example of a constant declaration:
-->

これが定数定義の例です:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

<!--
The constant’s name is `THREE_HOURS_IN_SECONDS` and its value is set to the
result of multiplying 60 (the number of seconds in a minute) by 60 (the number
of minutes in an hour) by 3 (the number of hours we want to count in this
program). Rust’s naming convention for constants is to use all uppercase with
underscores between words. The compiler is able to evaluate a limited set of
operations at compile time, which lets us choose to write out this value in a
way that’s easier to understand and verify, rather than setting this constant
to the value 10,800. See the [Rust Reference’s section on constant
evaluation][const-eval] for more information on what operations can be used
when declaring constants.
-->

定数の名前は`THREE_HOURS_IN_SECONDS`で、その値は60（1分あたりの秒数）×60（1時間あたりの分数）×3（このプログラムで数えたい時間数）の結果にセットされています。
Rustの定数の命名規則は、全て大文字でアンダースコアで単語区切りすることです。
コンパイラはコンパイル時に一部の演算を評価することができるので、この定数に10,800という値を設定する代わりに、理解し検証しやすい方法でこの値を書き出すことを選択できます。
定数宣言内でどの演算が使用できるかについてのさらなる情報は、[Rust Referenceのconstant evaluationの節][const-eval]をお読みください。

<!--

Constants are valid for the entire time a program runs, within the scope in
which they were declared. This property makes constants useful for values in
your application domain that multiple parts of the program might need to know
about, such as the maximum number of points any player of a game is allowed to
earn, or the speed of light.
-->

定数は、プログラムが走る期間、定義されたスコープ内でずっと有効です。
この性質のおかげで、定数はプログラムのいろんなところで使用される可能性のあるアプリケーション空間の値を定義するのに有用です。
例えば、ゲームでプレイヤーが取得可能なポイントの最高値や、光速度などですね。

<!--
Naming hardcoded values used throughout your program as constants is useful in
conveying the meaning of that value to future maintainers of the code. It also
helps to have only one place in your code you would need to change if the
hardcoded value needed to be updated in the future.
-->

プログラム中で使用されるハードコードされた値に対して、定数として名前付けすることは、
コードの将来的な管理者にとって値の意味を汲むのに役に立ちます。将来、ハードコードされた値を変える必要が出た時に、
たった1箇所を変更するだけで済むようにもしてくれます。

<!--
### Shadowing
-->

### シャドーイング

<!--
As you saw in the guessing game tutorial in [Chapter
2][comparing-the-guess-to-the-secret-number], you can declare a
new variable with the same name as a previous variable. Rustaceans say that the
first variable is *shadowed* by the second, which means that the second
variable is what the compiler will see when you use the name of the variable.
In effect, the second variable overshadows the first, taking any uses of the
variable name to itself until either it itself is shadowed or the scope ends.
We can shadow a variable by using the same variable’s name and repeating the
use of the `let` keyword as follows:
-->

[第2章][comparing-the-guess-to-the-secret-number]の数当てゲームのチュートリアルで見たように、
前に定義した変数と同じ名前の変数を新しく宣言できます。
Rustaceanはこれを、最初の変数は2番目の変数に*覆い隠さ*れたと言います。
これはその変数名を使用した際に、コンパイラは2番目の変数を見るという意味です。
2番目の変数は実質的に、最初の変数にその影を投げかけ、自身が覆い隠されるかスコープが終了するまで、
変数名の使用を自身へのものとして扱います。
以下のようにして、同じ変数名を用いて変数を覆い隠し、`let`キーワードの使用を繰り返します:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

<!--
This program first binds `x` to a value of `5`. Then it creates a new variable
`x` by repeating `let x =`, taking the original value and adding `1` so the
value of `x` is then `6`. Then, within an inner scope created with the curly
brackets, the third `let` statement also shadows `x` and creates a new
variable, multiplying the previous value by `2` to give `x` a value of `12`.
When that scope is over, the inner shadowing ends and `x` returns to being `6`.
When we run this program, it will output the following:
-->

このプログラムはまず、`x`を`5`という値に束縛します。それから`let x =`を繰り返すことで新しい変数`x`を作り、
元の値に`1`を加えることになるので、`x`の値は`6`になります。
次に波括弧によって作られた内側のスコープの中で、3番目の`let`文も`x`を覆い隠して新しい変数を作り、
以前の値に`2`をかけることになるので、`x`の最終的な値は`12`になります。
スコープが終わるとシャドーイングは終了し、`x`の値は元の`6`に戻ります。
このプログラムを走らせたら、以下のように出力するでしょう:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

<!--
Shadowing is different from marking a variable as `mut` because we’ll get a
compile-time error if we accidentally try to reassign to this variable without
using the `let` keyword. By using `let`, we can perform a few transformations
on a value but have the variable be immutable after those transformations have
been completed.
-->

シャドーイングは、変数を`mut`にするのとは違います。なぜなら、`let`キーワードを使わずに、
誤ってこの変数に再代入を試みようものなら、コンパイルエラーが出るからです。`let`を使うことで、
値にちょっとした加工は行えますが、その加工が終わったら、変数は不変になるわけです。

<!--
The other difference between `mut` and shadowing is that because we’re
effectively creating a new variable when we use the `let` keyword again, we can
change the type of the value but reuse the same name. For example, say our
program asks a user to show how many spaces they want between some text by
inputting space characters, and then we want to store that input as a number:
-->

`mut`と上書きのもう一つの違いは、再度`let`キーワードを使用したら、実効的には新しい変数を生成していることになるので、
値の型を変えつつ、同じ変数名を使いまわせることです。例えば、
プログラムがユーザに何らかのテキストに対して空白文字を入力することで何個分のスペースを表示したいかを尋ねて、
そうしたらこの入力を数値として保持したいとしましょう:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

<!--
The first `spaces` variable is a string type and the second `spaces` variable
is a number type. Shadowing thus spares us from having to come up with
different names, such as `spaces_str` and `spaces_num`; instead, we can reuse
the simpler `spaces` name. However, if we try to use `mut` for this, as shown
here, we’ll get a compile-time error:
-->

最初の`spaces`変数は文字列型であり、2番目の`spaces`変数は数値型です。故に、シャドーイングのおかげで、
異なる名前を思いつく必要がなくなるわけです。`spaces_str`と`spaces_num`などですね; 代わりに、
よりシンプルな`spaces`という名前を再利用できるわけです。一方で、この場合に`mut`を使おうとすると、
以下に示した通りですが、コンパイルエラーになるわけです:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

<!--
The error says we’re not allowed to mutate a variable’s type:
-->

変数の型を可変にすることは許されていないと言われているわけです:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

<!--
Now that we’ve explored how variables work, let’s look at more data types they
can have.
-->

さあ、変数が動作する方法を見てきたので、今度は変数が取りうるデータ型について見ていきましょう。

<!--
[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[data-types]: ch03-02-data-types.html#data-types
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#storing-values-with-variables
[const-eval]: ../reference/const_eval.html
-->

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#予想と秘密の数字を比較する
[data-types]: ch03-02-data-types.html#データ型
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#値を変数に保持する
[const-eval]: https://doc.rust-lang.org/reference/const_eval.html
