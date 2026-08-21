<!--
## Using Trait Objects That Allow for Values of Different Types
-->

## トレイトオブジェクトで異なる型の値を許容する

<!--
In Chapter 8, we mentioned that one limitation of vectors is that they can
store elements of only one type. We created a workaround in Listing 8-9 where
we defined a `SpreadsheetCell` enum that had variants to hold integers, floats,
and text. This meant we could store different types of data in each cell and
still have a vector that represented a row of cells. This is a perfectly good
solution when our interchangeable items are a fixed set of types that we know
when our code is compiled.
-->

第8章で、ベクタの1つの制限は、たった1つの型の要素を保持することしかできないことだと述べました。
リスト8-9で整数、浮動小数点数、テキストを保持する列挙子のある`SpreadsheetCell` enumを定義して、
これを回避しました。つまり、各セルに異なる型のデータを格納しつつ、1行のセルを表すベクタを保持するということです。
コンパイル時にわかるある固定されたセットの型にしか取り替え可能な要素がならない場合には、
完璧な解決策です。

<!--
However, sometimes we want our library user to be able to extend the set of
types that are valid in a particular situation. To show how we might achieve
this, we’ll create an example graphical user interface (GUI) tool that iterates
through a list of items, calling a `draw` method on each one to draw it to the
screen—a common technique for GUI tools. We’ll create a library crate called
`gui` that contains the structure of a GUI library. This crate might include
some types for people to use, such as `Button` or `TextField`. In addition,
`gui` users will want to create their own types that can be drawn: for
instance, one programmer might add an `Image` and another might add a
`SelectBox`.
-->

ところが、時として、ライブラリの使用者が特定の場面で合法になる型のセットを拡張できるようにしたくなることがあります。
これをどう実現する可能性があるか示すために、各アイテムに`draw`メソッドを呼び出してスクリーンに描画するという、
GUIツールで一般的なテクニックをしてあるリストの要素を走査する例のGUIツールを作ります。
GUIライブラリの構造を含む`gui`と呼ばれるライブラリクレートを作成します。
このクレートには、他人が使用できる`Button`や`TextField`などの型が包含されるかもしれません。
さらに、`gui`の使用者は、描画可能な独自の型を作成したくなるでしょう: 例えば、
ある人は`Image`を追加し、別の人は`SelectBox`を追加するかもしれません。

<!--
We won’t implement a fully fledged GUI library for this example but will show
how the pieces would fit together. At the time of writing the library, we can’t
know and define all the types other programmers might want to create. But we do
know that `gui` needs to keep track of many values of different types, and it
needs to call a `draw` method on each of these differently typed values. It
doesn’t need to know exactly what will happen when we call the `draw` method,
just that the value will have that method available for us to call.
-->

この例のために本格的なGUIライブラリは実装するつもりはありませんが、部品がどう組み合わさるかは示します。
ライブラリの記述時点では、他のプログラマが作成したくなる可能性のある型全てを知る由もなければ、定義することもできません。
しかし、`gui`は異なる型の多くの値を追いかけ、この異なる型の値に対して`draw`メソッドを呼び出す必要があることは、
確かにわかっています。`draw`メソッドを呼び出した時に正確に何が起きるかを知っている必要はありません。
値にそのメソッドが呼び出せるようあることだけわかっていればいいのです。

<!--
To do this in a language with inheritance, we might define a class named
`Component` that has a method named `draw` on it. The other classes, such as
`Button`, `Image`, and `SelectBox`, would inherit from `Component` and thus
inherit the `draw` method. They could each override the `draw` method to define
their custom behavior, but the framework could treat all of the types as if
they were `Component` instances and call `draw` on them. But because Rust
doesn’t have inheritance, we need another way to structure the `gui` library to
allow users to extend it with new types.
-->

継承のある言語でこれを行うには、`draw`という名前のメソッドがある`Component`というクラスを定義するかもしれません。
`Button`、`Image`、`SelectBox`などの他のクラスは、`Component`を継承し、故に`draw`メソッドを継承します。
個々に`draw`メソッドをオーバーライドして、独自の振る舞いを定義するものの、フレームワークは、
`Component`インスタンスであるかのようにその型全部を扱い、この型に対して`draw`を呼び出します。
ですが、Rustに継承は存在しないので、使用者に新しい型で拡張してもらうために`gui`ライブラリを構成する他の方法が必要です。

<!--
### Defining a Trait for Common Behavior
-->

### 一般的な振る舞いにトレイトを定義する

<!--
To implement the behavior we want `gui` to have, we’ll define a trait named
`Draw` that will have one method named `draw`. Then we can define a vector that
takes a *trait object*. A trait object points to both an instance of a type
implementing our specified trait and a table used to look up trait methods on
that type at runtime. We create a trait object by specifying some sort of
pointer, such as a `&` reference or a `Box<T>` smart pointer, then the `dyn`
keyword, and then specifying the relevant trait. (We’ll talk about the reason
trait objects must use a pointer in Chapter 19 in the section [“Dynamically
Sized Types and the `Sized` Trait.”][dynamically-sized]) We can
use trait objects in place of a generic or concrete type. Wherever we use a
trait object, Rust’s type system will ensure at compile time that any value
used in that context will implement the trait object’s trait. Consequently, we
don’t need to know all the possible types at compile time.
-->

`gui`に欲しい振る舞いを実装するには、`draw`という1つのメソッドを持つ`Draw`というトレイトを定義します。
それから*トレイトオブジェクト*を取るベクタを定義できます。トレイトオブジェクトは、
指定されたトレイトを実装するある型のインスタンスと、実行時にその型に対するトレイトメソッドを探すのに使用されるテーブルの、
両方を指します。`&`参照や`Box<T>`スマートポインタなどの、何らかのポインタを指定し、
それから`dyn`キーワード、そして対象のトレイトを指定する(トレイトオブジェクトがポインタを使用しなければならない理由については、
第19章の[「動的サイズ決定型と`Sized`トレイト」][dynamically-sized]節で語ります)ことでトレイトオブジェクトを作成します。
ジェネリックまたは具体的な型があるところにトレイトオブジェクトは使用できます。どこでトレイトオブジェクトを使用しようと、
Rustの型システムは、コンパイル時にその文脈で使用されているあらゆる値がそのトレイトオブジェクトのトレイトを実装していることを保証します。
結果としてコンパイル時に可能性のある型を全て知る必要はなくなるのです。

<!--
We’ve mentioned that, in Rust, we refrain from calling structs and enums
“objects” to distinguish them from other languages’ objects. In a struct or
enum, the data in the struct fields and the behavior in `impl` blocks are
separated, whereas in other languages, the data and behavior combined into one
concept is often labeled an object. However, trait objects *are* more like
objects in other languages in the sense that they combine data and behavior.
But trait objects differ from traditional objects in that we can’t add data to
a trait object. Trait objects aren’t as generally useful as objects in other
languages: their specific purpose is to allow abstraction across common
behavior.
-->

Rustでは、構造体とenumを他の言語のオブジェクトと区別するために「オブジェクト」と呼ぶことを避けていることに触れましたね。
構造体やenumにおいて、構造体のフィールドのデータや`impl`ブロックの振る舞いは区分けされているものの、
他の言語では1つの概念に押し込められるデータと振る舞いは、しばしばオブジェクトと分類されます。
しかしながら、トレイトオブジェクトは、データと振る舞いをごちゃ混ぜにするという観点で他の言語のオブジェクトに近い*です*。
しかし、トレイトオブジェクトは、データを追加できないという点で伝統的なオブジェクトと異なっています。
トレイトオブジェクトは、他の言語のオブジェクトほど一般的に有用ではありません:
その特定の目的は、共通の振る舞いに対して抽象化を行うことです。

<!--
Listing 17-3 shows how to define a trait named `Draw` with one method named
`draw`:
-->

リスト17-3は、`draw`という1つのメソッドを持つ`Draw`というトレイトを定義する方法を示しています:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-03/src/lib.rs}}
```

<!--
<span class="caption">Listing 17-3: Definition of the `Draw` trait</span>
-->

<span class="caption">リスト17-3: `Draw`トレイトの定義</span>

<!--
This syntax should look familiar from our discussions on how to define traits
in Chapter 10. Next comes some new syntax: Listing 17-4 defines a struct named
`Screen` that holds a vector named `components`. This vector is of type
`Box<dyn Draw>`, which is a trait object; it’s a stand-in for any type inside
a `Box` that implements the `Draw` trait.
-->

この記法は、第10章のトレイトの定義方法に関する議論で馴染み深いはずです。その次は、新しい記法です:
リスト17-4では、`components`というベクタを保持する`Screen`という名前の構造体を定義しています。
このベクタの型は`Box<dyn Draw>`で、これはトレイトオブジェクトです; `Draw`トレイトを実装する`Box`内部の任意の型に対する代役です。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-04/src/lib.rs:here}}

```

<!--
<span class="caption">Listing 17-4: Definition of the `Screen` struct with a
`components` field holding a vector of trait objects that implement the `Draw`
trait</span>
-->

<span class="caption">リスト17-4: `Draw`トレイトを実装するトレイトオブジェクトのベクタを保持する`components`フィールドがある
`Screen`構造体の定義</span>

<!--
On the `Screen` struct, we’ll define a method named `run` that will call the
`draw` method on each of its `components`, as shown in Listing 17-5:
-->

`Screen`構造体に、`components`の各要素に対して`draw`メソッドを呼び出す`run`というメソッドを定義します。
リスト17-5のようにですね:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-05/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 17-5: A `run` method on `Screen` that calls the
`draw` method on each component</span>
-->

<span class="caption">リスト17-5: 各コンポーネントに対して`draw`メソッドを呼び出す`Screen`の`run`メソッド</span>

<!--
This works differently from defining a struct that uses a generic type
parameter with trait bounds. A generic type parameter can only be substituted
with one concrete type at a time, whereas trait objects allow for multiple
concrete types to fill in for the trait object at runtime. For example, we
could have defined the `Screen` struct using a generic type and a trait bound
as in Listing 17-6:
-->

これは、トレイト境界を伴うジェネリックな型引数を使用する構造体を定義するのとは異なる動作をします。
ジェネリックな型引数は、一度に1つの具体型にしか置き換えられないのに対して、トレイトオブジェクトは、
実行時にトレイトオブジェクトに対して複数の具体型で埋めることができます。例として、
ジェネリックな型とトレイト境界を使用してリスト17-6のように`Screen`構造体を定義することもできました:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-06/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 17-6: An alternate implementation of the `Screen`
struct and its `run` method using generics and trait bounds</span>
-->

<span class="caption">リスト17-6: ジェネリクスとトレイト境界を使用した`Screen`構造体と`run`メソッドの対立的な実装</span>

<!--
This restricts us to a `Screen` instance that has a list of components all of
type `Button` or all of type `TextField`. If you’ll only ever have homogeneous
collections, using generics and trait bounds is preferable because the
definitions will be monomorphized at compile time to use the concrete types.
-->

こうすると、全てのコンポーネントの型が`Button`だったり、`TextField`だったりする`Screen`のインスタンスに制限されてしまいます。
絶対に同種のコレクションしか持つ予定がないのなら、ジェネリクスとトレイト境界は、
定義がコンパイル時に具体的な型を使用するように単相化されるので、望ましいです。

<!--
On the other hand, with the method using trait objects, one `Screen` instance
can hold a `Vec<T>` that contains a `Box<Button>` as well as a
`Box<TextField>`. Let’s look at how this works, and then we’ll talk about the
runtime performance implications.
-->

一方で、メソッドがトレイトオブジェクトを使用すると、1つの`Screen`インスタンスが、
`Box<Button>`と`Box<TextField>`を含む`Vec<T>`を保持できます。
この動作方法を見、それから実行時性能の裏の意味について語りましょう。

<!--
### Implementing the Trait
-->

### トレイトを実装する

<!--
Now we’ll add some types that implement the `Draw` trait. We’ll provide the
`Button` type. Again, actually implementing a GUI library is beyond the scope
of this book, so the `draw` method won’t have any useful implementation in its
body. To imagine what the implementation might look like, a `Button` struct
might have fields for `width`, `height`, and `label`, as shown in Listing 17-7:
-->

さて、`Draw`トレイトを実装する型を追加しましょう。`Button`型を提供します。ここも、実際にGUIライブラリを実装することは、
この本の範疇を超えているので、`draw`メソッドの本体は、何も有用な実装はしません。実装がどんな感じになるか想像するために、
`Button`構造体は、`width`、`height`、`label`フィールドを持っている可能性があります。
リスト17-7に示したようにですね:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-07/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 17-7: A `Button` struct that implements the
`Draw` trait</span>
-->

<span class="caption">リスト17-7: `Draw`トレイトを実装するある`Button`構造体</span>

<!--
The `width`, `height`, and `label` fields on `Button` will differ from the
fields on other components; for example, a `TextField` type might have those
same fields plus a `placeholder` field. Each of the types we want to draw on
the screen will implement the `Draw` trait but will use different code in the
`draw` method to define how to draw that particular type, as `Button` has here
(without the actual GUI code, as mentioned). The `Button` type, for instance,
might have an additional `impl` block containing methods related to what
happens when a user clicks the button. These kinds of methods won’t apply to
types like `TextField`.
-->

`Button`の`width`、`height`、`label`フィールドは、他のコンポーネントのフィールドとは異なるでしょう;
例えば`TextField`型は、それらと同じフィールドに加えて`placeholder`フィールドを持つかもしれません。
スクリーンに描画したい型のコンポーネントはそれぞれ`Draw`トレイトを実装しますが、
`Button`がここでしているように、`draw`メソッドでは異なるコードを使用してその特定の型を描画する方法を定義しています
(先にも書いた通り、実際のGUIコードはありませんが)。例えば、`Button`には、ユーザがボタンをクリックした時に起こることに関連するメソッドを含む、
追加の`impl`ブロックがある可能性があります。この種のメソッドは、`TextField`のような型には適用されません。

<!--
If someone using our library decides to implement a `SelectBox` struct that has
`width`, `height`, and `options` fields, they implement the `Draw` trait on the
`SelectBox` type as well, as shown in Listing 17-8:
-->

ライブラリの使用者が、`width`、`height`、`options`フィールドのある`SelectBox`構造体を実装しようと決めたら、
`SelectBox`型にも`Draw`トレイトを実装します。リスト17-8のようにですね:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-08/src/main.rs:here}}
```

<!--
<span class="caption">Listing 17-8: Another crate using `gui` and implementing
the `Draw` trait on a `SelectBox` struct</span>
-->

<span class="caption">リスト17-8: `gui`を使用し、`SelectBox`構造体に`Draw`トレイトを実装する別のクレート</span>

<!--
Our library’s user can now write their `main` function to create a `Screen`
instance. To the `Screen` instance, they can add a `SelectBox` and a `Button`
by putting each in a `Box<T>` to become a trait object. They can then call the
`run` method on the `Screen` instance, which will call `draw` on each of the
components. Listing 17-9 shows this implementation:
-->

ライブラリの使用者はもう、`main`関数を書き、`Screen`インスタンスを生成できます。`Screen`インスタンスには、
それぞれを`Box<T>`に放り込んでトレイトオブジェクト化して`SelectBox`と`Button`を追加できます。
それから`Screen`インスタンスに対して`run`メソッドを呼び出すことができ、そうすると各コンポーネントの`draw`が呼び出されます。
リスト17-9は、この実装を示しています:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-09/src/main.rs:here}}
```

<!--
<span class="caption">Listing 17-9: Using trait objects to store values of
different types that implement the same trait</span>
-->

<span class="caption">リスト17-9: トレイトオブジェクトを使って同じトレイトを実装する異なる型の値を格納する</span>

<!--
When we wrote the library, we didn’t know that someone might add the
`SelectBox` type, but our `Screen` implementation was able to operate on the
new type and draw it because `SelectBox` implements the `Draw` trait, which
means it implements the `draw` method.
-->

ライブラリを記述した時点では、誰かが`SelectBox`型を追加する可能性があるなんて知りませんでしたが、
`Screen`の実装は、新しい型を処理し、描画することができました。何故なら、`SelectBox`は`Draw`トレイト、
つまり、`draw`メソッドを実装しているからです。

<!--
This concept—of being concerned only with the messages a value responds to
rather than the value’s concrete type—is similar to the concept of *duck
typing* in dynamically typed languages: if it walks like a duck and quacks
like a duck, then it must be a duck! In the implementation of `run` on `Screen`
in Listing 17-5, `run` doesn’t need to know what the concrete type of each
component is. It doesn’t check whether a component is an instance of a `Button`
or a `SelectBox`, it just calls the `draw` method on the component. By
specifying `Box<dyn Draw>` as the type of the values in the `components`
vector, we’ve defined `Screen` to need values that we can call the `draw`
method on.
-->

この値の具体的な型ではなく、値が応答したメッセージにのみ関係するという概念は、
動的型付け言語の*ダックタイピング*に似た概念です: アヒルのように歩き、鳴くならば、
アヒルに違いないのです！リスト17-5の`Screen`の`run`の実装では、`run`は、
各コンポーネントの実際の型がなんであるか知る必要はありません。コンポーネントが、
`Button`や`SelectBox`のインスタンスであるかを確認することはなく、コンポーネントの`draw`メソッドを呼び出すだけです。
`components`ベクタで`Box<dyn Draw>`を値の型として指定することで、`Screen`を、
`draw`メソッドを呼び出せる値を必要とするように定義できたのです。

> #### 注釈: ダックタイピングについて
>
> ご存知かもしれませんが、ダックタイピングについて補足です。ダックタイピングとは、動的型付け言語やC++のテンプレートで使用される、
> 特定のフィールドやメソッドがあることを想定してコンパイルを行い、実行時に実際にあることを確かめるというプログラミング手法です。
> ダック・テストという思考法に由来するそうです。
>
> ダックタイピングの利点は、XMLやJSONなど、厳密なスキーマがないことが多い形式を扱いやすくなること、
> 欠点は、実行してみるまで動くかどうかわからないことでしょう。

<!--
The advantage of using trait objects and Rust’s type system to write code
similar to code using duck typing is that we never have to check whether a
value implements a particular method at runtime or worry about getting errors
if a value doesn’t implement a method but we call it anyway. Rust won’t compile
our code if the values don’t implement the traits that the trait objects need.
-->

トレイトオブジェクトとRustの型システムを使用してダックタイピングを活用したコードに似たコードを書くことの利点は、
実行時に値が特定のメソッドを実装しているか確認したり、値がメソッドを実装していない時にエラーになることを心配したりする必要は絶対になく、
とにかく呼び出せることです。コンパイラは、値が、トレイトオブジェクトが必要としているトレイトを実装していなければ、
コンパイルを通さないのです。

<!--
For example, Listing 17-10 shows what happens if we try to create a `Screen`
with a `String` as a component:
-->

例えば、リスト17-10は、コンポーネントに`String`のある`Screen`を作成しようとした時に起こることを示しています:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-10/src/main.rs}}
```

<!--
<span class="caption">Listing 17-10: Attempting to use a type that doesn’t
implement the trait object’s trait</span>
-->

<span class="caption">リスト17-10: トレイトオブジェクトのトレイトを実装しない型の使用を試みる</span>

<!--
We’ll get this error because `String` doesn’t implement the `Draw` trait:
-->

`String`は`Draw`トレイトを実装していないので、このようなエラーが出ます:

```console
{{#include ../listings/ch17-oop/listing-17-10/output.txt}}
```

<!--
This error lets us know that either we’re passing something to `Screen` we
didn’t mean to pass and so should pass a different type or we should implement
`Draw` on `String` so that `Screen` is able to call `draw` on it.
-->

このエラーは、渡すことを意図していないものを`Screen`に渡しているので、異なる型を渡すべきか、
`Screen`が`draw`を呼び出せるように`String`に`Draw`を実装するべきのどちらかであることを知らせてくれています。

<!--
### Trait Objects Perform Dynamic Dispatch
-->

### トレイトオブジェクトは、ダイナミックディスパッチを行う

<!--
Recall in the [“Performance of Code Using
Generics”][performance-of-code-using-generics] section in
Chapter 10 our discussion on the monomorphization process performed by the
compiler when we use trait bounds on generics: the compiler generates
nongeneric implementations of functions and methods for each concrete type that
we use in place of a generic type parameter. The code that results from
monomorphization is doing *static dispatch*, which is when the compiler knows
what method you’re calling at compile time. This is opposed to *dynamic
dispatch*, which is when the compiler can’t tell at compile time which method
you’re calling. In dynamic dispatch cases, the compiler emits code that at
runtime will figure out which method to call.
-->

第10章の[「ジェネリクスを使用したコードのパフォーマンス」][performance-of-code-using-generics]節でジェネリクスに対してトレイト境界を使用した時に、
コンパイラが行う単相化過程の議論を思い出してください: コンパイラは、関数やメソッドのジェネリックでない実装を、
ジェネリックな型引数の箇所に使用している具体的な型に対して生成するのでした。単相化の結果吐かれるコードは、
*スタティックディスパッチ*を行い、これは、コンパイル時にコンパイラがどのメソッドを呼び出しているかわかる時のことです。
これは、*ダイナミックディスパッチ*とは対照的で、この時、コンパイラは、コンパイル時にどのメソッドを呼び出しているのかわかりません。
ダイナミックディスパッチの場合、コンパイラは、どのメソッドを呼び出すか実行時に弾き出すコードを生成します。

<!--
When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t
know all the types that might be used with the code that’s using trait objects,
so it doesn’t know which method implemented on which type to call. Instead, at
runtime, Rust uses the pointers inside the trait object to know which method to
call. This lookup incurs a runtime cost that doesn’t occur with static
dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a
method’s code, which in turn prevents some optimizations. However, we did get
extra flexibility in the code that we wrote in Listing 17-5 and were able to
support in Listing 17-9, so it’s a trade-off to consider.
-->

トレイトオブジェクトを使用すると、コンパイラはダイナミックディスパッチを使用しなければなりません。
コンパイラは、トレイトオブジェクトを使用しているコードで使用される可能性のある型全てを把握しないので、
どの型に実装されたどのメソッドを呼び出すかわからないのです。代わりに実行時に、トレイトオブジェクト内でポインタを使用して、
コンパイラは、どのメソッドを呼ぶか知ります。この検索は、スタティックディスパッチでは発生しない実行時コストを招きます。
また、ダイナミックディスパッチは、コンパイラがメソッドのコードをインライン化することも妨げ、
そのため、ある種の最適化が不可能になります。ですが、リスト17-5で記述し、
リスト17-9ではサポートできたコードで追加の柔軟性を確かに得られたので、考慮すべき代償です。

<!--
[performance-of-code-using-generics]:
ch10-01-syntax.html#performance-of-code-using-generics
[dynamically-sized]: ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait
-->

[performance-of-code-using-generics]:
ch10-01-syntax.html#ジェネリクスを使用したコードのパフォーマンス
[dynamically-sized]: ch19-04-advanced-types.html#動的サイズ決定型とsizedトレイト
