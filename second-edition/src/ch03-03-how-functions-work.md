## 関数の動作方法

関数は、Rustのコードにおいてよく見かける存在です。すでに、言語において最も重要な関数のうちの
一つを目撃していますね: そう、`main`関数で、これは、多くのプログラムのエントリーポイントになります。
`fn`キーワードもすでに見かけましたね。これによって新しい関数を定義することができます。

Rustの関数と変数の命名規則は、*スネークケース*(`脚注`: some_variableのような命名規則)です。
スネークケースとは、全文字を小文字にし、単語区切りにアンダースコアを使うことです。
以下のプログラムで、サンプルの関数定義をご覧ください:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");  // 別の関数
}
```

Rustにおいて関数定義は、`fn`キーワードで始まり、関数名の後に丸かっこの組が続きます。
波かっこの位置が、コンパイラが関数本体の位置と判断する場所です。

定義した関数は、名前に丸かっこの組を続けることで呼び出すことができます。`another_function`関数が
プログラム内で定義されているので、`main`関数内から呼び出すことができるわけです。ソースコード中で
`another_function`を`main`関数の*後*に定義していることに注目してください; 勿論、main関数の前に
定義することもできます。コンパイラは、関数がどこで定義されているかは気にしません。
どこかで定義されていることのみ気にします。

*functions*という名前の新しいバイナリ生成プロジェクトを始めて、関数についてさらに深く探知していきましょう。
`another_function`の例を*src/main.rs*ファイルに配置して、走らせてください。
以下のような出力が得られるはずです:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
Hello, world!
Another function.
```

行出力は、`main`関数内に書かれた順序で実行されています。最初に、"Hello, world"メッセージが出、
それから`another_function`が呼ばれて、こちらのメッセージが出力されています。

### 関数の引数

関数は、引数を持つようにも定義できます。引数とは、関数シグニチャの一部になる特別な変数のことです。
関数に引数があると、引数の位置に実際の値を与えることができます。技術的にはこの実際の値は
*実引数*と呼ばれますが、普段の会話では、仮引数("parameter")と実引数("argument")を
関数定義の変数と関数呼び出し時に渡す実際の値、両方の意味に区別なく使います(`脚注`: 日本語では、
どちらも単に引数と呼ぶことが多いでしょう)。

以下の書き直した`another_function`では、Rustのパラメータがどんな見た目なのかを示しています:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);   // xの値は{}です
}
```

このプログラムを走らせてみてください; 以下のような出力が得られるはずです:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

`another_function`の定義には、`x`という名前の仮引数があります。`x`の型は、`i32`と
指定されています。値`5`が`another_function`に渡されると、`println!`マクロにより、
フォーマット文字列中の1組の波かっこがある位置に値`5`が出力されます。

関数シグニチャにおいて、各仮引数の型を宣言しなければなりません。これは、Rustの設計において、
意図的な判断です: 関数定義で型注釈が必要不可欠ということは、コンパイラがその意図するところを
推し量るのに、コードの他の箇所で使用する必要がないということを意味します。

関数に複数の仮引数をもたせたいときは、仮引数定義をカンマで区切ってください。
こんな感じです:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

この例では、2引数の関数を生成しています。そして、引数はどちらも`i32`型です。この関数は、
仮引数の値を両方出力します。関数引数は、全てが同じ型である必要はありません。今回は、
偶然同じになっただけです。

このコードを走らせてみましょう。今、*function*プロジェクトの*src/main.rs*ファイルに記載されている
プログラムを先ほどの例と置き換えて、`cargo run`で走らせてください:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

`x`に対して値`5`、`y`に対して値`6`を渡して関数を呼び出したので、この二つの文字列は、
この値で出力されました。

### 関数本体

関数本体は、文が並び、最後に式を置くか文を置くという形で形成されます。現在までには、式で終わらない
関数だけを見てきたわけですが、式が文の一部になっているものなら見かけましたね。Rustは、式志向言語なので、
これは理解しておくべき重要な差異になります。他の言語にこの差異はありませんので、文と式がなんなのかと、
その違いが関数本体にどんな影響を与えるかを見ていきましょう。

### 文と式

実のところ、もう文と式は使っています。*文*とは、なんらかの動作をして値を返さない命令です。
*式*は結果値に評価されます。ちょっと例を眺めてみましょう。

`let`キーワードを使用して変数を生成し、値を代入することは文になります。
リスト3-3で`let y = 6;`は文です:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    let y = 6;
}
```

<span class="caption">リスト3-3: 1文を含む`main`関数宣言</span>

関数定義も文になります。つまり、先の例は全体としても文になるわけです。

文は値を返しません。故に、`let`文を他の変数に代入することはできません。
以下のコードでは試みてますけどね:

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
fn main() {
    let x = (let y = 6);
}
```

このプログラムを実行すると、以下のようなエラーが出るでしょう:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
(エラー: 式を予期しましたが、文が見つかりました (`let`))
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
  |
  = note: variable declaration using `let` is a statement
    (注釈: `let`を使う変数宣言は、文です)
```

この`let y = 6`という文は値を返さないので、`x`を束縛する相手がないわけです。これは、
CやRubyなどの言語とは異なる動作です。CやRubyでは、代入は代入値を返します。これらの言語では、
`x = y = 6`と書いて、`x`も`y`も値6になるようにできるのですが、Rustにおいては、
そうは問屋が卸さないわけです。

式は何かに評価され、これからあなたが書くRustコードの多くを構成します。簡単な数学演算(`5 + 6`など)を
思い浮かべましょう。この例は、値`11`に評価される式です。式は文の一部になりえます: `let y = 6`という
文を含むリスト3-3では、`6`は値`6`に評価される式です。関数呼び出しも式です。マクロ呼び出しも式です。
新しいスコープを作る際に使用するブロック(`{}`)も式です:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

この式は:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

今回の場合、`4`に評価されるブロックです。その値が、`let`文の一部として`y`に束縛されます。
今まで見かけてきた行と異なり、セミコロンがついていない行に気をつけてください。式に終端のセミコロンは、
含みません。式の終端にセミコロンを付けたら、文に変えてしまいます。そして、文は値を返しません。
次に関数の戻り値や式を見ていく際にこのことを肝に命じておいてください。

### 戻り値のある関数

関数は、それを呼び出したコードに値を返すことができます。戻り値に名前付けはできませんが、
矢印(`->`)の後に型を書いて宣言します。Rustでは、関数の戻り値は、関数本体ブロックの最後の式の値と
同義です。こちらが、値を返す関数の例です:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

`five`関数内には、関数呼び出しもマクロ呼び出しも、`let`文でさえ存在しません。数字の5が単独であるだけです。
これは、Rustにおいて、完璧に問題ない関数です。関数の戻り値型が`-> i32`と指定されていることにも注目してください。
このコードを実行してみましょう; 出力はこんな感じになるはずです:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

`five`内の`5`が関数の戻り値です。だから、戻り値型が`i32`なのです。これについてもっと深く考察しましょう。
重要な箇所は2つあります: まず、`let x = five()`という行は、関数の戻り値を使って変数を初期化している
ことを示しています。関数`five`は`5`を返すので、この行は以下のように書くのと同義です:

```rust
let x = 5;
```

2番目に、`five`関数は仮引数をもたず、戻り値型を定義していますが、関数本体はセミコロンなしの`5`単独です。
なぜなら、これが返したい値になる式だからです。もう一つ例を見てみましょう:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

このコードを走らせると、`The value of x is: 6`と出力されるでしょう。では、`x + 1`を含む行の
終端にセミコロンを付けて、式から文に変えてみたら、どうなるでしょうか？

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

このコードを実行すると、以下のようにエラーが出ます:

```text
error[E0308]: mismatched types
              (型が合いません)
 --> src/main.rs:7:28
  |
7 |   fn plus_one(x: i32) -> i32 {
  |  ____________________________^
8 | |     x + 1;
9 | | }
  | |_^ expected i32, found ()
  |     (i32を予期したのに、()型が見つかりました)
  |
  = note: expected type `i32`
             found type `()`
help: consider removing this semicolon:
(助言: このセミコロンを除くことを考えてみてください)
 --> src/main.rs:8:10
  |
8 |     x + 1;
  |          ^
```

メインのエラーメッセージである「型が合いません」でこのコードの根本的な問題が明らかになるでしょう。
関数`plus_one`の定義では、`i32`型を返すと言っているのに、文は値に評価されないからです。このことは、
`()`、つまり空のタプルとして表現されています。それゆえに、何も戻り値がなく、これが関数定義と矛盾するので、
結果としてエラーになるわけです。この出力内で、コンパイラは問題を修正する手助けになりそうなメッセージも出していますね: 
セミコロンを削除するよう提言しています。そして、そうすれば、エラーは直るわけです。
