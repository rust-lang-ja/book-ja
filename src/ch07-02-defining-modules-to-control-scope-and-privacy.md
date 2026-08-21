<!--
## Defining Modules to Control Scope and Privacy
-->
## モジュールを定義して、スコープとプライバシーを制御する

<!--
In this section, we’ll talk about modules and other parts of the module system,
namely *paths* that allow you to name items; the `use` keyword that brings a
path into scope; and the `pub` keyword to make items public. We’ll also discuss
the `as` keyword, external packages, and the glob operator.
-->
この節では、モジュールと、その他のモジュールシステムの要素
――すなわち、要素に名前をつけるための *パス* 、パスをスコープに持ち込む`use`キーワード、要素を公開する`pub`キーワード――
について学びます。
また、`as`キーワード、外部パッケージ、glob演算子についても話します。

<!--
First, we’re going to start with a list of rules for easy reference when you’re
organizing your code in the future. Then we’ll explain each of the rules in
detail.
-->
まずは、将来コードを整理するときの簡単なリファレンスとして、規則の一覧を示します。
その後で各規則を詳細に説明します。

<!--
### Modules Cheat Sheet
-->
### モジュールのチートシート

<!--
Here we provide a quick reference on how modules, paths, the `use` keyword, and
the `pub` keyword work in the compiler, and how most developers organize their
code. We’ll be going through examples of each of these rules throughout this
chapter, but this is a great place to refer to as a reminder of how modules
work.
-->
以下に、モジュール、パス、`use`キーワード、そして`pub`キーワードがコンパイラ内でどう機能するか、そして多くの開発者はどのようにコードを整理するかについての、クイックリファレンスを提供します。
この章ではこれらの各規則の実例を見ていきますが、ここはモジュールがどう機能するか思い出すために見直すのに良い場所となるでしょう。

<!--
- **Start from the crate root**: When compiling a crate, the compiler first
  looks in the crate root file (usually *src/lib.rs* for a library crate or
  *src/main.rs* for a binary crate) for code to compile.
- **Declaring modules**: In the crate root file, you can declare new modules;
say, you declare a “garden” module with `mod garden;`. The compiler will look
for the module’s code in these places:
  - Inline, within curly brackets that replace the semicolon following `mod
    garden`
  - In the file *src/garden.rs*
  - In the file *src/garden/mod.rs*
- **Declaring submodules**: In any file other than the crate root, you can
  declare submodules. For example, you might declare `mod vegetables;` in
  *src/garden.rs*. The compiler will look for the submodule’s code within the
  directory named for the parent module in these places:
  - Inline, directly following `mod vegetables`, within curly brackets instead
    of the semicolon
  - In the file *src/garden/vegetables.rs*
  - In the file *src/garden/vegetables/mod.rs*
- **Paths to code in modules**: Once a module is part of your crate, you can
  refer to code in that module from anywhere else in that same crate, as long
  as the privacy rules allow, using the path to the code. For example, an
  `Asparagus` type in the garden vegetables module would be found at
  `crate::garden::vegetables::Asparagus`.
- **Private vs public**: Code within a module is private from its parent
  modules by default. To make a module public, declare it with `pub mod`
  instead of `mod`. To make items within a public module public as well, use
  `pub` before their declarations.
- **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to
  items to reduce repetition of long paths. In any scope that can refer to
  `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use
  crate::garden::vegetables::Asparagus;` and from then on you only need to
  write `Asparagus` to make use of that type in the scope.
-->
- **クレートルートから始める**: クレートをコンパイルするとき、
  コンパイラはまずコンパイル対象のコードとしてクレートルートファイル（通常は、ライブラリクレートでは *src/lib.rs*、バイナリクレートでは *src/main.rs*）の中を探します。
- **モジュールを宣言する**: クレートルートファイルの中で、新しいモジュールを宣言することができます;
  例えば、`mod garden;`として“garden”モジュールを宣言したとします。コンパイラは以下の場所からモジュールのコードを探します:
  - インライン。`mod garden`の後のセミコロンが波かっこで置き換えられているとき、その中
  - *src/garden.rs* ファイルの中
  - *src/garden/mod.rs* ファイルの中
- **サブモジュールを宣言する**: クレートルート以外のすべてのファイルの中で、サブモジュールを宣言することができます。
  例えば、*src/garden.rs* 内で`mod vegetables;`と宣言することができます。コンパイラはサブモジュールのコードを、親モジュールに対応するディレクトリ内の以下の場所から探します:
  - インライン。`mod vegetables`のすぐ後にセミコロンの代わりに波かっこがあるとき、その中
  - *src/garden/vegetables.rs* ファイルの中
  - *src/garden/vegetables/mod.rs* ファイルの中
- **モジュール内のコードへのパス**: モジュールがクレートの一部となったら、プライバシー規則が許す限り同じクレート内のどこからでも、そのモジュール内のコードをコードへのパスを使用して参照できます。
  例えば、garden vegetablesモジュール内の`Asparagus`型は`crate::garden::vegetables::Asparagus`で参照できます。
- **非公開と公開**: モジュール内のコードはデフォルトでは非公開で、親モジュールからアクセスすることができません。
  モジュールを公開にするには、`mod`ではなく`pub mod`で宣言してください。
  公開モジュール内の要素も公開にするには、それらの宣言の前に`pub`を付けてください。
- **`use`キーワード**: `use`キーワードは、長いパスの繰り返しを減らすために、要素へのショートカットをスコープ内に作成します。
  `crate::garden::vegetables::Asparagus`を参照できるスコープ内であれば、`use crate::garden::vegetables::Asparagus;`でショートカットを作成でき、
  以降はそのスコープ内ではその型を使用するためには`Asparagus`とだけ書けばよくなります。

<!--
Here we create a binary crate named `backyard` that illustrates these rules. The
crate’s directory, also named `backyard`, contains these files and directories:
-->
これらの規則を説明するための例として、ここに`backyard`という名前のバイナリクレートを作成します。
クレートのディレクトリは、同じく`backyard`と名付けられ、以下のファイルとディレクトリを含んでいます:

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

<!--
The crate root file in this case is *src/main.rs*, and it contains:
-->
この場合のクレートルートファイルは *src/main.rs* であり、以下の内容を含んでいます:

<!--
<span class="filename">Filename: src/main.rs</span>
-->
<span class="filename">ファイル名: src/main.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

<!--
The `pub mod garden;` line tells the compiler to include the code it finds in
*src/garden.rs*, which is:
-->
`pub mod garden;`の行は、コンパイラに*src/garden.rs*で見つかるコードを含めるように指示します。
その内容は:

<!--
<span class="filename">Filename: src/garden.rs</span>
-->
<span class="filename">ファイル名: src/garden.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden.rs}}
```

<!--
Here, `pub mod vegetables;` means the code in *src/garden/vegetables.rs* is
included too. That code is:
-->
この`pub mod vegetables;`は *src/garden/vegetables.rs* のコードも含めると言う意味です。
そのコードは:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden/vegetables.rs}}
```

<!--
Now let’s get into the details of these rules and demonstrate them in action!
-->
それでは、これらのルールの詳細を知り、実際に試してみましょう！

<!--
### Grouping Related Code in Modules
-->
### 関連するコードをモジュールにまとめる

<!--
*Modules* let us organize code within a crate for readability and easy reuse.
Modules also allow us to control the *privacy* of items, because code within a
module is private by default. Private items are internal implementation details
not available for outside use. We can choose to make modules and the items
within them public, which exposes them to allow external code to use and depend
on them.
-->
*モジュール (module)* はクレート内のコードを整理し、可読性と再利用性を上げるのに役に立ちます。
モジュール内のコードはデフォルトでは非公開なので、モジュールは要素の*プライバシー (priavacy)* の制御も可能にします。
非公開の要素は、外部からの使用ができない内部的な実装の詳細です。
モジュールとそれらの中の要素は公開にするかどうかを選ぶことができ、公開にすると、外部のコードがそれを使用し、それに依存できるようになります。

<!--
As an example, let’s write a library crate that provides the functionality of a
restaurant. We’ll define the signatures of functions but leave their bodies
empty to concentrate on the organization of the code, rather than the
implementation of a restaurant.
-->
例えば、レストランの機能を提供するライブラリクレートを書いてみましょう。
レストランの実装ではなくコードの関係性に注目したいので、関数にシグネチャをつけますが中身は空白のままにします。

<!--
In the restaurant industry, some parts of a restaurant are referred to as
*front of house* and others as *back of house*. Front of house is where
customers are; this encompasses where the hosts seat customers, servers take
orders and payment, and bartenders make drinks. Back of house is where the
chefs and cooks work in the kitchen, dishwashers clean up, and managers do
administrative work.
-->
レストラン業界では、レストランの一部を*接客部門 (front of house)* といい、その他を*後方部門 (back of house)* といいます。
接客部門とはお客さんがいるところです。接客係がお客様を席に案内し、給仕係が注文と支払いを受け付け、バーテンダーが飲み物を作ります。
後方部門とはシェフや料理人がキッチンで働き、皿洗い係が食器を片付け、マネージャが管理業務をする場所です。

<!--
To structure our crate in this way, we can organize its functions into nested
modules. Create a new library named `restaurant` by running `cargo new
restaurant --lib`; then enter the code in Listing 7-1 into *src/lib.rs* to
define some modules and function signatures. Here’s the front of house section:
-->
私達のクレートをこのような構造にするために、関数をネストしたモジュールにまとめましょう。
`restaurant`という名前の新しいライブラリを`cargo new --lib restaurant`と実行することで作成し、リスト7-1のコードを *src/lib.rs* に書き込み、モジュールと関数のシグネチャを定義してください。
以下が接客部門です:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-1: A `front_of_house` module containing other
modules that then contain functions</span>
-->
<span class="caption">リスト7-1: `front_of_house`モジュールにその他のモジュールが含まれ、さらにそれらが関数を含んでいる</span>

<!--
We define a module with the `mod` keyword followed by the name of the module
(in this case, `front_of_house`). The body of the module then goes inside curly
brackets. Inside modules, we can place other modules, as in this case with the
modules `hosting` and `serving`. Modules can also hold definitions for other
items, such as structs, enums, constants, traits, and—as in Listing
7-1—functions.
-->
モジュールは、`mod`キーワードと、それに続くモジュールの名前（今回の場合、`front_of_house`）によって定義されます。
次にモジュールの本体が波かっこの中に入ります。
モジュールの中には、今回だと`hosting`と`serving`のように、他のモジュールをおくこともできます。
モジュールにはその他の要素の定義も置くことができます。例えば、構造体、enum、定数、トレイト、そして（リスト7-1のように）関数です。

<!--
By using modules, we can group related definitions together and name why
they’re related. Programmers using this code can navigate the code based on the
groups rather than having to read through all the definitions, making it easier
to find the definitions relevant to them. Programmers adding new functionality
to this code would know where to place the code to keep the program organized.
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
この名前のわけは、 *モジュールツリー* と呼ばれるクレートのモジュール構造の根っこ （ルート）にこれら2つのファイルの中身が`crate`というモジュールを形成するからです。

<!--
Listing 7-2 shows the module tree for the structure in Listing 7-1.
-->
リスト7-2は、リスト7-1の構造のモジュールツリーを示しています。

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
<span class="caption">リスト7-2: リスト7-1 のコードのモジュールツリー</span>

<!--
This tree shows how some of the modules nest inside one another; for example,
`hosting` nests inside `front_of_house`. The tree also shows that some modules
are *siblings* to each other, meaning they’re defined in the same module;
`hosting` and `serving` are siblings defined within `front_of_house`. If module
A is contained inside module B, we say that module A is the *child* of module B
and that module B is the *parent* of module A. Notice that the entire module
tree is rooted under the implicit module named `crate`.
-->
このツリーを見ると、どのモジュールがどのモジュールの中にネストしているのかがわかります; 例えば、`hosting`は`front_of_house`の中にネストしています。
また、いくつかのモジュールはお互いに*兄弟 (siblings)* の関係にある、つまり、同じモジュール内で定義されていることもわかります; 例えば`hosting`と`serving`は`front_of_house`で定義されている兄弟同士です。
他にも、モジュールAがモジュールBの中に入っている時、AはBの*子 (child)* であるといい、BはAの*親 (parent)* であるといいます。
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
