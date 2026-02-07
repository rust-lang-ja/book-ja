<!--
## Publishing a Crate to Crates.io
-->

## Crates.ioにクレートを公開する

<!--
We’ve used packages from [crates.io](https://crates.io) as
dependencies of our project, but you can also share your code with other people
by publishing your own packages. The crate registry at
[crates.io](https://crates.io) distributes the source code of
your packages, so it primarily hosts code that is open source.
-->

プロジェクトの依存として[crates.io](https://crates.io)のパッケージを使用しましたが、
自分のパッケージを公開することで他の人とコードを共有することもできます。
[crates.io](https://crates.io)のクレート登録所は、自分のパッケージのソースコードを配布するので、
主にオープンソースのコードをホストします。

<!--
Rust and Cargo have features that help make your published package easier for
people to use and to find in the first place. We’ll talk about some of these
features next and then explain how to publish a package.
-->

RustとCargoは、公開したパッケージを人が使用し、そもそも見つけやすくしてくれる機能を有しています。
これらの機能の一部を次に語り、そして、パッケージの公開方法を説明します。

<!--
### Making Useful Documentation Comments
-->

### 役に立つドキュメンテーションコメントを行う

<!--
Accurately documenting your packages will help other users know how and when to
use them, so it’s worth investing the time to write documentation. In Chapter
3, we discussed how to comment Rust code using two slashes, `//`. Rust also has
a paticular kind of comment for documentation, known conveniently as
a *documentation comment*, that will generate HTML documentation. The HTML
displays the contents of documentation comments for public API items intended
for programmers interested in knowing how to *use* your crate as opposed to how
your crate is *implemented*.
-->

パッケージを正確にドキュメントすることで、他のユーザがパッケージを使用する方法や、いつ使用すべきかを理解する手助けをすることになるので、
ドキュメンテーションを書くことに時間を費やす価値があります。第3章で、2連スラッシュ、`//`でRustのコードにコメントをつける方法を議論しました。
Rustには、ドキュメンテーション用のコメントも用意されていて、便利なことに*ドキュメンテーションコメント*として知られ、
HTMLドキュメントを生成します。クレートの*実装*法とは対照的にクレートの*使用*法を知ることに興味のあるプログラマ向けの、
公開API用のドキュメンテーションコメントの中身をこのHTMLは表示します。

<!--
Documentation comments use three slashes, `///`, instead of two and support
Markdown notation for formatting the text. Place documentation comments just
before the item they’re documenting. Listing 14-1 shows documentation comments
for an `add_one` function in a crate named `my_crate`:
-->

ドキュメンテーションコメントは、2つではなく、3連スラッシュ、`///`を使用し、テキストを整形するMarkdown記法もサポートしています。
ドキュメント対象の要素の直前にドキュメンテーションコメントを配置してください。
リスト14-1は、`my_crate`という名のクレートの`add_one`関数用のドキュメンテーションコメントを示しています:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
/// Adds one to the number given.
/// 与えられた数値に1を足す。
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

<!--
<span class="caption">Listing 14-1: A documentation comment for a
function</span>
-->

<span class="caption">リスト14-1: 関数のドキュメンテーションコメント</span>

<!--
Here, we give a description of what the `add_one` function does, start a
section with the heading `Examples`, and then provide code that demonstrates
how to use the `add_one` function. We can generate the HTML documentation from
this documentation comment by running `cargo doc`. This command runs the
`rustdoc` tool distributed with Rust and puts the generated HTML documentation
in the *target/doc* directory.
-->

ここで、`add_one`関数がすることの説明を与え、`Examples`というタイトルでセクションを開始し、
`add_one`関数の使用法を模擬するコードを提供しています。このドキュメンテーションコメントから`cargo doc`を実行することで、
HTMLドキュメントを生成することができます。このコマンドはコンパイラとともに配布されている`rustdoc`ツールを実行し、
生成されたHTMLドキュメントを*target/doc*ディレクトリに配置します。

<!--
For convenience, running `cargo doc --open` will build the HTML for your
current crate’s documentation (as well as the documentation for all of your
crate’s dependencies) and open the result in a web browser. Navigate to the
`add_one` function and you’ll see how the text in the documentation comments is
rendered, as shown in Figure 14-1:
-->

利便性のために、`cargo doc --open`を走らせれば、現在のクレートのドキュメント用のHTML(と、
自分のクレートが依存している全てのドキュメント)を構築し、その結果をWebブラウザで開きます。
`add_one`関数まで下り、図14-1に示したように、ドキュメンテーションコメントのテキストがどう描画されるかを確認しましょう:

<!--
<img alt="Rendered HTML documentation for the `add_one` function of `my_crate`" src="img/trpl14-01.png" class="center" />
-->

<img alt="`my_crate`の`add_one`関数の描画済みのHTMLドキュメント" src="img/trpl14-01.png" class="center" />

<!--
<span class="caption">Figure 14-1: HTML documentation for the `add_one`
function</span>
-->

<span class="caption">図14-1: `add_one`関数のHTMLドキュメント</span>

<!--
#### Commonly Used Sections
-->

#### よく使われるセクション

<!--
We used the `# Examples` Markdown heading in Listing 14-1 to create a section
in the HTML with the title “Examples.” Here are some other sections that crate
authors commonly use in their documentation:
-->

`# Examples`マークダウンのタイトルをリスト14-1で使用し、「例」というタイトルのセクションをHTMLに生成しました。
こちらがこれ以外にドキュメントでよくクレート筆者が使用するセクションです:

<!--
* **Panics**: The scenarios in which the function being documented could
`panic!`. Callers of the function who don’t want their programs to panic should
make sure they don’t call the function in these situations.
* **Errors**: If the function returns a `Result`, describing the kinds of
errors that might occur and what conditions might cause those errors to be
returned can be helpful to callers so they can write code to handle the
different kinds of errors in different ways.
* **Safety**: If the function is `unsafe` to call (we discuss unsafety in
Chapter 19), there should be a section explaining why the function is unsafe
and covering the invariants that the function expects callers to uphold.
-->

* **Panics**: ドキュメント対象の関数が`panic!`する可能性のある筋書きです。プログラムをパニックさせたくない関数の使用者は、
これらの状況で関数が呼ばれないことを確かめる必要があります。
* **Errors**: 関数が`Result`を返すなら、起きうるエラーの種類とどんな条件がそれらのエラーを引き起こす可能性があるのか解説すると、
呼び出し側の役に立つので、エラーの種類によって処理するコードを変えて書くことができます。
* **Safety**: 関数が呼び出すのに`unsafe`(unsafeについては第19章で議論します)なら、
関数がunsafeな理由を説明し、関数が呼び出し元に保持していると期待する不変条件を講義するセクションがあるべきです。

<!--
Most documentation comments don’t need all of these sections, but this is a
good checklist to remind you of the aspects of your code that people calling
your code will be interested in knowing about.
-->

多くのドキュメンテーションコメントでは、これら全てのセクションが必要になることはありませんが、
これは自分のコードを呼び出している人が知りたいと思うコードの方向性を思い出させてくれるいいチェックリストになります。

<!--
#### Documentation Comments as Tests
-->

#### テストとしてのドキュメンテーションコメント

<!--
Adding example code blocks in your documentation comments can help demonstrate
how to use your library, and doing so has an additional bonus: running `cargo
test` will run the code examples in your documentation as tests! Nothing is
better than documentation with examples. But nothing is worse than examples
that don’t work because the code has changed since the documentation was
written. If we run `cargo test` with the documentation for the `add_one`
function from Listing 14-1, we will see a section in the test results like this:
-->

ドキュメンテーションコメントに例のコードブロックを追加すると、ライブラリの使用方法のデモに役立ち、
おまけもついてきます: `cargo test`を走らせると、ドキュメントのコード例をテストとして実行するのです！
例付きのドキュメントに上回るものはありません。しかし、ドキュメントが書かれてからコードが変更されたがために、
動かない例がついているよりも悪いものもありません。リスト14-1から`add_one`関数のドキュメンテーションとともに、
`cargo test`を走らせたら、テスト結果に以下のような区域が見られます:

```text
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

<!--
2行目最後、catch that S Vの用法は辞書に載っていなかった。Oxfordには載っているのかもしれません
-->

<!--
Now if we change either the function or the example so the `assert_eq!` in the
example panics and run `cargo test` again, we’ll see that the doc tests catch
that the example and the code are out of sync from one another!
-->

さて、例の`assert_eq!`がパニックするように、関数か例を変更し、再度`cargo test`を実行したら、
docテストが、例とコードがお互いに同期されていないことを捕捉するところを目撃するでしょう！

<!--
#### Commenting Contained Items
-->

#### 含まれている要素にコメントする

<!--
Another style of doc comment, `//!`, adds documentation to the item that
contains the comments rather than adding documentation to the items following
the comments. We typically use these doc comments inside the crate root file
(*src/lib.rs* by convention) or inside a module to document the crate or the
module as a whole.
-->

docコメントの別スタイル、`//!`は、コメントに続く要素にドキュメンテーションを付け加えるのではなく、
コメントを含む要素にドキュメンテーションを付け加えます。典型的には、クレートのルートファイル(慣例的には、*src/lib.rs*)内部や、
モジュールの内部で使用して、クレートやモジュール全体にドキュメントをつけます。

<!--
For example, if we want to add documentation that describes the purpose of the
`my_crate` crate that contains the `add_one` function, we can add documentation
comments that start with `//!` to the beginning of the *src/lib.rs* file, as
shown in Listing 14-2:
-->

例えば、`add_one`関数を含む`my_crate`クレートの目的を解説するドキュメンテーションを追加したいのなら、
`//!`で始まるドキュメンテーションコメントを*src/lib.rs*ファイルの先頭につけることができます。
リスト14-2に示したようにですね:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

//! #自分のクレート
//!
//! `my_crate`は、ユーティリティの集まりであり、特定の計算をより便利に行うことができます。

/// Adds one to the number given.
// --snip--
```

<!--
<span class="caption">Listing 14-2: Documentation for the `my_crate` crate as a
whole</span>
-->

<span class="caption">リスト14-2: 全体として`my_crate`クレートにドキュメントをつける</span>

<!--
Notice there isn’t any code after the last line that begins with `//!`. Because
we started the comments with `//!` instead of `///`, we’re documenting the item
that contains this comment rather than an item that follows this comment. In
this case, the item that contains this comment is the *src/lib.rs* file, which
is the crate root. These comments describe the entire crate.
-->

`//!`で始まる最後の行のあとにコードがないことに注目してください。`///`ではなく、`//!`でコメントを開始しているので、
このコメントに続く要素ではなく、このコメントを含む要素にドキュメントをつけているわけです。
今回の場合、このコメントを含む要素は*src/lib.rs*ファイルであり、クレートのルートです。
これらのコメントは、クレート全体を解説しています。

<!--
When we run `cargo doc --open`, these comments will display on the front
page of the documentation for `my_crate` above the list of public items in the
crate, as shown in Figure 14-2:
-->

`cargo doc --open`を実行すると、これらのコメントは、`my_crate`のドキュメントの最初のページ、
クレートの公開要素のリストの上部に表示されます。図14-2のようにですね:

<!--
<img alt="Rendered HTML documentation with a comment for the crate as a whole" src="img/trpl14-02.png" class="center" />
-->

<img alt="クレート全体のコメント付きの描画済みHTMLドキュメンテーション" src="img/trpl14-02.png" class="center" />

<!--
<span class="caption">Figure 14-2: Rendered documentation for `my_crate`,
including the comment describing the crate as a whole</span>
-->

<span class="caption">図14-2: クレート全体を解説するコメントを含む`my_crate`の描画されたドキュメンテーション</span>

<!--
Documentation comments within items are useful for describing crates and
modules especially. Use them to explain the overall purpose of the container to
help your crate users understand the crate’s organization.
-->

要素内のドキュメンテーションコメントは、特にクレートやモジュールを解説するのに有用です。
コンテナの全体の目的を説明し、クレートの使用者がクレートの体系を理解する手助けをするのに使用してください。

<!--
### Exporting a Convenient Public API with `pub use`
-->

### `pub use`で便利な公開APIをエクスポートする

<!--
In Chapter 7, we covered how to organize our code into modules using the `mod`
keyword, how to make items public using the `pub` keyword, and how to bring
items into a scope with the `use` keyword. However, the structure that makes
sense to you while you’re developing a crate might not be very convenient for
your users. You might want to organize your structs in a hierarchy containing
multiple levels, but then people who want to use a type you’ve defined deep in
the hierarchy might have trouble finding out that type exists. They might also
be annoyed at having to enter `use`
`my_crate::some_module::another_module::UsefulType;` rather than `use`
`my_crate::UsefulType;`.
-->

第7章において、`mod`キーワードを使用してモジュールにコードを体系化する方法、`pub`キーワードで要素を公開にする方法、
`use`キーワードで要素をスコープに導入する方法について講義しました。しかしながら、クレートの開発中に、
自分にとって意味のある構造は、ユーザにはあまり便利ではない可能性があります。複数階層を含む階層で、
自分の構造体を体系化したくなるかもしれませんが、それから階層の深いところで定義した型を使用したい人は、
型が存在することを見つけ出すのに困難を伴う可能性もあります。また、そのような人は、
`use my_crate::UsefulType`の代わりに`use my_crate::some_module::another_module::UsefulType;`と入力するのを煩わしく感じる可能性もあります。

<!--
The structure of your public API is a major consideration when publishing a
crate. People who use your crate are less familiar with the structure than you
are and might have difficulty finding the pieces they want to use if your crate
has a large module hierarchy.
-->

自分の公開APIの構造は、クレートを公開する際に考慮すべき点です。自分のクレートを使用したい人は、
自分よりもその構造に馴染みがないですし、クレートのモジュール階層が大きければ、使用したい部分を見つけるのが困難になる可能性があります。

<!--
The good news is that if the structure *isn’t* convenient for others to use
from another library, you don’t have to rearrange your internal organization:
instead, you can re-export items to make a public structure that’s different
than your private structure by using `pub use`. Re-exporting takes a public
item in one location and makes it public in another location, as if it were
defined in the other location instead.
-->

嬉しいお知らせは、構造が他人が他のライブラリから使用するのに便利では*ない*場合、内部的な体系を再構築する必要はないということです:
代わりに、要素を再エクスポートし、`pub use`で自分の非公開構造とは異なる公開構造にできます。
再エクスポートは、ある場所の公開要素を一つ取り、別の場所で定義されているかのように別の場所で公開します。

<!--
For example, say we made a library named `art` for modeling artistic concepts.
Within this library are two modules: a `kinds` module containing two enums
named `PrimaryColor` and `SecondaryColor` and a `utils` module containing a
function named `mix`, as shown in Listing 14-3:
-->

例えば、芸術的な概念をモデル化するために`art`という名のライブラリを作ったとしましょう。
このライブラリ内には、2つのモジュールがあります: `PrimaryColor`と`SecondaryColor`という名前の2つのenumを含む、
`kinds`モジュールと`mix`という関数を含む`utils`モジュールです。リスト14-3のようにですね:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
//! # Art
//!
//! A library for modeling artistic concepts.
//! #芸術
//!
//! 芸術的な概念をモデル化するライブラリ。

pub mod kinds {
    /// The primary colors according to the RYB color model.
    /// RYBカラーモデルによる主色
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    /// RYBカラーモデルによる副色
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    ///2つの主色を同じ割合で混合し、副色にする
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}
```

<!--
<span class="caption">Listing 14-3: An `art` library with items organized into
`kinds` and `utils` modules</span>
-->

<span class="caption">リスト14-3: `kinds`と`utils`モジュールに体系化される要素を含む`art`ライブラリ</span>

<!--
Figure 14-3 shows what the front page of the documentation for this crate
generated by `cargo doc` would look like:
-->

図14-3は、`cargo doc`により生成されるこのクレートのドキュメンテーションの最初のページがどんな見た目になるか示しています:

<!--
<img alt="Rendered documentation for the `art` crate that lists the `kinds` and `utils` modules" src="img/trpl14-03.png" class="center" />
-->

<img alt="`kinds`と`utils`モジュールを列挙する`art`クレートの描画されたドキュメンテーション" src="img/trpl14-03.png" class="center" />

<!--
<span class="caption">Figure 14-3: Front page of the documentation for `art`
that lists the `kinds` and `utils` modules</span>
-->

<span class="caption">図14-3: `kinds`と`utils`モジュールを列挙する`art`のドキュメンテーションのトップページ</span>

<!--
Note that the `PrimaryColor` and `SecondaryColor` types aren’t listed on the
front page, nor is the `mix` function. We have to click `kinds` and `utils` to
see them.
-->

`PrimaryColor`型も`SecondaryColor`型も、`mix`関数もトップページには列挙されていないことに注意してください。
`kinds`と`utils`をクリックしなければ、参照することができません。

<!--
Another crate that depends on this library would need `use` statements that
import the items from `art`, specifying the module structure that’s currently
defined. Listing 14-4 shows an example of a crate that uses the `PrimaryColor`
and `mix` items from the `art` crate:
-->

このライブラリに依存する別のクレートは、現在定義されているモジュール構造を指定して、
`art`の要素をインポートする`use`文が必要になるでしょう。リスト14-4は、
`art`クレートから`PrimaryColor`と`mix`要素を使用するクレートの例を示しています:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
extern crate art;

use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

<!--
<span class="caption">Listing 14-4: A crate using the `art` crate’s items with
its internal structure exported</span>
-->

<span class="caption">リスト14-4: 内部構造がエクスポートされて`art`クレートの要素を使用するクレート</span>

<!--
The author of the code in Listing 14-4, which uses the `art` crate, had to
figure out that `PrimaryColor` is in the `kinds` module and `mix` is in the
`utils` module. The module structure of the `art` crate is more relevant to
developers working on the `art` crate than to developers using the `art` crate.
The internal structure that organizes parts of the crate into the `kinds`
module and the `utils` module doesn’t contain any useful information for
someone trying to understand how to use the `art` crate. Instead, the `art`
crate’s module structure causes confusion because developers have to figure out
where to look, and the structure is inconvenient because developers must
specify the module names in the `use` statements.
-->

リスト14-4は`art`クレートを使用していますが、このコードの筆者は、`PrimaryColor`が`kinds`モジュールにあり、
`mix`が`utils`モジュールにあることを理解しなければなりませんでした。`art`クレートのモジュール構造は、
`art`クレートの使用者よりも、`art`クレートに取り組む開発者などに関係が深いです。
クレートの一部を`kinds`モジュールと`utils`モジュールに体系化する内部構造は、`art`クレートの使用方法を理解しようとする人には、
何も役に立つ情報を含んでいません。代わりに、開発者がどこを見るべきか計算する必要があるので、
`art`クレートのモジュール構造は混乱を招き、また、開発者はモジュール名を`use`文で指定しなければならないので、
この構造は不便です。

<!--
To remove the internal organization from the public API, we can modify the
`art` crate code in Listing 14-3 to add `pub use` statements to re-export the
items at the top level, as shown in Listing 14-5:
-->

公開APIから内部体系を除去するために、リスト14-3の`art`クレートコードを変更し、`pub use`文を追加して、
最上位で要素を再エクスポートすることができます。リスト14-5みたいにですね:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
//! # Art
//!
//! A library for modeling artistic concepts.

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}
```

<!--
<span class="caption">Listing 14-5: Adding `pub use` statements to re-export
items</span>
-->

<span class="caption">リスト14-5: `pub use`文を追加して要素を再エクスポートする</span>

<!--
The API documentation that `cargo doc` generates for this crate will now list
and link re-exports on the front page, as shown in Figure 14-4, making the
`PrimaryColor` and `SecondaryColor` types and the `mix` function easier to find.
-->

このクレートに対して`cargo doc`が生成するAPIドキュメンテーションは、これで図14-4のようにトップページに再エクスポートを列挙しリンクするので、
`PrimaryColor`型と`SecondaryColor`型と`mix`関数を見つけやすくします。

<!--
<img alt="Rendered documentation for the `art` crate with the re-exports on the front page" src="img/trpl14-04.png" class="center" />
-->

<img alt="トップページに再エクスポートのある`art`クレートの描画されたドキュメンテーション" src="img/trpl14-04.png" class="center" />

<!--
<span class="caption">Figure 14-4: The front page of the documentation for `art`
that lists the re-exports</span>
-->

<span class="caption">図14-4: 再エクスポートを列挙する`art`のドキュメンテーションのトップページ</span>

<!--
The `art` crate users can still see and use the internal structure from Listing
14-3 as demonstrated in Listing 14-4, or they can use the more convenient
structure in Listing 14-5, as shown in Listing 14-6:
-->

`art`クレートのユーザは、それでも、リスト14-4にデモされているように、リスト14-3の内部構造を見て使用することもできますし、
リスト14-5のより便利な構造を使用することもできます。リスト14-6に示したようにですね:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
extern crate art;

use art::PrimaryColor;
use art::mix;

fn main() {
    // --snip--
}
```

<!--
<span class="caption">Listing 14-6: A program using the re-exported items from
the `art` crate</span>
-->

<span class="caption">リスト14-6: `art`クレートの再エクスポートされた要素を使用するプログラム</span>

<!--
In cases where there are many nested modules, re-exporting the types at the top
level with `pub use` can make a significant difference in the experience of
people who use the crate.
-->

ネストされたモジュールがたくさんあるような場合、最上位階層で`pub use`により型を再エクスポートすることは、
クレートの使用者の経験に大きな違いを生みます。

<!--
Creating a useful public API structure is more of an art than a science, and
you can iterate to find the API that works best for your users. Choosing `pub
use` gives you flexibility in how you structure your crate internally and
decouples that internal structure from what you present to your users. Look at
some of the code of crates you’ve installed to see if their internal structure
differs from their public API.
-->

役に立つAPI構造を作ることは、科学というよりも芸術の領域であり、ユーザにとって何が最善のAPIなのか、
探究するために繰り返してみることができます。`pub use`は、内部的なクレート構造に柔軟性をもたらし、
その内部構造をユーザに提示する構造から切り離してくれます。インストールしてある他のクレートを見て、
内部構造が公開APIと異なっているか確認してみてください。

<!--
### Setting Up a Crates.io Account
-->

### Crates.ioのアカウントをセットアップする

<!--
Before you can publish any crates, you need to create an account on
[crates.io](https://crates.io) and get an API token. To do so,
visit the home page at [crates.io](https://crates.io) and log in
via a GitHub account. (The GitHub account is currently a requirement, but the
site might support other ways of creating an account in the future.) Once
you're logged in, visit your account settings at
[https://crates.io/me/](https://crates.io/me/) and retrieve your
API key. Then run the `cargo login` command with your API key, like this:
-->

クレートを公開する前に、[crates.io](https://crates.io)のアカウントを作成し、
APIトークンを取得する必要があります。そうするには、[crates.io](https://crates.io)のホームページを訪れ、
Githubアカウントでログインしてください。(現状は、Githubアカウントがなければなりませんが、
いずれは他の方法でもアカウントを作成できるようになる可能性があります。)ログインしたら、
[https://crates.io/me/](https://crates.io/me/)で自分のアカウントの設定に行き、
APIキーを取り扱ってください。そして、`cargo login`コマンドをAPIキーとともに実行してください。
以下のようにですね:

```text
$ cargo login abcdefghijklmnopqrstuvwxyz012345
```

<!--
This command will inform Cargo of your API token and store it locally in
*~/.cargo/credentials*. Note that this token is a *secret*: do not share it
with anyone else. If you do share it with anyone for any reason, you should
revoke it and generate a new token on [crates.io](https://crates.io)
-->

このコマンドは、CargoにAPIトークンを知らせ、*~/.cargo/credentials*にローカルに保存します。
このトークンは、*秘密*です: 他人とは共有しないでください。なんらかの理由で他人と実際に共有してしまったら、
古いものを破棄して[crates.io](https://crates.io)で新しいトークンを生成するべきです。

<!--
### Adding Metadata to a New Crate
-->

### 新しいクレートにメタデータを追加する

<!--
Now that you have an account, let’s say you have a crate you want to publish.
Before publishing, you’ll need to add some metadata to your crate by adding it
to the `[package]` section of the crate’s *Cargo.toml* file.
-->

アカウントはできたので、公開したいクレートがあるとしましょう。公開前に、
*Cargo.toml*ファイルの`[package]`セクションに追加することでクレートにメタデータを追加する必要があるでしょう。

<!--
Your crate will need a unique name. While you’re working on a crate locally,
you can name a crate whatever you’d like. However, crate names on
[crates.io](https://crates.io) are allocated on a first-come,
first-served basis. Once a crate name is taken, no one else can publish a crate
with that name. Search for the name you want to use on the site to find out
whether it has been used. If it hasn’t, edit the name in the *Cargo.toml* file
under `[package]` to use the name for publishing, like so:
-->

クレートには、独自の名前が必要でしょう。クレートをローカルで作成している間、
クレートの名前はなんでもいい状態でした。ところが、[crates.io](https://crates.io)のクレート名は、
最初に来たもの勝ちの精神で付与されていますので、一旦クレート名が取られてしまったら、
その名前のクレートを他の人が公開することは絶対できません。もう使われているか、
サイトで使いたい名前を検索してください。まだなら、*Cargo.toml*ファイルの`[package]`以下の名前を編集して、
名前を公開用に使ってください。以下のように:

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">ファイル名: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
```

<!--
Even if you’ve chosen a unique name, when you run `cargo publish` to publish
the crate at this point, you’ll get a warning and then an error:
-->

たとえ、独自の名前を選択していたとしても、この時点で`cargo publish`を実行すると、警告とエラーが出ます:

```text
$ cargo publish
    Updating registry `https://github.com/rust-lang/crates.io-index`
warning: manifest has no description, license, license-file, documentation,
homepage or repository.
(警告: マニフェストに説明、ライセンス、ライセンスファイル、ドキュメンテーション、ホームページ、
リポジトリがありません)
--snip--
error: api errors: missing or empty metadata fields: description, license.
(エラー: APIエラー: 存在しないメタデータフィールド: description, license)
```

<!--
The reason is that you’re missing some crucial information: a description and
license are required so people will know what your crate does and under what
terms they can use it. To rectify this error, you need to include this
information in the *Cargo.toml* file.
-->

原因は、大事な情報を一部入れていないからです: 説明とライセンスは、
他の人があなたのクレートは何をし、どんな条件の元で使っていいのかを知るために必要なのです。
このエラーを解消するには、*Cargo.toml*ファイルにこの情報を入れ込む必要があります。

<!--
Add a description that is just a sentence or two, because it will appear with
your crate in search results. For the `license` field, you need to give a
*license identifier value*. The [Linux Foundation’s Software Package Data
Exchange (SPDX)][spdx] lists the identifiers you can use for this value. For
example, to specify that you’ve licensed your crate using the MIT License, add
the `MIT` identifier:
-->

1文か2文程度の説明をつけてください。これは、検索結果に表示されますからね。
`license`フィールドには、*ライセンス識別子*を与える必要があります。
[Linux団体のSoftware Package Data Exchange(SPDX)][spdx]に、この値に使用できる識別子が列挙されています。
例えば、自分のクレートをMITライセンスでライセンスするためには、
`MIT`識別子を追加してください:

[spdx]: http://spdx.org/licenses/

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">ファイル名: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
license = "MIT"
```

<!--
If you want to use a license that doesn’t appear in the SPDX, you need to place
the text of that license in a file, include the file in your project, and then
use `license-file` to specify the name of that file instead of using the
`license` key.
-->

SPDXに出現しないライセンスを使用したい場合、そのライセンスをファイルに配置し、
プロジェクトにそのファイルを含め、それから`license`キーを使う代わりに、
そのファイルの名前を指定するのに`license-file`を使う必要があります。

<!--
Guidance on which license is appropriate for your project is beyond the scope
of this book. Many people in the Rust community license their projects in the
same way as Rust by using a dual license of `MIT OR Apache-2.0`. This practice
demonstrates that you can also specify multiple license identifiers separated
by `OR` to have multiple licenses for your project.
-->

どのライセンスが自分のプロジェクトに<ruby>相<rp>(</rp><rt>ふ</rt><rp>)</rp>応<rp>(</rp><rt>さわ</rt><rp>)</rp></ruby>しいかというガイドは、
この本の範疇を超えています。Rustコミュニティの多くの人間は、`MIT OR Apache-2.0`のデュアルライセンスを使用することで、
Rust自体と同じようにプロジェクトをライセンスします。この実践は、`OR`で区切られる複数のライセンス識別子を指定して、
プロジェクトに複数のライセンスを持たせることもできることを模擬しています。

<!--
With a unique name, the version, the author details that `cargo new` added
when you created the crate, your description, and a license added, the
*Cargo.toml* file for a project that is ready to publish might look like this:
-->

独自の名前、バージョン、クレート作成時に`cargo new`が追加した筆者の詳細、説明、ライセンスが追加され、
公開準備のできたプロジェクト用の`Cargo.toml`ファイルは以下のような見た目になっていることでしょう:

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">ファイル名: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "A fun game where you guess what number the computer has chosen."
              (コンピュータが選択した数字を言い当てる面白いゲーム)
license = "MIT OR Apache-2.0"

[dependencies]
```

<!--
[Cargo’s documentation](https://doc.rust-lang.org/cargo/) describes other
metadata you can specify to ensure others can discover and use your crate more
easily.
-->

[Cargoのドキュメンテーション](https://doc.rust-lang.org/cargo)には、
指定して他人が発見し、より容易くクレートを使用できることを保証する他のメタデータが解説されています。

<!--
### Publishing to Crates.io
-->

### Crates.ioに公開する

<!--
Now that you’ve created an account, saved your API token, chosen a name for
your crate, and specified the required metadata, you’re ready to publish!
Publishing a crate uploads a specific version to
[crates.io](https://crates.io) for others to use.
-->

アカウントを作成し、APIトークンを保存し、クレートの名前を決め、必要なメタデータを指定したので、
公開する準備が整いました！クレートを公開すると、特定のバージョンが、
[crates.io](http://crates.io)に他の人が使用できるようにアップロードされます。

<!--
Be careful when publishing a crate because a publish is *permanent*. The
version can never be overwritten, and the code cannot be deleted. One major
goal of [crates.io](https://crates.io) is to act as a permanent
archive of code so that builds of all projects that depend on crates from
[crates.io](https://crates.io) will continue to work. Allowing
version deletions would make fulfilling that goal impossible. However, there is
no limit to the number of crate versions you can publish.
-->

公開は*永久*なので、クレートの公開時には気をつけてください。バージョンは絶対に上書きできず、
コードも削除できません。[crates.io](https://crates.io)の一つの主な目標が、
[crates.io](https://crates.io)のクレートに依存している全てのプロジェクトのビルドが、
動き続けるようにコードの永久アーカイブとして機能することなのです。バージョン削除を可能にしてしまうと、
その目標を達成するのが不可能になってしまいます。ですが、公開できるクレートバージョンの数に制限はありません。

<!--
Run the `cargo publish` command again. It should succeed now:
-->

再度`cargo publish`コマンドを実行してください。今度は成功するはずです:

```text
$ cargo publish
 Updating registry `https://github.com/rust-lang/crates.io-index`
Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
 Finished dev [unoptimized + debuginfo] target(s) in 0.19 secs
Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
```

<!--
Congratulations! You’ve now shared your code with the Rust community, and
anyone can easily add your crate as a dependency of their project.
-->

おめでとうございます！Rustコミュニティとコードを共有し、誰でもあなたのクレートを依存として簡単に追加できます。

<!--
### Publishing a New Version of an Existing Crate
-->

### 既存のクレートの新バージョンを公開する

<!--
When you’ve made changes to your crate and are ready to release a new version,
you change the `version` value specified in your *Cargo.toml* file and
republish. Use the [Semantic Versioning rules][semver] to decide what an
appropriate next version number is based on the kinds of changes you’ve made.
Then run `cargo publish` to upload the new version.
-->

クレートに変更を行い、新バージョンをリリースする準備ができたら、
*Cargo.toml*ファイルに指定された`version`の値を変更し、再公開します。
[セマンティックバージョンルール][semver]を使用して加えた変更の種類に基づいて次の適切なバージョン番号を決定してください。
そして、`cargo publish`を実行し、新バージョンをアップロードします。

[semver]: http://semver.org/

<!--
### Removing Versions from Crates.io with `cargo yank`
-->

### `cargo yank`でCrates.ioからバージョンを削除する

<!--
最後の行。辞書には、yankingはグイっと引っ張ることとしかないが、ここでは取り下げと意訳しておく
-->

<!--
Although you can’t remove previous versions of a crate, you can prevent any
future projects from adding them as a new dependency. This is useful when a
crate version is broken for one reason or another. In such situations, Cargo
supports *yanking* a crate version.
-->

以前のバージョンのクレートを削除することはできないものの、将来のプロジェクトがこれに新たに依存することを防ぐことはできます。
これは、なんらかの理由により、クレートバージョンが壊れている場合に有用です。そのような場面において、
Cargoはクレートバージョンの *取り下げ(yank)* をサポートしています。

<!--
Yanking a version prevents new projects from starting to depend on that version
while allowing all existing projects that depend on it to continue to download
and depend on that version. Essentially, a yank means that all projects with a
*Cargo.lock* will not break, and any future *Cargo.lock* files generated will
not use the yanked version.
-->

バージョンを取り下げると、既存のプロジェクトは、引き続きダウンロードしたりそのバージョンに依存したりしつづけられますが、
新規プロジェクトが新しくそのバージョンに依存しだすことは防止されます。つまるところ、取り下げは、
すでに*Cargo.lock*が存在するプロジェクトは壊さないが、将来的に生成された*Cargo.lock*ファイルは
取り下げられたバージョンを使わない、ということを意味します。

<!--
To yank a version of a crate, run `cargo yank` and specify which version you
want to yank:
-->

あるバージョンのクレートを取り下げるには、`cargo yank`を実行し、取り下げたいバージョンを指定します:

```text
$ cargo yank --vers 1.0.1
```

<!--
By adding `--undo` to the command, you can also undo a yank and allow projects
to start depending on a version again:
-->

`--undo`をコマンドに付与することで、取り下げを取り消し、再度あるバージョンにプロジェクトを依存させ始めることもできます:

```text
$ cargo yank --vers 1.0.1 --undo
```

<!--
A yank *does not* delete any code. For example, the yank feature is not
intended for deleting accidentally uploaded secrets. If that happens, you must
reset those secrets immediately.
-->

取り下げは、コードの削除は一切し*ません*。例として、取り下げ機能は、誤ってアップロードされた秘密鍵を削除するためのものではありません。
もしそうなってしまったら、即座に秘密鍵をリセットしなければなりません。
