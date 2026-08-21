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
For example, let’s start from the code in Listing 7-17 that had multiple
restaurant modules. We’ll extract modules into files instead of having all the
modules defined in the crate root file. In this case, the crate root file is
*src/lib.rs*, but this procedure also works with binary crates whose crate root
file is *src/main.rs*.
-->
例として、複数のレストランモジュールを持つリスト7-17のコードからはじめましょう。
すべてのモジュールをクレートルートファイルで定義するのをやめて、複数のファイルにモジュールを抽出することにします。
今回、クレートルートファイルは`src/lib.rs`ですが、この手続きはクレートルートファイルが`src/main.rs`であるバイナリクレートでもうまく行きます。

<!--
First, we’ll extract the `front_of_house` module to its own file. Remove the
code inside the curly brackets for the `front_of_house` module, leaving only
the `mod front_of_house;` declaration, so that *src/lib.rs* contains the code
shown in Listing 7-21. Note that this won’t compile until we create the
*src/front_of_house.rs* file in Listing 7-22.
-->
まず、`front_of_house`モジュールをそれ専用のファイルに抽出しましょう。
`front_of_house`モジュールの波かっこの中のコードを削除し、`mod front_of_house;`宣言だけを残して、 *src/lib.rs* がリスト7-21に示すコードを含むようにしてください。
リスト7-22で *src/front_of_house.rs* ファイルを作成するまで、このコードはコンパイルできないことに注意してください。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-21: Declaring the `front_of_house` module whose
body will be in *src/front_of_house.rs*</span>
-->
<span class="caption">リスト7-21: `front_of_house`モジュールを宣言する。その中身は`src/front_of_house.rs`内にある</span>

<!--
Next, place the code that was in the curly brackets into a new file named
*src/front_of_house.rs*, as shown in Listing 7-22. The compiler knows to look
in this file because it came across the module declaration in the crate root
with the name `front_of_house`.
-->
次に、リスト7-22に示すように、*src/front_of_house.rs* という名前の新しいファイルに、波かっこの中にあったコードを配置してください。
コンパイラは、クレートルートで`front_of_house`という名前のモジュール宣言を見つけたときは、このファイルを探せばいいということを知っています。

<!--
<span class="filename">Filename: src/front_of_house.rs</span>
-->
<span class="filename">ファイル名: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

<!--
<span class="caption">Listing 7-22: Definitions inside the `front_of_house`
module in *src/front_of_house.rs*</span>
-->
<span class="caption">リスト7-22: *src/front_of_house.rs*における、`front_of_house`モジュール内部の定義</span>

<!--
Note that you only need to load a file using a `mod` declaration *once* in your
module tree. Once the compiler knows the file is part of the project (and knows
where in the module tree the code resides because of where you’ve put the `mod`
statement), other files in your project should refer to the loaded file’s code
using a path to where it was declared, as covered in the [“Paths for Referring
to an Item in the Module Tree”][paths] section. In other words,
`mod` is *not* an “include” operation that you may have seen in other
programming languages.
-->
`mod`宣言を使用したファイルのロードは、モジュールツリー内で*一回*のみ行う必要があることに注意してください。
一度コンパイラが、そのファイルがプロジェクトの一部であることを認識したら（そして、`mod`文を書いた場所に応じてモジュールツリー内のどこにコードがあることになるかを認識したら）、[「モジュールツリーの要素を示すためのパス」][paths]節で扱ったように、プロジェクト内の他のファイルは、モジュールが宣言された場所へのパスを使用してロードされたファイルのコードを参照するべきです。
別の言い方をすれば、`mod`は他のプログラミング言語で見られるような“include”操作では*ありません*。

<!--
Next, we’ll extract the `hosting` module to its own file. The process is a bit
different because `hosting` is a child module of `front_of_house`, not of the
root module. We’ll place the file for `hosting` in a new directory that will be
named for its ancestors in the module tree, in this case *src/front_of_house/*.
-->
つづけて`hosting`モジュールをそれ専用のファイルに抽出します。
`hosting`はルートモジュールの子ではなく`front_of_house`の子モジュールなので、このプロセスは少し異なります。
`hosting`のためのファイルを、モジュールツリー上での祖先に対応して名付けられた新しいディレクトリ（今回の場合は *src/front_of_house/*）内に配置しましょう。

<!--
To start moving `hosting`, we change *src/front_of_house.rs* to contain only the
declaration of the `hosting` module:
-->
`hosting`の移動を開始するために、`src/front_of_house.rs`が`hosting`モジュールの宣言のみを含むように変更します：

<!--
<span class="filename">Filename: src/front_of_house.rs</span>
-->
<span class="filename">ファイル名: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

<!--
Then we create a *src/front_of_house* directory and a file *hosting.rs* to
contain the definitions made in the `hosting` module:
-->
さらに*src/front_of_house* ディレクトリと*hosting.rs* ファイルを作って、`hosting`モジュール内でなされていた定義を持つようにします。

<!--
<span class="filename">Filename: src/front_of_house/hosting.rs</span>
-->
<span class="filename">ファイル名: src/front_of_house/hosting.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

<!--
If we instead put *hosting.rs* in the *src* directory, the compiler would
expect the *hosting.rs* code to be in a `hosting` module declared in the crate
root, and not declared as a child of the `front_of_house` module. The
compiler’s rules for which files to check for which modules’ code means the
directories and files more closely match the module tree.
-->
もし*src*ディレクトリ内に*hosting.rs*を置いた場合は、コンパイラは*hosting.rs*のコードは`front_of_house`モジュールの子としてではなく、クレートルート内で宣言された`hosting`モジュールにあると期待するでしょう。
コンパイラがどのモジュールのコードのためにどのファイルをチェックするかの規則は、ディレクトリとファイルがモジュールツリーと密接に一致することを意味します。

<!--
> ### Alternate File Paths
>
> So far we’ve covered the most idiomatic file paths the Rust compiler uses,
> but Rust also supports an older style of file path. For a module named
> `front_of_house` declared in the crate root, the compiler will look for the
> module’s code in:
>
> * *src/front_of_house.rs* (what we covered)
> * *src/front_of_house/mod.rs* (older style, still supported path)
>
> For a module named `hosting` that is a submodule of `front_of_house`, the
> compiler will look for the module’s code in:
>
> * *src/front_of_house/hosting.rs* (what we covered)
> * *src/front_of_house/hosting/mod.rs* (older style, still supported path)
>
> If you use both styles for the same module, you’ll get a compiler error. Using
> a mix of both styles for different modules in the same project is allowed, but
> might be confusing for people navigating your project.
>
> The main downside to the style that uses files named *mod.rs* is that your
> project can end up with many files named *mod.rs*, which can get confusing
> when you have them open in your editor at the same time.
-->
> ### 別のファイルパス
>
> ここまでRustコンパイラが使用するもっとも慣例的なファイルパスについて扱ってきましたが、Rustは古いスタイルのファイルパスもサポートしています。
> クレートルート内で宣言された`front_of_house`という名前のモジュールに対して、コンパイラは以下の場所からコードを探します:
>
> * *src/front_of_house.rs* （ここまで扱ってきたもの）
> * *src/front_of_house/mod.rs* （古いスタイルの、今もサポートされているパス）
>
> `front_of_house`のサブモジュールである、`hosting`という名前のモジュールに対しては、コンパイラは以下の場所からコードを探します:
>
> * *src/front_of_house/hosting.rs* （ここまで扱ってきたもの）
> * *src/front_of_house/hosting/mod.rs* （古いスタイルの、今もサポートされているパス）
>
> 同一のモジュールに対して両方のスタイルを使用するとコンパイルエラーになります。
> 異なるモジュールに対して両方のスタイルを混ぜて使用することは許可されていますが、プロジェクトを見て回る人たちにとって混乱を招くかもしれません。
>
> *mod.rs* という名前のファイルを使用するスタイルの主な欠点は、プロジェクト内に *mod.rs* という名前のファイルが大量にできることになり、エディタで同時に開いたときに混乱を招きやすいことです。

<!--
We’ve moved each module’s code to a separate file, and the module tree remains
the same. The function calls in `eat_at_restaurant` will work without any
modification, even though the definitions live in different files. This
technique lets you move modules to new files as they grow in size.
-->
各モジュールのコードを独立したファイルに移動しましたが、モジュールツリーは同じままです。
定義が別のファイルにあるにもかかわらず、`eat_at_restaurant`内での関数呼び出しもなんの変更もなくうまく行きます。
このテクニックのおかげで、モジュールが大きくなってきた段階で新しいファイルへ動かす、ということができます。

<!--
Note that the `pub use crate::front_of_house::hosting` statement in
*src/lib.rs* also hasn’t changed, nor does `use` have any impact on what files
are compiled as part of the crate. The `mod` keyword declares modules, and Rust
looks in a file with the same name as the module for the code that goes into
that module.
-->
*src/lib.rs* における`pub use crate::front_of_house::hosting` という文も変わっていないし、`use`はどのファイルがクレートの一部としてコンパイルされるかになんの影響も与えないということに注意してください。
`mod`キーワードがモジュールを宣言したなら、Rustはそのモジュールに挿入するためのコードを求めて、モジュールと同じ名前のファイルの中を探すというわけです。

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
Rustでは、パッケージを複数のクレートに、そしてクレートを複数のモジュールに分割して、あるモジュールで定義された要素を他のモジュールから参照することができます。
これは絶対パスか相対パスを指定することで行なえます。
これらのパスは`use`文でスコープに持ち込むことができ、こうすると、そのスコープで要素を複数回使う時に、より短いパスで済むようになります。
モジュールのコードは標準では非公開ですが、`pub`キーワードを追加することで定義を公開することができます。

<!--
In the next chapter, we’ll look at some collection data structures in the
standard library that you can use in your neatly organized code.
-->
次の章では、きちんと整理されたあなたのコードで使うことができる、標準ライブラリのいくつかのコレクションデータ構造を見ていきます。

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
