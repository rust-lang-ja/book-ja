<!--
## Paths for Referring to an Item in the Module Tree
-->
## モジュールツリーの要素を示すためのパス

<!--
To show Rust where to find an item in a module tree, we use a path in the same
way we use a path when navigating a filesystem. To call a function, we need to
know its path.
-->
ファイルシステムの中を移動する時と同じように、Rustにモジュールツリー内の要素を見つけるためにはどこを探せばいいのか教えるためにパスを使います。
関数を呼び出すためには、そのパスを知っていなければなりません。

<!--
A path can take two forms:
-->
パスは2つの形を取ることができます：

<!--
* An *absolute path* is the full path starting from a crate root; for code
  from an external crate, the absolute path begins with the crate name, and for
  code from the current crate, it starts with the literal `crate`.
* A *relative path* starts from the current module and uses `self`, `super`, or
  an identifier in the current module.
-->
* *絶対パス* は、クレートルートを起点とするフルパスです;
  絶対パスは、外部のクレートに属するコードに関してはそのクレート名で始まり、現在のクレートに属するコードに関してはリテラル`crate`で始まります。
* *相対パス* は、現在のモジュールを起点とし、`self`、`super`、または今のモジュール内の識別子を使います。

<!--
Both absolute and relative paths are followed by one or more identifiers
separated by double colons (`::`).
-->
絶対パスも相対パスも、その後に一つ以上の識別子がダブルコロン(`::`)で仕切られて続きます。

<!--
Returning to Listing 7-1, say we want to call the `add_to_waitlist` function.
This is the same as asking: what’s the path of the `add_to_waitlist` function?
Listing 7-3 contains Listing 7-1 with some of the modules and functions
removed.
-->
リスト7-1に戻ってみて、例えば`add_to_waitlist`関数を呼びたいとしましょう。
これはこう聞くのと同じです: `add_to_waitlist`のパスは何でしょうか？
リスト7-3は、リスト7-1からいくつかのモジュールと関数を削除したものを含んでいます。

<!--
We’ll show two ways to call the `add_to_waitlist` function from a new function
`eat_at_restaurant` defined in the crate root. These paths are correct, but
there’s another problem remaining that will prevent this example from compiling
as-is. We’ll explain why in a bit.
-->
これを使って、クレートルートに定義された新しい`eat_at_restaurant`という関数から、`add_to_waitlist`関数を呼び出す2つの方法を示しましょう。
これらのパスは正しいものですが、この例をこのままではコンパイルできなくしている問題が他に残っています。
理由はすぐに説明します。

<!--
The `eat_at_restaurant` function is part of our library crate’s public API, so
we mark it with the `pub` keyword. In the [“Exposing Paths with the `pub`
Keyword”][pub] section, we’ll go into more detail about `pub`.
-->
`eat_at_restaurant`関数はこのライブラリクレートの公開 (public) APIの1つなので、`pub`キーワードをつけておきます。
`pub`については、[「パスを`pub`キーワードで公開する」][pub]の節でより詳しく学びます。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-3: Calling the `add_to_waitlist` function using
absolute and relative paths</span>
-->
<span class="caption">リスト7-3: `add_to_waitlist` 関数を絶対パスと相対パスで呼び出す</span>

<!--
The first time we call the `add_to_waitlist` function in `eat_at_restaurant`,
we use an absolute path. The `add_to_waitlist` function is defined in the same
crate as `eat_at_restaurant`, which means we can use the `crate` keyword to
start an absolute path. We then include each of the successive modules until we
make our way to `add_to_waitlist`. You can imagine a filesystem with the same
structure: we’d specify the path `/front_of_house/hosting/add_to_waitlist` to
run the `add_to_waitlist` program; using the `crate` name to start from the
crate root is like using `/` to start from the filesystem root in your shell.
-->
`eat_at_restaurant`で最初に`add_to_waitlist`関数を呼び出す時、絶対パスを使っています。
`add_to_waitlist`関数は`eat_at_restaurant`と同じクレートで定義されているので、`crate`キーワードで絶対パスを始めることができます。
続けて、`add_to_waitlist`にたどり着くまで、後に続くモジュールを書き込んでいます。
同じ構造のファイルシステムを想像してもよいでしょう: `/front_of_house/hosting/add_to_waitlist`とパスを指定して`add_to_waitlist`を実行していることに相当します。
`crate`という名前を使ってクレートルートからスタートするというのは、`/`を使ってファイルシステムのルートからスタートするようなものです。

<!--
The second time we call `add_to_waitlist` in `eat_at_restaurant`, we use a
relative path. The path starts with `front_of_house`, the name of the module
defined at the same level of the module tree as `eat_at_restaurant`. Here the
filesystem equivalent would be using the path
`front_of_house/hosting/add_to_waitlist`. Starting with a module name means
that the path is relative.
-->
`eat_at_restaurant`で2回目に`add_to_waitlist`関数を呼び出す時、相対パスを使っています。
パスは、モジュールツリーにおいて`eat_at_restaurant`と同じ階層で定義されているモジュールである`front_of_house`からスタートします。
これはファイルシステムで`front_of_house/hosting/add_to_waitlist`というパスを使っているのに相当します。
モジュール名から始めるのは、パスが相対パスであることを意味します。

<!--
Choosing whether to use a relative or absolute path is a decision you’ll make
based on your project, and depends on whether you’re more likely to move item
definition code separately from or together with the code that uses the item.
For example, if we move the `front_of_house` module and the `eat_at_restaurant`
function into a module named `customer_experience`, we’d need to update the
absolute path to `add_to_waitlist`, but the relative path would still be valid.
However, if we moved the `eat_at_restaurant` function separately into a module
named `dining`, the absolute path to the `add_to_waitlist` call would stay the
same, but the relative path would need to be updated. Our preference in general
is to specify absolute paths because it’s more likely we’ll want to move code
definitions and item calls independently of each other.
-->
相対パスを使うか絶対パスを使うかは、プロジェクトによって決めましょう。
要素を定義するコードを、その要素を使うコードと別々に動かすか一緒に動かすか、どちらが起こりそうかによって決めるのが良いです。
例えば、`front_of_house`モジュールと`eat_at_restaurant`関数を`customer_experience`というモジュールに移動させると、`add_to_waitlist`への絶対パスを更新しないといけませんが、相対パスは有効なままです。
しかし、`eat_at_restaurant`関数だけを`dining`というモジュールに移動させると、`add_to_waitlist`への絶対パスは同じままですが、相対パスは更新しないといけないでしょう。
一般論としては、コードの定義と、その要素の呼び出しは独立に動かしたいことが多いと思われるので、絶対パスのほうが好ましいです。

<!--
Let’s try to compile Listing 7-3 and find out why it won’t compile yet! The
error we get is shown in Listing 7-4.
-->
では、リスト7-3をコンパイルしてみて、どうしてこれはまだコンパイルできないのか考えてみましょう！
エラーをリスト7-4に示しています。

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

<!--
<span class="caption">Listing 7-4: Compiler errors from building the code in
Listing 7-3</span>
-->
<span class="caption">リスト7-4: リスト7-3のコードをビルドしたときのコンパイルエラー</span>

<!--
The error messages say that module `hosting` is private. In other words, we
have the correct paths for the `hosting` module and the `add_to_waitlist`
function, but Rust won’t let us use them because it doesn’t have access to the
private sections. In Rust, all items (functions, methods, structs, enums,
modules, and constants) are private to parent modules by default. If you want
to make an item like a function or struct private, you put it in a module.

-->
エラーメッセージは、`hosting`は非公開 (private) だ、と言っています。
言い換えるなら、`hosting`モジュールと`add_to_waitlist`関数へのパスは正しいが、非公開な部分へのアクセスは許可されていないので、Rustがそれを使わせてくれないということです。
Rustでは、すべての要素（関数、メソッド、構造体、enum、モジュール、そして定数）はデフォルトでは親モジュールに対して非公開です。
関数や構造体などの要素を非公開にしたければ、モジュールの中に置いてください。

<!--
Items in a parent module can’t use the private items inside child modules, but
items in child modules can use the items in their ancestor modules. This is
because child modules wrap and hide their implementation details, but the child
modules can see the context in which they’re defined. To continue with our
metaphor, think of the privacy rules as being like the back office of a
restaurant: what goes on in there is private to restaurant customers, but
office managers can see and do everything in the restaurant they operate.
-->
親モジュールの要素は子モジュールの非公開要素を使えませんが、子モジュールの要素はその祖先モジュールの要素を使えます。
これは、子モジュールは実装の詳細を覆い隠しますが、子モジュールは自分の定義された文脈を見ることができるためです。
レストランの喩えを続けるなら、レストランの後方部門になったつもりでプライバシーのルールを考えてみてください。レストランの顧客にはそこで何が起こっているのかは非公開ですが、そこを運営するオフィスマネージャには、レストランのことは何でも見えるし何でもできるのです。

<!--
Rust chose to have the module system function this way so that hiding inner
implementation details is the default. That way, you know which parts of the
inner code you can change without breaking outer code. However, Rust does give
you the option to expose inner parts of child modules’ code to outer ancestor
modules by using the `pub` keyword to make an item public.
-->
Rustは、内部実装の詳細を隠すことが標準であるようにモジュールシステムを機能させることを選択しました。
こうすることで、内部のコードのどの部分が、外部のコードを壊すことなく変更できるのかを知ることができます。
しかし、Rustは`pub`キーワードを使って要素を公開することで、子モジュールの内部部品を外部の祖先モジュールに見せるという選択肢も与えています。


<!--
### Exposing Paths with the `pub` Keyword
-->
### パスを`pub`キーワードで公開する

<!--
Let’s return to the error in Listing 7-4 that told us the `hosting` module is
private. We want the `eat_at_restaurant` function in the parent module to have
access to the `add_to_waitlist` function in the child module, so we mark the
`hosting` module with the `pub` keyword, as shown in Listing 7-5.
-->
リスト7-4の、`hosting`モジュールが非公開だと言ってきていたエラーに戻りましょう。
親モジュールの`eat_at_restaurant`関数が子モジュールの`add_to_waitlist`関数にアクセスできるようにしたいので、`hosting`モジュールに`pub`キーワードをつけます。リスト7-5のようになります。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-5: Declaring the `hosting` module as `pub` to
use it from `eat_at_restaurant`</span>
-->
<span class="caption">リスト7-5: `hosting` モジュールを `pub` として宣言することで`eat_at_restaurant`から使う</span>

<!--
Unfortunately, the code in Listing 7-5 still results in an error, as shown in
Listing 7-6.
-->
残念ながら、リスト7-5のコードもリスト7-6に示されるようにエラーとなります。

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

<!--
<span class="caption">Listing 7-6: Compiler errors from building the code in
Listing 7-5</span>
-->
<span class="caption">リスト7-6: リスト7-5のコードをビルドしたときのコンパイルエラー</span>

<!--
What happened? Adding the `pub` keyword in front of `mod hosting` makes the
module public. With this change, if we can access `front_of_house`, we can
access `hosting`. But the *contents* of `hosting` are still private; making the
module public doesn’t make its contents public. The `pub` keyword on a module
only lets code in its ancestor modules refer to it, not access its inner code.
Because modules are containers, there’s not much we can do by only making the
module public; we need to go further and choose to make one or more of the
items within the module public as well.
-->
何が起きたのでしょう？`pub`キーワードを`mod hosting`の前に追加したことで、このモジュールは公開されました。
この変更によって、`front_of_house`にアクセスできるなら、`hosting`にもアクセスできるようになりました。
しかし`hosting`の *中身* はまだ非公開です。モジュールを公開してもその中身は公開されないのです。
モジュールに`pub`キーワードがついていても、祖先モジュールのコードはモジュールを参照できるようになるだけで、その内部のコードへのアクセスは許可されません。
モジュールはコンテナなので、モジュールを公開しただけでは、できることはあまりありません;
さらに先へ進み、モジュール内のひとつまたは複数の要素も公開することを選択する必要があります。

<!--
The errors in Listing 7-6 say that the `add_to_waitlist` function is private.
The privacy rules apply to structs, enums, functions, and methods as well as
modules.
-->
リスト7-6のエラーは`add_to_waitlist`関数が非公開だと言っています。
プライバシーのルールは、モジュール同様、構造体、enum、関数、メソッドにも適用されるのです。

<!--
Let’s also make the `add_to_waitlist` function public by adding the `pub`
keyword before its definition, as in Listing 7-7.
-->
`add_to_waitlist`の定義の前に`pub`キーワードを追加して、これも公開しましょう。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-7: Adding the `pub` keyword to `mod hosting`
and `fn add_to_waitlist` lets us call the function from
`eat_at_restaurant`</span>
-->
<span class="caption">リスト7-7: `pub`キーワードを`mod hosting`と`fn add_to_waitlist`に追加することで、`eat_at_restaurant`からこの関数を呼べるようになる</span>

<!--
Now the code will compile! To see why adding the `pub` keyword lets us use
these paths in `add_to_waitlist` with respect to the privacy rules, let’s look
at the absolute and the relative paths.
-->
これでこのコードはコンパイルできます！
どうして`pub`キーワードを追加することで`add_to_waitlist`のそれらのパスを使えるようになるのか、
プライバシールールの観点から確認するために、絶対パスと相対パスを見てみましょう。

<!--
In the absolute path, we start with `crate`, the root of our crate’s module
tree. The `front_of_house` module is defined in the crate root. While
`front_of_house` isn’t public, because the `eat_at_restaurant` function is
defined in the same module as `front_of_house` (that is, `eat_at_restaurant`
and `front_of_house` are siblings), we can refer to `front_of_house` from
`eat_at_restaurant`. Next is the `hosting` module marked with `pub`. We can
access the parent module of `hosting`, so we can access `hosting`. Finally, the
`add_to_waitlist` function is marked with `pub` and we can access its parent
module, so this function call works!
-->
絶対パスは、クレートのモジュールツリーのルートである`crate`から始まります。
クレートルートの中に`front_of_house`が定義されています。
`front_of_house`は公開されていませんが、`eat_at_restaurant`関数は`front_of_house`と同じモジュール内で定義されている（つまり、`eat_at_restaurant`と`front_of_house`は兄弟な）ので、`eat_at_restaurant`から`front_of_house`を参照することができます。
次は`pub`の付いた`hosting`モジュールです。
`hosting`の親モジュールにアクセスできるので、`hosting`にもアクセスできます。
最後に、`add_to_waitlist`関数は`pub`が付いており、私達はその親モジュールにアクセスできるので、この関数呼び出しはうまく行くというわけです。

<!--
In the relative path, the logic is the same as the absolute path except for the
first step: rather than starting from the crate root, the path starts from
`front_of_house`. The `front_of_house` module is defined within the same module
as `eat_at_restaurant`, so the relative path starting from the module in which
`eat_at_restaurant` is defined works. Then, because `hosting` and
`add_to_waitlist` are marked with `pub`, the rest of the path works, and this
function call is valid!
-->
相対パスについても、最初のステップを除けば同じ理屈です。パスをクレートルートから始めるのではなくて、`front_of_house`から始めるのです。
`front_of_house`モジュールは`eat_at_restaurant`と同じモジュールで定義されているので、`eat_at_restaurant`が定義されている場所からの相対パスが使えます。
そして、`hosting`と`add_to_waitlist`は`pub`が付いていますから、残りのパスについても問題はなく、この関数呼び出しは有効というわけです。

<!--
If you plan on sharing your library crate so other projects can use your code,
your public API is your contract with users of your crate that determines how
they can interact with your code. There are many considerations around managing
changes to your public API to make it easier for people to depend on your
crate. These considerations are out of the scope of this book; if you’re
interested in this topic, see [The Rust API Guidelines][api-guidelines].
-->
他のプロジェクトからあなたのコードを利用できるようにライブラリクレートを共有するつもりなら、
公開APIは、クレート利用者があなたのコードとどう相互作用できるかを決定する、クレート利用者との契約となります。
人々があなたのクレートに依存しやすくするためには、公開APIへの変更の管理に関する多数の考慮事項があります。
これらの考慮事項はこの本のスコープ外です; このトピックに興味がある場合は、[The Rust API Guidelines][api-guidelines]を参照してください。

<!--
> #### Best Practices for Packages with a Binary and a Library
>
> We mentioned a package can contain both a *src/main.rs* binary crate root as
> well as a *src/lib.rs* library crate root, and both crates will have the
> package name by default. Typically, packages with this pattern of containing
> both a library and a binary crate will have just enough code in the binary
> crate to start an executable that calls code with the library crate. This
> lets other projects benefit from the most functionality that the package
> provides, because the library crate’s code can be shared.
>
> The module tree should be defined in *src/lib.rs*. Then, any public items can
> be used in the binary crate by starting paths with the name of the package.
> The binary crate becomes a user of the library crate just like a completely
> external crate would use the library crate: it can only use the public API.
> This helps you design a good API; not only are you the author, you’re also a
> client!
>
> In [Chapter 12][ch12], we’ll demonstrate this organizational
> practice with a command-line program that will contain both a binary crate
> and a library crate.
-->

> #### バイナリとライブラリの両方を持つパッケージでのベストプラクティス
>
> パッケージは*src/main.rs*バイナリクレートルートと*src/lib.rs*ライブラリクレートルートの両方を含むことができ、
> デフォルトでは両方のクレートがパッケージ名を持つことを説明しました。
> 典型的には、ライブラリとバイナリのクレート両方を含むこのパターンのパッケージでは、バイナリクレートには
> ライブラリクレートのコードを呼び出して、実行可能形式を開始するために必要な最低限のコードを持たせます。
> こうすることで、ライブラリクレートのコードは共有できるので、
> 他のプロジェクトはこのパッケージが提供するほとんどの機能を活用することができます。
>
> モジュールツリーは*src/lib.rs*内に定義してください。
> その場合、バイナリクレートからは、パッケージ名でから始まるパスを使って公開された要素を使用することができます。
> バイナリクレートは、そのライブラリクレートを利用する完全に外部のクレートとまったく同じように、
> ライブラリクレートの利用者になります: 公開APIしか使用することができません。
> これは良いAPIを設計する助けになります; あなたは作者であるだけでなく、利用者にもなるのです！
>
> [第12章][ch12]では、バイナリクレートとライブラリクレートの両方を含むコマンドラインプログラムを使って、
> この整理法の実践を示します。

<!--
### Starting Relative Paths with `super`
-->
### 相対パスを`super`で始める

<!--
We can construct relative paths that begin in the parent module, rather than
the current module or the crate root, by using `super` at the start of the
path. This is like starting a filesystem path with the `..` syntax. Using
`super` allows us to reference an item that we know is in the parent module,
which can make rearranging the module tree easier when the module is closely
related to the parent, but the parent might be moved elsewhere in the module
tree someday.
-->
現在のモジュールやクレートルートではなく、親モジュールから始まる相対パスなら、`super`を最初につけることで構成できます。
ファイルシステムパスを`..`構文で始めるのに似ています。
`super`を使用することで、親モジュールにあることを知っている要素を参照することができます。
これにより、そのモジュールが親と密接に関連しているが、いつか親がモジュールツリー内のどこかに移動されるかもしれないという場合に、モジュールツリーを再編成するのがより簡単になるかもしれません。

<!--
Consider the code in Listing 7-8 that models the situation in which a chef
fixes an incorrect order and personally brings it out to the customer. The
function `fix_incorrect_order` defined in the `back_of_house` module calls the
function `deliver_order` defined in the parent module by specifying the path to
`deliver_order` starting with `super`:
-->
シェフが間違った注文を修正し、自分でお客さんに持っていくという状況をモデル化している、リスト7-8を考えてみてください。
`back_of_house`モジュールで定義されている`fix_incorrect_order`関数は、親モジュールで定義されている`deliver_order`関数を呼び出すために、`super`から始まる`deliver_order`関数へのパスを使っています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-8: Calling a function using a relative path
starting with `super`</span>
-->
<span class="caption">リスト7-8: `super` で始まる相対パスを使って関数を呼び出す</span>

<!--
The `fix_incorrect_order` function is in the `back_of_house` module, so we can
use `super` to go to the parent module of `back_of_house`, which in this case
is `crate`, the root. From there, we look for `deliver_order` and find it.
Success! We think the `back_of_house` module and the `deliver_order` function
are likely to stay in the same relationship to each other and get moved
together should we decide to reorganize the crate’s module tree. Therefore, we
used `super` so we’ll have fewer places to update code in the future if this
code gets moved to a different module.
-->

`fix_incorrect_order`関数は`back_of_house`モジュールの中にあるので、`super`を使って`back_of_house`の親モジュールにいけます。親モジュールは、今回の場合ルートである`crate`です。
そこから、`deliver_order`を探し、見つけ出します。
成功！
もしクレートのモジュールツリーを再編成することにした場合でも、`back_of_house`モジュールと`deliver_order`関数は同じ関係性で有り続け、一緒に動くように思われます。
そのため、`super`を使うことで、将来このコードが別のモジュールに移動するとしても、更新する場所が少なくて済むようにしました。

<!--
### Making Structs and Enums Public
-->
### 構造体とenumを公開する

<!--
We can also use `pub` to designate structs and enums as public, but there are a
few details extra to the usage of `pub` with structs and enums. If we use `pub`
before a struct definition, we make the struct public, but the struct’s fields
will still be private. We can make each field public or not on a case-by-case
basis. In Listing 7-9, we’ve defined a public `back_of_house::Breakfast` struct
with a public `toast` field but a private `seasonal_fruit` field. This models
the case in a restaurant where the customer can pick the type of bread that
comes with a meal, but the chef decides which fruit accompanies the meal based
on what’s in season and in stock. The available fruit changes quickly, so
customers can’t choose the fruit or even see which fruit they’ll get.
-->
構造体やenumも`pub`を使って公開するよう指定できますが、構造体とenumでの`pub`の使用に関しては追加の細目がいくつかあります。
構造体定義の前に`pub`を使うと、構造体は公開されますが、構造体のフィールドは非公開のままなのです。
それぞれのフィールドを公開するか否かを個々に決められます。
リスト7-9では、公開の`toast`フィールドと、非公開の`seasonal_fruit`フィールドをもつ公開の`back_of_house::Breakfast`構造体を定義しました。
これは、例えば、レストランで、お客さんが食事についてくるパンの種類は選べるけれど、食事についてくるフルーツは季節と在庫に合わせてシェフが決める、という状況をモデル化しています。
提供できるフルーツはすぐに変わるので、お客さんはフルーツを選ぶどころかどんなフルーツが提供されるのか知ることもできません。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-9: A struct with some public fields and some
private fields</span>
-->
<span class="caption">リスト7-9: 公開のフィールドと非公開のフィールドとを持つ構造体</span>

<!--
Because the `toast` field in the `back_of_house::Breakfast` struct is public,
in `eat_at_restaurant` we can write and read to the `toast` field using dot
notation. Notice that we can’t use the `seasonal_fruit` field in
`eat_at_restaurant` because `seasonal_fruit` is private. Try uncommenting the
line modifying the `seasonal_fruit` field value to see what error you get!
-->
`back_of_house::Breakfast`の`toast`フィールドは公開されているので、`eat_at_restaurant`において`toast`をドット記法を使って読み書きできます。
`seasonal_fruit`は非公開なので、`eat_at_restaurant`において`seasonal_fruit`は使えないということに注意してください。
`seasonal_fruit`を修正している行のコメントを外して、どのようなエラーが得られるか試してみてください！

<!--
Also, note that because `back_of_house::Breakfast` has a private field, the
struct needs to provide a public associated function that constructs an
instance of `Breakfast` (we’ve named it `summer` here). If `Breakfast` didn’t
have such a function, we couldn’t create an instance of `Breakfast` in
`eat_at_restaurant` because we couldn’t set the value of the private
`seasonal_fruit` field in `eat_at_restaurant`.
-->
また、`back_of_house::Breakfast`は非公開のフィールドを持っているので、`Breakfast`のインスタンスを作成 (construct) する公開された関連関数が構造体によって提供されている必要があります（ここでは`summer`と名付けました）。
もし`Breakfast`にそのような関数がなかったら、`eat_at_restaurant`において非公開である`seasonal_fruit`の値を設定できないので、`Breakfast`のインスタンスを作成できません。

<!--
In contrast, if we make an enum public, all of its variants are then public. We
only need the `pub` before the `enum` keyword, as shown in Listing 7-10.
-->
一方で、enumを公開すると、そのヴァリアントはすべて公開されます。
リスト7-10に示されているように、`pub`は`enum`キーワードの前にだけおけばよいのです。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

<!--
<span class="caption">Listing 7-10: Designating an enum as public makes all its
variants public</span>
-->
<span class="caption">リスト7-10: enumを公開に指定することはそのヴァリアントをすべて公開にする</span>

<!--
Because we made the `Appetizer` enum public, we can use the `Soup` and `Salad`
variants in `eat_at_restaurant`.
-->
`Appetizer`というenumを公開したので、`Soup`と`Salad`というヴァリアントも`eat_at_restaurant`で使えます。

<!--
Enums aren’t very useful unless their variants are public; it would be annoying
to have to annotate all enum variants with `pub` in every case, so the default
for enum variants is to be public. Structs are often useful without their
fields being public, so struct fields follow the general rule of everything
being private by default unless annotated with `pub`.
-->
enumはヴァリアントが公開されてないとあまり便利ではないのですが、毎回enumのすべてのヴァリアントに`pub`をつけるのは面倒なので、enumのヴァリアントは標準で公開されるようになっているのです。
構造体はフィールドが公開されていなくても便利なことが多いので、構造体のフィールドは、`pub`がついてない限り標準で非公開という通常のルールに従うわけです。

<!--
There’s one more situation involving `pub` that we haven’t covered, and that is
our last module system feature: the `use` keyword. We’ll cover `use` by itself
first, and then we’ll show how to combine `pub` and `use`.
-->
まだ勉強していない、`pub`の関わるシチュエーションがもう一つあります。モジュールシステムの最後の機能、`use`キーワードです。
`use`自体の勉強をした後、`pub`と`use`を組み合わせる方法についてお見せします。

<!--
[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html
-->
[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#パスをpubキーワードで公開する
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html
