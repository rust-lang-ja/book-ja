<!--
## Defining and Instantiating Structs
-->

## 構造体を定義し、インスタンス化する

<!--
Structs are similar to tuples, which were discussed in Chapter 3. Like tuples,
the pieces of a struct can be different types. Unlike with tuples, you'll name
each piece of data so it’s clear what the values mean. As a result of these
names, structs are more flexible than tuples: we don’t have to rely on the
order of the data to specify or access the values of an instance.
-->

構造体は第 3 章で議論したタプルと似ています。タプル同様、構造体の一部を異なる型にできます。
一方タプルとは違って、各データ片には名前をつけるので、値の意味が明確になります。
この名前のおかげで、構造体はタプルに比して、より柔軟になるわけです：データの順番に頼って、
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
データ片の名前と型を定義し、これは*フィールド*と呼ばれます。例えば、リスト 5-1 では、
ユーザアカウントに関する情報を保持する構造体を示しています。

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

<!--
<span class="caption">Listing 5-1: A `User` struct definition</span>
-->

<span class="caption">リスト 5-1: `User`構造体定義</span>

<!--
To use a struct after we’ve defined it, we create an *instance* of that struct
by specifying concrete values for each of the fields. We create an instance by
stating the name of the struct, and then add curly brackets containing `key:
value` pairs, where the keys are the names of the fields and the values are the
data we want to store in those fields. We don’t have to specify the fields in
the same order in which we declared them in the struct. In other words, the
struct definition is like a general template for the type, and instances fill
in that template with particular data to create values of the type. For
example, we can declare a particular user as shown in Listing 5-2.
-->

構造体を定義した後に使用するには、各フィールドに対して具体的な値を指定して構造体の*インスタンス*を生成します。
インスタンスは、構造体名を記述し、`key: value`ペアを含む波かっこを付け加えることで生成します。
ここで、キーはフィールド名、値はそのフィールドに格納したいデータになります。フィールドは、
構造体で宣言した通りの順番に指定する必要はありません。換言すると、構造体定義とは、
型に対する一般的な雛形のようなものであり、インスタンスは、その雛形を特定のデータで埋め、その型の値を生成するわけです。
例えば、リスト 5-2 で示されたように特定のユーザを宣言することができます。

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

<!--
<span class="caption">Listing 5-2: Creating an instance of the `User`
struct</span>
-->

<span class="caption">リスト 5-2: `User`構造体のインスタンスを生成する</span>

<!--
To get a specific value from a struct, we can use dot notation. If we wanted
just this user’s email address, we can use `user1.email` wherever we wanted
to use this value. If the instance is mutable, we can change a value by using
the dot notation and assigning into a particular field. Listing 5-3 shows how
to change the value in the `email` field of a mutable `User` instance.
-->

構造体から特定の値を得るには、ドット記法が使えます。このユーザの E メールアドレスだけが欲しいなら、
この値を使いたかった場所全部で`user1.email`が使えます。インスタンスが可変であれば、
ドット記法を使い特定のフィールドに代入することで値を変更できます。リスト 5-3 では、
可変な`User`インスタンスの`email`フィールド値を変更する方法を示しています。

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

<!--
<span class="caption">Listing 5-3: Changing the value in the `email` field of a
`User` instance</span>
-->

<span class="caption">リスト 5-3: ある`User`インスタンスの`email`フィールド値を変更する</span>

<!--
Note that the entire instance must be mutable; Rust doesn’t allow us to mark
only certain fields as mutable. As with any expression, we can construct a new
instance of the struct as the last expression in the function body to
implicitly return that new instance.
-->

インスタンス全体が可変でなければならないことに注意してください; Rust では、一部のフィールドのみを可変にすることはできないのです。
また、あらゆる式同様、構造体の新規インスタンスを関数本体の最後の式として生成して、
そのインスタンスを返すことを暗示できます。

<!--
Listing 5-4 shows a `build_user` function that returns a `User` instance with
the given email and username. The `active` field gets the value of `true`, and
the `sign_in_count` gets a value of `1`.
-->

リスト 5-4 は、与えられた email と username で`User`インスタンスを生成する`build_user`関数を示しています。
`active`フィールドには`true`値が入り、`sign_in_count`には値`1`が入ります。

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

<!--
<span class="caption">Listing 5-4: A `build_user` function that takes an email
and username and returns a `User` instance</span>
-->

<span class="caption">リスト 5-4: E メールとユーザ名を取り、`User`インスタンスを返す`build_user`関数</span>

<!--
It makes sense to name the function parameters with the same name as the struct
fields, but having to repeat the `email` and `username` field names and
variables is a bit tedious. If the struct had more fields, repeating each name
would get even more annoying. Luckily, there's a convenient shorthand!
-->

構造体のフィールドと同じ名前を関数の引数にもつけることは筋が通っていますが、
`email`と`username`というフィールド名と変数を繰り返さなきゃいけないのは、ちょっと面倒です。
構造体にもっとフィールドがあれば、名前を繰り返すことはさらに煩わしくなるでしょう。
幸運なことに、便利な省略記法があります！

<!--
### Using the Field Init Shorthand when Variables and Fields Have the Same Name
-->

### フィールドと変数が同名の時にフィールド初期化省略記法を使う

<!--
Because the parameter names and the struct field names are exactly the same in
Listing 5-4, we can use the *field init shorthand* syntax to rewrite
`build_user` so that it behaves exactly the same but doesn’t have the
repetition of `email` and `username`, as shown in Listing 5-5.
-->

仮引数名と構造体のフィールド名がリスト 5-4 では、全く一緒なので、*フィールド初期化省略*記法を使って`build_user`を書き換えても、
振る舞いは全く同じにしつつ、リスト 5-5 に示したように`email`と`username`を繰り返さなくてもよくなります。

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

<!--
<span class="caption">Listing 5-5: A `build_user` function that uses field init
shorthand because the `email` and `username` parameters have the same name as
struct fields</span>
-->

<span class="caption">リスト 5-5: `email`と`username`引数が構造体のフィールドと同名なので、
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
### Creating Instances From Other Instances With Struct Update Syntax
-->

### 構造体更新記法で他のインスタンスからインスタンスを生成する

<!--
It’s often useful to create a new instance of a struct that uses most of an old
instance’s values, but changes some. You'll do this using *struct update syntax*.
-->

多くは前のインスタンスの値を使用しつつ、変更する箇所もある形で新しいインスタンスを生成できるとしばしば有用です。
*構造体更新記法*でそうすることができます。

<!--
First, Listing 5-6 shows how we create a new `User` instance in `user2` without
the update syntax. We set new values for `email` and `username`, but otherwise
use the same values from `user1` that we created in Listing 5-2.
-->

まず、リスト 5-6 では、更新記法なしで`user2`に新しい`User`インスタンスを生成する方法を示しています。
`email`と`username`には新しい値をセットしていますが、それ以外にはリスト 5-2 で生成した`user1`の値を使用しています。

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
# let user1 = User {
#     email: String::from("someone@example.com"),
#     username: String::from("someusername123"),
#     active: true,
#     sign_in_count: 1,
# };
#
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

<!--
<span class="caption">Listing 5-6: Creating a new `User` instance using some of
the values from `user1`</span>
-->

<span class="caption">リスト 5-6: `user1`の一部の値を使用しつつ、新しい`User`インスタンスを生成する</span>

<!--
Using struct update syntax, we can achieve the same effect with less code,
shown in Listing 5-7. The syntax `..` specifies that the remaining fields not
explicitly set should have the same value as the fields in the given instance.
-->

構造体更新記法を使用すると、リスト 5-7 に示したように、コード量を減らしつつ、同じ効果を達成できます。`..`という記法により、
明示的にセットされていない残りのフィールドが、与えられたインスタンスのフィールドと同じ値になるように指定します。

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
# let user1 = User {
#     email: String::from("someone@example.com"),
#     username: String::from("someusername123"),
#     active: true,
#     sign_in_count: 1,
# };
#
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

<!--
<span class="caption">Listing 5-7: Using struct update syntax to set new
`email` and `username` values for a `User` instance but use the rest of the
values from the fields of the instance in the `user1` variable</span>
-->

<span class="caption">リスト 5-7: 構造体更新記法を使用して、新しい`User`インスタンス用の値に新しい`email`と`username`をセットしつつ、
残りの値は、`user1`変数のフィールド値を使う</span>

<!--
The code in Listing 5-7 also creates an instance in `user2` that has a
different value for `email` and `username` but has the same values for the
`active` and `sign_in_count` fields from `user1`.
-->

リスト 5-7 のコードも、`email`と`username`については異なる値、`active`と`sign_in_count`フィールドについては、
`user1`と同じ値になるインスタンスを`user2`に生成します。

<!--
### Using Tuple Structs without Named Fields to Create Different Types
-->

### 異なる型を生成する名前付きフィールドのないタプル構造体を使用する

<!--
We can also define structs that look similar to tuples, called *tuple
structs*. Tuple structs have the added meaning the struct name provides but
don’t have names associated with their fields; rather, they just have the types
of the fields. Tuple structs are useful when you want to give the whole tuple a
name and make the tuple be a different type than other tuples, and naming each
field as in a regular struct would be verbose or redundant.
-->

構造体名により追加の意味を含むものの、フィールドに紐づけられた名前がなく、むしろフィールドの型だけの*タプル構造体*と呼ばれる、
タプルに似た構造体を定義することもできます。タプル構造体は、構造体名が提供する追加の意味は含むものの、
フィールドに紐付けられた名前はありません; むしろ、フィールドの型だけが存在します。タプル構造体は、タプル全体に名前をつけ、
そのタプルを他のタプルとは異なる型にしたい場合に有用ですが、普通の構造体のように各フィールド名を与えるのは、
冗長、または余計になるでしょう。

<!--
To define a tuple struct, you start with the `struct` keyword and the struct name
followed by the types in the tuple. For example, here are definitions and
usages of two tuple structs named `Color` and `Point`:
-->

タプル構造体を定義するには、`struct`キーワードの後に構造体名、さらにタプルに含まれる型を続けます。
例えば、こちらは、`Color`と`Point`という 2 種類のタプル構造体の定義と使用法です：

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

<!--
Note that the `black` and `origin` values are different types, because they’re
instances of different tuple structs. Each struct we define is its own type,
even though the fields within the struct have the same types. For example, a
function that takes a parameter of type `Color` cannot take a `Point` as an
argument, even though both types are made up of three `i32` values. Otherwise,
tuple struct instances behave like tuples: you destructure them into their
individual pieces, you can use a `.` followed by the index to access an
individual value, and so on.
-->

`black`と`origin`の値は、違う型であることに注目してください。これらは、異なるタプル構造体のインスタンスだからですね。
定義された各構造体は、構造体内のフィールドが同じ型であっても、それ自身が独自の型になります。
例えば、`Color`型を引数に取る関数は、`Point`を引数に取ることはできません。たとえ、両者の型が、
3 つの`i32`値からできていてもです。それ以外については、タプル構造体のインスタンスは、
タプルと同じように振る舞います：分配して個々の部品にしたり、`.`と添え字を使用して個々の値にアクセスするなどです。

<!--
### Unit-Like Structs without Any Fields
-->

### フィールドのないユニット<ruby>様<rp>(</rp><rt>よう</rt><rp>)</rp></ruby>構造体

<!--
We can also define structs that don’t have any fields! These are called
*unit-like structs* because they behave similarly to `()`, the unit type.
Unit-like structs can be useful in situations in which you need to implement a
trait on some type but you don’t have any data that you want to store in the type
itself. We’ll discuss traits in Chapter 10.
-->

また、一切フィールドのない構造体を定義することもできます！これらは、`()`、ユニット型と似たような振る舞いをすることから、
*ユニット様構造体*と呼ばれます。ユニット様構造体は、ある型にトレイトを実装するけれども、
型自体に保持させるデータは一切ない場面に有効になります。トレイトについては第 10 章で議論します。

<!--
以下のパラグラフでは、引用されてるブロックの後に、和訳を示します。こうしないと、意図通りのレイアウトにならないようです
-->

<!--
### Ownership of Struct Data
In the `User` struct definition in Listing 5-1, we used the owned `String`
type rather than the `&str` string slice type. This is a deliberate choice
because we want instances of this struct to own all of its data and for that
data to be valid for as long as the entire struct is valid.

It’s possible for structs to store references to data owned by something else,
but to do so requires the use of *lifetimes*, a Rust feature that we’ll
discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct
is valid for as long as the struct is. Let’s say you try to store a reference
in a struct without specifying lifetimes, like this, which won't work:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
struct User {
username: &str,
email: &str,
sign_in_count: u64,
active: bool,
}

fn main() {
let user1 = User {
email: "someone@example.com",
username: "someusername123",
active: true,
sign_in_count: 1,
};
}
```

The compiler will complain that it needs lifetime specifiers:

```text
error[E0106]: missing lifetime specifier

|
2 |     username: &str,
|               ^ expected lifetime parameter

error[E0106]: missing lifetime specifier

|
3 |     email: &str,
|            ^ expected lifetime parameter
```

In Chapter 10, we’ll discuss how to fix these errors so you can store
references in structs, but for now, we’ll fix errors like these using owned
types like `String` instead of references like `&str`.
-->

> ### 構造体データの所有権
>
> リスト 5-1 の`User`構造体定義において、`&str`文字列スライス型ではなく、所有権のある`String`型を使用しました。
> これは意図的な選択です。というのも、この構造体のインスタンスには全データを所有してもらう必要があり、
> このデータは、構造体全体が有効な間はずっと有効である必要があるのです。
>
> 構造体に、他の何かに所有されたデータへの参照を保持させることもできますが、
> そうするには*ライフタイム*という第 10 章で議論する Rust の機能を使用しなければなりません。
> ライフタイムのおかげで構造体に参照されたデータが、構造体自体が有効な間、ずっと有効であることを保証してくれるのです。
> ライフタイムを指定せずに構造体に参照を保持させようとしたとしましょう。以下の通りですが、これは動きません：
>
> <span class="filename">ファイル名：src/main.rs</span>
>
> ```rust,ignore
> struct User {
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
>     active: bool,
> }
>
> fn main() {
>     let user1 = User {
>         email: "someone@example.com",
>         username: "someusername123",
>         active: true,
>         sign_in_count: 1,
>     };
> }
> ```
>
> コンパイラは、ライフタイム指定子が必要だと怒るでしょう：
>
> ```text
> error[E0106]: missing lifetime specifier
> (エラー: ライフタイム指定子がありません)
>  -->
>   | 
> 2 |     username: &str,
>   |               ^ expected lifetime parameter
>                    (ライフタイム引数を予期しました)
>
> error[E0106]: missing lifetime specifier
>  -->
>   | 
> 3 |     email: &str,
>   |            ^ expected lifetime parameter
> ```
>
> 第 10 章で、これらのエラーを解消して構造体に参照を保持する方法について議論しますが、
> 当面、今回のようなエラーは、`&str`のような参照の代わりに、`String`のような所有された型を使うことで修正します。
