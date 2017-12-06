<!-- ## Referring to Names in Different Modules -->

## 異なるモジュールの名前を参照する

<!-- We’ve covered how to call functions defined within a module using the module -->
<!-- name as part of the call, as in the call to the `nested_modules` function shown -->
<!-- here in Listing 7-7: -->

モジュール名を呼び出しの一部に使用して、モジュール内に定義された関数の呼び出し方法を解説しました。
リスト7-7に示した`nested_modules`関数の呼び出しのような感じですね:

<!-- <span class="filename">Filename: src/main.rs</span> -->

<span class="filename">ファイル名: src/main.rs</span>

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}
```

<!-- <span class="caption">Listing 7-7: Calling a function by fully specifying its -->
<!-- enclosing module’s path</span> -->

<span class="caption">リスト7-7: 囲まれたモジュールをフルパス指定して関数を呼び出す</span>

<!-- As you can see, referring to the fully qualified name can get quite lengthy. -->
<!-- Fortunately, Rust has a keyword to make these calls more concise. -->

見てお分かりの通り、フルパス指定した名前を参照すると非常に長ったらしくなります。
幸い、Rustには、これらの呼び出しをもっと簡潔にするキーワードが用意されています。

<!-- ### Bringing Names into Scope with the `use` keyword -->

### `use`キーワードで名前をスコープに導入する

<!-- Rust’s `use` keyword shortens lengthy function calls by bringing the modules of -->
<!-- the function you want to call into scope. Here’s an example of bringing the -->
<!-- `a::series::of` module into a binary crate’s root scope: -->

Rustの`use`キーワードは、呼び出したい関数のモジュールをスコープに導入することで、
長ったらしい関数呼び出しを短縮します。以下は、
`a::series::of`モジュールをバイナリクレートのルートスコープに持ってくる例です:

<span class="filename">Filename: src/main.rs</span>

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}
```

<!-- The line `use a::series::of;` means that rather than using the full -->
<!-- `a::series::of` path wherever we want to refer to the `of` module, we can use -->
<!-- `of`. -->

`use a::series::of;`の行は、`of`モジュールを参照したい箇所全部でフルパスの`a::series::of`を使用するのではなく、
`of`を利用できることを意味しています。

<!-- The `use` keyword brings only what we’ve specified into scope: it does not -->
<!-- bring children of modules into scope. That’s why we still have to use -->
<!-- `of::nested_modules` when we want to call the `nested_modules` function. -->

この`use`キーワードは、指定したものだけをスコープに入れます: モジュールの子供はスコープに導入しないのです。
そのため、`nested_modules`関数を呼び出したい際に、それでもまだ`of::nested_modules`を使わなければならないのです。

<!-- We could have chosen to bring the function into scope by instead specifying the -->
<!-- function in the `use` as follows: -->

以下のように、代わりに`use`で関数を指定して、関数をスコープに入れることもできました:

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;

fn main() {
    nested_modules();
}
```

<!-- Doing so allows us to exclude all the modules and reference the function -->
<!-- directly. -->

そうすれば、モジュールをすべて取り除き、関数を直接参照することができます。

<!-- Because enums also form a sort of namespace like modules, we can bring an -->
<!-- enum’s variants into scope with `use` as well. For any kind of `use` statement,  -->
<!-- if you're bringing multiple items from one namespace into scope, you can list  -->
<!-- them using curly brackets and commas in the last position, like so: -->

enumもモジュールのようにある種の名前空間をなすので、enumのバリアントを`use`でスコープに導入することもできます。
どんな`use`文に関しても、一つの名前空間から複数の要素をスコープに導入する場合、波かっことお尻にカンマを使用することで列挙できます。
こんな感じで:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
```

<!-- We’re still specifying the `TrafficLight` namespace for the `Green` variant -->
<!-- because we didn’t include `Green` in the `use` statement. -->

`Green`を`use`文に含んでいないので、まだ`Green`バリアント用に`TrafficLight`名前空間を指定しています。

<!-- ### Bringing All Names into Scope with a Glob -->

### Globで全ての名前をスコープに導入する

<!-- To bring all the items in a namespace into scope at once, we can use the `*` syntax, which is called the *glob operator*. This example brings all the variants of an enum into scope without having to list each specifically: -->

ある名前空間の要素を全て一度にスコープに導入するには、`*`表記が使用でき、これはglob(塊)演算子と呼ばれます。
この例は、あるenumのバリアントを各々を列挙せずに全てスコープに導入しています:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
```

<!-- The `*` will bring into scope all the visible items in the `TrafficLight` -->
<!-- namespace. You should use globs sparingly: they are convenient, but this might -->
<!-- also pull in more items than you expected and cause naming conflicts. -->

`*`は`TrafficLight`名前空間に存在する全て公開要素をスコープに導入します。
あまりglobは使用するべきではありません: 便利ではありますが、予想以上の要素を引き込んで、
名前衝突を引き起こす可能性があるのです。

<!-- ### Using `super` to Access a Parent Module -->

### `super`を使用して親モジュールにアクセスする

<!-- As we saw at the beginning of this chapter, when you create a library crate, -->
<!-- Cargo makes a `tests` module for you. Let’s go into more detail about that now. -->
<!-- In your `communicator` project, open *src/lib.rs*: -->

この章の頭で見かけたように、ライブラリクレートを作成する際、Cargoは`tests`モジュールを用意してくれました。
今からそれについて詳しく掘り下げていくことにしましょう。`communicator`プロジェクトで*src/lib.rs*を開いてください:

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

<!-- Chapter 11 explains more about testing, but parts of this example should make -->
<!-- sense now: we have a module named `tests` that lives next to our other modules -->
<!-- and contains one function named `it_works`. Even though there are special -->
<!-- annotations, the `tests` module is just another module! So our module hierarchy -->
<!-- looks like this: -->

第11章でテストについて詳しく説明しますが、これでこの例の一部が持つ意味がわかったのではないでしょうか:
他のモジュールに隣接する`tests`という名前のモジュールがあり、このモジュールは`it_works`という名前の関数を含んでいます。
特別な注釈があるものの、`tests`モジュールもただのモジュールです！よって、モジュール階層は以下のような見た目になります:

```text
communicator
 ├── client
 ├── network
 |   └── client
 └── tests
```

<!-- Tests are for exercising the code within our library, so let’s try to call our -->
<!-- `client::connect` function from this `it_works` function, even though we won’t -->
<!-- be checking any functionality right now. This won't work yet: -->

テストは、ライブラリ内でコードの準備運動を行うためのものなので、この`it_works`関数から`client::connect`関数を呼び出してみましょう。
まあ、尤も今のところ、機能の検査は何もしないんですけどね。これはまだ動きません:

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        client::connect();
    }
}
```

<!-- Run the tests by invoking the `cargo test` command: -->

`cargo test`コマンドを呼び出してテストを実行してください:

```text
$ cargo test
   Compiling communicator v0.1.0 (file:///projects/communicator)
error[E0433]: failed to resolve. Use of undeclared type or module `client`
(エラー: 解決に失敗しました。未定義の型、またはモジュール`client`を使用しています)
 --> src/lib.rs:9:9
  |
9 |         client::connect();
  |         ^^^^^^ Use of undeclared type or module `client`
```

<!-- The compilation failed, but why? We don’t need to place `communicator::` in -->
<!-- front of the function like we did in *src/main.rs* because we are definitely -->
<!-- within the `communicator` library crate here. The reason is that paths are -->
<!-- always relative to the current module, which here is `tests`. The only -->
<!-- exception is in a `use` statement, where paths are relative to the crate root -->
<!-- by default. Our `tests` module needs the `client` module in its scope! -->

コンパイルが失敗しましたが、なぜでしょうか？*src/main.rs*のように、関数の直前に`communicator::`を配置する必要はありません。
なぜなら、間違いなくここでは、`communicator`ライブラリクレート内にいるからです。
原因は、パスが常に現在のモジュールに対して相対的になり、ここでは`tests`になっているからです。
唯一の例外は、`use`文内であり、パスは標準でクレートのルートに相対的になります。
`tests`モジュールは、`client`モジュールがスコープに存在する必要があるのです！

<!-- So how do we get back up one module in the module hierarchy to call the -->
<!-- `client::connect` function in the `tests` module? In the `tests` module, we can -->
<!-- either use leading colons to let Rust know that we want to start from the root -->
<!-- and list the whole path, like this: -->

では、どうやってモジュール階層を一つ上がり、`tests`モジュールの`client::connect`関数を呼び出すのでしょうか？
`tests`モジュールにおいて、先頭にコロンを使用して、コンパイラにルートから始めて、フルパスを列挙したいと知らせることもできます。
こんな感じで:

```rust,ignore
::client::connect();
```

<!-- Or, we can use `super` to move up one module in the hierarchy from our current -->
<!-- module, like this: -->

あるいは、`super`を使用して現在のモジュールからモジュール階層を一つ上がることもできます。
以下のように:

```rust,ignore
super::client::connect();
```

<!-- These two options don’t look that different in this example, but if you’re -->
<!-- deeper in a module hierarchy, starting from the root every time would make your -->
<!-- code lengthy. In those cases, using `super` to get from the current module to -->
<!-- sibling modules is a good shortcut. Plus, if you’ve specified the path from the -->
<!-- root in many places in your code and then you rearrange your modules by moving -->
<!-- a subtree to another place, you’d end up needing to update the path in several -->
<!-- places, which would be tedious. -->

この例では、これら二つの選択はそれほど異なるようには見えませんが、モジュール階層がもっと深ければ、
常にルートから書き始めるのは、コードを冗長にする原因になります。そのような場合、
`super`を使用して現在のモジュールから兄弟のモジュールに辿り着くのは、いいショートカットになります。
さらに、コードのいろんなところでルートからパスを指定し、モジュール構造を変化させた場合、
複数箇所でパスを更新する必要が出てきて、面倒なことになるでしょう。

<!-- It would also be annoying to have to type `super::` in each test, but you’ve -->
<!-- already seen the tool for that solution: `use`! The `super::` functionality -->
<!-- changes the path you give to `use` so it is relative to the parent module -->
<!-- instead of to the root module. -->

各テストで`super::`と入力しなければならないのも不快なことですが、それを解決してくれる道具をもう見かけています:
`use`です！`super::`の機能は、`use`に与えるパスを変更するので、ルートモジュールではなく、
親モジュールに対して相対的になります。

<!-- For these reasons, in the `tests` module especially, `use super::something` is -->
<!-- usually the best solution. So now our test looks like this: -->

このような理由から、ことに`tests`モジュールにおいて`use super::somthing`は通常、
最善策になるわけです。故に、今ではテストはこんな見た目になりました:

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
```

<!-- When we run `cargo test` again, the test will pass and the first part of the -->
<!-- test result output will be the following: -->

再度`cargo test`を実行すると、テストは通り、テスト結果出力の最初の部分は以下のようになるでしょう:

```text
$ cargo test
   Compiling communicator v0.1.0 (file:///projects/communicator)
     Running target/debug/communicator-92007ddb5330fa5a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

<!-- ## Summary -->

## まとめ

<!-- Now you know some new techniques for organizing your code! Use these techniques -->
<!-- to group related functionality together, keep files from becoming too long, and -->
<!-- present a tidy public API to your library users. -->

これでコードを体系化する新しいテクニックを知りましたね！これらのテクニックを使用して、
関連のある機能をまとめ上げ、ファイルが長くなりすぎるのを防ぎ、ライブラリの使用者に整理整頓された公開APIを提供してください。

<!-- Next, we’ll look at some collection data structures in the standard library -->
<!-- that you can use in your nice, neat code! -->

次は、自分の素晴らしく綺麗なコードで使用できる標準ライブラリのコレクションデータ構造について見ていきましょう！
