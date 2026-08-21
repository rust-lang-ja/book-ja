<!--
## Unrecoverable Errors with `panic!`
-->

## `panic!`で回復不能なエラー

<!--
Sometimes, bad things happen in your code, and there’s nothing you can do about
it. In these cases, Rust has the `panic!` macro. There are two ways to cause a
panic in practice: by taking an action that causes our code to panic (such as
accessing an array past the end) or by explicitly calling the `panic!` macro.
In both cases, we cause a panic in our program. By default, these panics will
print a failure message, unwind, clean up the stack, and quit. Via an
environment variable, you can also have Rust display the call stack when a
panic occurs to make it easier to track down the source of the panic.
-->

時として、コード内で悪いことが起き、それに対してできることは何も無い、ということがあるでしょう。
このような場面で、Rustには`panic!`マクロが用意されています。
実際にパニックを発生させる方法は2つあります:
コードをパニックさせるような操作（配列にその終端を超えてアクセスするなど）を行う方法と、明示的に`panic!`マクロを呼び出す方法です。
いずれの場合でも、プログラムでパニックが発生します。
デフォルトでは、これらのパニックは失敗メッセージを出力し、スタックを巻き戻し、片付け、プログラムを終了させます。
パニックが発生したときにはそのパニックの発生源を特定しやすくするために、
環境変数を介してコールスタックを表示するように指示することもできます。


<!--
> ### Unwinding the Stack or Aborting in Response to a Panic
>
> By default, when a panic occurs, the program starts *unwinding*, which
> means Rust walks back up the stack and cleans up the data from each function
> it encounters. However, this walking back and cleanup is a lot of work. Rust,
> therefore, allows you to choose the alternative of immediately *aborting*,
> which ends the program without cleaning up.
>
> Memory that the program was using will then need to be cleaned
> up by the operating system. If in your project you need to make the resulting
> binary as small as possible, you can switch from unwinding to aborting upon a
> panic by adding `panic = 'abort'` to the appropriate `[profile]` sections in
> your *Cargo.toml* file. For example, if you want to abort on panic in release
> mode, add this:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```
-->

> ### パニックに対してスタックを巻き戻すかアボートするか
>
> デフォルトでは、パニックが発生すると、プログラムは*巻き戻し*を始めます。
> つまり、コンパイラ生成コードがスタックを遡り、遭遇した各関数のデータを片付けるということです。
> しかし、この遡行と片付けはすべきことが多くなります。そのためRustでは、
> 即座に*アボート*し、片付けをせずにプログラムを終了させることを、代替として選択できるようになっています。
>
> そうなると、プログラムが使用していたメモリは、
> OSが片付ける必要があります。プロジェクトにおいて、実行可能ファイルを極力小さくする必要があれば、
> *Cargo.toml*ファイルの適切な`[profile]`欄に`panic = 'abort'`を追記することで、
> パニック時に巻き戻しからアボートするように切り替えることができます。例として、
> リリースモード時にアボートするようにしたければ、以下を追記してください:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

<!--
Let’s try calling `panic!` in a simple program:
-->

単純なプログラムで`panic!`の呼び出しを試してみましょう:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

<!--
When you run the program, you’ll see something like this:
-->

このプログラムを実行すると、以下のような出力を目の当たりにするでしょう:

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

<!--
The call to `panic!` causes the error message contained in the last two lines.
The first line shows our panic message and the place in our source code where
the panic occurred: *src/main.rs:2:5* indicates that it’s the second line,
fifth character of our *src/main.rs* file.
-->

`panic!`の呼び出しが、最後の2行に含まれるエラーメッセージを発生させているのです。
1行目にパニックメッセージとソースコード中でパニックが発生した箇所を示唆しています:
*src/main.rs:2:5*は、*src/main.rs*ファイルの2行目5文字目であることを示しています。

<!--
In this case, the line indicated is part of our code, and if we go to that
line, we see the `panic!` macro call. In other cases, the `panic!` call might
be in code that our code calls, and the filename and line number reported by
the error message will be someone else’s code where the `panic!` macro is
called, not the line of our code that eventually led to the `panic!` call. We
can use the backtrace of the functions the `panic!` call came from to figure
out the part of our code that is causing the problem. We’ll discuss backtraces
in more detail next.
-->

この場合、示唆される行は、自分のコードの一部で、その箇所を見に行けば、`panic!`マクロ呼び出しがあるわけです。
それ以外では、`panic!`呼び出しが、自分のコードが呼び出しているコードの一部になっている可能性もあるわけです。
エラーメッセージで報告されるファイル名と行番号が、結果的に`panic!`呼び出しに導いた自分のコードの行ではなく、
`panic!`マクロが呼び出されている他人のコードになるでしょう。`panic!`呼び出しの発生元である関数のバックトレースを使用して、
問題を起こしている自分のコードの箇所を割り出すことができます。次はバックトレースについてはより詳しく議論しましょう。

<!--
### Using a `panic!` Backtrace
-->

### `panic!`バックトレースを使用する

<!--
Let’s look at another example to see what it’s like when a `panic!` call comes
from a library because of a bug in our code instead of from our code calling
the macro directly. Listing 9-1 has some code that attempts to access an
index in a vector beyond the range of valid indexes.
-->

別の例を眺めて、自分のコードでマクロを直接呼び出す代わりに、コードに存在するバグにより、
ライブラリで`panic!`呼び出しが発生するとどんな感じなのか確かめてみましょう。リスト9-1は、
ベクタに有効な添え字の範囲の外の添え字でアクセスを試みるコードです。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

<!--
<span class="caption">Listing 9-1: Attempting to access an element beyond the
end of a vector, which will cause a call to `panic!`</span>
-->

<span class="caption">リスト9-1: ベクタの境界を超えて要素へのアクセスを試み、`panic!`の呼び出しを発生させる</span>

<!--
Here, we’re attempting to access the 100th element of our vector (which is at
index 99 because indexing starts at zero), but the vector has only 3 elements.
In this situation, Rust will panic. Using `[]` is supposed to return an
element, but if you pass an invalid index, there’s no element that Rust could
return here that would be correct.
-->

ここでは、ベクタの100番目の要素(添え字は0始まりなので添え字99)にアクセスを試みていますが、ベクタには3つしか要素がありません。
この場面では、Rustはパニックします。`[]`の使用は、要素を返すと想定されるものの、
無効な添え字を渡せば、ここでRustが返せて正しいと思われる要素は何もないわけです。

<!--
In C, attempting to read beyond the end of a data structure is undefined
behavior. You might get whatever is at the location in memory that would
correspond to that element in the data structure, even though the memory
doesn’t belong to that structure. This is called a *buffer overread* and can
lead to security vulnerabilities if an attacker is able to manipulate the index
in such a way as to read data they shouldn’t be allowed to that is stored after
the data structure.
-->

Cでは、データ構造の終端を超えて読み込みを行おうとすることは未定義動作です。
メモリがデータ構造に属していないにもかかわらず、そのデータ構造内のその要素に対応するメモリ上の箇所にある何かを返してくるかもしれません。
これは、*バッファオーバーリード* (*buffer overread*)と呼ばれ、
攻撃者が、データ構造の後に格納された読めるべきでないデータを読み出せるように添え字を操作できたら、
セキュリティ脆弱性につながる可能性があります。

<!--
To protect your program from this sort of vulnerability, if you try to read an
element at an index that doesn’t exist, Rust will stop execution and refuse to
continue. Let’s try it and see:
-->

この種の脆弱性からプログラムを保護するために、存在しない添え字の要素を読もうとしたら、
Rustは実行を中止し、継続を拒みます。試して確認してみましょう:

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

<!--
This error points at line 4 of our `main.rs` where we attempt to access index
99. The next note line tells us that we can set the `RUST_BACKTRACE`
environment variable to get a backtrace of exactly what happened to cause the
error. A *backtrace* is a list of all the functions that have been called to
get to this point. Backtraces in Rust work as they do in other languages: the
key to reading the backtrace is to start from the top and read until you see
files you wrote. That’s the spot where the problem originated. The lines above
that spot are code that your code has called; the lines below are code that
called your code. These before-and-after lines might include core Rust code,
standard library code, or crates that you’re using. Let’s try getting a
backtrace by setting the `RUST_BACKTRACE` environment variable to any value
except 0. Listing 9-2 shows output similar to what you’ll see.
-->

このエラーは*main.rs*の4行目を指していて、ここではインデックス99にアクセスを試みています。
その次の注釈行は、`RUST_BACKTRACE`環境変数をセットして、まさしく何が起き、
エラーが発生したのかのバックトレースを得られることを教えてくれています。
*バックトレース*とは、ここに至るまでに呼び出された全関数の一覧です。Rustのバックトレースも、
他の言語同様に動作します: バックトレースを読むコツは、頭からスタートして自分のファイルを見つけるまで読むことです。
そこが、問題が発生した場所です。この場所より上の行は、自分のコードが呼び出したコードになります;
それより下の行は、自分のコードを呼び出しているコードになります。これらの前後の行には、Rustの核となるコード、標準ライブラリのコード、
使用しているクレートなどが含まれるかもしれません。`RUST_BACKTRACE`環境変数を0以外の値にセットして、
バックトレースを出力してみましょう。リスト9-2のような出力が得られるでしょう。

<!-- manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/std/src/panicking.rs:645:5
   1: core::panicking::panic_fmt
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/panicking.rs:72:14
   2: core::panicking::panic_bounds_check
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/panicking.rs:208:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/slice/index.rs:255:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/alloc/src/vec/mod.rs:2770:9
   6: panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

<!--
<span class="caption">Listing 9-2: The backtrace generated by a call to
`panic!` displayed when the environment variable `RUST_BACKTRACE` is set</span>
-->

<span class="caption">リスト9-2: `RUST_BACKTRACE`環境変数をセットした時に表示される、
`panic!`呼び出しが生成するバックトレース</span>

<!--
That’s a lot of output! The exact output you see might be different depending
on your operating system and Rust version. In order to get backtraces with this
information, debug symbols must be enabled. Debug symbols are enabled by
default when using `cargo build` or `cargo run` without the `--release` flag,
as we have here.
-->

出力が多いですね！OSやRustのバージョンによって、出力の詳細は変わる可能性があります。この情報とともに、
バックトレースを得るには、デバッグシンボルを有効にしなければなりません。デバッグシンボルは、
`--release`オプションなしで`cargo build`や`cargo run`を使用していれば、標準で有効になり、
ここではそうなっています。

<!--
In the output in Listing 9-2, line 6 of the backtrace points to the line in our
project that’s causing the problem: line 4 of *src/main.rs*. If we don’t want
our program to panic, we should start our investigation at the location pointed
to by the first line mentioning a file we wrote. In Listing 9-1, where we
deliberately wrote code that would panic, the way to fix the panic is to not
request an element beyond the range of the vector indexes. When your code
panics in the future, you’ll need to figure out what action the code is taking
with what values to cause the panic and what the code should do instead.
-->

リスト9-2の出力で、バックトレースの6行目が問題発生箇所を指し示しています: *src/main.rs*の4行目です。
プログラムにパニックしてほしくなければ、自分のファイルについて言及している最初の行で示されている箇所から調査を開始すべきです。
わざとパニックするコードを書いたリスト9-1において、パニックを解消する方法は、
ベクタの添え字の範囲を超えた要素を要求しないようにすることです。
将来コードがパニックしたら、パニックを引き起こすどんな値でコードがどんな動作をしているのかと、
代わりにコードは何をすべきなのかを算出する必要があるでしょう。

<!--
We’ll come back to `panic!` and when we should and should not use `panic!` to
handle error conditions in the [“To `panic!` or Not to
`panic!`”][to-panic-or-not-to-panic] section later in this
chapter. Next, we’ll look at how to recover from an error using `Result`.
-->

この章の後ほど、[「`panic!`すべきかするまいか」][to-panic-or-not-to-panic]節で`panic!`とエラー状態を扱うのに`panic!`を使うべき時と使わぬべき時に戻ってきます。
次は、`Result`を使用してエラーから回復する方法を見ましょう。

<!--
[to-panic-or-not-to-panic]:
ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic
-->

[to-panic-or-not-to-panic]:
ch09-03-to-panic-or-not-to-panic.html#panicすべきかするまいか
