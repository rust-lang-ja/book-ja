<!--
## Bringing Paths into Scope with the `use` Keyword
-->
## `use`キーワードでパスをスコープに持ち込む

<!--
It might seem like the paths we’ve written to call functions so far are
inconveniently long and repetitive. For example, in Listing 7-7, whether we
chose the absolute or relative path to the `add_to_waitlist` function, every
time we wanted to call `add_to_waitlist` we had to specify `front_of_house` and
`hosting` too. Fortunately, there’s a way to simplify this process. We can
bring a path into a scope once and then call the items in that path as if
they’re local items with the `use` keyword.
-->
これまで関数呼び出しのために書いてきたパスは、長く、繰り返しも多くて不便なものでした。
例えば、Listing 7-7 においては、絶対パスを使うか相対パスを使うかにかかわらず、`add_to_waitlist`関数を呼ぼうと思うたびに`front_of_house`と`hosting`も指定しないといけませんでした。
ありがたいことに、この手続きを簡単化する方法があります。
`use`キーワードを使うことで、パスを一度スコープに持ち込んでしまえば、それ以降はパス内の要素がローカルにあるかのように呼び出すことができるのです。

<!--
In Listing 7-11, we bring the `crate::front_of_house::hosting` module into the
scope of the `eat_at_restaurant` function so we only have to specify
`hosting::add_to_waitlist` to call the `add_to_waitlist` function in
`eat_at_restaurant`.
-->
Listing 7-11 では、`crate::front_of_house::hosting`モジュールを`eat_at_restaurant`関数のスコープに持ち込むことで、`eat_at_restaurant`において、`hosting::add_to_waitlist`と指定するだけで`add_to_waitlist`関数を呼び出せるようにしています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 7-11: Bringing a module into scope with
`use`</span>
-->
<span class="caption">Listing 7-11: `use` でモジュールをスコープに持ち込む</span>

<!--
Adding `use` and a path in a scope is similar to creating a symbolic link in
the filesystem. By adding `use crate::front_of_house::hosting` in the crate
root, `hosting` is now a valid name in that scope, just as though the `hosting`
module had been defined in the crate root. Paths brought into scope with `use`
also check privacy, like any other paths.
-->
`use`とパスをスコープに追加することは、ファイルシステムにおいてシンボリックリンクを張ることに似ています。
`use crate::front_of_house::hosting`をクレートルートに追加することで、`hosting`はこのスコープで有効な名前となり、まるで`hosting`はクレートルートで定義されていたかのようになります。
スコープに`use`で持ち込まれたパスも、他のパスと同じようにプライバシーがチェックされます。

<!--
You can also bring an item into scope with `use` and a relative path. Listing
7-12 shows how to specify a relative path to get the same behavior as in
Listing 7-11.
-->
`use`と相対パスで要素をスコープに持ち込むこともできます。
Listing 7-12 は Listing 7-11 と同じふるまいを得るためにどう相対パスを書けば良いかを示しています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 7-12: Bringing a module into scope with `use` and
a relative path</span>
-->
<span class="caption">Listing 7-12: モジュールを`use`と相対パスを使ってスコープに持ち込む</span>

<!--
### Creating Idiomatic `use` Paths
-->
### 慣例に従った`use`パスを作る

<!--
In Listing 7-11, you might have wondered why we specified `use
crate::front_of_house::hosting` and then called `hosting::add_to_waitlist` in
`eat_at_restaurant` rather than specifying the `use` path all the way out to
the `add_to_waitlist` function to achieve the same result, as in Listing 7-13.
-->
Listing 7-11 を見て、なぜ`use crate::front_of_house::hosting`と書いて`eat_at_restaurant`内で`hosting::add_to_waitlist`と呼び出したのか不思議に思っているかもしれません。Listing 7-13 のように、`use`で`add_to_waitlist`までのパスをすべて指定しても同じ結果が得られるのに、と。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 7-13: Bringing the `add_to_waitlist` function
into scope with `use`, which is unidiomatic</span>
-->
<span class="caption">Listing 7-13: `add_to_waitlist` 関数を`use` でスコープに持ち込む。このやりかたは慣例的ではない</span>

<!--
Although both Listing 7-11 and 7-13 accomplish the same task, Listing 7-11 is
the idiomatic way to bring a function into scope with `use`. Bringing the
function’s parent module into scope with `use` so we have to specify the parent
module when calling the function makes it clear that the function isn’t locally
defined while still minimizing repetition of the full path. The code in Listing
7-13 is unclear as to where `add_to_waitlist` is defined.
-->
Listing 7-11 も 7-13 もおなじ仕事をしてくれますが、関数をスコープに`use`で持ち込む場合、Listing 7-11 のほうが慣例的なやり方です。
関数の親モジュールを`use`で持ち込むことで、関数を呼び出す際、毎回親モジュールを指定しなければならないようにすれば、フルパスを繰り返して書くことを抑えつつ、関数がローカルで定義されていないことを明らかにできます。
Listing 7-13 のコードではどこで`add_to_waitlist`が定義されたのかが不明瞭です。

<!--
On the other hand, when bringing in structs, enums, and other items with `use`,
it’s idiomatic to specify the full path. Listing 7-14 shows the idiomatic way
to bring the standard library’s `HashMap` struct into the scope of a binary
crate.
-->
一方で、構造体や enum その他の要素を`use`で持ち込むときは、フルパスを書くのが慣例的です。
Listing 7-14 は標準ライブラリの`HashMap`構造体をバイナリクレートのスコープに持ち込む慣例的なやり方を示しています。


<!--
<span class="filename">Filename: src/main.rs</span>
-->
<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

<!--
<span class="caption">Listing 7-14: Bringing `HashMap` into scope in an
idiomatic way</span>
-->
<span class="caption">Listing 7-14: `HashMap`を慣例的なやり方でスコープに持ち込む</span>

<!--
There’s no strong reason behind this idiom: it’s just the convention that has
emerged, and folks have gotten used to reading and writing Rust code this way.
-->
こちらの慣例の背後には、はっきりとした理由はありません。自然に発生した慣習であり、みんな Rust のコードをこのやり方で読み書きするのに慣れてしまったというだけです。

<!--
The exception to this idiom is if we’re bringing two items with the same name
into scope with `use` statements, because Rust doesn’t allow that. Listing 7-15
shows how to bring two `Result` types into scope that have the same name but
different parent modules and how to refer to them.
-->
同じ名前の 2 つの要素を`use`でスコープに持ち込むのは Rust では許されないので、そのときこの慣例は例外的に不可能です。
Listing 7-15 は、同じ名前を持つけれど異なる親モジュールを持つ 2 つの`Result`型をスコープに持ち込み、それらを参照するやり方を示しています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 7-15: Bringing two types with the same name into
the same scope requires using their parent modules.</span>
-->
<span class="caption">Listing 7-15: 同じ名前を持つ 2 つの型を同じスコープに持ち込むには親モジュールを使わないといけない。</span>

<!--
As you can see, using the parent modules distinguishes the two `Result` types.
If instead we specified `use std::fmt::Result` and `use std::io::Result`, we’d
have two `Result` types in the same scope and Rust wouldn’t know which one we
meant when we used `Result`.
-->
このように、親モジュールを使うことで 2 つの`Result`型を区別できます。
もし`use std::fmt::Result` と `use std::io::Result`と書いていたとしたら、2 つの`Result`型が同じスコープに存在することになり、私達が`Result`を使ったときにどちらのことを意味しているのか Rust はわからなくなってしまいます。

<!--
### Providing New Names with the `as` Keyword
-->
### 新しい名前を`as`キーワードで与える

<!--
There’s another solution to the problem of bringing two types of the same name
into the same scope with `use`: after the path, we can specify `as` and a new
local name, or alias, for the type. Listing 7-16 shows another way to write the
code in Listing 7-15 by renaming one of the two `Result` types using `as`.
-->
同じ名前の 2 つの型を`use`を使って同じスコープに持ち込むという問題には、もう一つ解決策があります。パスの後に、`as`と型の新しいローカル名、即ちエイリアスを指定すればよいのです。
Listing 7-16 は、Listing 7-15 のコードを、2 つの`Result`型のうち一つを`as`を使ってリネームするという別のやり方で書いたものを表しています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 7-16: Renaming a type when it’s brought into
scope with the `as` keyword</span>
-->
<span class="caption">Listing 7-16: 型がスコープに持ち込まれた時、`as`キーワードを使ってその名前を変えている</span>

<!--
In the second `use` statement, we chose the new name `IoResult` for the
`std::io::Result` type, which won’t conflict with the `Result` from `std::fmt`
that we’ve also brought into scope. Listing 7-15 and Listing 7-16 are
considered idiomatic, so the choice is up to you!
-->
2 つめの`use`文では、`std::io::Result`に、`IoResult`という新たな名前を選んでやります。`std::fmt`の`Result`もスコープに持ち込んでいますが、この名前はこれとは衝突しません。
Listing 7-15 も Listing 7-16 も慣例的とみなされているので、どちらを使っても構いませんよ！

<!--
### Re-exporting Names with `pub use`
-->
### `pub use`を使って名前を再公開する

<!--
When we bring a name into scope with the `use` keyword, the name available in
the new scope is private. To enable the code that calls our code to refer to
that name as if it had been defined in that code’s scope, we can combine `pub`
and `use`. This technique is called *re-exporting* because we’re bringing
an item into scope but also making that item available for others to bring into
their scope.
-->
`use`キーワードで名前をスコープに持ちこんだ時、新しいスコープで使用できるその名前は非公開です。
私達のコードを呼び出すコードが、まるでその名前が私達のコードのスコープで定義されていたかのように参照できるようにするためには、`pub`と`use`を組み合わせればいいです。
このテクニックは、要素を自分たちのスコープに持ち込むだけでなく、他の人がその要素をその人のスコープに持ち込むことも可能にすることから、*再公開 (re-exporting)* と呼ばれています。

<!--
Listing 7-17 shows the code in Listing 7-11 with `use` in the root module
changed to `pub use`.
-->
Listing 7-17 は Listing 7-11 のコードのルートモジュールでの`use`を`pub use`に変更したものを示しています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 7-17: Making a name available for any code to use
from a new scope with `pub use`</span>
-->
<span class="caption">Listing 7-17: `pub use`で、新たなスコープのコードがその名前を使えるようにする</span>

<!--
By using `pub use`, external code can now call the `add_to_waitlist` function
using `hosting::add_to_waitlist`. If we hadn’t specified `pub use`, the
`eat_at_restaurant` function could call `hosting::add_to_waitlist` in its
scope, but external code couldn’t take advantage of this new path.
-->
`pub use`を使うことで、外部のコードが`hosting::add_to_waitlist`を使って`add_to_waitlist`関数を呼び出せるようになりました。
`pub use`を使っていなければ、`eat_at_restaurant`関数は`hosting::add_to_waitlist`を自らのスコープ内で使えるものの、外部のコードはこの新しいパスを利用することはできないでしょう。

<!--
Re-exporting is useful when the internal structure of your code is different
from how programmers calling your code would think about the domain. For
example, in this restaurant metaphor, the people running the restaurant think
about “front of house” and “back of house.” But customers visiting a restaurant
probably won’t think about the parts of the restaurant in those terms. With
`pub use`, we can write our code with one structure but expose a different
structure. Doing so makes our library well organized for programmers working on
the library and programmers calling the library.
-->
再公開は、あなたのコードの内部構造と、あなたのコードを呼び出すプログラマーたちのその領域に関しての見方が異なるときに有用です。
例えば、レストランの比喩では、レストランを経営している人は「接客部門 (front of house)」と「後方部門 (back of house)」のことについて考えるでしょう。
しかし、レストランを訪れるお客さんは、そのような観点からレストランの部門について考えることはありません。
`pub use`を使うことで、ある構造でコードを書きつつも、別の構造で公開するということが可能になります。
こうすることで、私達のライブラリを、ライブラリを開発するプログラマにとっても、ライブラリを呼び出すプログラマにとっても、よく整理されたものとすることができます。

<!--
### Using External Packages
-->
### 外部のパッケージを使う

<!--
In Chapter 2, we programmed a guessing game project that used an external
package called `rand` to get random numbers. To use `rand` in our project, we
added this line to *Cargo.toml*:
-->
2 章で、乱数を得るために`rand`という外部パッケージを使って、数当てゲームをプログラムしました。
`rand`を私達のプロジェクトで使うために、次の行を *Cargo.toml* に書き加えましたね：

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<!--
<span class="filename">Filename: Cargo.toml</span>
-->
<span class="filename">ファイル名：Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

<!--
Adding `rand` as a dependency in *Cargo.toml* tells Cargo to download the
`rand` package and any dependencies from [crates.io](https://crates.io/) and
make `rand` available to our project.
-->
`rand`を依存 (dependency) として *Cargo.toml* に追加すると、`rand`パッケージとそのすべての依存を[crates.io](https://crates.io/)からダウンロードして、私達のプロジェクトで`rand`が使えるようにするよう Cargo に命令します。

<!--
Then, to bring `rand` definitions into the scope of our package, we added a
`use` line starting with the name of the crate, `rand`, and listed the items
we wanted to bring into scope. Recall that in the [“Generating a Random
Number”][rand] section in Chapter 2, we brought the `Rng` trait
into scope and called the `rand::thread_rng` function:
-->
そして、`rand`の定義を私達のパッケージのスコープに持ち込むために、クレートの名前である`rand`から始まる`use`の行を追加し、そこにスコープに持ち込みたい要素を並べました。
2 章の[乱数を生成する][rand]の節で、`Rng`トレイトをスコープに持ち込み`rand::thread_rng`関数を呼び出したことを思い出してください。

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

<!--
Members of the Rust community have made many packages available at
[crates.io](https://crates.io/), and pulling any of them into your package
involves these same steps: listing them in your package’s *Cargo.toml* file and
using `use` to bring items from their crates into scope.
-->
Rust コミュニティに所属する人々が[crates.io](https://crates.io/)でたくさんのパッケージを利用できるようにしてくれており、上と同じステップを踏めばそれらをあなたのパッケージに取り込むことができます：あなたのパッケージの *Cargo.toml* ファイルにそれらを書き並べ、`use`を使って要素をクレートからスコープへと持ち込めばよいのです。

<!--
Note that the standard library (`std`) is also a crate that’s external to our
package. Because the standard library is shipped with the Rust language, we
don’t need to change *Cargo.toml* to include `std`. But we do need to refer to
it with `use` to bring items from there into our package’s scope. For example,
with `HashMap` we would use this line:
-->
標準ライブラリ (`std`) も、私達のパッケージの外部にあるクレートだということに注意してください。
標準ライブラリは Rust 言語に同梱されているので、 *Cargo.toml* を `std`を含むように変更する必要はありません。
しかし、その要素をそこから私達のパッケージのスコープに持ち込むためには、`use`を使って参照する必要はあります。
例えば、`HashMap`には次の行を使います。

```rust
use std::collections::HashMap;
```

<!--
This is an absolute path starting with `std`, the name of the standard library
crate.
-->
これは標準ライブラリクレートの名前`std`から始まる絶対パスです。

<!--
### Using Nested Paths to Clean Up Large `use` Lists
-->
### 巨大な`use`のリストをネストしたパスを使って整理する

<!--
If we’re using multiple items defined in the same crate or same module,
listing each item on its own line can take up a lot of vertical space in our
files. For example, these two `use` statements we had in the Guessing Game in
Listing 2-4 bring items from `std` into scope:
-->
同じクレートか同じモジュールで定義された複数の要素を使おうとする時、それぞれの要素を一行一行並べると、縦に大量のスペースを取ってしまいます。
例えば、Listing 2-4 の数当てゲームで使った次の 2 つの`use`文が`std`からスコープへ要素を持ち込みました。

<!--
<span class="filename">Filename: src/main.rs</span>
-->
<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

<!--
Instead, we can use nested paths to bring the same items into scope in one
line. We do this by specifying the common part of the path, followed by two
colons, and then curly brackets around a list of the parts of the paths that
differ, as shown in Listing 7-18.
-->
代わりに、ネストしたパスを使うことで、同じ一連の要素を 1 行でスコープに持ち込めます。
これをするには、Listing 7-18 に示されるように、パスの共通部分を書き、2 つのコロンを続け、そこで波括弧で互いに異なる部分のパスのリストを囲みます。

<!--
<span class="filename">Filename: src/main.rs</span>
-->
<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

<!--
<span class="caption">Listing 7-18: Specifying a nested path to bring multiple
items with the same prefix into scope</span>
-->
<span class="caption">Listing 7-18: 同じプレフィックスをもつ複数の要素をスコープに持ち込むためにネストしたパスを指定する</span>

<!--
In bigger programs, bringing many items into scope from the same crate or
module using nested paths can reduce the number of separate `use` statements
needed by a lot!
-->
大きなプログラムにおいては、同じクレートやモジュールからのたくさんの要素をネストしたパスで持ち込むようにすれば、独立した`use`文の数を大きく減らすことができます！

<!--
We can use a nested path at any level in a path, which is useful when combining
two `use` statements that share a subpath. For example, Listing 7-19 shows two
`use` statements: one that brings `std::io` into scope and one that brings
`std::io::Write` into scope.
-->
ネストしたパスはパスのどの階層においても使うことができます。これはサブパスを共有する 2 つの`use`文を合体させるときに有用です。
例えば、Listing 7-19 は 2 つの`use`文を示しています：1 つは`std::io`をスコープに持ち込み、もう一つは`std::io::Write`をスコープに持ち込んでいます。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-19: Two `use` statements where one is a subpath
of the other</span>
-->
<span class="caption">Listing 7-19: 片方がもう片方のサブパスである 2 つの`use`文</span>

<!--
The common part of these two paths is `std::io`, and that’s the complete first
path. To merge these two paths into one `use` statement, we can use `self` in
the nested path, as shown in Listing 7-20.
-->
これらの 2 つのパスの共通部分は`std::io`であり、そしてこれは最初のパスにほかなりません。これらの 2 つのパスを 1 つの`use`文へと合体させるには、Listing 7-20 に示されるように、ネストしたパスに`self`を使いましょう。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-20: Combining the paths in Listing 7-19 into
one `use` statement</span>
-->
<span class="caption">Listing 7-20: Listing 7-19 のパスを一つの `use` 文に合体させる</span>

<!--
This line brings `std::io` and `std::io::Write` into scope.
-->
この行は `std::io` と`std::io::Write` をスコープに持ち込みます。

<!--
### The Glob Operator
-->
### glob 演算子

<!--
If we want to bring *all* public items defined in a path into scope, we can
specify that path followed by `*`, the glob operator:
-->
パスにおいて定義されているすべての公開要素をスコープに持ち込みたいときは、glob 演算子 `*` をそのパスの後ろに続けて書きましょう：

```rust
use std::collections::*;
```

<!--
This `use` statement brings all public items defined in `std::collections` into
the current scope. Be careful when using the glob operator! Glob can make it
harder to tell what names are in scope and where a name used in your program
was defined.
-->
この`use`文は`std::collections`のすべての公開要素を現在のスコープに持ち込みます。
glob 演算子を使う際にはご注意を！
glob をすると、どの名前がスコープ内にあり、プログラムで使われている名前がどこで定義されたのか分かりづらくなります。

<!--
The glob operator is often used when testing to bring everything under test
into the `tests` module; we’ll talk about that in the [“How to Write
Tests”][writing-tests] section in Chapter 11. The glob operator
is also sometimes used as part of the prelude pattern: see [the standard
library documentation](../std/prelude/index.html#other-preludes)
for more information on that pattern.
-->
glob 演算子はしばしば、テストの際、テストされるあらゆるものを`tests`モジュールに持ち込むために使われます。これについては 11 章[テストの書き方][writing-tests]の節で話します。
glob 演算子はプレリュードパターンの一部としても使われることがあります：そのようなパターンについて、より詳しくは[標準ライブラリのドキュメント](https://doc.rust-lang.org/std/prelude/index.html#other-preludes)をご覧ください。

[rand]: ch02-00-guessing-game-tutorial.html#乱数を生成する
[writing-tests]: ch11-01-writing-tests.html#テストの記述法
