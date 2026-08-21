<!--
## Defining and Instantiating Structs
-->

## 構造体を定義し、インスタンス化する

<!--
Structs are similar to tuples, discussed in [“The Tuple Type”][tuples]
section, in that both hold multiple related values. Like tuples, the
pieces of a struct can be different types. Unlike with tuples, in a struct
you’ll name each piece of data so it’s clear what the values mean. Adding these
names means that structs are more flexible than tuples: you don’t have to rely
on the order of the data to specify or access the values of an instance.
-->

構造体は、[「タプル型」][tuples]の節で議論したタプルと、どちらも関係する複数の値を抱えるという点で似ています。
タプル同様、構造体の一部を異なる型にできます。
一方タプルとは違って、構造体では各データ片には名前をつけるので、値の意味が明確になります。
これらの名前が付いていることで、構造体はタプルに比して、より柔軟になるわけです: データの順番に頼って、
インスタンスの値を指定したり、アクセスしたりする必要がないのです。

<!--
To define a struct, we enter the keyword `struct` and name the entire struct. A
struct’s name should describe the significance of the pieces of data being
grouped together. Then, inside curly brackets, we define the names and types of
the pieces of data, which we call *fields*. For example, Listing 5-1 shows a
struct that stores information about a user account.
-->

構造体の定義は、`struct`キーワードを入れ、構造体全体に名前を付けます。構造体名は、
一つにグループ化されるデータ片の意義を表すものであるべきです。そして、波かっこ内に、
データ片の名前と型を定義し、これは*フィールド*と呼ばれます。例えば、リスト5-1では、
ユーザアカウントに関する情報を保持する構造体を示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

<!--
<span class="caption">Listing 5-1: A `User` struct definition</span>
-->

<span class="caption">リスト5-1: `User`構造体定義</span>

<!--
To use a struct after we’ve defined it, we create an *instance* of that struct
by specifying concrete values for each of the fields. We create an instance by
stating the name of the struct and then add curly brackets containing *key:
value* pairs, where the keys are the names of the fields and the values are the
data we want to store in those fields. We don’t have to specify the fields in
the same order in which we declared them in the struct. In other words, the
struct definition is like a general template for the type, and instances fill
in that template with particular data to create values of the type. For
example, we can declare a particular user as shown in Listing 5-2.
-->

構造体を定義した後に使用するには、各フィールドに対して具体的な値を指定して構造体の*インスタンス*を生成します。
インスタンスは、構造体名を記述し、*key: value*ペアを含む波かっこを付け加えることで生成します。
ここで、キーはフィールド名、値はそのフィールドに格納したいデータになります。フィールドは、
構造体で宣言した通りの順番に指定する必要はありません。換言すると、構造体定義とは、
型に対する一般的な雛形のようなものであり、インスタンスは、その雛形を特定のデータで埋め、その型の値を生成するわけです。
例えば、リスト5-2で示されたように特定のユーザを宣言することができます。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

<!--
<span class="caption">Listing 5-2: Creating an instance of the `User`
struct</span>
-->

<span class="caption">リスト5-2: `User`構造体のインスタンスを生成する</span>

<!--
To get a specific value from a struct, we use dot notation. For example, to
access this user’s email address, we use `user1.email`. If the instance is
mutable, we can change a value by using the dot notation and assigning into a
particular field. Listing 5-3 shows how to change the value in the `email`
field of a mutable `User` instance.
-->

構造体から特定の値を得るには、ドット記法を使います。例えば、
このユーザのEメールアドレスにアクセスするには、`user1.email`を使います。
インスタンスが可変であれば、ドット記法を使い特定のフィールドに代入することで値を変更できます。
リスト5-3では、可変な`User`インスタンスの`email`フィールド値を変更する方法を示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

<!--
<span class="caption">Listing 5-3: Changing the value in the `email` field of a
`User` instance</span>
-->

<span class="caption">リスト5-3: ある`User`インスタンスの`email`フィールド値を変更する</span>

<!--
Note that the entire instance must be mutable; Rust doesn’t allow us to mark
only certain fields as mutable. As with any expression, we can construct a new
instance of the struct as the last expression in the function body to
implicitly return that new instance.
-->

インスタンス全体が可変でなければならないことに注意してください; Rustでは、一部のフィールドのみを可変にすることはできないのです。
また、あらゆる式同様、構造体の新規インスタンスを関数本体の最後の式として生成して、
そのインスタンスを返すことを暗示できます。

<!--
Listing 5-4 shows a `build_user` function that returns a `User` instance with
the given email and username. The `active` field gets the value of `true`, and
the `sign_in_count` gets a value of `1`.
-->

リスト5-4は、与えられたemailとusernameで`User`インスタンスを生成する`build_user`関数を示しています。
`active`フィールドには`true`値が入り、`sign_in_count`には値`1`が入ります。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

<!--
<span class="caption">Listing 5-4: A `build_user` function that takes an email
and username and returns a `User` instance</span>
-->

<span class="caption">リスト5-4: Eメールとユーザ名を取り、`User`インスタンスを返す`build_user`関数</span>

<!--
It makes sense to name the function parameters with the same name as the struct
fields, but having to repeat the `email` and `username` field names and
variables is a bit tedious. If the struct had more fields, repeating each name
would get even more annoying. Luckily, there’s a convenient shorthand!
-->

構造体のフィールドと同じ名前を関数の引数にもつけることは筋が通っていますが、
`email`と`username`というフィールド名と変数を繰り返さなきゃいけないのは、ちょっと面倒です。
構造体にもっとフィールドがあれば、名前を繰り返すことはさらに煩わしくなるでしょう。
幸運なことに、便利な省略記法があります！

<!-- Old heading. Do not remove or links may break. -->
<!--
<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>
-->

<!--
### Using the Field Init Shorthand
-->

### フィールド初期化省略記法を使う

<!--
Because the parameter names and the struct field names are exactly the same in
Listing 5-4, we can use the *field init shorthand* syntax to rewrite
`build_user` so it behaves exactly the same but doesn’t have the repetition of
`username` and `email`, as shown in Listing 5-5.
-->

仮引数名と構造体のフィールド名がリスト5-4では、全く一緒なので、*フィールド初期化省略*記法を使って`build_user`を書き換えても、
振る舞いは全く同じにしつつ、リスト5-5に示したように`username`と`email`を繰り返さなくてもよくなります。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

<!--
<span class="caption">Listing 5-5: A `build_user` function that uses field init
shorthand because the `username` and `email` parameters have the same name as
struct fields</span>
-->

<span class="caption">リスト5-5: `username`と`email`引数が構造体のフィールドと同名なので、
フィールド初期化省略法を使用する`build_user`関数</span>

<!--
Here, we’re creating a new instance of the `User` struct, which has a field
named `email`. We want to set the `email` field’s value to the value in the
`email` parameter of the `build_user` function. Because the `email` field and
the `email` parameter have the same name, we only need to write `email` rather
than `email: email`.
-->

ここで、`email`というフィールドを持つ`User`構造体の新規インスタンスを生成しています。
`email`フィールドを`build_user`関数の`email`引数の値にセットしたいわけです。
`email`フィールドと`email`引数は同じ名前なので、`email: email`と書くのではなく、
`email`と書くだけで済むのです。

<!--
### Creating Instances from Other Instances with Struct Update Syntax
-->

### 構造体更新記法で他のインスタンスからインスタンスを生成する

<!--
It’s often useful to create a new instance of a struct that includes most of
the values from another instance, but changes some. You can do this using
*struct update syntax*.
-->

他のインスタンスからの値の多くの部分を含みつつ、一部を変更する形で新しいインスタンスを生成できるとしばしば有用です。
*構造体更新記法*でそうすることができます。

<!--
First, in Listing 5-6 we show how to create a new `User` instance in `user2`
regularly, without the update syntax. We set a new value for `email` but
otherwise use the same values from `user1` that we created in Listing 5-2.
-->

まず、リスト5-6では、更新記法なしで普通に`user2`に新しい`User`インスタンスを生成する方法を示しています。
`email`には新しい値をセットしていますが、それ以外にはリスト5-2で生成した`user1`の値を使用しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

<!--
<span class="caption">Listing 5-6: Creating a new `User` instance using one of
the values from `user1`</span>
-->

<span class="caption">リスト5-6: `user1`の一部の値を使用しつつ、新しい`User`インスタンスを生成する</span>

<!--
Using struct update syntax, we can achieve the same effect with less code, as
shown in Listing 5-7. The syntax `..` specifies that the remaining fields not
explicitly set should have the same value as the fields in the given instance.
-->

構造体更新記法を使用すると、リスト5-7に示したように、コード量を減らしつつ、同じ効果を達成できます。`..`という記法により、
明示的にセットされていない残りのフィールドが、与えられたインスタンスのフィールドと同じ値になるように指定します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

<!--
<span class="caption">Listing 5-7: Using struct update syntax to set a new
`email` value for a `User` instance but to use the rest of the values from
`user1`</span>
-->

<span class="caption">リスト5-7: 構造体更新記法を使用して、新しい`User`インスタンス用の値に新しい`email`をセットしつつ、
残りの値は`user1`を使う</span>

<!--
The code in Listing 5-7 also creates an instance in `user2` that has a
different value for `email` but has the same values for the `username`,
`active`, and `sign_in_count` fields from `user1`. The `..user1` must come last
to specify that any remaining fields should get their values from the
corresponding fields in `user1`, but we can choose to specify values for as
many fields as we want in any order, regardless of the order of the fields in
the struct’s definition.
-->

リスト5-7のコードも、`email`については`user1`とは異なる値、`username`、`active`と`sign_in_count`フィールドについては、
`user1`と同じ値になるインスタンスを`user2`に生成します。
`..user1`は、残りのフィールドについては`user1`の対応するフィールドから値を取る、ということを示すために最後に来る必要がありますが、
フィールドについては好きなだけ多く、構造体定義中のフィールドの順序とは無関係に好きな順で、値を指定してかまいません。

<!--
Note that the struct update syntax uses `=` like an assignment; this is because
it moves the data, just as we saw in the [“Variables and Data Interacting with
Move”][move] section. In this example, we can no longer use
`user1` as a whole after creating `user2` because the `String` in the
`username` field of `user1` was moved into `user2`. If we had given `user2` new
`String` values for both `email` and `username`, and thus only used the
`active` and `sign_in_count` values from `user1`, then `user1` would still be
valid after creating `user2`. Both `active` and `sign_in_count` are types that
implement the `Copy` trait, so the behavior we discussed in the [“Stack-Only
Data: Copy”][copy] section would apply.
-->

構造体更新記法は代入と同様に`=`を使います; これは、[「ムーブによる変数とデータの相互作用」][move]の節で見たのと同じように、
データをムーブするからです。この例で言うと、`user2`を作成した後は、もう`user1`をそっくりそのまま使うことはできません。
`user1`の`username`フィールド中の`String`が`user2`の中にムーブされてしまったからです。
もし`user2`に、`email`と`username`のために新しい`String`値を与えていたなら、つまり、
`user1`からは`active`と`sign_in_count`の値だけを使用していたなら、`user2`を作成した後も`user1`はまだ有効だったでしょう。
`active`と`sign_in_count`はどちらも`Copy`トレイトを実装した型なので、[「スタックのみのデータ: コピー」][copy]節で議論した振る舞いが適用されるからです。

<!--
### Using Tuple Structs Without Named Fields to Create Different Types
-->

### 異なる型を生成する名前付きフィールドのないタプル構造体を使用する

<!--
Rust also supports structs that look similar to tuples, called *tuple structs*.
Tuple structs have the added meaning the struct name provides but don’t have
names associated with their fields; rather, they just have the types of the
fields. Tuple structs are useful when you want to give the whole tuple a name
and make the tuple a different type from other tuples, and when naming each
field as in a regular struct would be verbose or redundant.
-->

Rustは、構造体名により追加の意味を含むものの、フィールドに紐づけられた名前がなく、むしろフィールドの型だけの*タプル構造体*と呼ばれる、
タプルに似た構造体もサポートしています。タプル構造体は、構造体名が提供する追加の意味は含むものの、
フィールドに紐付けられた名前はありません; むしろ、フィールドの型だけが存在します。タプル構造体は、タプル全体に名前をつけ、
そのタプルを他のタプルとは異なる型にしたいが、普通の構造体のように各フィールド名を与えるのは、
冗長、または余計という場合に有用です。

<!--
To define a tuple struct, start with the `struct` keyword and the struct name
followed by the types in the tuple. For example, here we define and use two
tuple structs named `Color` and `Point`:
-->

タプル構造体を定義するには、`struct`キーワードの後に構造体名、さらにタプルに含まれる型を続けてください。
例えば、ここでは、`Color`と`Point`という2種類のタプル構造体の定義して使用します:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

<!--
Note that the `black` and `origin` values are different types because they’re
instances of different tuple structs. Each struct you define is its own type,
even though the fields within the struct might have the same types. For
example, a function that takes a parameter of type `Color` cannot take a
`Point` as an argument, even though both types are made up of three `i32`
values. Otherwise, tuple struct instances are similar to tuples in that you can
destructure them into their individual pieces, and you can use a `.` followed
by the index to access an individual value.
-->

`black`と`origin`の値は、違う型であることに注目してください。これらは、異なるタプル構造体のインスタンスだからですね。
定義された各構造体は、構造体内のフィールドが同じ型であっても、それ自身が独自の型になります。
例えば、`Color`型を引数に取る関数は、`Point`を引数に取ることはできません。たとえ、両者の型が、
3つの`i32`値からできていてもです。それ以外については、タプル構造体のインスタンスは、
分配して個々の部品にしたり、`.`と添え字を使用して個々の値にアクセスできるという点で、タプルと似ています。

<!--
### Unit-Like Structs Without Any Fields
-->

### フィールドのないユニット<ruby>様<rp>(</rp><rt>よう</rt><rp>)</rp></ruby>構造体

<!--
You can also define structs that don’t have any fields! These are called
*unit-like structs* because they behave similarly to `()`, the unit type that
we mentioned in [“The Tuple Type”][tuples] section. Unit-like
structs can be useful when you need to implement a trait on some type but don’t
have any data that you want to store in the type itself. We’ll discuss traits
in Chapter 10. Here’s an example of declaring and instantiating a unit struct
named `AlwaysEqual`:
-->

また、一切フィールドのない構造体を定義することもできます！これらは、`()`、
[「タプル型」][tuples]の節で言及したユニット型と似たような振る舞いをすることから、
*ユニット様構造体*と呼ばれます。ユニット様構造体は、ある型にトレイトを実装するけれども、
型自体に保持させるデータは一切ない場面に有効になります。トレイトについては第10章で議論します。
以下は、`AlwaysEqual`という名前のユニット様構造体を宣言し、インスタンス化する例です:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

<!--
To define `AlwaysEqual`, we use the `struct` keyword, the name we want, and
then a semicolon. No need for curly brackets or parentheses! Then we can get an
instance of `AlwaysEqual` in the `subject` variable in a similar way: using the
name we defined, without any curly brackets or parentheses. Imagine that later
we’ll implement behavior for this type such that every instance of
`AlwaysEqual` is always equal to every instance of any other type, perhaps to
have a known result for testing purposes. We wouldn’t need any data to
implement that behavior! You’ll see in Chapter 10 how to define traits and
implement them on any type, including unit-like structs.
-->

`AlwaysEqual`を定義するためには、`struct`キーワード、付けたい名前、そしてセミコロンを使います。
波括弧や丸括弧は不要です！
次に、同じようにして、`subject`変数に`AlwaysEqual`のインスタンスを得られます: 波括弧や丸括弧を付けずに、定義した名前を使います。
後でこの型の振る舞いを、おそらくはテスト目的で既知の結果を得るために、`AlwaysEqual`のすべてのインスタンスが常に他の任意の型と等価であるように実装することを想像してください。
この挙動を実装するためにデータはまったく必要ないですね！
トレイトを定義して、ユニット様構造体も含めた任意の型にそれを実装する方法については10章で触れます。

<!--
以下のパラグラフでは、引用されてるブロックの後に、和訳を示します。こうしないと、意図通りのレイアウトにならないようです
-->

<!--
> ### Ownership of Struct Data
>
> In the `User` struct definition in Listing 5-1, we used the owned `String`
> type rather than the `&str` string slice type. This is a deliberate choice
> because we want each instance of this struct to own all of its data and for
> that data to be valid for as long as the entire struct is valid.
>
> It’s also possible for structs to store references to data owned by something
> else, but to do so requires the use of *lifetimes*, a Rust feature that we’ll
> discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct
> is valid for as long as the struct is. Let’s say you try to store a reference
> in a struct without specifying lifetimes, like the following; this won’t work:
>
> <span class="filename">Filename: src/main.rs</span>
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 --
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         active: true,
>         username: "someusername123",
>         email: "someone@example.com",
>         sign_in_count: 1,
>     };
> }
> ```
>
> The compiler will complain that it needs lifetime specifiers:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  -- src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  -- src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` due to 2 previous errors
> ```
>
> In Chapter 10, we’ll discuss how to fix these errors so you can store
> references in structs, but for now, we’ll fix errors like these using owned
> types like `String` instead of references like `&str`.
-->

> ### 構造体データの所有権
>
> リスト5-1の`User`構造体定義において、`&str`文字列スライス型ではなく、所有権のある`String`型を使用しました。
> これは意図的な選択です。というのも、この構造体の各インスタンスには自身の全データを所有してもらう必要があり、
> このデータは、構造体全体が有効な間はずっと有効である必要があるのです。
>
> 構造体に、他の何かに所有されたデータへの参照を保持させることもできますが、
> そうするには*ライフタイム*という第10章で議論するRustの機能を使用しなければなりません。
> ライフタイムのおかげで構造体に参照されたデータが、構造体自体が有効な間、ずっと有効であることを保証してくれるのです。
> 次のように、ライフタイムを指定せずに構造体に参照を保持させようとしたとしましょう。これは動きません:
>
> <span class="filename">ファイル名: src/main.rs</span>
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         active: true,
>         username: "someusername123",
>         email: "someone@example.com",
>         sign_in_count: 1,
>     };
> }
> ```
>
> コンパイラは、ライフタイム指定子が必要だと怒るでしょう:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
> (エラー: ライフタイム指定子がありません)
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |                (ライフタイム引数を予期しました)
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` (bin "structs") due to 2 previous errors
> ```
>
> 第10章で、これらのエラーを解消して構造体に参照を保持する方法について議論しますが、
> 当面、今回のようなエラーは、`&str`のような参照の代わりに、`String`のような所有された型を使うことで修正します。

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->

<!--
[tuples]: ch03-02-data-types.html#the-tuple-type
[move]: ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
[copy]: ch04-01-what-is-ownership.html#stack-only-data-copy
-->

[tuples]: ch03-02-data-types.html#タプル型
[move]: ch04-01-what-is-ownership.html#ムーブによる変数とデータの相互作用
[copy]: ch04-01-what-is-ownership.html#スタックのみのデータ-コピー
