<!--
## Separating Modules into Different Files
-->
## モジュールを複数のファイルに分割する

<!--
So far, all the examples in this chapter defined multiple modules in one file.
When modules get large, you might want to move their definitions to a separate
file to make the code easier to navigate.
-->
この章のすべての例において、今までのところ、複数のモジュールを一つのファイルに定義していました。
モジュールが大きくなる時、コードを読み進めやすくするため、それらの定義を別のファイルへ移動させたくなるかもしれません。

<!--
For example, let’s start from the code in Listing 7-17 and move the
`front_of_house` module to its own file *src/front_of_house.rs* by changing the
crate root file so it contains the code shown in Listing 7-21. In this case,
the crate root file is *src/lib.rs*, but this procedure also works with binary
crates whose crate root file is *src/main.rs*.
-->
例えば、Listing 7-17 のコードからはじめましょう。クレートルートのファイルを Listing 7-21 のコードを持つように変更して、`front_of_house`モジュールをそれ専用のファイル`src/front_of_house.rs`に動かしましょう。
今回、クレートルートファイルは`src/lib.rs`ですが、この手続きはクレートルートファイルが`src/main.rs`であるバイナリクレートでもうまく行きます。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-21: Declaring the `front_of_house` module whose
body will be in *src/front_of_house.rs*</span>
-->
<span class="caption">Listing 7-21: `front_of_house`モジュールを宣言する。その中身は`src/front_of_house.rs`内にある</span>

<!--
And *src/front_of_house.rs* gets the definitions from the body of the
`front_of_house` module, as shown in Listing 7-22.
-->
そして、Listing 7-22 のように、*src/front_of_house.rs* には`front_of_house` モジュールの中身の定義を与えます。

<!--
<span class="filename">Filename: src/front_of_house.rs</span>
-->
<span class="filename">ファイル名：src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

<!--
<span class="caption">Listing 7-22: Definitions inside the `front_of_house`
module in *src/front_of_house.rs*</span>
-->
<span class="caption">Listing 7-22: *src/front_of_house.rs*における、`front_of_house`モジュール内部の定義</span>

<!--
Using a semicolon after `mod front_of_house` rather than using a block tells
Rust to load the contents of the module from another file with the same name as
the module. To continue with our example and extract the `hosting` module to
its own file as well, we change *src/front_of_house.rs* to contain only the
declaration of the `hosting` module:
-->
`mod front_of_house`の後にブロックではなくセミコロンを使うと、Rust にモジュールの中身をモジュールと同じ名前をした別のファイルから読み込むように命令します。
私達の例で、つづけて`hosting`モジュールをそれ専用のファイルに抽出するには、`src/front_of_house.rs`が`hosting`モジュールの宣言のみを含むように変更します：

<!--
<span class="filename">Filename: src/front_of_house.rs</span>
-->
<span class="filename">ファイル名：src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

<!--
Then we create a *src/front_of_house* directory and a file
*src/front_of_house/hosting.rs* to contain the definitions made in the
`hosting` module:
-->
さらに*src/front_of_house* ディレクトリと*src/front_of_house/hosting.rs* ファイルを作って、`hosting`モジュール内でなされていた定義を持つようにします。

<!--
<span class="filename">Filename: src/front_of_house/hosting.rs</span>
-->
<span class="filename">ファイル名：src/front_of_house/hosting.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

<!--
The module tree remains the same, and the function calls in `eat_at_restaurant`
will work without any modification, even though the definitions live in
different files. This technique lets you move modules to new files as they grow
in size.
-->
定義は別のファイルにあるにもかかわらず、モジュールツリーは同じままであり、`eat_at_restaurant`内での関数呼び出しもなんの変更もなくうまく行きます。
このテクニックのおかげで、モジュールが大きくなってきた段階で新しいファイルへ動かす、ということができます。

<!--
Note that the `pub use crate::front_of_house::hosting` statement in
*src/lib.rs* also hasn’t changed, nor does `use` have any impact on what files
are compiled as part of the crate. The `mod` keyword declares modules, and Rust
looks in a file with the same name as the module for the code that goes into
that module.
-->
*src/lib.rs* における`pub use crate::front_of_house::hosting` という文も変わっていないし、`use`はどのファイルがクレートの一部としてコンパイルされるかになんの影響も与えないということに注意してください。
`mod`キーワードがモジュールを宣言したなら、Rust はそのモジュールに挿入するためのコードを求めて、モジュールと同じ名前のファイルの中を探すというわけです。

<!--
## Summary
-->
## まとめ

<!--
Rust lets you split a package into multiple crates and a crate into modules
so you can refer to items defined in one module from another module. You can do
this by specifying absolute or relative paths. These paths can be brought into
scope with a `use` statement so you can use a shorter path for multiple uses of
the item in that scope. Module code is private by default, but you can make
definitions public by adding the `pub` keyword.
-->
Rust では、パッケージを複数のクレートに、そしてクレートを複数のモジュールに分割して、あるモジュールで定義された要素を他のモジュールから参照することができます。
これは絶対パスか相対パスを指定することで行なえます。
これらのパスは`use`文でスコープに持ち込むことができ、こうすると、そのスコープで要素を複数回使う時に、より短いパスで済むようになります。
モジュールのコードは標準では非公開ですが、`pub`キーワードを追加することで定義を公開することができます。

<!--
In the next chapter, we’ll look at some collection data structures in the
standard library that you can use in your neatly organized code.
-->
次の章では、きちんと整理されたあなたのコードで使うことができる、標準ライブラリのいくつかのコレクションデータ構造を見ていきます。
