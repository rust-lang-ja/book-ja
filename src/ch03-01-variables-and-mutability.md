<!--
## Variables and Mutability
-->

## 変数と可変性

<!--
As mentioned in Chapter 2, by default variables are immutable. This is one of
many nudges Rust gives you to write your code in a way that takes advantage of
the safety and easy concurrency that Rust offers. However, you still have the
option to make your variables mutable. Let’s explore how and why Rust
encourages you to favor immutability and why sometimes you might want to opt
out.
-->

第2章で触れた通り、変数は標準で不変になります。これは、
Rustが提供する安全性や簡便な並行性の利点を享受する形でコードを書くための選択の1つです。
ところが、まだ変数を可変にするという選択肢も残されています。
どのように、そしてなぜRustは不変性を推奨するのか、さらには、なぜそれとは違う道を選びたくなることがあるのか見ていきましょう。

<!--
When a variable is immutable, once a value is bound to a name, you can't change
that value. To illustrate this, let’s generate a new project called *variables*
in your *projects* directory by using `cargo new --bin variables`.
-->

変数が不変であると、値が一旦名前に束縛されたら、その値を変えることができません。
これを具体的に説明するために、*projects*ディレクトリに`cargo new --bin variables`コマンドを使って、
*variables*という名前のプロジェクトを生成しましょう。

<!--
Then, in your new *variables* directory, open *src/main.rs* and replace its
code with the following code that won't compile just yet:
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
Save and run the program using `cargo run`. You should receive an error
message, as shown in this output:
-->

これを保存し、`cargo run`コマンドでプログラムを走らせてください。次の出力に示されているようなエラーメッセージを受け取るはずです:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

<!--
This example shows how the compiler helps you find errors in your programs.
Even though compiler errors can be frustrating, they only mean your program
isn’t safely doing what you want it to do yet; they do *not* mean that you’re
not a good programmer! Experienced Rustaceans still get compiler errors.
-->

この例では、コンパイラがプログラムに潜むエラーを見つけ出す手助けをしてくれることが示されています。
コンパイルエラーは、イライラすることもあるものですが、まだプログラムにしてほしいことを安全に行えていないだけということなのです。
エラーが出るからといって、あなたがいいプログラマではないという意味ではあり*ません*！
経験豊富なRustaceanでも、コンパイルエラーを出すことはあります。

<!--
The error message indicates that the cause of the error is that you `cannot
assign twice to immutable variable x`, because you tried to assign a second
value to the immutable `x` variable.
-->

このエラーは、エラーの原因が`不変変数xに2回代入できない`であると示しています。不変な`x`という変数に別の値を代入しようとしたからです。

<!--
It’s important that we get compile-time errors when we attempt to change a
value that we previously designated as immutable because this very situation
can lead to bugs. If one part of our code operates on the assumption that a
value will never change and another part of our code changes that value, it’s
possible that the first part of the code won’t do what it was designed to do.
This cause of bugs can be difficult to track down after the fact,
especially when the second piece of code changes the value only *sometimes*.
-->

以前に不変と指定された値を変えようとした時に、コンパイルエラーが出るのは重要なことです。
なぜなら、この状況はまさしく、バグに繋がるからです。コードのある部分は、
値が変わることはないという前提のもとに処理を行い、別の部分がその値を変更していたら、
最初の部分が目論見通りに動いていない可能性があるのです。このようなバグは、発生してしまってからでは原因が追いかけづらいものです。
特に第2のコード片が、値を*時々*しか変えない場合、尚更です。

<!--
In Rust the compiler guarantees that when you state that a value won’t change,
it really won’t change. That means that when you’re reading and writing code,
you don’t have to keep track of how and where a value might change. Your code
is thus easier to reason through.
-->

Rustでは、値が不変であると宣言したら、本当に変わらないことをコンパイラが担保してくれます。
つまり、コードを読み書きする際に、どこでどうやって値が変化しているかを追いかける必要がなくなります。
故にコードを通して正しいことを確認するのが簡単になるのです。

<!--
But mutability can be very useful. Variables are immutable only by default; as
you did in Chapter 2, you can make them mutable by adding `mut` in front of the
variable name. In addition to allowing this value to change, `mut` conveys
intent to future readers of the code by indicating that other parts of the code
will be changing this variable value.
-->

しかし、可変性は時として非常に有益なこともあります。変数は、標準でのみ、不変です。つまり、
第2章のように変数名の前に`mut`キーワードを付けることで、可変にできるわけです。この値が変化できるようにするとともに、
`mut`により、未来の読者に対してコードの別の部分がこの変数の値を変える可能性を示すことで、その意図を汲ませることができるのです。

<!--
For example, change *src/main.rs* to the following:
-->

例として、*src/main.rs*ファイルを以下のように書き換えてください:

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
We’re allowed to change the value that `x` binds to from `5` to `6` when `mut`
is used. In some cases, you’ll want to make a variable mutable because it makes
the code more convenient to write than if it had only immutable variables.
-->

`mut`キーワードが使われると、`x`が束縛している値を`5`から`6`に変更できます。
変数を可変にする方が、不変変数だけがあるよりも書きやすくなるので、変数を可変にしたくなることもあるでしょう。

<!--
There are multiple trade-offs to consider, in addition to the prevention of
bugs. For example, in cases where you’re using large data structures, mutating
an instance in place may be faster than copying and returning newly allocated
instances. With smaller data structures, creating new instances and writing in
a more functional programming style may be easier to think through, so lower
performance might be a worthwhile penalty for gaining that clarity.
-->

考えるべきトレードオフはバグの予防以外にも、いくつかあります。例えば、大きなデータ構造を使う場合などです。
インスタンスを可変にして変更できるようにする方が、いちいちインスタンスをコピーして新しくメモリ割り当てされたインスタンスを返すよりも速くなります。
小規模なデータ構造なら、新規インスタンスを生成して、もっと関数型っぽいコードを書く方が通して考えやすくなるため、
低パフォーマンスは、その簡潔性を得るのに足りうるペナルティになるかもしれません。

<!--
### Differences Between Variables and Constants
-->

### 変数と定数(constants)の違い

<!--
Being unable to change the value of a variable might have reminded you of
another programming concept that most other languages have: *constants*. Like
immutable variables, constants are values that are bound to a name and are not
allowed to change, but there are a few differences between constants and
variables.
-->

変数の値を変更できないようにするといえば、他の多くの言語も持っている別のプログラミング概念を思い浮かべるかもしれません:
*定数*です。不変変数のように、定数は名前に束縛され、変更することが叶わない値のことですが、
定数と変数の間にはいくつかの違いがあります。

<!--
First, we aren’t allowed to use `mut` with constants. Constants aren't just
immutable by default-they're always immutable.
-->

まず、定数には`mut`キーワードは使えません: 定数は標準で不変であるだけでなく、常に不変なのです。

<!--
You declare constants using the `const` keyword instead of the `let` keyword,
and the type of the value *must* be annotated. We're about to cover types and
type annotations in the next section, “Data Types,” so don't worry about the
details right now. Just know that we must always annotate the type.
-->

定数は`let`キーワードの代わりに、`const`キーワードで宣言し、値の型は*必ず*注釈しなければなりません。
型と型注釈については次のセクション、「データ型」で講義しますので、その詳細について気にする必要はありません。
ただ単に型は常に注釈しなければならないのだと思っていてください。

<!--
Constants can be declared in any scope, including the global scope, which makes
them useful for values that many parts of code need to know about.
-->

定数はどんなスコープでも定義できます。グローバルスコープも含めてです。なので、
いろんなところで使用される可能性のある値を定義するのに役に立ちます。

<!--
The last difference is that constants may be set only to a constant expression,
not the result of a function call or any other value that could only be
computed at runtime.
-->

最後の違いは、定数は定数式にしかセットできないことです。関数呼び出し結果や、実行時に評価される値にはセットできません。

<!--
Here's an example of a constant declaration where the constant's name is
`MAX_POINTS` and its value is set to 100,000. (Rust naming convention for
constants is to use all upper case with underscores between words):
-->

定数の名前が`MAX_POINTS`で、値が100,000にセットされた定数定義の例をご覧ください。(Rustの定数の命名規則は、
全て大文字でアンダースコアで単語区切りすることです):

```rust
const MAX_POINTS: u32 = 100_000;
```

<!--
Constants are valid for the entire time a program runs, within the scope they
were declared in, making them a useful choice for values in your application
domain that multiple parts of the program might need to know about, such as the
maximum number of points any player of a game is allowed to earn or the speed
of light.
-->

定数は、プログラムが走る期間、定義されたスコープ内でずっと有効です。従って、
プログラムのいろんなところで使用される可能性のあるアプリケーション空間の値を定義するのに有益な選択肢になります。
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
As you saw in the guessing game tutorial in the “Comparing the Guess to the
Secret Number” section in Chapter 2, you can declare a new variable with the
same name as a previous variable, and the new variable shadows the previous
variable. Rustaceans say that the first variable is *shadowed* by the second,
which means that the second variable’s value is what appears when the variable
is used. We can shadow a variable by using the same variable’s name and
repeating the use of the `let` keyword as follows:
-->

第2章の数当てゲームのチュートリアル、「予想と秘密の数字を比較する」節で見たように、前に定義した変数と同じ名前の変数を新しく宣言でき、
新しい変数は、前の変数を覆い隠します。Rustaceanはこれを最初の変数は、
2番目の変数に*覆い隠さ*れたと言い、この変数を使用した際に、2番目の変数の値が現れるということです。
以下のようにして、同じ変数名を用いて変数を覆い隠し、`let`キーワードの使用を繰り返します:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

<!--
This program first binds `x` to a value of `5`. Then it shadows `x` by
repeating `let x =`, taking the original value and adding `1` so the value of
`x` is then `6`. Then, within an inner scope, the third `let` statement also
shadows `x`, multiplying the previous value by `2` to give `x` a value of `12`.
When that scope is over, the inner shadowing ends and `x` returns to being `6`.
When we run this program, it will output the following:
-->

このプログラムはまず、`x`を`5`という値に束縛します。それから`let x =`を繰り返すことで`x`を覆い隠し、
元の値に`1`を加えることになるので、`x`の値は`6`になります。
3番目の`let`文も`x`を覆い隠し、以前の値に`2`をかけることになるので、`x`の最終的な値は`12`になります。
括弧を抜けるとシャドーイングは終了し、`x`の値は元の`6`に戻ります。
このプログラムを走らせたら、以下のように出力するでしょう:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

<!--
Shadowing is different than marking a variable as `mut`, because we’ll get a
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
inputting space characters, but we really want to store that input as a number:
-->

`mut`と上書きのもう一つの違いは、再度`let`キーワードを使用したら、実効的には新しい変数を生成していることになるので、
値の型を変えつつ、同じ変数名を使いまわせることです。例えば、
プログラムがユーザに何らかのテキストに対して空白文字を入力することで何個分のスペースを表示したいかを尋ねますが、
ただ、実際にはこの入力を数値として保持したいとしましょう:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

<!--
This construct is allowed because the first `spaces` variable is a string type
and the second `spaces` variable, which is a brand-new variable that happens to
have the same name as the first one, is a number type. Shadowing thus spares us
from having to come up with different names, like `spaces_str` and
`spaces_num`; instead, we can reuse the simpler `spaces` name. However, if we
try to use `mut` for this, as shown here, we'll get a compile-time error:
-->

この文法要素は、容認されます。というのも、最初の`spaces`変数は文字列型であり、2番目の`spaces`変数は、
たまたま最初の変数と同じ名前になったまっさらな変数のわけですが、数値型になるからです。故に、シャドーイングのおかげで、
異なる名前を思いつく必要がなくなるわけです。`spaces_str`と`spaces_num`などですね; 代わりに、
よりシンプルな`spaces`という名前を再利用できるわけです。一方で、この場合に`mut`を使おうとすると、
以下に示した通りですが、コンパイルエラーになるわけです:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

<!--
The error says we’re not allowed to mutate a variable’s
type:
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
