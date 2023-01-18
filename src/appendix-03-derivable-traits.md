<!--
## Appendix C: Derivable Traits
-->

## 付録 C: 導出可能なトレイト

<!--
In various places in the book, we’ve discussed the `derive` attribute, which
you can apply to a struct or enum definition. The `derive` attribute generates
code that will implement a trait with its own default implementation on the
type you’ve annotated with the `derive` syntax.
-->

本のいろんな箇所で`derive`属性について議論しました。これは構造体や、enum 定義に適用できます。
`derive`属性は、`derive`記法で注釈した型に対して独自の既定の実装でトレイトを実装するコードを生成します。

<!--
In this appendix, we provide a reference of all the traits in the standard
library that you can use with `derive`. Each section covers:
-->

この付録では、標準ライブラリの`derive`と共に使用できる全トレイトの参照を提供します。各節は以下を講義します：

<!--
* What operators and methods deriving this trait will enable
* What the implementation of the trait provided by `derive` does
* What implementing the trait signifies about the type
* The conditions in which you’re allowed or not allowed to implement the trait
* Examples of operations that require the trait
-->

* このトレイトを導出する演算子やメソッドで可能になること
* `derive`が提供するトレイトの実装がすること
* トレイトを実装することが型についてどれほど重要か
* そのトレイトを実装できたりできなかったりする条件
* そのトレイトが必要になる処理の例

<!--
If you want different behavior than that provided by the `derive` attribute,
consult the standard library documentation for each trait for details on how to
manually implement them.
-->

`derive`属性が提供する以外の異なる振る舞いが欲しいなら、それらを手動で実装する方法の詳細について、
各トレイトの標準ライブラリのドキュメンテーションを調べてください。

<!--
The rest of the traits defined in the standard library can’t be implemented on
your types using `derive`. These traits don’t have sensible default behavior,
so it’s up to you to implement them in the way that makes sense for what you’re
trying to accomplish.
-->

標準ライブラリで定義されている残りのトレイトは、`derive`で自分の型に実装することはできません。
これらのトレイトには知覚できるほどの既定の振る舞いはないので、自分が達成しようしていることに対して、
道理が通る方法でそれらを実装するのはあなた次第です。

<!--
An example of a trait that can’t be derived is `Display`, which handles
formatting for end users. You should always consider the appropriate way to
display a type to an end user. What parts of the type should an end user be
allowed to see? What parts would they find relevant? What format of the data
would be most relevant to them? The Rust compiler doesn’t have this insight, so
it can’t provide appropriate default behavior for you.
-->

導出できないトレイトの例は`Display`で、これはエンドユーザ向けのフォーマットを扱います。常に、エンドユーザ向けに型を表示する適切な方法について、
考慮すべきです。型のどの部分をエンドユーザは見ることができるべきでしょうか？どの部分を関係があると考えるでしょうか？
どんな形式のデータがエンドユーザにとって最も関係があるでしょうか？Rust コンパイラには、
この見識がないため、適切な既定動作を提供してくれないのです。

<!--
The list of derivable traits provided in this appendix is not comprehensive:
libraries can implement `derive` for their own traits, making the list of
traits you can use `derive` with truly open-ended. Implementing `derive`
involves using a procedural macro, which is covered in Appendix D.
-->

この付録で提供される導出可能なトレイトのリストは、包括的ではありません：ライブラリは、自身のトレイトに`derive`を実装でき、
`derive`と共に使用できるトレイトのリストが実に限りのないものになってしまうのです。`derive`の実装には、
プロシージャルなマクロが関連します。マクロについては、付録 D で講義します。

<!--
### `Debug` for Programmer Output
-->

### プログラマ用の出力の`Debug`

<!--
The `Debug` trait enables debug formatting in format strings, which you
indicate by adding `:?` within `{}` placeholders.
-->

`Debug`トレイトにより、フォーマット文字列でのデバッグ整形が可能になり、
`{}`プレースホルダー内に`:?`を追記することで表します。

<!--
The `Debug` trait allows you to print instances of a type for debugging
purposes, so you and other programmers using your type can inspect an instance
at a particular point in a program’s execution.
-->

`Debug`トレイトにより、デバッグ目的で型のインスタンスを出力できるようになるので、あなたや型を使用する他のプログラマが、
プログラムの実行の特定の箇所でインスタンスを調べられます。

<!--
The `Debug` trait is required, for example, in use of the `assert_eq!` macro.
This macro prints the values of instances given as arguments if the equality
assertion fails so programmers can see why the two instances weren’t equal.
-->

`Debug`トレイトは、例えば、`assert_eq!`マクロを使用する際などに必要になります。
このマクロは、プログラマがどうして 2 つのインスタンスが等価でなかったのか確認できるように、
等価アサートが失敗したら、引数として与えられたインスタンスの値を出力します。

<!--
### `PartialEq` and `Eq` for Equality Comparisons
-->

### 等価比較のための`PartialEq`と`Eq`

<!--
The `PartialEq` trait allows you to compare instances of a type to check for
equality and enables use of the `==` and `!=` operators.
-->

`PartialEq`トレイトにより、型のインスタンスを比較して、等価性をチェックでき、`==`と`!=`演算子の使用を可能にします。

<!--
Deriving `PartialEq` implements the `eq` method. When `PartialEq` is derived on
structs, two instances are equal only if *all* fields are equal, and the
instances are not equal if any fields are not equal. When derived on enums,
each variant is equal to itself and not equal to the other variants.
-->

`PartialEq`を導出すると、`eq`メソッドを実装します。構造体に`PartialEq`を導出すると、
*全*フィールドが等しい時のみ 2 つのインスタンスは等価になり、いずれかのフィールドが等価でなければ、
インスタンスは等価ではなくなります。enum に導出すると、各列挙子は、自身には等価ですが、他の列挙子には等価ではありません。

<!--
The `PartialEq` trait is required, for example, with the use of the
`assert_eq!` macro, which needs to be able to compare two instances of a type
for equality.
-->

`PartialEq`トレイトは例えば、`assert_eq!`マクロを使用する際に必要になります。
これは、等価性のためにとある型の 2 つのインスタンスを比較できる必要があります。

<!--
The `Eq` trait has no methods. Its purpose is to signal that for every value of
the annotated type, the value is equal to itself. The `Eq` trait can only be
applied to types that also implement `PartialEq`, although not all types that
implement `PartialEq` can implement `Eq`. One example of this is floating point
number types: the implementation of floating point numbers states that two
instances of the not-a-number (`NaN`) value are not equal to each other.
-->

`Eq`トレイトにはメソッドはありません。その目的は、注釈された型の全値に対して、値が自身と等しいことを通知することです。
`Eq`トレイトは、`PartialEq`を実装する全ての型が`Eq`を実装できるわけではないものの、
`PartialEq`も実装する型に対してのみ適用できます。これの一例は、浮動小数点数型です：
浮動小数点数の実装により、非数字 (`NaN`) 値の 2 つのインスタンスはお互いに等価ではないことが宣言されます。

<!--
An example of when `Eq` is required is for keys in a `HashMap<K, V>` so the
`HashMap<K, V>` can tell whether two keys are the same.
-->

`Eq`が必要になる一例が、`HashMap<K, V>`のキーで、`HashMap<K, V>`が、2 つのキーが同じであると判定できます。

<!--
### `PartialOrd` and `Ord` for Ordering Comparisons
-->

### 順序付き比較のための`PartialOrd`と`Ord`

<!--
The `PartialOrd` trait allows you to compare instances of a type for sorting
purposes. A type that implements `PartialOrd` can be used with the `<`, `>`,
`<=`, and `>=` operators. You can only apply the `PartialOrd` trait to types
that also implement `PartialEq`.
-->

`PartialOrd`トレイトにより、ソートする目的で型のインスタンスを比較できます。`PartialOrd`を実装する型は、
`<`、`>`、`<=`、`>=`演算子を使用することができます。`PartialEq`も実装する型に対してのみ、
`PartialOrd`トレイトを適用できます。

<!--
Deriving `PartialOrd` implements the `partial_cmp` method, which returns an
`Option<Ordering>` that will be `None` when the values given don’t produce an
ordering. An example of a value that doesn’t produce an ordering, even though
most values of that type can be compared, is the not-a-number (`NaN`) floating
point value. Calling `partial_cmp` with any floating point number and the `NaN`
floating point value will return `None`.
-->

`PartialOrd`を導出すると、`partial_cmp`メソッドを実装し、これは、与えられた値が順序付けられない時に`None`になる`Option<Ordering>`を返します。
その型のほとんどの値は比較できるものの、順序付けできない値の例として、非数字 (`NaN`) 浮動小数点値が挙げられます。
`partial_cmp`をあらゆる浮動小数点数と`NaN`浮動小数点数で呼び出すと、`None`が返るでしょう。

<!--
When derived on structs, `PartialOrd` compares two instances by comparing the
value in each field in the order in which the fields appear in the struct
definition. When derived on enums, variants of the enum declared earlier in the
enum definition are considered less than the variants listed later.
-->

構造体に導出すると、フィールドが構造体定義で現れる順番で各フィールドの値を比較することで 2 つのインスタンスを比較します。
enum に導出すると、enum 定義で先に定義された列挙子が、後に列挙された列挙子よりも小さいと考えられます。

<!--
The `PartialOrd` trait is required, for example, for the `gen_range` method
from the `rand` crate that generates a random value in the range specified by a
low value and a high value.
-->

`PartialOrd`トレイトが必要になる例には、低い値と高い値で指定される範囲の乱数を生成する`rand`クレートの`gen_range`メソッドが挙げられます。

<!--
The `Ord` trait allows you to know that for any two values of the annotated
type, a valid ordering will exist. The `Ord` trait implements the `cmp` method,
which returns an `Ordering` rather than an `Option<Ordering>` because a valid
ordering will always be possible. You can only apply the `Ord` trait to types
that also implement `PartialOrd` and `Eq` (and `Eq` requires `PartialEq`). When
derived on structs and enums, `cmp` behaves the same way as the derived
implementation for `partial_cmp` does with `PartialOrd`.
-->

`Ord`トレイトにより、注釈した型のあらゆる 2 つの値に対して、合法な順序付けが行えることがわかります。
`Ord`トレイトは`cmp`メソッドを実装し、これは、常に合法な順序付けが可能なので、`Option<Ordering>`ではなく、
`Ordering`を返します。`PartialOrd`と`Eq`(`Eq`は`PartialEq`も必要とします) も実装している型にしか、
`Ord`トレイトを適用することはできません。構造体と enum で導出したら、`PartialOrd`で、
`partial_cmp`の導出した実装と同じように`cmp`は振る舞います。

<!--
An example of when `Ord` is required is when storing values in a `BTreeSet<T>`,
a data structure that stores data based on the sort order of the values.
-->

`Ord`が必要になる例は、`BTreeSet<T>`に値を格納する時です。
これは、値のソート順に基づいてデータを格納するデータ構造です。

<!--
### `Clone` and `Copy` for Duplicating Values
-->

### 値を複製する`Clone`と`Copy`

<!--
The `Clone` trait allows you to explicitly create a deep copy of a value, and
the duplication process might involve running arbitrary code and copying heap
data. See the “Ways Variables and Data Interact: Clone” section in Chapter 4
for more information on `Clone`.
-->

`Clone`トレイトにより値のディープコピーを明示的に行うことができ、複製のプロセスは、任意のコードを実行し、
ヒープデータをコピーすることに関係がある可能性があります。`Clone`について詳しくは、
第 4 章の「変数とデータの相互作用法：Clone」節を参照されたし。

<!--
Deriving `Clone` implements the `clone` method, which when implemented for the
whole type, calls `clone` on each of the parts of the type. This means all the
fields or values in the type must also implement `Clone` to derive `Clone`.
-->

`Clone`を導出すると、`clone`メソッドを実装し、これは型全体に対して実装されると、
型の各部品に対して`clone`を呼び出します。要するに、`Clone`を導出するには、
型のフィールドと値全部も`Clone`を実装していなければならないということです。

<!--
An example of when `Clone` is required is when calling the `to_vec` method on a
slice. The slice doesn’t own the type instances it contains, but the vector
returned from `to_vec` will need to own its instances, so `to_vec` calls
`clone` on each item. Thus, the type stored in the slice must implement `Clone`.
-->

`Clone`が必要になる例は、スライスに対して`to_vec`メソッドを呼び出すことです。スライスは、
含んでいる型のインスタンスの所有権を持たないが、`to_vec`で返されるベクタはそのインスタンスを所有する必要があるので、
`to_vec`は各要素に対して`clone`を呼び出します。故に、スライスに格納される型は、`Clone`を実装しなければならないのです。

<!--
The `Copy` trait allows you to duplicate a value by only copying bits stored on
the stack; no arbitrary code is necessary. See the “Stack-Only Data: Copy”
section in Chapter 4 for more information on `Copy`.
-->

`Copy`トレイトにより、スタックに格納されたビットをコピーするだけで値を複製できます; 任意のコードは必要ありません。
`Copy`について詳しくは、第 4 章の「スタックのみのデータ：Copy」を参照されたし。

<!--
The `Copy` trait doesn’t define any methods to prevent programmers from
overloading those methods and violating the assumption that no arbitrary code
is being run. That way, all programmers can assume that copying a value will be
very fast.
-->

`Copy`トレイトは、プログラマがメソッドをオーバーロードし、任意のコードが実行されないという前提を侵害することを妨げるメソッドは何も定義しません。
そのため、全プログラマは、値のコピーは非常に高速であることを前提にすることができます。

<!--
You can derive `Copy` on any type whose parts all implement `Copy`. You can
only apply the `Copy` trait to types that also implement `Clone`, because a
type that implements `Copy` has a trivial implementation of `Clone` that
performs the same task as `Copy`.
-->

部品すべてが`Copy`を実装する任意の型に対して`Copy`を導出することができます。`Clone`も実装する型に対してのみ、
`Copy`トレイトを適用することができます。何故なら、`Copy`を実装する型には、
`Copy`と同じ作業を行う`Clone`の<ruby>瑣末<rp>(</rp><rt>さまつ</rt><rp>)</rp></ruby>な実装があるからです。

<!--
The `Copy` trait is rarely required; types that implement `Copy` have
optimizations available, meaning you don’t have to call `clone`, which makes
the code more concise.
-->

`Copy`トレイトは稀にしか必要になりません; `Copy`を実装する型では最適化が利用可能になります。
つまり、`clone`を呼び出す必要がなくなり、コードがより簡潔になるということです。

<!--
Everything possible with `Copy` you can also accomplish with `Clone`, but the
code might be slower or have to use `clone` in places.
-->

`Copy`で可能なこと全てが`Clone`でも達成可能ですが、コードがより遅い可能性や、
`clone`を使用しなければならない箇所があったりします。

<!--
### `Hash` for Mapping a Value to a Value of Fixed Size
-->

### 値を固定サイズの値にマップする`Hash`

<!--
The `Hash` trait allows you to take an instance of a type of arbitrary size and
map that instance to a value of fixed size using a hash function. Deriving
`Hash` implements the `hash` method. The derived implementation of the `hash`
method combines the result of calling `hash` on each of the parts of the type,
meaning all fields or values must also implement `Hash` to derive `Hash`.
-->

`Hash`トレイトにより、任意のサイズの型のインスタンスを取り、そのインスタンスをハッシュ関数で固定サイズの値にマップできます。
`Hash`を導出すると、`hash`メソッドを実装します。`hash`の導出された実装は、
型の各部品に対して呼び出した`hash`の結果を組み合わせます。つまり、`Hash`を導出するには、
全フィールドと値も`Hash`を実装しなければならないということです。

<!--
An example of when `Hash` is required is in storing keys in a `HashMap<K, V>`
to store data efficiently.
-->

`Hash`が必要になる例は、`HashMap<K, V>`にキーを格納し、データを効率的に格納することです。

<!--
### `Default` for Default Values
-->

### 既定値のための`Default`

<!--
The `Default` trait allows you to create a default value for a type. Deriving
`Default` implements the `default` function. The derived implementation of the
`default` function calls the `default` function on each part of the type,
meaning all fields or values in the type must also implement `Default` to
derive `Default.`
-->

`Default`トレイトにより、型に対して既定値を生成できます。`Default`を導出すると、`default`関数を実装します。
`default`関数の導出された実装は、型の各部品に対して`default`関数を呼び出します。つまり、
`Default`を導出するには、型の全フィールドと値も`Default`を実装しなければならないということです。

<!--
The `Default::default` function is commonly used in combination with the struct
update syntax discussed in the “Creating Instances From Other Instances With
Struct Update Syntax” section in Chapter 5. You can customize a few fields of a
struct and then set and use a default value for the rest of the fields by using
`..Default::default()`.
-->

`Default::default`関数は、
第 5 章の「構造体更新記法で他のインスタンスからインスタンスを生成する」節で議論した構造体更新記法と組み合わせてよく使用されます。
構造体のいくつかのフィールドをカスタマイズし、それから`..Default::default()`を使用して、
残りのフィールドに対して既定値をセットし使用することができます。

<!--
The `Default` trait is required when you use the method `unwrap_or_default` on
`Option<T>` instances, for example. If the `Option<T>` is `None`, the method
`unwrap_or_default` will return the result of `Default::default` for the type
`T` stored in the `Option<T>`.
-->

例えば、`Default`トレイトは、`Option<T>`インスタンスに対してメソッド`unwrap_or_default`を使用する時に必要になります。
`Option<T>`が`None`ならば、メソッド`unwrap_or_default`は、`Option<T>`に格納された型`T`に対して`Default::default`の結果を返します。
