<!--
## An Example Program Using Structs
-->

## 構造体を使ったプログラム例

<!--
To understand when we might want to use structs, let’s write a program that
calculates the area of a rectangle. We’ll start by using single variables, and
then refactor the program until we’re using structs instead.
-->

構造体を使用したくなる可能性のあるケースを理解するために、長方形の面積を求めるプログラムを書きましょう。
単一の変数から始め、代わりに構造体を使うようにプログラムをリファクタリングします。

<!--
Let’s make a new binary project with Cargo called *rectangles* that will take
the width and height of a rectangle specified in pixels and calculate the area
of the rectangle. Listing 5-8 shows a short program with one way of doing
exactly that in our project’s *src/main.rs*.
-->

Cargoで*rectangles*という新規バイナリプロジェクトを作成しましょう。このプロジェクトは、
長方形の幅と高さをピクセルで指定し、その面積を求めます。リスト5-8に、プロジェクトの*src/main.rs*で、
正にそうする一例を短いプログラムとして示しました。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

<!--
<span class="caption">Listing 5-8: Calculating the area of a rectangle
specified by separate width and height variables</span>
-->

<span class="caption">リスト5-8: 個別の幅と高さ変数を指定して長方形の面積を求める</span>

<!--
Now, run this program using `cargo run`:
-->

では、`cargo run`でこのプログラムを走らせてください:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

<!--
This code succeeds in figuring out the area of the rectangle by calling the
`area` function with each dimension, but we can do more to make this code clear
and readable.
-->

このコードは、各寸法を与えて`area`関数を呼び出すことで長方形の面積を割り出すことができますが、
このコードはもっと簡潔で読みやすくすることができます。

<!--
The issue with this code is evident in the signature of `area`:
-->

このコードの問題点は、`area`のシグニチャから明らかです:

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

<!--
The `area` function is supposed to calculate the area of one rectangle, but the
function we wrote has two parameters, and it’s not clear anywhere in our
program that the parameters are related. It would be more readable and more
manageable to group width and height together. We’ve already discussed one way
we might do that in [“The Tuple Type”][the-tuple-type] section
of Chapter 3: by using tuples.
-->

`area`関数は、1長方形の面積を求めるものと考えられますが、今書いた関数には引数が2つあり、
そしてこのプログラム内のどこを見ても、これらの引数に関連性があることが明確になっていません。
幅と高さを一緒にグループ化する方が、より読みやすく、扱いやすくなるでしょう。
それをする一つの方法については、第3章の[「タプル型」][the-tuple-type]節ですでに議論しました: タプルを使うのです。

<!--
### Refactoring with Tuples
-->

### タプルでリファクタリングする

<!--
Listing 5-9 shows another version of our program that uses tuples.
-->

リスト5-9は、タプルを使う別バージョンのプログラムを示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

<!--
<span class="caption">Listing 5-9: Specifying the width and height of the
rectangle with a tuple</span>
-->

<span class="caption">リスト5-9: タプルで長方形の幅と高さを指定する</span>

<!--
In one way, this program is better. Tuples let us add a bit of structure, and
we’re now passing just one argument. But in another way, this version is less
clear: tuples don’t name their elements, so we have to index into the parts of
the tuple, making our calculation less obvious.
-->

ある意味では、このプログラムはマシです。タプルのおかげで少し構造的になり、一引数を渡すだけになりました。
しかし別の意味では、このバージョンは明確性を失っています: タプルは要素に名前を付けないので、
タプルの要素に添え字でアクセスする必要があり、計算が不明瞭になったのです。

<!--
Mixing up the width and height wouldn’t matter for the area calculation, but if
we want to draw the rectangle on the screen, it would matter! We would have to
keep in mind that `width` is the tuple index `0` and `height` is the tuple
index `1`. This would be even harder for someone else to figure out and keep in
mind if they were to use our code. Because we haven’t conveyed the meaning of
our data in our code, it’s now easier to introduce errors.
-->

面積計算では幅と高さを混同しても問題ないですが、長方形を画面に描画したいとなると、これは問題になります！
タプルの添え字`0`が`幅`で、添え字`1`が`高さ`であることを肝に銘じておかなければなりません。

他人がこのコードをいじることになったら、このことを割り出し、同様に肝に銘じなければならないでしょう。
容易く、このことを忘れたり、これらの値を混ぜこぜにしたりして間違いを発生させてしまうでしょう。
データの意味をコードに載せていないからです。

<!--
### Refactoring with Structs: Adding More Meaning
-->

### 構造体でリファクタリングする: より意味付けする

<!--
We use structs to add meaning by labeling the data. We can transform the tuple
we’re using into a struct with a name for the whole as well as names for the
parts, as shown in Listing 5-10.
-->

データのラベル付けで意味を付与するために構造体を使います。現在使用しているタプルを全体と一部に名前のある構造体に、
変形することができます。そう、リスト5-10に示したように。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

<!--
<span class="caption">Listing 5-10: Defining a `Rectangle` struct</span>
-->

<span class="caption">リスト5-10: `Rectangle`構造体を定義する</span>

<!--
Here we’ve defined a struct and named it `Rectangle`. Inside the curly
brackets, we defined the fields as `width` and `height`, both of which have
type `u32`. Then, in `main`, we created a particular instance of `Rectangle`
that has a width of `30` and a height of `50`.
-->

ここでは、構造体を定義し、`Rectangle`という名前にしています。波括弧の中で`width`と`height`というフィールドを定義し、
`u32`という型にしました。それから`main`内で`Rectangle`の特定のインスタンスを生成し、
幅を`30`、高さを`50`にしました。

<!--
Our `area` function is now defined with one parameter, which we’ve named
`rectangle`, whose type is an immutable borrow of a struct `Rectangle`
instance. As mentioned in Chapter 4, we want to borrow the struct rather than
take ownership of it. This way, `main` retains its ownership and can continue
using `rect1`, which is the reason we use the `&` in the function signature and
where we call the function.
-->

これで`area`関数は引数が一つになり、この引数は名前が`rectangle`、型は`Rectangle`構造体インスタンスへの不変借用になりました。
第4章で触れたように、構造体の所有権を奪うよりも借用する必要があります。こうすることで`main`は所有権を保って、
`rect1`を使用し続けることができ、そのために関数シグニチャと関数呼び出し時に`&`を使っているわけです。

<!--
The `area` function accesses the `width` and `height` fields of the `Rectangle`
instance (note that accessing fields of a borrowed struct instance does not
move the field values, which is why you often see borrows of structs). Our
function signature for `area` now says exactly what we mean: calculate the area
of `Rectangle`, using its `width` and `height` fields. This conveys that the
width and height are related to each other, and it gives descriptive names to
the values rather than using the tuple index values of `0` and `1`. This is a
win for clarity.
-->

`area`関数は、`Rectangle`インスタンスの`width`と`height`フィールドにアクセスしています。
（借用された構造体インスタンスのフィールドにアクセスしても、そのフィールドの値はムーブされないことに注意してください。
構造体の借用をよく使うのはこのためです）
これで、`area`の関数シグニチャは、我々の意図をズバリ示すようになりました: `width`と`height`フィールドを使って、
`Rectangle`の面積を計算します。これにより、幅と高さが相互に関係していることが伝わり、
タプルの`0`や`1`という添え字を使うよりも、これらの値に説明的な名前を与えられるのです。プログラムの意図が明瞭になりました。

<!--
### Adding Useful Functionality with Derived Traits
-->

### トレイトの導出で有用な機能を追加する

<!--
It’d be useful to be able to print an instance of `Rectangle` while we’re
debugging our program and see the values for all its fields. Listing 5-11 tries
using the [`println!` macro][println] as we have used in
previous chapters. This won’t work, however.
-->

プログラムのデバッグをしている間に、`Rectangle`のインスタンスを出力し、フィールドの値を確認できると便利でしょう。
リスト5-11では、以前の章のように、[`println!`マクロ][println]を試しに使用しようとしています。
ですが、これは動きません。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

<!--
<span class="caption">Listing 5-11: Attempting to print a `Rectangle`
instance</span>
-->

<span class="caption">リスト5-11: `Rectangle`のインスタンスを出力しようとする</span>

<!--
When we compile this code, we get an error with this core message:
-->

このコードをコンパイルすると、こんな感じのエラーが出ます:

<!-- output.txtに日本語訳の行があるので行番号が原文からずれています -->

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3:4}}
```

<!--
The `println!` macro can do many kinds of formatting, and by default, the curly
brackets tell `println!` to use formatting known as `Display`: output intended
for direct end user consumption. The primitive types we’ve seen so far
implement `Display` by default because there’s only one way you’d want to show
a `1` or any other primitive type to a user. But with structs, the way
`println!` should format the output is less clear because there are more
display possibilities: Do you want commas or not? Do you want to print the
curly brackets? Should all the fields be shown? Due to this ambiguity, Rust
doesn’t try to guess what we want, and structs don’t have a provided
implementation of `Display` to use with `println!` and the `{}` placeholder.
-->

`println!`マクロには、様々な整形があり、標準では、波括弧は`Display`として知られる整形をするよう、
`println!`に指示するのです: 直接エンドユーザ向けの出力です。これまでに見てきた基本型は、
標準で`Display`を実装しています。というのも、`1`や他の基本型をユーザに見せる方法は一つしかないからです。
しかし構造体では、`println!`が出力を整形する方法は自明ではなくなります。出力方法がいくつもあるからです:
カンマは必要なの？波かっこを出力する必要はある？全フィールドが見えるべき？この曖昧性のため、
Rustは必要なものを推測しようとせず、構造体は`println!`と`{}`プレースホルダで使用される`Display`実装を提供しないのです。

<!--
If we continue reading the errors, we’ll find this helpful note:
-->

エラーを読み下すと、こんな有益な注意書きがあります:

<!-- output.txtに日本語訳の行があるので行番号が原文からずれています -->

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:10:13}}
```

<!--
Let’s try it! The `println!` macro call will now look like `println!("rect1 is
{:?}", rect1);`. Putting the specifier `:?` inside the curly brackets tells
`println!` we want to use an output format called `Debug`. The `Debug` trait
enables us to print our struct in a way that is useful for developers so we can
see its value while we’re debugging our code.
-->

試してみましょう！`pritnln!`マクロ呼び出しは、`println!("rect1 is {:?}", rect1);`という見た目になるでしょう。
波括弧内に`:?`という指定子を書くと、`println!`に`Debug`と呼ばれる出力整形を使いたいと指示するのです。
`Debug`トレイトは、開発者にとって有用な方法で構造体を出力させてくれるので、
コードをデバッグしている最中に、値を確認することができます。

<!--
Compile the code with this change. Drat! We still get an error:
-->

変更してコードをコンパイルしてください。なに！まだエラーが出ます:

<!-- output.txtに日本語訳の行があるので行番号が原文からずれています -->

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3:4}}
```

<!--
But again, the compiler gives us a helpful note:
-->

しかし今回も、コンパイラは有益な注意書きを残してくれています:

<!-- output.txtに日本語訳の行があるので行番号が原文からずれています -->

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:10:13}}
```

<!--
Rust *does* include functionality to print out debugging information, but we
have to explicitly opt in to make that functionality available for our struct.
To do that, we add the outer attribute `#[derive(Debug)]` just before the
struct definition, as shown in Listing 5-12.
-->

*確かに*Rustにはデバッグ用の情報を出力する機能が備わっていますが、この機能を構造体で使えるようにするには、
明示的な選択をしなければならないのです。そうするには、構造体定義の直前に`#[derive(Debug)]`という外部属性を追加します。
そう、リスト5-12で示されている通りです。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

<!--
<span class="caption">Listing 5-12: Adding the attribute to derive the `Debug`
trait and printing the `Rectangle` instance using debug formatting</span>
-->

<span class="caption">リスト5-12: `Debug`トレイトを導出する属性を追加し、
    `Rectangle`インスタンスをデバッグ用整形機で出力する</span>

<!--
Now when we run the program, we won’t get any errors, and we’ll see the
following output:
-->

これでプログラムを実行すれば、エラーは出ず、以下のような出力が得られるでしょう:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

<!--
Nice! It’s not the prettiest output, but it shows the values of all the fields
for this instance, which would definitely help during debugging. When we have
larger structs, it’s useful to have output that’s a bit easier to read; in
those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string. In
this example, using the `{:#?}` style will output the following:
-->

素晴らしい！最善の出力ではないものの、このインスタンスの全フィールドの値を出力しているので、
デバッグ中には間違いなく役に立つでしょう。より大きな構造体があるなら、もう少し読みやすい出力の方が有用です;
そのような場合には、`println!`文字列中の`{:?}`の代わりに`{:#?}`を使うことができます。
この例で`{:#?}`というスタイルを使用したら、出力は以下のようになるでしょう:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

<!--
Another way to print out a value using the `Debug` format is to use the [`dbg!`
macro][dbg], which takes ownership of an expression (as opposed
to `println!`, which takes a reference), prints the file and line number of
where that `dbg!` macro call occurs in your code along with the resultant value
of that expression, and returns ownership of the value.
-->

`Debug`整形を使用して値を出力するためのもう一つの方法は、[`dbg!`マクロ][dbg]を使用することです。
`dbg!`マクロは式の所有権を奪い（参照を取る`println!`とは対照的です）、その呼び出しが発生したコード内のファイル名と行番号とともに式を評価した結果を出力して、
その値の所有権を返します。

<!--
> Note: Calling the `dbg!` macro prints to the standard error console stream
> (`stderr`), as opposed to `println!`, which prints to the standard output
> console stream (`stdout`). We’ll talk more about `stderr` and `stdout` in the
> [“Writing Error Messages to Standard Error Instead of Standard Output”
> section in Chapter 12][err].
-->

> 注釈: 標準出力コンソールストリーム（`stdout`）に出力する`println!`とは異なり、`dbg!`マクロの呼び出しは標準エラーコンソールストリーム（`stderr`）に出力します。
> `stderr`と`stdout`については[12章の「標準出力ではなく標準エラーにエラーメッセージを書き込む」節][err]でより詳しく触れます。

<!--
Here’s an example where we’re interested in the value that gets assigned to the
`width` field, as well as the value of the whole struct in `rect1`:
-->

以下は、`width`フィールドに代入される値と、`rect1`の構造体全体の値に関心がある場合の例です:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

<!--
We can put `dbg!` around the expression `30 * scale` and, because `dbg!`
returns ownership of the expression’s value, the `width` field will get the
same value as if we didn’t have the `dbg!` call there. We don’t want `dbg!` to
take ownership of `rect1`, so we use a reference to `rect1` in the next call.
Here’s what the output of this example looks like:
-->

`dbg!`は式が評価された値の所有権を返すため、式`30 * scale`を囲うように`dbg!`を書くことで、`width`フィールドはここで`dbg!`の呼び出しをしなかった場合とまったく同じ値になります。
`rect1`の所有権は奪ってほしくないので、次の`dbg!`呼び出しでは`rect1`への参照を使用しています。
この例の出力は以下のようになります:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

<!--
We can see the first bit of output came from *src/main.rs* line 10 where we’re
debugging the expression `30 * scale`, and its resultant value is `60` (the
`Debug` formatting implemented for integers is to print only their value). The
`dbg!` call on line 14 of *src/main.rs* outputs the value of `&rect1`, which is
the `Rectangle` struct. This output uses the pretty `Debug` formatting of the
`Rectangle` type. The `dbg!` macro can be really helpful when you’re trying to
figure out what your code is doing!
-->

出力の前半は*src/main.rs*の10行目からの出力です。
ここでは式`30 * scale`をデバッグ出力していて、その結果の値は`60`です（整数に実装されている`Debug`整形を使用して出力されています）。
*src/main.rs*の14行目の`dbg!`呼び出しは`&rect1`の値を出力し、これは`Rectangle`構造体です。
この出力は `Rectangle`型の pretty `Debug`整形を使用します。
`dbg!`マクロは、コードが何をしているのか理解しようとするときには非常に有用です！

<!--
In addition to the `Debug` trait, Rust has provided a number of traits for us
to use with the `derive` attribute that can add useful behavior to our custom
types. Those traits and their behaviors are listed in [Appendix C][app-c]
. We’ll cover how to implement these traits with custom behavior as
well as how to create your own traits in Chapter 10. There are also many
attributes other than `derive`; for more information, see [the “Attributes”
section of the Rust Reference][attributes].
-->

`Debug`トレイトの他にも、Rustでは`derive`属性で使えるトレイトが多く提供されており、独自の型に有用な振る舞いを追加することができます。
そのようなトレイトとその振る舞いは、付録Cで一覧になっています。
これらのトレイトを独自の動作とともに実装する方法だけでなく、独自のトレイトを生成する方法については、第10章で解説します。
また、`derive`の他にも多数の属性が存在します; さらなる情報については[Rust Referenceの“Attributes”節][attributes]を参照してください。

<!--
Our `area` function is very specific: it only computes the area of rectangles.
It would be helpful to tie this behavior more closely to our `Rectangle` struct
because it won’t work with any other type. Let’s look at how we can continue to
refactor this code by turning the `area` function into an `area` *method*
defined on our `Rectangle` type.
-->

`area`関数は、非常に特殊です: 長方形の面積を算出するだけです。`Rectangle`構造体とこの動作をより緊密に結び付けられると、
役に立つでしょう。なぜなら、他のどんな型でもうまく動作しなくなるからです。
`area`関数を`Rectangle`型に定義された`area`*メソッド*に変形することで、
このコードをリファクタリングし続けられる方法について見ていきましょう。

<!--
[the-tuple-type]: ch03-02-data-types.html#the-tuple-type
[app-c]: appendix-03-derivable-traits.md
[println]: ../std/macro.println.html
[dbg]: ../std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: ../reference/attributes.html
-->

[the-tuple-type]: ch03-02-data-types.html#タプル型
[app-c]: appendix-03-derivable-traits.md
[println]: https://doc.rust-lang.org/std/macro.println.html
[dbg]: https://doc.rust-lang.org/std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: https://doc.rust-lang.org/reference/attributes.html
