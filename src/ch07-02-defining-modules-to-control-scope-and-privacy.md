<!--
## Defining Modules to Control Scope and Privacy
-->
## モジュールを定義して、スコープとプライバシーを制御する

<!--
In this section, we’ll talk about modules and other parts of the module system,
namely *paths* that allow you to name items; the `use` keyword that brings a
path into scope; and the `pub` keyword to make items public. We’ll also discuss
the `as` keyword, external packages, and the glob operator. For now, let’s
focus on modules!
-->
この節では、モジュールと、その他のモジュールシステムの要素
――すなわち、要素に名前をつけるための *パス* 、パスをスコープに持ち込む`use`キーワード、要素を公開する`pub`キーワード――
について学びます。
また、`as`キーワード、外部パッケージ、glob 演算子についても話します。
とりあえず、今はモジュールに集中しましょう！

<!--
*Modules* let us organize code within a crate into groups for readability and
easy reuse. Modules also control the *privacy* of items, which is whether an
item can be used by outside code (*public*) or is an internal implementation
detail and not available for outside use (*private*).
-->
*モジュール* はクレート内のコードをグループ化し、可読性と再利用性を上げるのに役に立ちます。
モジュールは要素の *プライバシー* も制御できます。プライバシーとは、要素がコードの外側で使える *(公開 public)* のか、内部の実装の詳細であり外部では使えない *(非公開 private)* のかです。
<!--
As an example, let’s write a library crate that provides the functionality of a
restaurant. We’ll define the signatures of functions but leave their bodies
empty to concentrate on the organization of the code, rather than actually
implement a restaurant in code.
-->
例えば、レストランの機能を提供するライブラリクレートを書いてみましょう。
実際にレストランを実装することではなく、コードの関係性に注目したいので、関数にシグネチャをつけますが中身は空白のままにします。

<!--
In the restaurant industry, some parts of a restaurant are referred to as
*front of house* and others as *back of house*. Front of house is where
customers are; this is where hosts seat customers, servers take orders and
payment, and bartenders make drinks. Back of house is where the chefs and cooks
work in the kitchen, dishwashers clean up, and managers do administrative work.
-->
レストラン業界では、レストランの一部を *接客部門 (front of house)* といい、その他を *後方部門 (back of house)* といいます。
接客部門とはお客さんがいるところです。接客係がお客様を席に案内し、給仕係が注文と支払いを受け付け、バーテンダーが飲み物を作ります。
後方部門とはシェフや料理人がキッチンで働き、皿洗い係が食器を片付け、マネージャが管理業務をする場所です。

<!--
To structure our crate in the same way that a real restaurant works, we can
organize the functions into nested modules. Create a new library named
`restaurant` by running `cargo new --lib restaurant`; then put the code in
Listing 7-1 into *src/lib.rs* to define some modules and function signatures.
-->
私達のクレートを現実のレストランと同じような構造にするために、関数をネストしたモジュールにまとめましょう。
`restaurant`という名前の新しいライブラリを`cargo new --lib restaurant`と実行することで作成し、Listing 7-1 のコードを *src/lib.rs* に書き込み、モジュールと関数のシグネチャを定義してください。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 7-1: A `front_of_house` module containing other
modules that then contain functions</span>
-->
<span class="caption">Listing 7-1: `front_of_house`モジュールにその他のモジュールが含まれ、さらにそれらが関数を含んでいる</span>

<!--
We define a module by starting with the `mod` keyword and then specify the
name of the module (in this case, `front_of_house`) and place curly brackets
around the body of the module. Inside modules, we can have other modules, as in
this case with the modules `hosting` and `serving`. Modules can also hold
definitions for other items, such as structs, enums, constants, traits, or—as
in Listing 7-1—functions.
-->
モジュールは、`mod`キーワードを書き、次にモジュールの名前（今回の場合、`front_of_house`）を指定することで定義されます。
モジュールの中には、今回だと`hosting`と`serving`のように、他のモジュールをおくこともできます。
モジュールにはその他の要素の定義も置くことができます。例えば、構造体、enum、定数、トレイト、そして（Listing 7-1 のように）関数です。

<!--
By using modules, we can group related definitions together and name why
they’re related. Programmers using this code would have an easier time finding
the definitions they wanted to use because they could navigate the code based
on the groups rather than having to read through all the definitions.
Programmers adding new functionality to this code would know where to place the
code to keep the program organized.
-->
モジュールを使うことで、関連する定義を一つにまとめ、関連する理由を名前で示せます。
このコードを使うプログラマーは、定義を全部読むことなく、グループ単位でコードを読み進められるので、欲しい定義を見つけ出すのが簡単になるでしょう。
このコードに新しい機能を付け加えるプログラマーは、プログラムのまとまりを保つために、どこにその機能のコードを置けば良いのかがわかるでしょう。

<!--
Earlier, we mentioned that *src/main.rs* and *src/lib.rs* are called crate
roots. The reason for their name is that the contents of either of these two
files form a module named `crate` at the root of the crate’s module structure,
known as the *module tree*.
-->
以前、 *src/main.rs* と *src/lib.rs* はクレートルートと呼ばれていると言いました。
この名前のわけは、 *モジュールツリー* と呼ばれるクレートのモジュール構造の根っこ（ルート）にこれら 2 つのファイルの中身が`crate`というモジュールを形成するからです。

<!--
Listing 7-2 shows the module tree for the structure in Listing 7-1.
-->
Listing 7-2 は、Listing 7-1 の構造のモジュールツリーを示しています。

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

<!--
<span class="caption">Listing 7-2: The module tree for the code in Listing
7-1</span>
-->
<span class="caption">Listing 7-2: Listing 7-1 のコードのモジュールツリー</span>

<!--
This tree shows how some of the modules nest inside one another (for example,
`hosting` nests inside `front_of_house`). The tree also shows that some modules
are *siblings* to each other, meaning they’re defined in the same module
(`hosting` and `serving` are defined within `front_of_house`). To continue the
family metaphor, if module A is contained inside module B, we say that module A
is the *child* of module B and that module B is the *parent* of module A.
Notice that the entire module tree is rooted under the implicit module named
`crate`.
-->
このツリーを見ると、どのモジュールがどのモジュールの中にネストしているのかがわかります（例えば、`hosting`は`front_of_house`の中にネストしています）。
また、いくつかのモジュールはお互いに *兄弟* の関係にある、つまり、同じモジュール内で定義されていることもわかります（例えば`hosting`と`serving`は`front_of_house`で定義されています）。
他にも、家族関係の比喩を使って、モジュール A がモジュール B の中に入っている時、A は B の *子* であるといい、B は A の *親* であるといいます。
モジュールツリー全体が、暗黙のうちに作られた`crate`というモジュールの下にあることにも注目してください。

<!--
The module tree might remind you of the filesystem’s directory tree on your
computer; this is a very apt comparison! Just like directories in a filesystem,
you use modules to organize your code. And just like files in a directory, we
need a way to find our modules.
-->
モジュールツリーを見ていると、コンピュータのファイルシステムのディレクトリツリーを思い出すかもしれません。その喩えはとても適切です！
ファイルシステムのディレクトリのように、モジュールはコードをまとめるのに使われるのです。
そしてディレクトリからファイルを見つけるように、目的のモジュールを見つけ出す方法が必要になります。
