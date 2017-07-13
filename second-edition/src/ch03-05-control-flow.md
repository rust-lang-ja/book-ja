## フロー制御

条件が真かどうかによってコードを走らせるかどうかを決定したり、条件が真の間繰り返しコードを走らせるか
決定したりすることは、多くのプログラミング言語において、基本的な構成ブロックです。Rustコードの
実行フローを制御する最も一般的な文法要素は、`if`式とループです。

### `if`式

if式によって、条件に依存して枝分かれをさせることができます。条件を与え、以下のように宣言します。
「もし条件が合ったら、この一連のコードを実行しろ。条件に合わなければ、この一連のコードは実行するな」と。

*projects*ディレクトリに*branches*という名のプロジェクトを作って`if`式について掘り下げていきましょう。
*src/main.rs*ファイルに、以下のように入力してください:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");       // 条件は真です
    } else {
        println!("condition was false");      // 条件は偽です
    }
}
```

<!-- NEXT PARAGRAPH WRAPPED WEIRD INTENTIONALLY SEE #199 -->

`if`式は全て、キーワードの`if`から始め、条件式を続けます。今回の場合、条件式は変数`number`が
５未満の値になっているかどうかをチェックします。条件が真の時に実行したい一連のコードを条件式の直後に
波かっこで包んで配置します。`if`式の条件式と紐付けられる一連のコードは、時として*アーム*と呼ばれることがあります。
第2章の「予想と秘密の数字を比較する」の節で議論した`match`式のアームのようですね。オプションとして、
`else`式を含むこともでき(ここではそうしています)、これによりプログラムは、条件式が偽になった時に
実行するコードを与えられることになります。仮に、`else`式を与えずに条件式が偽になったら、プログラムは単に
`if`ブロックを無視して次のコードを実行しにいきます。

このコードを走らせてみましょう; 以下のような出力を目の当たりにするはずです:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was true
```

`number`の値を条件が`false`になるような値に変更してどうなるか確かめてみましょう:

```rust,ignore
let number = 7;
```

再度プログラムを実行して、出力に注目してください:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was false
```

このコード内の条件式は、`bool`型で*なければならない*ことにも触れる価値があります。条件式が、
`bool`型でない時に何が起こるかを確かめるために、以下のコードを走らせてみましょう:

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
fn main() {
    let number = 3;

    if number {
        println!("number was three");     // 数値は3です
    }
}
```

今回、`if`の条件式は`3`という値に評価され、コンパイラがエラーを投げます:

```text
error[E0308]: mismatched types
              (型が合いません)
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected bool, found integral variable
  |               (bool型を予期したのに、整数変数が見つかりました)
  |
  = note: expected type `bool`
             found type `{integer}`
```

このエラーは、コンパイラは`bool`型を予期していたのに、整数だったことを示唆しています。Rustでは、
論理値以外の値が、自動的に論理値に変換されることはありません。RubyやJavaScriptなどの言語とは
異なりますね。明示的に必ず`if`には条件式として、`論理値`を与えなければなりません。例えば、
数値が`0`以外の時だけ`if`のコードを走らせたいなら、以下のように`if`式を変更することができます:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");   // 数値は0以外の何かです
    }
}
```

このコードを実行したら、`number was something other than zero`と表示されるでしょう。

#### `else if`で複数の条件

`if`と`else`を組み合わせて`else if`式にすることで複数の条件を持たせることもできます。例です:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        // 数値は4で割り切れます
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // 数値は3で割り切れます
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // 数値は2で割り切れます
        println!("number is divisible by 2");
    } else {
        // 数値は4、3、2で割り切れません
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

このプログラムには、通り道が4つあります。実行後、以下のような出力を目の当たりにするはずです:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
number is divisible by 3
```

このプログラムを実行すると、`if`式が順番に吟味され、最初に条件が真になった本体が実行されます。
6は2で割り切れるものの、`number is devisible by 2`や`else`ブロックの`number is not divisible by 4, 3, or 2`
という出力はされないことに注目してください。その原因は、言語が最初の真条件のブロックのみを実行し、
条件に合ったものが見つかったら、残りはチェックすらしないということだからです。

`else if`式を使いすぎると、コードがめちゃくちゃになってしまうので、1つ以上あるなら、
コードをリファクタリングしたくなるかもしれません。これらのケースに有用な`match`と呼ばれる
強力なRustの枝分かれ文法要素については第6章で解説します。

#### `let`文内で`if`式を使う

`if`は式なので、`let`文の右辺に持ってくることができます。例はリスト3-4の通り:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    // numberの値は、{}です
    println!("The value of number is: {}", number);
}
```

<span class="caption">リスト3-4: `if`式の結果を変数に代入する</span>

この`number`変数は、`if`式の結果に基づいた値に束縛されます。このコードを走らせてどうなるか確かめてください:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
The value of number is: 5
```

一連のコードは、そのうちの最後の式に評価され、数値はそれ単独でも式になることを思い出してください。
今回の場合、この`if`式全体の値は、どのブロックのコードが実行されるかに基づきます。これはつまり、
`if`の各アームの結果になる可能性がある値は、同じ型でなければならないということになります;
リスト3-4で、`if`アームも`else`アームも結果は、`i32`の整数でした。では、以下の例のように、
型が合わない時には、どうなるのでしょうか？

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
```

このコードを走らせようとすると、エラーになります。`if`と`else`アームは互換性のない値の型になり、
コンパイラがプログラム内で問題の見つかった箇所をスバリ指摘してくれます:

```text
error[E0308]: if and else have incompatible types
              (ifとelseの型に互換性がありません)
 --> src/main.rs:4:18
  |
4 |       let number = if condition {
  |  __________________^
5 | |         5
6 | |     } else {
7 | |         "six"
8 | |     };
  | |_____^ expected integral variable, found reference
  |         (整数変数を予期しましたが、参照が見つかりました)
  |
  = note: expected type `{integer}`
             found type `&'static str`
```

`if`ブロックの式は整数に評価され、`else`ブロックの式は文字列に評価されます。これでは動作しません。
変数は単独の型でなければならないのです。コンパイラは、コンパイル時に`number`変数の型を確実に
把握する必要があるため、コンパイル時に`number`が使われている箇所全部で型が有効かどうか
検査することができるのです。`number`の型が実行時にしか決まらないのであれば、コンパイラは
それを実行することができなくなってしまいます; コンパイラはより複雑であり、どの変数に対しても、
架空の複数の型があることを追いかけなければならないのであれば、コードに対して行える保証が
少なくなってしまうでしょう。

### ループでの繰り返し

一連のコードを1回以上実行できることは、しばしば便利です。この作業用に、Rustにはいくつかの
*ループ*が用意されています。ループは、本体内のコードを最後まで実行し、直後にまた最初から開始します。
ループを試してみるのに、*loops*という名の新プロジェクトを作りましょう。

Rustには3種類のループが存在します: `loop`と`while`と`for`です。 それぞれ試してみましょう。

#### `loop`でコードを繰り返す

`loop`キーワードを使用すると、同じコードを何回も何回も永遠に明示的にやめさせるまで実行します。

例として、*loops*ディレクトリの*src/main.rs*ファイルを以下のような感じに書き換えましょう:

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
fn main() {
    loop {
        println!("again!");   // また
    }
}
```

このプログラムを実行すると、プログラムを手動で止めるまで、何度も何度も続けて`again!`と出力するでしょう。
ほとんどのターミナルでctrl-Cというショートカットが使え、永久ループに囚われてしまったプログラムを終了させられます。
試しにやってみましょう:

```text
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

`^C`という記号が出た場所が、ctrl-Cを押した場所です。`^C`の後には`again!`と表示されたり、
されなかったりします。ストップシグナルをコードが受け取った時にループのどこにいたかによります。

幸いなことに、Rustにはループを抜け出す別のより信頼できる手段があります。ループ内に`break`
キーワードを配置することでプログラムに実行を終了すべきタイミングを教えることができます。
第2章の「正しい予想をした後に終了する」節の数当てゲーム内でこれをして、ユーザが予想を的中させ、
ゲームに勝った時にプログラムを終了させたことを思い出してください。

#### `while`で条件付きループ

プログラムにとってループ内で条件式を評価できると、便利なことがしばしばあります。条件が真の間、
ループが走るわけです。条件が真でなくなった時に`break`を呼び出し、ループを終了します。
このタイプのループは、`loop`、`if`、`else`、`break`を組み合わせることで実装できます;
なんならプログラムで試してみるのもいいでしょう。

しかし、このパターンは頻出するので、Rustにはそれ用の文法要素が用意されており、`while`ループと呼ばれます。
以下の例では、`while`を使用しています: プログラムは3回ループし、それぞれカウントダウンします。
そして、ループ後に別のメッセージを表示して終了します:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    // 発射！
    println!("LIFTOFF!!!");
}
```

この文法要素により、`loop`、`if`、`else`、`break`を使った時に必要になるネストがなくなり、
より明確になります。条件が真の間、コードは実行されます; そうでなければ、ループを抜けます.

#### `for`でコレクションを覗き見る

`while`要素を使って配列などのコレクションの要素を覗き見ることができます。例です:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        // 値は{}です
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}
```

<span class="caption">リスト3-5: `while`ループでコレクションの各要素を覗き見る</span>

ここで、コードは配列の要素を順番にカウントアップして覗いています。番号0から始まり、
配列の最終番号に到達するまでループします(つまり、`index < 5`が真でなくなる時です)。
このコードを走らせると、配列内の全要素が出力されます:

```text
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

予想通り、配列の5つの要素が全てターミナルに出力されました。`index`変数の値はどこかで`5`という値になるものの、
配列から6番目の値を拾おうとする前にループは実行を終了します。

しかし、このアプローチは間違いが発生しやすいです; 添え字の長さが間違っていれば、プログラムは
パニックしてしまいます。また遅いです。コンパイラが実行時にループの各回ごとに境界値チェックを行う
ようなコードを追加するからです。

より効率的な対立案として、`for`ループを使ってコレクションの各アイテムにコードを実行することができます。
`for`ループはこんな見た目です:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        // 値は{}です
        println!("the value is: {}", element);
    }
}
```

<span class="caption">リスト3-6: `for`ループを使ってコレクションの各要素を覗き見る</span>

このコードを走らせたら、リスト3-5と同じ出力が得られるでしょう。より重要なのは、コードの安全性を
向上させ、配列の終端を超えてアクセスしたり、終端に届く前にループを終えてアイテムを見逃してしまったりする
バグの可能性を完全に排除したことです。

例えば、リスト3-5のコードで、`a`配列からアイテムを1つ削除したのに、条件式を`while index < 4`に
するのを忘れていたら、コードはパニックします。`for`ループを使っていれば、配列の要素数を変えても、
他のコードをいじることを覚えておく必要はなくなるわけです。

`for`ループのこの安全性と簡潔性により、Rustで使用頻度の最も高いループになっています。
リスト3-5で`while`ループを使ったカウントダウンサンプルのように、一定の回数、同じコードを実行したい
ような状況であっても、多くのRust市民は、`for`ループを使うでしょう。どうやってやるかといえば、
`Range`型を使うのです。Range型は、標準ライブラリで提供される片方の数字から始まって、もう片方の数字未満の
数値を順番に生成する型です。

`for`ループを使い、まだ話していない別のメソッド`rev`を使って範囲を逆順にした
カウントダウンはこうなります:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

こちらのコードの方が少しいいでしょう？

## まとめ

やりましたね！結構長い章でした: 変数とスカラー値、`if`式、そして、ループについて学びました！
この章で議論した概念について経験を積みたいのであれば、以下のことをするプログラムを組んでみてください:

* 温度を華氏と摂氏で変換する。
* フィボナッチ数列のn番目を生成する。
* クリスマスキャロルの定番、"The Twelve Days of Christmas"の歌詞を
曲の反復性を利用して出力する。

次に進む準備ができたら、他の言語にはあまり存在*しない*Rustの概念について話しましょう: 所有権です。
