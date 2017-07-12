# 数当てゲーム

実物のプロジェクトに一緒に取り組むことで、Rustの世界へ飛び込みましょう！
この章では、実際のプログラム内で使用する方法を通じて、いくつかの一般的なRustの概念に触れます。
let文、match式、メソッド、関連関数、外部クレートの使用などについて学ぶでしょう！
後ほどの章でこれらの概念について深く知ることになります。この章では、基礎部分だけにしましょう。

古典的な初心者向けのプログラミング問題を実装してみましょう: 数当てゲームです。 
これは以下のように動作します: プログラムは1から100までの乱数整数を生成します。
さらにプレーヤーに予想を入力するよう促します。予想を入力し終わったら、プログラムは、
その予想が少なすぎたか多すぎたかを出力します。予想が当たっていれば、ゲームが祝福してくれ、
そのまま終了します。

## 新規プロジェクトの立ち上げ

新規プロジェクトを立ち上げるには、第1章で作成した*projects*ディレクトリに行き、
Cargoを使って新規プロジェクトを作成します。そう、以下のように:

```text
$ cargo new guessing_game --bin
$ cd guessing_game
```

最初のコマンド`cargo new`は、プロジェクト名を第1引数に取ります(`guessing_game`ですね)。
`--bin`というフラグは、Cargoにバイナリ生成プロジェクトを作成させます。第1章のものと似ていますね。
2番目のコマンドで新規プロジェクトのディレクトリに移動します。

生成された*Cargo.toml*ファイルを見てみましょう:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["名前 <you@example.com>"]

[dependencies]
```

もし、Cargoがあなたの環境から自動取得した書き手情報が間違っていたら、
ファイルを編集して保存し直してください。

第1章でも見かけたように、`cargo new`コマンドは、"Hello, world!"プログラムを生成してくれます。
*src/main.rs*ファイルをチェックしてみましょう:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

さて、この"Hello, world!"プログラムをコンパイルし、`cargo run`コマンドを使用して、
以前と同じように動かしてみましょう:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/debug/guessing_game`
Hello, world!
```

`run`コマンドは、プロジェクトに段階を踏んで取り掛かる必要がある場合に有用であり、
このゲームはその類のプロジェクトになります。
つまり、次のステップに進む前に各段階を急速にテストする必要があるわけです。

*src/main.rs*ファイルを開き直しましょう。ここにすべてのコードを書いてきます。

## 予想を処理する

プログラムの最初のパートは、ユーザに入力を求め、その入力を処理し、予期した形態になっていることを確認します。
手始めにプレーヤーが予想を入力できるようにしましょう。
リスト2-1のコードを*src/main.rs*に入力してください。

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::io;

fn main() {
    println!("Guess the number!");          // 数を当ててごらん

    println!("Please input your guess.");   // ほら、予想を入力してね

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");     // 行の読み込みに失敗しました

    println!("You guessed: {}", guess);     // 次のように予想しました: {}
}
```

<span class="caption">リスト2-1: ユーザに予想を入力してもらい、それを出力するコード</span>

> 注釈: The programming language Rust第1版の翻訳者によると、ソースコードのコメント中以外に
> 日本語文字があるとコンパイルに失敗することがあるそうなので、文字列の英語は、コメントに和訳を載せます。
> また、重複する内容の場合には、最初の1回だけ掲載するようにします。

このコードには、たくさんの情報が詰め込まれてますね。なので、少しずつ噛み砕いていきましょう。
ユーザ入力を受け付け、結果を出力するためには、`io`(入/出力)ライブラリをスコープに導入する必要があります。
`io`ライブラリは、標準ライブラリ(`std`として知られています)に存在します。:

```rust,ignore
use std::io;
```

標準では、コンパイラは、[*prelude*][prelude]<!-- ignored -->に存在するいくつかの型しかプログラムで使用させてくれません。
もし、使用したい型がpreludeにない場合は、`use`文で明示的にその型をスコープに導入する必要があります。
`std::io`ライブラリを使用することで、実用的な`入出力`関連の機能を使用することができます。
ユーザ入力を受け付ける機能も含めてね。

[prelude]: ../../std/prelude/index.html

第1章で示した通り、`main`関数がプログラムへのエントリーポイント(スタート地点)になります:

```rust,ignore
fn main() {
```

`fn`記法が関数を新しく定義し、`()`は引数がないことを示し、`{`が関数本体のスタート地点になります。

また、第1章で学んだように、`println!`マクロは、文字列を画面に表示するマクロになります:

```rust,ignore
println!("Guess the number!");

println!("Please input your guess.");
```

このコードは、このゲームが何かを出力し、ユーザに入力を求めるだけです。

### 値を変数に保持する

次に、ユーザ入力を保持する場所を作りましょう。こんな感じに:

```rust,ignore
let mut guess = String::new();
```

さあ、プログラムが面白くなってきましたね。このたった1行でいろんなことが起きています。
これが`let`文であることに注目してください。これを使用して*変数*を生成しています。
こちらは、別の例です:

```rust,ignore
let foo = bar;
```

この行では、`foo`という名前の新しい変数を作成し、`bar`の値に束縛しています。
Rustでは、変数は標準で不変(immutable)です。以下の例には、変数名の前に`mut`修飾子をつけて
変数を可変にする方法が示されています:

```rust
let foo = 5; // immutable
let mut bar = 5; // mutable
```

> 注釈: `//`という記法は、行末まで続くコメントを記述します。
> コンパイラは、コメントを一切無視します。

さあ、`let mut guess`が`guess`という名前の可変変数を導入するとわかりましたね。
イコール記号(`=`)の逆側には、変数`guess`が束縛される値があります。この値は、今回の場合、
`String::new`関数の呼び出し結果であり、この関数は、`String`型のオブジェクトを返します。
[`String`][string]<!-- ignore -->型は、標準ライブラリによって提供される文字列型で、
サイズ可変、UTF-8エンコードされたテキスト破片になります。

[string]: ../../std/string/struct.String.html

`::new`行にある`::`という記法は、`new`が`String`型の*関連付け関数*であることを表しています。
関連付け関数とは、`String`型の特定のオブジェクトよりも型(この場合は`String`)に対して
実装された関数のことであり、*静的メソッド*と呼ばれる言語もあります。

この`new`関数は、新しく空の`String`オブジェクトを生成します。`new`関数は、いろんな型に見られます。
なぜなら、何らかの新規値を生成する関数にとってありふれた名前だからです。

まとめると、`let mut guess = String::new();`という行は、現在、新規で空の`String`オブジェクトに束縛されている
可変変数を作っているわけです。ふう！

プログラムの1行目で、`use std::io`として、標準ライブラリから入/出力機能を取り込んだことを思い出してください。
今度は、`io`型の`stdin`関連付け関数を呼び出しましょう:

```rust,ignore
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

仮に、プログラムの冒頭で`use std::io`としていなければ、この関数呼び出しは、`std::io::stdin`と記述していたでしょう。
この`stdin`関数は、 [`std::io::Stdin`][iostdin]<!-- ignore -->オブジェクトを返し、この型は、
ターミナルの標準入力へのハンドルを表す型になります。

[iostdin]: ../../std/io/struct.Stdin.html

その次のコード破片、`.read_line(&mut guess)`は、標準入力ハンドルの[`read_line`][read_line]<!-- ignore -->
メソッドを呼び出して、ユーザから入力を受け付けます。また、`read_line`メソッドに対して、引数を一つ渡していますね: `&mut
guess`.

[read_line]: ../../std/io/struct.Stdin.html#method.read_line

`read_line`メソッドの仕事は、ユーザが標準入力したものすべてを取り出し、文字列に格納することなので、
格納する文字列を引数として取ります。この文字列引数は、可変である必要があります。
メソッドがユーザ入力を追記して、文字列の中身を変えられるようにってことですね。

`&`という記号は、この引数が*参照*であることを表し、これのおかげで、データを複数回メモリにコピーせずとも、
コードの複数箇所で同じデータにアクセスできるようになるわけです。参照は複雑な機能であり、
とても安全かつ簡単に参照を使うことができることは、Rustの主要な利点の一つでもあります。
そのような詳細は知らなくても、このプログラムを完成させることはできます:
第4章で参照について詳しく見ることにしましょう。現時点では、変数のように、参照も標準で不変であることを
知っておけばいいでしょう。故に、`&guess`と書くのではなく、`&mut guess`と書いて、可変にする必要があるのです。

まだ、この行は終わりではありませんよ。テキストでは1行ですが、コードとしての論理行としては、
まだ所詮最初の部分でしかないのです。2番目の部分はこのメソッドです。:

```rust,ignore
.expect("Failed to read line");
```

`.foo()`という記法で、メソッドを呼び出す時、改行と空白で長い行を分割するのは賢いことです。
今回の場合、こう書くこともできますよね:

```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

しかし、長い行は読みづらいものです。なので、分割しましょう。2回のメソッド呼び出しに、2行です。
さて、この行が何をしているのかについて議論しましょうか。

### `Result`型で、失敗する可能性について対処する

以前にも述べたように、`read_line`メソッドは、渡された文字列にユーザが入力したものを入れ込むだけでなく、
値も返します(今回は[`io::Result`][ioresult]<!-- ignore -->です)。 Rustには`Result`と名のついた型が
標準ライブラリにたくさんあります: ジェネリクスバージョンの[`Result`][result]<!-- ignore -->の他、
サブモジュール用の`io::Result`などの特別版まで。

[ioresult]: ../../std/io/type.Result.html
[result]: ../../std/result/enum.Result.html

この`Result`型は、[*列挙型*][enums]<!-- ignore -->であり、普通、*enum*(イーナム)と呼ばれます。
列挙型とは、固定された種類の値を持つ型のことであり、それらの値は、enumの*バリアント*(variant)と呼ばれます。
enumについては、第6章で詳しく解説します。

[enums]: ch06-00-enums.html

`Result`型に関しては、取りうる型の値(variant)は`Ok`か`Err`です。値`Ok`は、処理が成功したことを表し、
中に生成された値を保持します。`Err`は、処理が失敗したことを意味し、`Err`は、処理が失敗した過程や、
理由などの情報を含有します。

これら`Result`型の目的は、エラー処理の情報をエンコードすることです。`Result`型の値も、他の型同様、
メソッドが定義されています。`io::Result`オブジェクトには、呼び出し可能な
[`expect`メソッド][expect]<!-- ignore -->があります。
この`io::Result`オブジェクトが`Err`値の場合、`expect`メソッドはプロラグムをクラッシュさせ、
引数として渡されたメッセージを表示します。`read_line`メソッドが`Err`を返したら、
根底にあるOSによるエラーに起因する可能性が高くなります。
この`io::Result`オブジェクトが`Ok`値の場合、`expect`メソッドは、`Ok`バリアントが保持する
返り値を取り出して、ただその値を返すので、これを使用することができるかもしれません。
今回の場合、その返り値とは、ユーザが標準入力に入力したバイト数になります。

[expect]: ../../std/result/enum.Result.html#method.expect

もし、`expect`メソッドを呼び出さなかったら、コンパイルは通るものの、警告が出るでしょう:

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
src/main.rs:10:5: 10:39 warning: unused result which must be used
                       (警告: 使用するべき結果が使用されていません),
#[warn(unused_must_use)] on by default
src/main.rs:10     io::stdin().read_line(&mut guess);
                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
```

コンパイラは、私たちが`read_line`メソッドから返ってきた`Result`値を使用していないと警告してきており、
これは、プログラムがエラーの可能性に対処していないことを示します。警告を抑制する正しい手段は、実際にエラー対処
コードを書くことですが、今は、問題が起きた時にプロラグムをただ単にクラッシュさせたいので、`expect`を使用できるわけです。
エラーから復旧する方法については、第9章で学ぶでしょう。

### `println!`マクロのプレースホルダーで値を出力する

閉じ波かっこを除けば、ここまでに追加されたコードのうち議論すべきものは、残り1行であり、それは以下の通りです:

```rust,ignore
println!("You guessed: {}", guess);
```

この行は、ユーザ入力を保存した文字列の中身を出力する。1組の`{}`は、値を保持しておくプレースホルダーの役目を果たします。
`{}`記法を使って一つ以上の値を出力できます: 最初の`{}`の組は、フォーマット文字列の後に列挙された最初の値に対応し、
2組目は、2つ目の値、とそんな感じで続いていきます。1回の`println!`マクロの呼び出しで複数値を出力するコードは、
以下のような感じになります。:

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

このコードは、`x = 5 and y = 10`と出力するでしょう.

### 最初の部分をテストする

数当てゲームの最初の部分をテストしてみましょう。`cargo run`コマンドでプログラムを走らせることができます:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/debug/guessing_game`
Guess the number!           (数を当ててごらん)
Please input your guess.    (ほら、予想を入力して)
6
You guessed(次のように予想したよ): 6
```

ここまでで、ゲームの最初の部分は完成になります: キーボードからの入力を受け付け、出力できるようになりました。

## 秘密の数字を生成する

次に、ユーザが数当てに挑戦する秘密の数字を生成する必要があります。毎回この秘密の数字は、変わるべきです。
ゲームが何回も楽しめるようにですね。ゲームが難しくなりすぎないように、1から100までの乱数を使用しましょう。
Rustの標準ライブラリには、乱数機能はまだ含まれていません。ですが、Rustチームが[`rand`クレート][randcrate]を
用意してくれています。

[randcrate]: https://crates.io/crates/rand

### クレートを使用して機能を追加する

*クレート*はRustコードのパッケージであることを思い出してください。私たちがここまで作ってきたプロジェクトは、
*バイナリークレート*であり、これは実行可能形式になります。`rand`クレートは*ライブラリクレート*であり、
他のプロラグムで使用する用のコードが含まれています。

Cargoを使って外部クレートを使用すると、Cargoがとても輝きます。`rand`を使ったコードを書くためには、
*Cargo.toml*ファイルを編集して、`rand`クレートを依存ファイルとして取り込む必要があります。
このファイルを開いて、以下の行をCargoが自動生成した`[dependencies]`セクションヘッダーの一番下に追記しましょう:

<span class="filename">ファイル名: Cargo.toml</span>

```toml
[dependencies]

rand = "0.3.14"
```

*Cargo.toml*ファイルにおいて、ヘッダーに続くものは全て、他のセクションが始まるまで続くセクションの一部になります。
`[dependecies]`セクションは、プロジェクトが依存する外部クレートと必要とするバージョンを記述するところです。
今は、`rand`クレートで、意味論的バージョンには`0.3.14`を指定します。Cargoは[意味論的バージョン付け][semver]<!-- ignore -->
(時に*SemVer*と呼ばれる)を理解し、 意味論的バージョン付けは、バージョンナンバー記述の標準規格です。
`0.3.14`という数字は、実際には`^0.3.14`の省略記法で、これは、「バージョン0.3.14と互換性のある公開APIを持つ
バージョンならなんでも」を意味します。

[semver]: http://semver.org

さて、コードは一切変えずに、プロジェクトをビルドしましょう。リスト2-2に示したようにね:

```text
$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index` (レジストリを更新しています)
 Downloading rand v0.3.14                                            (rand v0.3.14をダウンロードしています)
 Downloading libc v0.2.14                                            (libc v0.2.14をダウンロードしています)
   Compiling libc v0.2.14                                            (libc v0.2.14をコンパイルしています)
   Compiling rand v0.3.14                                            (rand v0.3.14をコンパイルしています)
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)   (guessing_game v0.1.0をコンパイルしています)
```

<span class="caption">リスト2-2: randクレートを依存ファイルとして追加した後の`cargo build`コマンドの出力</span>

もしかしたら、バージョンナンバーは違うかもしれません(でも、互換性はあります、SemVerのおかげでね！)
そして、行の出力順序も違うかもしれません。

今や、外部依存ファイルを持つようになったので、Cargoは*registry*(登録所)から最新バージョンを拾ってきます。
*レジストリ*とは、[Crates.io][cratesio]のデータのコピーです. Crates.ioとは、Rustのエコシステムにいる人間が
他の人も使えるように自分のオープンソースのRustプロジェクトを投稿する場所です。

[cratesio]: https://crates.io

レジストリの更新後、Cargoは`[dependencies]`セクションをチェックし、まだ取得していないものを全部ダウンロードします。
今回の場合、`rand`しか依存ファイルには列挙していませんが、Cargoは`libc`のコピーも拾ってきます。
`rand`クレートが`libc`に依存しているからですね。ダウンロード完了後、コンパイラは依存ファイル、
そして、依存ファイルが利用可能な状態でプロジェクトをコンパイルします。

何も変更せず即座に`cargo build`コマンドを走らせたら、何も出力されないでしょう。
Cargoは、すでに依存ファイルをダウンロードしてコンパイル済みであることを検知し、プログラマが
*Cargo.toml*ファイルを弄ってないからです。さらに、Cargoはプログラマがコードを変更していないことも
検知するので、再度コンパイルすることもありません。することがないので、ただ単に終了します。
*src/main.rs*ファイルを開き、些細な変更をし、保存して再度ビルドを行えば、1行だけ出力があるでしょう:

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
```

この行は、Cargoが*src/main.rs*ファイルへの取るに足らない変更に対してビルドを更新していることを示しています。
依存ファイルは変更していないので、Cargoは、すでにダウンロードし、コンパイル済みの依存ファイルを使用できると
検知します。自分で書いたコードのみ再ビルドをかけるわけです。

#### *Cargo.lock*ファイルで再生成可能なビルドを保証する

Cargoには、プログラマが自分のコードを更新するたびに同じ生成物を再構成することを保証してくれるメカニズムを
備えています: Cargoは、プログラマが明示するまで、指定したバージョンの依存ファイルのみを使用してくれるでしょう。
例として、`rand`クレートの次週のバージョン`v0.3.15`が登場し、重要なバグ修正がなされているけれども、
自分のコードを破壊してしまう互換性破壊があった場合はどうなるでしょう？

この問題に対する回答は、*Cargo.lock*ファイルであり、このファイルは、初めて`cargo build`コマンドを
走らせた時に生成され、*guessing_game*ディレクトリに存在しています。プロジェクトを始めてビルドする際に、
Cargoは判断基準(criteria)に合致する依存ファイルのバージョンを割り出し、*Cargo.lock*ファイルに記述します。
次にプロジェクトをビルドする際には、Cargoは*Cargo.lock*ファイルが存在することを確かめ、
再度バージョン割り出しの作業を行うのではなく、そこに指定されているバージョンを使用するでしょう。
このことにより、自動的に再生成可能なビルドを構成できるのです。つまり、明示的にアップグレードしない限り、
プロジェクトが使用するバージョンは`0.3.14`に保たれるのです。*Cargo.lock*ファイルのおかげでね。

#### クレートを更新して新バージョンを取得する

クレートを*本当に*アップグレードする必要が出てきたら、Cargoの別のコマンド(`update`)を使用しましょう。これは:

1. *Cargo.lock*ファイルを無視して*Cargo.toml*ファイルに指定された通りの最新バージョンを全て割り出します。
1. それがうまくいったら、Cargoはそれらのバージョンを*Cargo.lock*ファイルに記述します。

しかし標準でCargoは、`0.3.0`以上、`0.4.0`未満のバージョンのみを検索します。`rand`クレートの新バージョンが
2つリリースされていたら(`0.3.15`と`0.4.0`ですね)、`cargo update`コマンドを走らせた時に以下のような
メッセージを目の当たりにするでしょう:

```text
$ cargo update
    Updating registry `https://github.com/rust-lang/crates.io-index`
    (レジストリ`https://github.com/rust-lang/crates-io-index`を更新しています)
    Updating rand v0.3.14 -> v0.3.15
    (randクレートをv0.3.14 -> v0.3.15に更新しています)
```

ここで、プログラマはさらに*Cargo.lock*ファイルの中身が、現在使用している`rand`クレートのバージョンは、
`0.3.15`になっていることに気付くでしょう。

`rand`のバージョン`0.4.0`または、`0.4.x`シリーズのどれかを使用したかったら、
代わりに*Cargo.toml*ファイルを以下のように更新しなければならないでしょう:

```toml
[dependencies]

rand = "0.4.0"
```

次回、`cargo build`コマンドを走らせたら、Cargoは利用可能なクレート登録所を更新し、`rand`クレートの
必要条件を指定した新しいバージョンに再評価します。

まだ第14章で議論する[Cargo][doccargo]<!-- ignore -->と[そのエコシステム][doccratesio]<!-- ignore -->
については述べたいことが山ほどありますが、とりあえずは、これで知っておくべきことは全てです。
Cargoのおかげでライブラリはとても簡単に再利用ができるので、Rust市民(Rustaceans)は数多くのパッケージから
構成された小規模のプロジェクトを書くことができるのです。

[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html

### 乱数を生成する

`rand`クレートを*使用*開始しましょう。次のステップは、*src/main.rs*ファイルを更新することです。リスト2-3みたいにね:

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);    //秘密の数字は次の通り

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");                     //行の読み込みに失敗しました

    println!("You guessed: {}", guess);
}
```

<span class="caption">リスト2-3: 乱数を生成するのに必要なコードの変更</span>

冒頭に`extern crate rand;`行を追加して、コンパイラにこの外部依存ファイルを使用することを知らせています。
これにより、`use rand`を呼ぶのと同じ効果が得られるので、`rand`クレートのものを`rand::`
という接頭辞をつけて呼び出せるようになりました。

次に、別の`use`行を追加しています: `use rand::Rng`ですね。`Rng`とは乱数生成器が実装するメソッドを定義した
トレイトであり、このトレイトがスコープにないと、メソッドを使用できないのです。トレイトについて詳しくは、
第10章を参照されたし。

また、途中に2行追加もしています。`rand::thread_rng`関数は、私たちが使う特定の乱数生成器を
返してくれます: この乱数生成器は、実行スレッドに特有で、OSにより、シード値を与えられています。
次に、この乱数生成器の`gen_range`メソッドを呼び出しています。このメソッドは、`use rand::Rng`文で
スコープに導入した`Rng`トレイトで定義されています。`gen_range`メソッドは二つの数字を引数に取り、
それらの間の乱数を生成してくれます。最低値は含むものの、最高値は含まないため、`1`と`101`と指定しないと
1から100の範囲の数字は得られません。

使用すべきトレイトとクレートから呼び出すべき関数とメソッドを知ることが、単純に*知っている*ことではないでしょう。
クレートの使用方法は、各クレートのドキュメントにある。Cargoの別の巧妙な機能は、`cargo doc --open`コマンドを
走らせてローカルに存在する依存ファィルすべてのドキュメントをビルドし、Webブラウザで閲覧できる機能です。例えば、
`rand`クレートの他の機能に興味があるなら、`cargo doc --open`コマンドを走らせて、左側のサイドバーから
`rand`をクリックすればいいわけです。

コードに追加した2行目は、秘密の数字を出力してくれます。これは、プログラムをテストする構築中には便利であるが、
最終版からは削除する予定です。プログラムがスタートと同時に答えを出力しちゃったら、ゲームにならないからですね！

何回かプログラムを走らせてみてください:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/debug/guessing_game`
Guess the number!                         (何回も出ているので、ここでは和訳は省略します)
The secret number is: 7
Please input your guess.
4
You guessed: 4
$ cargo run
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

毎回異なる乱数が出、その数字はすべて1から100の範囲になるはずです。よくやりました！

## 予想と秘密の数字を比較する

今や、ユーザ入力と乱数生成ができるようになったので、比較することができますね。
このステップはリスト2-4に示されています:

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),    //小さすぎ！
        Ordering::Greater => println!("Too big!"),      //大きすぎ！
        Ordering::Equal   => println!("You win!"),      //やったね！
    }
}
```

<span class="caption">リスト2-4: 2値比較の返り値を処理する</span>

最初の新しい点は、別の`use`文です。これで、`std::cmp::Ordering`という型を標準ライブラリから
スコープに導入しています。`Ordering`もenumです。`Result`のようにね。ただ、`Ordering`が取りうる値は、
`Less`、`Greater`そして、`Equal`です。これらは、2値比較した時に発生しうる3種類の結果です。

それから、一番下に5行追加して`Ordering`型を使用しています:

```rust,ignore
match guess.cmp(&secret_number) {
    Ordering::Less    => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal   => println!("You win!"),
}
```

`cmp`メソッドは、2値を比較し、比較できるものに対してならなんに対しても呼び出せます。このメソッドは、
比較したいものへの参照を取ります: ここでは、`guess`変数と`secret_number`変数を比較しています。
`cmp`メソッドは`use`文でスコープに導入した`Ordering`列挙型の値を返します。 
[`match`][match]<!-- ignore -->式を使用して、`guess`変数と`secret_number`を`cmp`に渡して
返ってきた`Ordering`の値に基づき、次の動作を決定しています。

[match]: ch06-02-match.html

`match`式は、複数の*アーム*(腕)からできています。一つのアームは、パターンとそのパターンに
`match`式の冒頭で与えた値がマッチした時に走るコードから構成されています。Rustは、`match`に与えられた
値を取り、各アームのパターンを順番に吟味していきます。`match`式とパターンは、コードを書く際に
目の当たりにする様々なシチュエーションを表現させてくれ、すべてのシチュエーションに対処する手助けをしてくれる
Rustの強力な機能です。これらの機能は、それぞれ、第6章と第18章で詳しく解説することにします。

ここで使われている`match`式でどんなことが起こるかの例をじっくり観察してみましょう！例えば、
ユーザは50と予想し、ランダム生成された秘密の数字は今回、38だったとしましょう。コードが50と38を比較すると、
`cmp`メソッドは`Ordering::Greater`を返します。50は38よりも大きいからですね。`Ordering::Greater`が、
`match`式に渡される値になります。まず、最初のアームのパターンを吟味します(`Ordering::Less`ですね)。しかし、
値の`Ordering::Greater`と`Ordering::Less`はマッチしないため、このアームのコードは無視され、
次のアームに移ります。次のアームのパターン、`Ordering::Greater`は*見事に*`Ordering::Greater`とマッチします！
このアームに紐づけられたコードが実行され、画面に`Too big!`が表示されます。
これで`match`式の実行は終わりになります。この筋書きでは、最後のアームを吟味する必要はもうないからですね。

ところが、リスト2-4のコードは、まだコンパイルが通りません。試してみましょう:

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types          (型が合いません)
  --> src/main.rs:23:21
   |
23 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `std::string::String`, found integral variable
   |                                    (構造体`std::string::String`を予期したけど、整数型変数が見つかりました)
   |
   = note: expected type `&std::string::String`
   = note:    found type `&{integer}`

error: aborting due to previous error   (先のエラーのため、処理を中断します)
Could not compile `guessing_game`.      (`guessing_game`をコンパイルできませんでした)
```

このエラーの核は、*型の不一致*があると言っています。Rustは、強力な静的型付けシステムを持っています。
しかし、型推論にも対応しています。`let guess = String::new()`と書いた時、コンパイラは、
`guess`が`String`型であるべきと推論してくれ、その型を明示させられることはありませんでした。
一方で、`secret_number`変数は、数値型です。少数の数値型しか1から100を表すことはできません:
`i32`は32ビットの数字; `u32`は32ビットの非負数字; `i64`は64ビットの数字;などです。
Rustでの標準は、`i32`型であり、型情報をどこかに追加して、コンパイラに異なる数値型だと推論させない限り、
`secret_number`の型はこれになります。エラーの原因は、Rustでは、文字列と数値型を比較できないことです。

究極的には、プログラムが入力として読み込む`String`型を現実の数値型に変換し、予想と数値として比較できるように
したいわけです。これは、以下の2行を`main`関数の本体に追記することでできます:

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");                 //数値を入力してください！

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }
}
```

その2行とは以下のようなものです:

```rust,ignore
let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```

`guess`という名前の変数を生成しています。あれ、でも待って。もうプログラムには`guess`という名前の変数が
ありませんでしたっけ？確かにありますが、Rustでは、新しい値で`guess`の値を*多重定義*(shadow)することが
許されているのです。この機能は、今回のような、値を別の型に変換したいシチュエーションでよく使われます。
多重定義のおかげで別々の変数を2つ作らされることなく、`guess`という変数名を再利用することができるのです。
`guess_str`と`guess`みたいなね(多重定義については、第3章でもっと掘り下げます)。

`guess`を`guess.trim().parse()`という式に束縛しています。この式中の`guess`は、入力が入った
`String`型の元々の`guess`を指しています。`String`オブジェクトの`trim`メソッドは、
両端の空白をすべて除去します。`u32`型は、数字しか含むことができませんが、ユーザは、`read_line`の
処理を終えるためにリターンキーを押さなければなりません。ユーザがリターンキーを押したら、改行文字が
文字列に追加されます。具体例として、ユーザが5を入力して、リターンキーを押せば、`guess`変数は
次のようになります: `5\n`。この`\n`が改行、つまりリターンキーを表しているわけです。
`trim`メソッドは、`\n`を削除するので、ただの`5`になります。

[文字列の`parse`メソッド][parse]<!-- ignore -->は、文字列を解析して何らかの数値にします。
このメソッドは、いろんな数値型を解析できるので、`let guess: u32`としてコンパイラに私たちが
求めている型をズバリ示唆する必要があるのです。`guess`の後のコロン(`:`)がコンパイラに変数の型を
注釈する合図になります。Rustには、組み込みの数値型がいくつかあります; ここで見られる`u32`型は、
32ビットの非負整数です。小さな非負整数は、良い基準になります。他の数値型については、第3章で学ぶでしょう。
付け加えると、このサンプルプログラムの`u32`という注釈と`secret_number`変数との比較は、
`secret_number`変数も`u32`型であるとコンパイラが推論することを意味します。さて、従って、比較が同じ型の
2つの値で行われることになります。

[parse]: ../../std/primitive.str.html#method.parse

`parse`メソッドの呼び出しは、エラーになりやすいです。例としては、文字列が`A👍%`を含んでいたら、
数値に変換できるわけがないわけです。失敗する可能性があるので、`parse`メソッドは、`Result`型を
返すわけです。ちょうど、「Result型で失敗する可能性に対処する」節で先ほど議論した`read_line`メソッドが
するようにというわけですね。今回も、`expect`メソッドを使用して`Result`型を同じように扱います。
もし、文字列から数値を生成できなかったために、`parse`メソッドが`Result`型の`Err`値を返したら、
`expect`メソッドの呼び出しは、ゲームをクラッシュさせ、与えたメッセージを表示します。
もし、`parse`メソッドが文字列の数値への変換に成功したら、`Result`型の`Ok`値を返し、
`expect`メソッドは、`Ok`値から必要な数値を返してくれます。

さあ、プログラムを走らせましょう！

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

いいですね！予想の前にスペースを追加したにもかかわらず、プログラムはちゃんとユーザが76と予想したことを
導き出しました。プログラムを何回か走らせて、異なる入力の異なる振る舞いを確認してください: つまり、
数字を正しく言い当てたり、大きすぎる値を予想したり、低すぎる数字を入力したりということです。

ここまでで大方ゲームはうまく動くようになりましたが、まだユーザは1回しか予想できません。
ループを追加して、その部分を変更しましょう！

## ループで複数回の予想を可能にする

`loop`キーワードは、無限ループを作り出します。これを追加して、ユーザが何回も予想できるようにしましょう:

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => println!("You win!"),
        }
    }
}
```

見てわかる通り、予想入力部分以降をループに入れ込みました。変更した行にインデントを追加するのを忘れないようにして、
またプログラムを走らせてみましょう。新たな問題が発生したことに気をつけてください。
プログラムが教えた通りに動作しているからですね: 永遠に予想入力を求めるわけです！これでは、終了できません！

ユーザは、`Ctrl-C`というキーボードショートカットを使って、いつでもプログラムを強制終了させられます。
しかし、「予想を秘密の数字と比較する」節の`parse`メソッドに関する議論で触れたこの貪欲なモンスターを
回避する別の方法があります: ユーザが数字以外の答えを入力すれば、プログラムはクラッシュするのです。
ユーザは、その利点を活かして、終了することができます。以下のようにね:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:785
(スレッド'main'は'数字を入力してください！: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:785でパニックしました)
note: Run with `RUST_BACKTRACE=1` for a backtrace.
(注釈: `RUST_BACKTRACE=1`で走らせるとバックトレースを見れます)
error: Process didn't exit successfully: `target/debug/guess` (exit code: 101)
(エラー: プロセスは予期なく終了しました)
```

`quit`と入力すれば、実際にゲームを終了できるわけですが、別に他の数字以外の入力でもそうなります。
しかしながら、これは最低限度と言えるでしょう。正しい数字が予想されたら、自動的にゲームが停止してほしいわけです。

### 正しい予想をした後に終了する

`break`文を追加してユーザが勝った時にゲームを終了するようにしましょう:

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
```

`break`文の1行を`You win!`の後に追記することで、ユーザが秘密の数字を正確に予想したら、プログラムは
ループを抜けるようになりました。ついでに、ループを抜けることは、プログラムを終了することを意味します。
ループが`main`関数の最後の部分だからですね。

### 不正な入力を処理する

さらにゲームの振る舞いを改善するために、ユーザが数値以外を入力した時にプログラムをクラッシュさせるのではなく、
非数値を無視してユーザが数当てを続けられるようにしましょう！これは、`guess`が`String`型から
`u32`方に変換される行を改変することで達成できます:

```rust,ignore
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

`expect`メソッドの呼び出しから`match`式に切り替えることは、エラーでクラッシュする動作から
実際にエラー処理を行う処理へ変更する一般的な手段になります。`parse`メソッドは、`Result`型を
返し、`Result`は`Ok`か`Err`の値を取りうるenumであることを思い出してください。
ここでは`match`式を使っています。`cmp`メソッドの`Ordering`という結果でしたのと同じですね。

`parse`メソッドは、文字列から数値への変換に成功したら、結果の数値を保持する`Ok`値を返します。
この`Ok`値は、最初のアームのパターンにマッチし、この`match`式は`parse`メソッドが生成し、
`Ok`値に格納した`num`の値を返すだけです。その数値が最終的に生成した新しい`guess`変数に
含まれます。

`parse`メソッドは、文字列から数値への変換に*失敗*したら、エラーに関する情報を多く含む`Err`値を
返します。この`Err`値は、最初の`match`アームの`Ok(num)`というパターンにはマッチしないものの、
2番目のアームの`Err(_)`というパターンにはマッチするわけです。この`_`は、包括値です; この例では、
保持している情報がどんなものでもいいから全ての`Err`値にマッチさせたいと宣言しています。
従って、プログラムは2番目のアームのコードを実行し(`continue`ですね)、これは、`loop`の
次の段階に移り、再度予想入力を求めることを意味します。故に実効的には、プログラムは`parse`メソッドが
遭遇しうる全てのエラーを無視するようになります！

さて、プログラムの全てがうまく予想通りに動くはずです。`cargo run`コマンドで走らせて、試してみましょう:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

素晴らしい！最後にひとつまみ変更を加えて、数当てゲームを完了にしましょう: プログラムが未だに
秘密の数字を出力していることを思い出してください。テスト中はうまく動くけど、
ゲームを台無しにしてしまいます。秘密の数字を出力する`println!`マクロを削除しましょう。
リスト2-5が成果物のコードです:

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
```

<span class="caption">リスト2-5: 数当てゲームの完全なコード</span>

## まとめ

ここまでで、数当てゲームの作成に成功しました！おめでとうございます！

このプロジェクトは、たくさんの新しいRustの概念に触れる実践的な方法でした:
`let`文、`match`式、メソッド、関連付け関数、外部クレートの使用などなど。
以降の数章で、これらの概念についてより深く学ぶことになるでしょう。
第3章では、ほとんどのプログラミング言語が持っている、変数、データ型、関数などの概念について解説し、
それらのRustでの使用方法について示します。
第4章では、所有権について見ます。所有権は、他の言語とかけ離れているRustの機能の一つです。
第5章では、構造体とメソッド記法について議論し、第6章ではenumについて説明する努力をしましょう。
