<!-- ## Defining and Instantiating Structs -->

## 構造体を定義し、インスタンス化する

<!-- Structs are similar to tuples, which were discussed in Chapter 3. Like tuples, -->
<!-- the pieces of a struct can be different types. Unlike tuples, we name each -->
<!-- piece of data so it’s clear what the values mean. As a result of these names, -->
<!-- structs are more flexible than tuples: we don’t have to rely on the order of -->
<!-- the data to specify or access the values of an instance. -->

構造体は第3章で議論したタプルと似ています。タプル同様、構造体の一部を異なる型にできます。
一方タプルとは違って、各データ片には名前をつけるので、値の意味が明確になります。
この名前のおかげで、構造体はタプルに比して、より柔軟になるわけです: データの順番に頼って、
インスタンスの値を指定したり、アクセスしたりする必要がないのです。

<!-- To define a struct, we enter the keyword `struct` and name the entire struct. A -->
<!-- struct’s name should describe the significance of the pieces of data being -->
<!-- grouped together. Then, inside curly braces, we define the names and types of -->
<!-- the pieces of data, which we call *fields*. For example, Listing 5-1 shows a -->
<!-- struct to store information about a user account: -->

構造体の定義は、`struct`キーワードを入れ、構造体全体に名前を付けます。構造体名は、
一つにグループ化されるデータ片の意義を表すものであるべきです。そして、波かっこ内に、
データ片の名前と型を定義し、これは*フィールド*と呼ばれます。例えば、リスト5-1では、
ユーザアカウントに関する情報を保持する構造体を示しています:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

<!-- <span class="caption">Listing 5-1: A `User` struct definition</span> -->

<span class="caption">リスト5-1: `User`構造体定義</span>

<!-- To use a struct after we’ve defined it, we create an *instance* of that struct -->
<!-- by specifying concrete values for each of the fields. We create an instance by -->
<!-- stating the name of the struct, and then add curly braces containing `key: -->
<!-- value` pairs where the keys are the names of the fields and the values are the -->
<!-- data we want to store in those fields. We don’t have to specify the fields in -->
<!-- the same order in which we declared them in the struct. In other words, the -->
<!-- struct definition is like a general template for the type, and instances fill -->
<!-- in that template with particular data to create values of the type. For -->
<!-- example, we can declare a particular user as shown in Listing 5-2: -->

構造体を定義した後に使用するには、各フィールドに対して具体的な値を指定して構造体の*インスタンス*を生成します。
インスタンスは、構造体名を記述し、`key: value`ペアを含む波かっこを付け加えることで生成します。
ここで、キーはフィールド名、値はそのフィールドに格納したいデータになります。フィールドは、
構造体で定義した時通りの順番に指定する必要はありません。換言すると、構造体定義とは、
型に対する一般的な雛形のようなものであり、インスタンスは、その雛形を特定のデータで埋め、その型の値を生成するわけです。
例えば、リスト5-2で示されたように特定のユーザを宣言することができます。

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

<!-- <span class="caption">Listing 5-2: Creating an instance of the `User` -->
<!-- struct</span> -->

<span class="caption">リスト5-2: `User`構造体のインスタンスを生成する</span>

<!-- To get a specific value from a struct, we can use dot notation. If we wanted -->
<!-- just this user’s email address, we can use `user1.email` wherever we want to -->
<!-- use this value. To change a value in a struct, if the instance is mutable, we -->
<!-- can use the dot notation and assign into a particular field. Listing 5-3 shows -->
<!-- how to change the value in the `email` field of a mutable `User` instance: -->

構造体から特定の値を得るには、ドット記法が使えます。このユーザのEメールアドレスだけが欲しいなら、
この値を使いたい場所全部で`user1.email`が使えます。構造体の値を変更するには、インスタンスが可変であれば、
ドット記法を使い特定のフィールドに代入することができます。リスト5-3では、
可変な`User`インスタンスの`email`フィールド値を変更する方法を示しています:

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

<!-- <span class="caption">Listing 5-3: Changing the value in the `email` field of a -->
<!-- `User` instance</span> -->

<span class="caption">リスト5-3: ある`User`インスタンスの`email`フィールド値を変更する</span>

<!-- ### Field Init Shorthand when Variables Have the Same Name as Fields -->

### フィールドと同名の変数があるときのフィールド初期化省略記法

<!-- If you have variables with the same names as struct fields, you can use *field -->
<!-- init shorthand*. This can make functions that create new instances of structs -->
<!-- more concise. First, let’s look at the more verbose way to initialize a struct -->
<!-- instance. The function named `build_user` shown here in Listing 5-4 has -->
<!-- parameters named `email` and `username`. The function creates and returns a -->
<!-- `User` instance: -->

構造体のフィールドと同名の変数がある場合、*フィールド初期化省略記法*を使用することができます。
これにより、構造体の新規インスタンスを生成する関数をより簡潔にすることができます。
まず、構造体インスタンスを初期化する、より冗長的な方法を見てみましょう。リスト5-4で示されている`build_user`という名前の関数には、
`email`と`username`という引数があります。この関数は、`User`インスタンスを生成して返します:

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

<!-- <span class="caption">Listing 5-4: A `build_user` function that takes an email -->
<!-- and username and returns a `User` instance</span> -->

<span class="caption">リスト5-4: Eメールとユーザ名を取り、`User`インスタンスを返す`build_user`関数</span>

<!-- Because the parameter names `email` and `username` are the same as the `User` -->
<!-- struct’s field names `email` and `username`, we can write `build_user` without -->
<!-- the repetition of `email` and `username` as shown in Listing 5-5. This version -->
<!-- of `build_user` behaves the same way as the one in Listing 5-4. The field init -->
<!-- syntax can make cases like this shorter to write, especially when structs have -->
<!-- many fields. -->

`email`と`username`という引数名と、`email`と`username`という`User`構造体のフィールド名が同じなので、
リスト5-5に示したように、`email`と`username`を繰り返すことなく`build_user`を書くことができます。
このバージョンの`build_user`もリスト5-4のものと同じように振る舞います。フィールド初期化記法は、
今回のようなケース(特に構造体に多くのフィールドがあるとき)を短く書けるようにします。

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

<!-- <span class="caption">Listing 5-5: A `build_user` function that uses field init -->
<!-- syntax since the `email` and `username` parameters have the same name as struct -->
<!-- fields</span> -->

<span class="caption">リスト5-5: `email`と`username`引数が構造体のフィールドと同名なので、
フィールド初期化記法を使用する`build_user`関数</span>

<!-- ### Creating Instances From Other Instances With Struct Update Syntax -->

### 構造体更新記法で他のインスタンスからインスタンスを生成する

<!-- It’s often useful to create a new instance from an old instance, using most of -->
<!-- the old instance’s values but changing some. Listing 5-6 shows an example of -->
<!-- creating a new `User` instance in `user2` by setting the values of `email` and -->
<!-- `username` but using the same values for the rest of the fields from the -->
<!-- `user1` instance we created in Listing 5-2: -->

大部分は古いインスタンスの値を使いつつ、変更する箇所もある形で、古いインスタンスから新しいインスタンスを生成できると、
しばしば有用なわけです。リスト5-6では、`email`と`username`の値をセットしつつ、残りのフィールドにはリスト5-2で生成した、
`User1`インスタンスと同じ値を使って、`user2`に新規`User`インスタンスを生成する例を示しました。

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

<!-- <span class="caption">Listing 5-6: Creating a new `User` instance, `user2`, and -->
<!-- setting some fields to the values of the same fields from `user1`</span> -->

<span class="caption">リスト5-6: 新しい`User`インスタンス、`user2`を生成し、
    一部のフィールドを`user1`と同じ値にセットする</span>

<!-- The *struct update syntax* achieves the same effect as the code in Listing 5-6 -->
<!-- using less code. The struct update syntax uses `..` to specify that the -->
<!-- remaining fields not set explicitly should have the same value as the fields in -->
<!-- the given instance. The code in Listing 5-7 also creates an instance in `user2` -->
<!-- that has a different value for `email` and `username` but has the same values -->
<!-- for the `active` and `sign_in_count` fields that `user1` has: -->

*構造体更新記法*は、リスト5-6のコードと同じ効果を達成しつつ、コード量を減らせます。構造体更新記法は、
`..`を使い、明示的にセットされていない残りのフィールドが与えられたインスタンスの値と同じになるように指定します。
リスト5-7のコードも、`email`と`username`の値は異なり、`active`と`sign_in_count`の値は、
`user1`と同じになる`user2`というインスタンスを生成します。

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

<!-- <span class="caption">Listing 5-7: Using struct update syntax to set a new -->
<!-- `email` and `username` values for a `User` instance but use the rest of the -->
<!-- values from the fields of the instance in the `user1` variable</span> -->

<span class="caption">リスト5-7: 構造体更新記法を使用して、新しい`User`インスタンス用の値に新しい`email`と`username`をセットしつつ、
残りの値は、`user1`変数のフィールド値を使う</span>

<!-- ### Tuple Structs without Named Fields to Create Different Types -->

### 異なる型を生成する名前付きフィールドのないタプル構造体

<!-- We can also define structs that look similar to tuples, called *tuple structs*, -->
<!-- that have the added meaning the struct name provides, but don’t have names -->
<!-- associated with their fields, just the types of the fields. The definition of a -->
<!-- tuple struct still starts with the `struct` keyword and the struct name, which -->
<!-- are followed by the types in the tuple. For example, here are definitions and -->
<!-- usages of tuple structs named `Color` and `Point`: -->

構造体名により追加の意味を含むものの、フィールドに紐づけられた名前がなく、フィールドの型だけの*タプル構造体*と呼ばれる、
タプルに似た構造体を定義することもできます。タプル構造体の定義も`struct`キーワードと構造体名から始まり、
タプルに含まれる型が続きます。例として、こちらは、`Color`と`Point`という名前のタプル構造体の定義と使用法です:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

<!-- Note that the `black` and `origin` values are different types, since they’re -->
<!-- instances of different tuple structs. Each struct we define is its own type, -->
<!-- even though the fields within the struct have the same types. Otherwise, tuple -->
<!-- struct instances behave like tuples, which we covered in Chapter 3. -->

`black`と`origin`の値は、違う型であることに注目してください。これらは、異なるタプル構造体のインスタンスだからですね。
定義された各構造体は、構造体内のフィールドが同じ型であっても、それ自身が独自の型になります。
それ以外については、タプル構造体のインスタンスは、第3章で解説したタプルと同じように振る舞います。

<!-- ### Unit-Like Structs without Any Fields -->

### フィールドのないユニット様構造体

<!-- We can also define structs that don’t have any fields! These are called -->
<!-- *unit-like structs* since they behave similarly to `()`, the unit type. -->
<!-- Unit-like structs can be useful in situations such as when you need to -->
<!-- implement a trait on some type, but you don’t have any data that you want to -->
<!-- store in the type itself. We’ll be discussing traits in Chapter 10. -->

また、一切フィールドのない構造体を定義することもできます！これらは、`()`、ユニット型と似たような振る舞いをすることから、
*ユニット様(よう)構造体*と呼ばれます。ユニット様構造体は、ある型にトレイトを実装するけれども、
型自体に保持させるデータは一切ないような場合に有効になります。トレイトについては第10章で議論します。

<!-- 以下のパラグラフでは、引用されてるブロックの後に、和訳を示します。こうしないと、意図通りのレイアウトにならないようです -->

<!-- ### Ownership of Struct Data -->
<!--  In the `User` struct definition in Listing 5-1, we used the owned `String` -->
<!--  type rather than the `&str` string slice type. This is a deliberate choice -->
<!--  because we want instances of this struct to own all of its data and for that -->
<!--  data to be valid for as long as the entire struct is valid. -->
<!--  -->
<!--  It’s possible for structs to store references to data owned by something else, -->
<!--  but to do so requires the use of *lifetimes*, a Rust feature that is discussed -->
<!--  in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid -->
<!--  for as long as the struct is. Let’s say you try to store a reference in a -->
<!--  struct without specifying lifetimes, like this: -->
<!--  -->
<!--  <span class="filename">Filename: src/main.rs</span> -->
<!--  -->
<!--  ```rust,ignore -->
<!--  struct User { -->
<!--      username: &str, -->
<!--      email: &str, -->
<!--      sign_in_count: u64, -->
<!--      active: bool, -->
<!--  } -->
<!--  -->
<!--  fn main() { -->
<!--      let user1 = User { -->
<!--          email: "someone@example.com", -->
<!--          username: "someusername123", -->
<!--          active: true, -->
<!--          sign_in_count: 1, -->
<!--      }; -->
<!--  } -->
<!--  ``` -->
<!--  -->
<!--  The compiler will complain that it needs lifetime specifiers: -->
<!--  -->
<!--  ```text -->
<!--  error[E0106]: missing lifetime specifier -->
<!--   -->
<!-- >   | -->
<!-- > 2 |     username: &str, -->
<!-- >   |               ^ expected lifetime parameter -->
<!-- > -->
<!-- > error[E0106]: missing lifetime specifier -->
<!-- >  -->
<!-- >   | -->
<!-- > 3 |     email: &str, -->
<!-- >   |            ^ expected lifetime parameter -->
<!-- > ``` -->
<!-- > -->
<!-- > We’ll discuss how to fix these errors so you can store references in structs -->
<!-- > in Chapter 10, but for now, we’ll fix errors like these using owned types like -->
<!-- > `String` instead of references like `&str`. -->

> ### 構造体データの所有権
>
> リスト5-1の`User`構造体定義において、`&str`文字列スライス型ではなく、所有権のある`String`型を使用しました。
> これは意図的な選択です。というのも、この構造体のインスタンスには全データを所有してもらう必要があり、
> このデータは、構造体全体が有効な間はずっと有効である必要があるのです。
>
> 構造体に、他の何かに所有されたデータへの参照を保持させることもできますが、
> そうするには*ライフタイム*という第10章で議論されるRustの機能を使用しなければなりません。
> ライフタイムのおかげで構造体に参照されたデータが、構造体自体が有効な間ずっと有効であることを保証してくれるのです。
> ライフタイムを指定せずに構造体に参照を保持させようとしたとしましょう。このように:
>
> <span class="filename">ファイル名: src/main.rs</span>
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
> コンパイラは、ライフタイム指定子が必要だと怒るでしょう:
> ```text
> error[E0106]: missing lifetime specifier
> (エラー: ライフタイム指定子がありません)
>  -->
>   | 
> 2 |     username: &str,
>   |               ^ expected lifetime parameter
>
> error[E0106]: missing lifetime specifier
>  -->
>   | 
> 3 |     email: &str,
>   |            ^ expected lifetime parameter
> ```
>
> これらのエラーを解消して構造体に参照を保持する方法については、第10章で議論しますが、
> 当面、今回のようなエラーは、`&str`のような参照の代わりに、`String`のような所有された型を使うことで解消します。
