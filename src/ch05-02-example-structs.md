<!--
## An Example Program Using Structs
-->

## 構造体を使ったプログラム例

<!--
To understand when we might want to use structs, let’s write a program that
calculates the area of a rectangle. We’ll start with single variables, and then
refactor the program until we’re using structs instead.
-->

構造体を使用したくなる可能性のあるケースを理解するために、長方形の面積を求めるプログラムを書きましょう。
単一の変数から始め、代わりに構造体を使うようにプログラムをリファクタリングします。

<!--
Let’s make a new binary project with Cargo called *rectangles* that will take
the width and height of a rectangle specified in pixels and will calculate the area
of the rectangle. Listing 5-8 shows a short program with one way of doing
exactly that in our project’s *src/main.rs*.
-->

Cargo で*rectangles*という新規バイナリプロジェクトを作成しましょう。このプロジェクトは、
長方形の幅と高さをピクセルで指定し、その面積を求めます。リスト 5-8 に、プロジェクトの*src/main.rs*で、
正にそうする一例を短いプログラムとして示しました。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

<!--
<span class="caption">Listing 5-8: Calculating the area of a rectangle
specified by separate width and height variables</span>
-->

<span class="caption">リスト 5-8: 個別の幅と高さ変数を指定して長方形の面積を求める</span>

<!--
Now, run this program using `cargo run`:
-->

では、`cargo run`でこのプログラムを走らせてください：

```text
The area of the rectangle is 1500 square pixels.
(長方形の面積は、1500 平方ピクセルです)
```

<!--
### Refactoring with Tuples
-->

### タプルでリファクタリングする

<!--
Even though Listing 5-8 works and figures out the area of the rectangle by
calling the `area` function with each dimension, we can do better. The width
and the height are related to each other because together they describe one
rectangle.
-->

リスト 5-8 のコードはうまく動き、各寸法を与えて`area`関数を呼び出すことで長方形の面積を割り出しますが、
改善点があります。幅と高さは、組み合わせると一つの長方形を表すので、相互に関係があるわけです。

<!--
The issue with this code is evident in the signature of `area`:
-->

このコードの問題点は、`area`のシグニチャから明らかです：

```rust,ignore
fn area(width: u32, height: u32) -> u32 {
```

<!--
The `area` function is supposed to calculate the area of one rectangle, but the
function we wrote has two parameters. The parameters are related, but that’s
not expressed anywhere in our program. It would be more readable and more
manageable to group width and height together. We’ve already discussed one way
we might do that in the "The Tuple Type" section of Chapter 3: by using tuples.
-->

`area`関数は、1 長方形の面積を求めるものと考えられますが、今書いた関数には、引数が 2 つあります。
引数は関連性があるのに、このプログラム内のどこにもそのことは表現されていません。
幅と高さを一緒にグループ化する方が、より読みやすく、扱いやすくなるでしょう。
それをする一つの方法については、第 3 章の「タプル型」節ですでに議論しました：タプルを使うのです。

<!--
### Refactoring with Tuples
-->

### タプルでリファクタリングする

<!--
Listing 5-9 shows another version of our program that uses tuples.
-->

リスト 5-9 は、タプルを使う別バージョンのプログラムを示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

<!--
<span class="caption">Listing 5-9: Specifying the width and height of the
rectangle with a tuple</span>
-->

<span class="caption">リスト 5-9: タプルで長方形の幅と高さを指定する</span>

<!--
In one way, this program is better. Tuples let us add a bit of structure, and
we’re now passing just one argument. But in another way, this version is less
clear: tuples don’t name their elements, so our calculation has become more
confusing because we have to index into the parts of the tuple.
-->

ある意味では、このプログラムはマシです。タプルのおかげで少し構造的になり、一引数を渡すだけになりました。
しかし別の意味では、このバージョンは明確性を失っています：タプルは要素に名前を付けないので、
計算が不明瞭になったのです。なぜなら、タプルの一部に添え字アクセスする必要があるからです。

<!--
It doesn’t matter if we mix up width and height for the area calculation, but
if we want to draw the rectangle on the screen, it would matter! We would have
to keep in mind that `width` is the tuple index `0` and `height` is the tuple
index `1`. If someone else worked on this code, they would have to figure this
out and keep it in mind as well. It would be easy to forget or mix up these
values and cause errors, because we haven’t conveyed the meaning of our data in
our code.
-->

面積計算で幅と高さを混在させるのなら問題はないのですが、長方形を画面に描画したいとなると、問題になるのです！
タプルの添え字`0`が`幅`で、添え字`1`が`高さ`であることを肝に銘じておかなければなりません。
他人がこのコードをいじることになったら、このことを割り出し、同様に肝に銘じなければならないでしょう。
容易く、このことを忘れたり、これらの値を混ぜこぜにしたりしてエラーを発生させてしまうでしょう。
データの意味をコードに載せていないからです。

<!--
### Refactoring with Structs: Adding More Meaning
-->

### 構造体でリファクタリングする：より意味付けする

<!--
We use structs to add meaning by labeling the data. We can transform the tuple
we’re using into a data type with a name for the whole as well as names for the
parts, as shown in Listing 5-10.
-->

データのラベル付けで意味を付与するために構造体を使います。現在使用しているタプルを全体と一部に名前のあるデータ型に、
変形することができます。そう、リスト 5-10 に示したように。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

<!--
<span class="caption">Listing 5-10: Defining a `Rectangle` struct</span>
-->

<span class="caption">リスト 5-10: `Rectangle`構造体を定義する</span>

<!--
Here we’ve defined a struct and named it `Rectangle`. Inside the curly
brackets, we defined the fields as `width` and `height`, both of which have
type `u32`. Then in `main`, we create a particular instance of a `Rectangle`
that has a width of 30 and a height of 50.
-->

ここでは、構造体を定義し、`Rectangle`という名前にしています。波括弧の中で`width`と`height`というフィールドを定義し、
`u32`という型にしました。それから`main`内で`Rectangle`の特定のインスタンスを生成し、
幅を 30、高さを 50 にしました。

<!--
Our `area` function is now defined with one parameter, which we’ve named
`rectangle`, whose type is an immutable borrow of a struct `Rectangle`
instance. As mentioned in Chapter 4, we want to borrow the struct rather than
take ownership of it. This way, `main` retains its ownership and can continue
using `rect1`, which is the reason we use the `&` in the function signature and
where we call the function.
-->

これで`area`関数は引数が一つになり、この引数は名前が`rectangle`、型は`Rectangle`構造体インスタンスへの不変借用になりました。
第 4 章で触れたように、構造体の所有権を奪うよりも借用する必要があります。こうすることで`main`は所有権を保って、
`rect1`を使用し続けることができ、そのために関数シグニチャと関数呼び出し時に`&`を使っているわけです。

<!--
The `area` function accesses the `width` and `height` fields of the `Rectangle`
instance. Our function signature for `area` now indicates exactly what we mean:
calculate the area of a `Rectangle`, using its `width` and `height` fields. This
conveys that the width and height are related to each other, and it gives
descriptive names to the values rather than using the tuple index values of `0`
and `1`. This is a win for clarity.
-->

`area`関数は、`Rectangle`インスタンスの`width`と`height`フィールドにアクセスしています。
これで、`area`の関数シグニチャは、我々の意図をズバリ示すようになりました：`width`と`height`フィールドを使って、
`Rectangle`の面積を計算します。これにより、幅と高さが相互に関係していることが伝わり、
タプルの`0`や`1`という添え字を使うよりも、これらの値に説明的な名前を与えられるのです。プログラムの意図が明瞭になりました。

<!--
### Adding Useful Functionality with Derived Traits
-->

### トレイトの導出で有用な機能を追加する

<!--
It’d be nice to be able to print an instance of `Rectangle` while we’re
debugging our program and see the values for all its fields. Listing 5-11 tries
using the `println!` macro as we have used it in previous chapters. This won't
work, however.
-->

プログラムのデバッグをしている間に、`Rectangle`のインスタンスを出力し、フィールドの値を確認できると、
素晴らしいわけです。リスト 5-11 では、以前の章のように、`println!`マクロを試しに使用しようとしていますが、動きません。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // rect1 は{}です
    println!("rect1 is {}", rect1);
}
```

<!--
<span class="caption">Listing 5-11: Attempting to print a `Rectangle`
instance</span>
-->

<span class="caption">リスト 5-11: `Rectangle`のインスタンスを出力しようとする</span>

<!--
When we run this code, we get an error with this core message:
-->

このコードを走らせると、こんな感じのエラーが出ます：

```text
error[E0277]: the trait bound `Rectangle: std::fmt::Display` is not satisfied
(エラー: トレイト境界`Rectangle: std::fmt::Display`が満たされていません)
```

<!--
The `println!` macro can do many kinds of formatting, and by default, the curly
brackets tell `println!` to use formatting known as `Display`: output intended
for direct end user consumption. The primitive types we’ve seen so far
implement `Display` by default, because there’s only one way you’d want to show
a `1` or any other primitive type to a user. But with structs, the way
`println!` should format the output is less clear because there are more
display possibilities: do you want commas or not? Do you want to print the
curly brackets? Should all the fields be shown? Due to this ambiguity, Rust
doesn't try to guess what we want, and structs don’t have a provided
implementation of `Display`.
-->

`println!`マクロには、様々な整形があり、標準では、波括弧は`Display`として知られる整形をするよう、
`println!`に指示するのです：直接エンドユーザ向けの出力です。これまでに見てきた基本型は、
標準で`Display`を実装しています。というのも、`1`や他の基本型をユーザに見せる方法は一つしかないからです。
しかし構造体では、`println!`が出力を整形する方法は自明ではなくなります。出力方法がいくつもあるからです：
カンマは必要なの？波かっこを出力する必要はある？全フィールドが見えるべき？この曖昧性のため、
Rust は必要なものを推測しようとせず、構造体には`Display`実装が提供されないのです。

<!--
If we continue reading the errors, we’ll find this helpful note:
-->

エラーを読み下すと、こんな有益な注意書きがあります：

```text
`Rectangle` cannot be formatted with the default formatter; try using
`:?` instead if you are using a format string
(注釈：`Rectangle`は、デフォルト整形機では、整形できません; フォーマット文字列を使うのなら
代わりに`:?`を試してみてください)
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
Run the code with this change. Drat! We still get an error:
-->

変更してコードを走らせてください。なに！まだエラーが出ます：

```text
error[E0277]: the trait bound `Rectangle: std::fmt::Debug` is not satisfied
(エラー: トレイト境界`Rectangle: std::fmt::Debug`が満たされていません)
```

<!--
But again, the compiler gives us a helpful note:
-->

しかし今回も、コンパイラは有益な注意書きを残してくれています：

```text
`Rectangle` cannot be formatted using `:?`; if it is defined in your
crate, add `#[derive(Debug)]` or manually implement it
(注釈：`Rectangle`は`:?`を使って整形できません; 自分のクレートで定義しているのなら
`#[derive(Debug)]`を追加するか、手動で実装してください)
```

<!--
Rust *does* include functionality to print out debugging information, but we
have to explicitly opt in to make that functionality available for our struct.
To do that, we add the annotation `#[derive(Debug)]` just before the struct
definition, as shown in Listing 5-12.
-->

*確かに*Rust にはデバッグ用の情報を出力する機能が備わっていますが、この機能を構造体で使えるようにするには、
明示的な選択をしなければならないのです。そうするには、構造体定義の直前に`#[derive(Debug)]`という注釈を追加します。
そう、リスト 5-12 で示されている通りです。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}
```

<!--
<span class="caption">Listing 5-12: Adding the annotation to derive the `Debug`
trait and printing the `Rectangle` instance using debug formatting</span>
-->

<span class="caption">リスト 5-12: `Debug`トレイトを導出する注釈を追加し、
    `Rectangle`インスタンスをデバッグ用整形機で出力する</span>

<!--
Now when we run the program, we won’t get any errors, and we’ll see the
following output:
-->

これでプログラムを実行すれば、エラーは出ず、以下のような出力が得られるでしょう：

```text
rect1 is Rectangle { width: 30, height: 50 }
```

<!--
Nice! It’s not the prettiest output, but it shows the values of all the fields
for this instance, which would definitely help during debugging. When we have
larger structs, it’s useful to have output that’s a bit easier to read; in
those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string.
When we use the `{:#?}` style in the example, the output will look like this:
-->

素晴らしい！最善の出力ではないものの、このインスタンスの全フィールドの値を出力しているので、
デバッグ中には間違いなく役に立つでしょう。より大きな構造体があるなら、もう少し読みやすい出力の方が有用です;
そのような場合には、`println!`文字列中の`{:?}`の代わりに`{:#?}`を使うことができます。
この例で`{:#?}`というスタイルを使用したら、出力は以下のようになるでしょう：

```text
rect1 is Rectangle {
    width: 30,
    height: 50
}
```

<!--
Rust has provided a number of traits for us to use with the `derive` annotation
that can add useful behavior to our custom types. Those traits and their
behaviors are listed in Appendix C. We’ll cover how to implement these traits
with custom behavior as well as how to create your own traits in Chapter 10.
-->

Rust には、`derive`注釈で使えるトレイトが多く提供されており、独自の型に有用な振る舞いを追加することができます。
そのようなトレイトとその振る舞いは、付録 C で一覧になっています。
これらのトレイトを独自の動作とともに実装する方法だけでなく、独自のトレイトを生成する方法については、第 10 章で解説します。

<!--
Our `area` function is very specific: it only computes the area of rectangles.
It would be helpful to tie this behavior more closely to our `Rectangle`
struct, because it won’t work with any other type. Let’s look at how we can
continue to refactor this code by turning the `area` function into an `area`
*method* defined on our `Rectangle` type.
-->

`area`関数は、非常に特殊です：長方形の面積を算出するだけです。`Rectangle`構造体とこの動作をより緊密に結び付けられると、
役に立つでしょう。なぜなら、他のどんな型でもうまく動作しなくなるからです。
`area`関数を`Rectangle`型に定義された`area`*メソッド*に変形することで、
このコードをリファクタリングし続けられる方法について見ていきましょう。
