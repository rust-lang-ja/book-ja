<!-- ## Controlling Visibility with `pub` -->

## `pub`で公開するか制御する

<!-- We resolved the error messages shown in Listing 7-4 by moving the `network` and -->
<!-- `network::server` code into the *src/network/mod.rs* and -->
<!-- *src/network/server.rs* files, respectively. At that point, `cargo build` was -->
<!-- able to build our project, but we still get warning messages about the -->
<!-- `client::connect`, `network::connect`, and `network::server::connect` functions -->
<!-- not being used: -->

リスト7-4に示したエラーメッセージを`network`と`network::server`のコードを、
*src/network/mod.rs*と*src/network/server.rs*ファイルにそれぞれ移動することで解決しました。
その時点で`cargo build`はプロジェクトをビルドできましたが、`client::connect`と`network::connect`と`network::server::connect`関数が、
まだ使用されていないという警告メッセージが出ていました:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
src/client.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

<!-- So why are we receiving these warnings? After all, we’re building a library -->
<!-- with functions that are intended to be used by our *users*, not necessarily by -->
<!-- us within our own project, so it shouldn’t matter that these `connect` -->
<!-- functions go unused. The point of creating them is that they will be used by -->
<!-- another project, not our own. -->

では、何故このような警告を受けているのでしょうか？結局のところ、必ずしも自分のプロジェクト内ではなく、
*利用者*に利用されることを想定した関数を含むライブラリを構成しているので、
これらの`connect`関数が使用されていかないということは、問題になるはずはありません。
これらの関数を生成することの要点は、自分ではなく、他のプロジェクトで使用することにあるのです。

<!-- To understand why this program invokes these warnings, let’s try using the -->
<!-- `connect` library from another project, calling it externally. To do that, -->
<!-- we’ll create a binary crate in the same directory as our library crate by -->
<!-- making a *src/main.rs* file containing this code: -->

このプログラムがこのような警告を引き起こす理由を理解するために、外部から`connect`ライブラリを呼び出して、
他のプロジェクトからこれを使用してみましょう。そうするには、以下のようなコードを含む*src/main.rs*を作成して、
ライブラリクレートと同じディレクトリにバイナリクレートを作成します。

<!-- <span class="filename">Filename: src/main.rs</span> -->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
extern crate communicator;

fn main() {
    communicator::client::connect();
}
```

<!-- We use the `extern crate` command to bring the `communicator` library crate -->
<!-- into scope. Our package now contains *two* crates. Cargo treats *src/main.rs* -->
<!-- as the root file of a binary crate, which is separate from the existing library -->
<!-- crate whose root file is *src/lib.rs*. This pattern is quite common for -->
<!-- executable projects: most functionality is in a library crate, and the binary -->
<!-- crate uses that library crate. As a result, other programs can also use the -->
<!-- library crate, and it’s a nice separation of concerns. -->

`extern crate`コマンドを使用して、`communicator`ライブラリクレートをスコープに導入しています。
パッケージには*2つ*のクレートが含まれるようになりました。Cargoは、*src/main.rs*をバイナリクレートのルートファイルとして扱い、
これはルートファイルが*src/lib.rs*になる既存のライブラリクレートとは区別されます。このパターンは、
実行形式プロジェクトで非常に一般的です: ほとんどの機能はライブラリクレートにあり、バイナリクレートはそれを使用するわけです。
結果として、他のプログラムもまたこのライブラリクレートを使用でき、良い責任の分離になるわけです。

<!-- From the point of view of a crate outside the `communicator` library looking -->
<!-- in, all the modules we’ve been creating are within a module that has the same -->
<!-- name as the crate, `communicator`. We call the top-level module of a crate the -->
<!-- *root module*. -->

`communicator`ライブラリの外部のクレートが検索するという観点から言えば、これまでに作ってきたモジュールは全て、
`communicator`というクレートと同じ名前を持つモジュール内にあります。クレートのトップ階層のモジュールを、
*ルートモジュール*と呼びます。

<!-- Also note that even if we’re using an external crate within a submodule of our -->
<!-- project, the `extern crate` should go in our root module (so in *src/main.rs* -->
<!-- or *src/lib.rs*). Then, in our submodules, we can refer to items from external -->
<!-- crates as if the items are top-level modules. -->

プロジェクトのサブモジュール内で外部クレートを使用しているとしても、`extern crate`はルートモジュール(つまり、*src/main.rs*、
または*src/lib.rs*)に書くべきということにも、注目してください。それから、
サブモジュールで外部クレートの要素をトップ階層のモジュールかのように参照できるわけです。

<!-- Right now, our binary crate just calls our library’s `connect` function from -->
<!-- the `client` module. However, invoking `cargo build` will now give us an error -->
<!-- after the warnings: -->

現状、バイナリクレートは、`client`モジュールからライブラリの`connect`関数を呼び出しているだけです。
ところが、`cargo build`を呼び出すと、警告の後にエラーが発生します:

```text
error: module `client` is private
(エラー: `client`モジュールは非公開です)
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

<!-- Ah ha! This error tells us that the `client` module is private, which is the -->
<!-- crux of the warnings. It’s also the first time we’ve run into the concepts of -->
<!-- *public* and *private* in the context of Rust. The default state of all code in -->
<!-- Rust is private: no one else is allowed to use the code. If you don’t use a -->
<!-- private function within your program, because your program is the only code -->
<!-- allowed to use that function, Rust will warn you that the function has gone -->
<!-- unused. -->

ああ！このエラーは、`client`モジュールが非公開であることを教えてくれ、それが警告の肝だったわけです。
Rustの文脈において、*公開*とか*非公開*という概念にぶち当たったのは、これが初めてでもあります。
全コードの初期状態は、非公開です: 誰も他の人はコードを使用できないわけです。プログラム内で非公開の関数を使用していなければ、
自分のプログラムだけがその関数を使用することを許可された唯一のコードなので、コンパイラは関数が未使用と警告してくるのです。

<!-- After we specify that a function like `client::connect` is public, not only -->
<!-- will our call to that function from our binary crate be allowed, but the -->
<!-- warning that the function is unused will go away. Marking a function as public -->
<!-- lets Rust know that the function will be used by code outside of our program. -->
<!-- Rust considers the theoretical external usage that’s now possible as the -->
<!-- function “being used.” Thus, when something is marked public, Rust will not -->
<!-- require that it be used in our program and will stop warning that the item is -->
<!-- unused. -->

`client::connect`のような関数を公開にすると指定した後は、バイナリクレートからその関数への呼び出しが許可されるだけでなく、
関数が未使用であるという警告も消え去るわけです。関数を公開にすれば、コンパイラは、
関数が自分のプログラム外のコードからも使用されることがあると知ります。コンパイラは、
関数が「使用されている」という架空の外部使用の可能性を考慮してくれます。それ故に、何かが公開とマークされれば、
コンパイラはそれが使用されるべきという要求をなくし、その要素が未使用という警告も止めるのです。

<!-- ### Making a Function Public -->

### 関数を公開にする

<!-- To tell Rust to make something public, we add the `pub` keyword to the start of -->
<!-- the declaration of the item we want to make public. We’ll focus on fixing the -->
<!-- warning that indicates `client::connect` has gone unused for now, as well as -->
<!-- the `` module `client` is private `` error from our binary crate. Modify -->
<!-- *src/lib.rs* to make the `client` module public, like so: -->

コンパイラに何かを公開すると指示するには、公開にしたい要素の定義の先頭に`pub`キーワードを追記します。
今は、`client::connect`が未使用であるとする警告とバイナリークレートの``モジュール`client`が非公開である``エラーの解消に努めます。
*src/lib.rs*を弄って、`client`モジュールを公開にしてください。そう、こんな感じに:

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
pub mod client;

mod network;
```

<!-- The `pub` keyword is placed right before `mod`. Let’s try building again: -->

`pub`キーワードは、`mod`の直前に配置されています。再度ビルドしてみましょう:

```text
error: function `connect` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

<!-- Hooray! We have a different error! Yes, different error messages are a cause -->
<!-- for celebration. The new error shows `` function `connect` is private ``, so -->
<!-- let’s edit *src/client.rs* to make `client::connect` public too: -->

やった！違うエラーになりました！そうです、別のエラーメッセージは、祝杯を上げる理由になるのです。
新エラーは、``関数`connect`は非公開です``と示しているので、*src/client.rs*を編集して、
`client::connect`も公開にしましょう:

<!-- <span class="filename">Filename: src/client.rs</span> -->

<span class="filename">ファイル名: src/client.rs</span>

```rust
pub fn connect() {
}
```

<!-- Now run `cargo build` again: -->

さて、再び、`cargo build`を走らせてください:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

<!-- The code compiled, and the warning about `client::connect` not being used is -->
<!-- gone! -->

コードのコンパイルが通り、`client:connect`が未使用という警告はなくなりました！

<!-- 3行目、could be alerting you to code you...のところがちょっと不安 -->

<!-- Unused code warnings don’t always indicate that an item in your code needs to -->
<!-- be made public: if you *didn’t* want these functions to be part of your public -->
<!-- API, unused code warnings could be alerting you to code you no longer need that -->
<!-- you can safely delete. They could also be alerting you to a bug if you had just -->
<!-- accidentally removed all places within your library where this function is -->
<!-- called. -->

コード未使用警告が必ずしも、コード内の要素を公開にしなければならないことを示唆しているわけではありません:
これらの関数を公開APIの一部にしたく*なかった*ら、未使用コード警告がもう必要なく、安全に削除できるコードに注意を向けてくれる可能性もあります。
また未使用コード警告は、ライブラリ内でこの関数を呼び出している箇所全てを誤って削除した場合に、
バグに目を向けさせてくれる可能性もあります。

<!-- But in this case, we *do* want the other two functions to be part of our -->
<!-- crate’s public API, so let’s mark them as `pub` as well to get rid of the -->
<!-- remaining warnings. Modify *src/network/mod.rs* to look like the following: -->

しかし今回は、本当に他の2つの関数もクレートの公開APIにしたいので、これも`pub`とマークして残りの警告を除去しましょう。
*src/network/mod.rs*を変更して以下のようにしてください:

<!-- <span class="filename">Filename: src/network/mod.rs</span> -->

<span class="filename">ファイル名: src/network/mod.rs</span>

```rust,ignore
pub fn connect() {
}

mod server;
```

<!-- Then compile the code: -->

そして、コードをコンパイルします:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | pub fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

<!-- Hmmm, we’re still getting an unused function warning, even though -->
<!-- `network::connect` is set to `pub`. The reason is that the function is public -->
<!-- within the module, but the `network` module that the function resides in is not -->
<!-- public. We’re working from the interior of the library out this time, whereas -->
<!-- with `client::connect` we worked from the outside in. We need to change -->
<!-- *src/lib.rs* to make `network` public too, like so: -->

んんー、`nework::connect`は`pub`に設定されたにもかかわらず、まだ未使用関数警告が出ます。
その理由は、関数はモジュール内で公開になったものの、関数が存在する`network`モジュールは公開ではないからです。
今回は、ライブラリの内部から外に向けて作業をした一方、`client::connect`では、外から内へ作業をしていました。
*src/lib.rs*を変えて`network`も公開にする必要があります。以下のように:

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
pub mod client;

pub mod network;
```

<!-- Now when we compile, that warning is gone: -->

これでコンパイルすれば、あの警告はなくなります:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

<!-- Only one warning is left! Try to fix this one on your own! -->

残る警告は1つ！自分で解消してみましょう！

<!-- ### Privacy Rules -->

### プライバシー規則

<!-- Overall, these are the rules for item visibility: -->

まとめると、要素の公開性は以下のようなルールになります:

<!-- 1. If an item is public, it can be accessed through any of its parent modules. -->
<!-- 2. If an item is private, it can be accessed only by its immediate parent -->
<!--    module and any of the parent’s child modules. -->

1. 要素が公開なら、どの親モジュールを通してもアクセス可能です。
2. 要素が非公開なら、直接の親モジュールとその親の子モジュールのみアクセスできます。

<!-- ### Privacy Examples -->

### プライバシー例

<!-- Let’s look at a few more privacy examples to get some practice. Create a new -->
<!-- library project and enter the code in Listing 7-5 into your new project’s -->
<!-- *src/lib.rs*: -->

もうちょっと鍛錬を得るために、もういくつかプライバシー例を見てみましょう。新しいライブラリプロジェクトを作成し、
リスト7-5のコードを新規プロジェクトの*src/lib.rs*に入力してください:

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
```

<!-- <span class="caption">Listing 7-5: Examples of private and public functions, -->
<!-- some of which are incorrect</span> -->

<span class="caption">リスト7-5: 公開と非公開関数の例。不正なものもあります</span>

<!-- Before you try to compile this code, make a guess about which lines in the -->
<!-- `try_me` function will have errors. Then, try compiling the code to see whether -->
<!-- you were right, and read on for the discussion of the errors! -->

このコードをコンパイルする前に、`try_me`関数のどの行がエラーになるか当ててみてください。
それからコンパイルを試して、合ってたかどうか確かめ、エラーの議論を読み進めてください！

<!-- #### Looking at the Errors -->

#### エラーを確かめる

<!-- The `try_me` function is in the root module of our project. The module named -->
<!-- `outermost` is private, but the second privacy rule states that the `try_me` -->
<!-- function is allowed to access the `outermost` module because `outermost` is in -->
<!-- the current (root) module, as is `try_me`. -->

`try_me`関数は、プロジェクトのルートモジュールに存在しています。`outermost`という名前のモジュールは非公開ですが、
プライバシー規則の2番目にある通り、`try_me`そのままに、`outermost`は現在(ルート)のモジュールなので、
`try_me`関数は、`outermost`にアクセスすることを許可されるのです。

<!-- The call to `outermost::middle_function` will work because `middle_function` is -->
<!-- public, and `try_me` is accessing `middle_function` through its parent module -->
<!-- `outermost`. We determined in the previous paragraph that this module is -->
<!-- accessible. -->

`middle_function`は公開なので、`outermost::middle_function`という呼び出しも動作し、
`try_me`は`middle_function`にその親モジュールの`outermost`を通してアクセスしています。
前の段落でこのモジュールは、アクセス可能と決定しました。

<!-- The call to `outermost::middle_secret_function` will cause a compilation error. -->
<!-- `middle_secret_function` is private, so the second rule applies. The root -->
<!-- module is neither the current module of `middle_secret_function` (`outermost` -->
<!-- is), nor is it a child module of the current module of `middle_secret_function`. -->

`outermost::middle_secret_function`の呼び出しは、コンパイルエラーになります。
`middle_secret_function`は非公開なので、2番目の規則が適用されます。ルートモジュールは、
`middle_secret_function`の現在のモジュール(`outermost`がそうです)でも、`middle_secret_function`のモジュールの子供でもないのです。

<!-- The module named `inside` is private and has no child modules, so it can only -->
<!-- be accessed by its current module `outermost`. That means the `try_me` function -->
<!-- is not allowed to call `outermost::inside::inner_function` or -->
<!-- `outermost::inside::secret_function`. -->

`inside`という名前のモジュールは非公開で子モジュールを持たないので、現在のモジュールである`outermost`からのみアクセスできます。
つまり、`try_me`関数は、`outermost::inside::inner_function`も`outermost::inside::secret_function`も呼び出すことを許されないのです。

<!-- #### Fixing the Errors -->

#### エラーを修正する

<!-- Here are some suggestions for changing the code in an attempt to fix the -->
<!-- errors. Before you try each one, make a guess as to whether it will fix the -->
<!-- errors, and then compile the code to see whether or not you’re right, using the -->
<!-- privacy rules to understand why. -->

エラーを修正しようとする過程でできるコード変更案は、以下の通りです。各々試してみる前に、
エラーを解消できるか当ててみてください。それからコンパイルして正しかったか間違っていたか確かめ、
プライバシー規則を使用して理由を理解してください。

<!-- * What if the `inside` module was public? -->
<!-- * What if `outermost` was public and `inside` was private? -->
<!-- * What if, in the body of `inner_function`, you called -->
<!--   `::outermost::middle_secret_function()`? (The two colons at the beginning mean -->
<!--   that we want to refer to the modules starting from the root module.) -->

* `inside`モジュールが公開だったらどうだろうか？
* `outermost`が公開で、`inside`が非公開ならどうだろうか？
* `inner_function`の本体で`::outermost::middle_secret_function()`を呼び出したらどうだろうか？
  (頭の二つのコロンは、ルートモジュールから初めてモジュールを参照したいということを意味します)

<!-- Feel free to design more experiments and try them out! -->

自由にもっと実験を企てて、試してみてくださいね！

<!-- Next, let’s talk about bringing items into scope with the `use` keyword. -->

今度は、`use`キーワードで要素をスコープに導入する話をしましょう。
