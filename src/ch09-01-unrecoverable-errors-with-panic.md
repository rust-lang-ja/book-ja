<!--
## Unrecoverable Errors with `panic!`
-->

## `panic!`で回復不能なエラー

<!--
Sometimes, bad things happen in your code, and there’s nothing you can do about
it. In these cases, Rust has the `panic!` macro. When the `panic!` macro
executes, your program will print a failure message, unwind and clean up the
stack, and then quit. This most commonly occurs when a bug of some kind has
been detected, and it’s not clear to the programmer how to handle the error.
-->

時として、コードで悪いことが起きるものです。そして、それに対してできることは何もありません。
このような場面で、Rust には`panic!`マクロが用意されています。`panic!`マクロが実行されると、
プログラムは失敗のメッセージを表示し、スタックを巻き戻し掃除して、終了します。これが最もありふれて起こるのは、
何らかのバグが検出された時であり、プログラマには、どうエラーを処理すればいいか明確ではありません。

<!--
> ### Unwinding the Stack or Aborting in Response to a Panic
>
> By default, when a panic occurs, the program starts *unwinding*, which
> means Rust walks back up the stack and cleans up the data from each function
> it encounters. But this walking back and cleanup is a lot of work. The
> alternative is to immediately *abort*, which ends the program without
> cleaning up. Memory that the program was using will then need to be cleaned
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

> ### パニックに対してスタックを巻き戻すか異常終了するか
>
> 標準では、パニックが発生すると、プログラムは*巻き戻し*を始めます。つまり、言語がスタックを遡り、
> 遭遇した各関数のデータを片付けるということです。しかし、この遡りと片付けはすべきことが多くなります。
> 対立案は、即座に異常終了し、片付けをせずにプログラムを終了させることです。そうなると、プログラムが使用していたメモリは、
> OS が片付ける必要があります。プロジェクトにおいて、実行可能ファイルを極力小さくする必要があれば、
> *Cargo.toml*ファイルの適切な`[profile]`欄に`panic = 'abort'`を追記することで、
> パニック時に巻き戻しから異常終了するように切り替えることができます。例として、
> リリースモード時に異常終了するようにしたければ、以下を追記してください：
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

<!--
Let’s try calling `panic!` in a simple program:
-->

単純なプログラムで`panic!`の呼び出しを試してみましょう：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,should_panic
fn main() {
    panic!("crash and burn");  //クラッシュして炎上
}
```

<!--
When you run the program, you’ll see something like this:
-->

このプログラムを実行すると、以下のような出力を目の当たりにするでしょう：

```text
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25 secs
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:4
('main'スレッドは src/main.rs:2:4 の「クラッシュして炎上」でパニックしました)
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

<!--
The call to `panic!` causes the error message contained in the last two lines.
The first line shows our panic message and the place in our source code where
the panic occurred: *src/main.rs:2:4* indicates that it’s the second line,
fourth character of our *src/main.rs* file.
-->

`panic!`の呼び出しが、最後の 2 行に含まれるエラーメッセージを発生させているのです。
1 行目にパニックメッセージとソースコード中でパニックが発生した箇所を示唆しています：
*src/main.rs:2:4*は、*src/main.rs*ファイルの2行目4文字目であることを示しています。

<!--
In this case, the line indicated is part of our code, and if we go to that
line, we see the `panic!` macro call. In other cases, the `panic!` call might
be in code that our code calls, and the filename and line number reported by
the error message will be someone else’s code where the `panic!` macro is
called, not the line of our code that eventually led to the `panic!` call. We
can use the backtrace of the functions the `panic!` call came from to figure
out the part of our code that is causing the problem. We’ll discuss what a
backtrace is in more detail next.
-->

この場合、示唆される行は、自分のコードの一部で、その箇所を見に行けば、`panic!`マクロ呼び出しがあるわけです。
それ以外では、`panic!`呼び出しが、自分のコードが呼び出しているコードの一部になっている可能性もあるわけです。
エラーメッセージで報告されるファイル名と行番号が、結果的に`panic!`呼び出しに導いた自分のコードの行ではなく、
`panic!`マクロが呼び出されている他人のコードになるでしょう。`panic!`呼び出しの発生元である関数のバックトレースを使用して、
問題を起こしている自分のコードの箇所を割り出すことができます。バックトレースがどんなものか、次に議論しましょう。

<!--
### Using a `panic!` Backtrace
-->

### `panic!`バックトレースを使用する

<!--
Let’s look at another example to see what it’s like when a `panic!` call comes
from a library because of a bug in our code instead of from our code calling
the macro directly. Listing 9-1 has some code that attempts to access an
element by index in a vector.
-->

別の例を眺めて、自分のコードでマクロを直接呼び出す代わりに、コードに存在するバグにより、
ライブラリで`panic!`呼び出しが発生するとどんな感じなのか確かめてみましょう。リスト 9-1 は、
添え字でベクタの要素にアクセスを試みる何らかのコードです。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,should_panic
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

<!--
<span class="caption">Listing 9-1: Attempting to access an element beyond the
end of a vector, which will cause a call to `panic!`</span>
-->

<span class="caption">リスト 9-1: ベクタの境界を超えて要素へのアクセスを試み、`panic!`の呼び出しを発生させる</span>

<!--
Here, we’re attempting to access the 100th element of our vector (which is at
index 99 because indexing starts at zero), but it has only 3 elements. In this
situation, Rust will panic. Using `[]` is supposed to return an element, but if
you pass an invalid index, there’s no element that Rust could return here that
would be correct.
-->

ここでは、ベクタの 100 番目の要素 (添え字は 0 始まりなので添え字 99) にアクセスを試みていますが、ベクタには 3 つしか要素がありません。
この場面では、Rust はパニックします。`[]`の使用は、要素を返すと想定されるものの、
無効な添え字を渡せば、ここで Rust が返せて正しいと思われる要素は何もないわけです。

<!--
Other languages, like C, will attempt to give you exactly what you asked for in
this situation, even though it isn’t what you want: you’ll get whatever is at
the location in memory that would correspond to that element in the vector,
even though the memory doesn’t belong to the vector. This is called a *buffer
overread* and can lead to security vulnerabilities if an attacker is able to
manipulate the index in such a way as to read data they shouldn’t be allowed to
that is stored after the array.
-->

他の言語 (C など) では、この場面で欲しいものではないにもかかわらず、まさしく要求したものを返そうとしてきます：
メモリがベクタに属していないにもかかわらず、ベクタ内のその要素に対応するメモリ上の箇所にあるものを何か返してくるのです。
これは、*バッファー外読み出し*(buffer overread; `訳注`: バッファー読みすぎとも解釈できるか) と呼ばれ、
攻撃者が、配列の後に格納された読めるべきでないデータを読み出せるように添え字を操作できたら、
セキュリティ脆弱性につながる可能性があります。

<!--
To protect your program from this sort of vulnerability, if you try to read an
element at an index that doesn’t exist, Rust will stop execution and refuse to
continue. Let’s try it and see:
-->

この種の脆弱性からプログラムを保護するために、存在しない添え字の要素を読もうとしたら、
Rust は実行を中止し、継続を拒みます。試して確認してみましょう：

```text
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
99', /checkout/src/liballoc/vec.rs:1555:10
('main'スレッドは、/checkout/src/liballoc/vec.rs:1555:10 の
「境界外番号：長さは 3 なのに、添え字は 99 です」でパニックしました)
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

<!--
This error points at a file we didn’t write, *vec.rs*. That’s the
implementation of `Vec<T>` in the standard library. The code that gets run when
we use `[]` on our vector `v` is in *vec.rs*, and that is where the `panic!` is
actually happening.
-->

このエラーは、自分のファイルではない*vec.rs*ファイルを指しています。
標準ライブラリの`Vec<T>`の実装です。ベクタ`v`に対して`[]`を使った時に走るコードは、
*vec.rs*に存在し、ここで実際に`panic!`が発生しているのです。

<!--
The next note line tells us that we can set the `RUST_BACKTRACE` environment
variable to get a backtrace of exactly what happened to cause the error. A
*backtrace* is a list of all the functions that have been called to get to this
point. Backtraces in Rust work like they do in other languages: the key to
reading the backtrace is to start from the top and read until you see files you
wrote. That’s the spot where the problem originated. The lines above the lines
mentioning your files are code that your code called; the lines below are code
that called your code. These lines might include core Rust code, standard
library code, or crates that you’re using. Let’s try getting a backtrace by
setting the `RUST_BACKTRACE` environment variable to any value except 0.
Listing 9-2 shows output similar to what you’ll see.
-->

その次の注釈行は、`RUST_BACKTRACE`環境変数をセットして、まさしく何が起き、
エラーが発生したのかのバックトレースを得られることを教えてくれています。
*バックトレース*とは、ここに至るまでに呼び出された全関数の一覧です。Rust のバックトレースも、
他の言語同様に動作します：バックトレースを読むコツは、頭からスタートして自分のファイルを見つけるまで読むことです。
そこが、問題の根源になるのです。自分のファイルを言及している箇所以前は、自分のコードで呼び出したコードになります;
以後は、自分のコードを呼び出しているコードになります。これらの行には、Rust の核となるコード、標準ライブラリのコード、
使用しているクレートなどが含まれるかもしれません。`RUST_BACKTRACE`環境変数を 0 以外の値にセットして、
バックトレースを出力してみましょう。リスト 9-2 のような出力が得られるでしょう。

```text
$ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /checkout/src/liballoc/vec.rs:1555:10
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:397
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:611
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:572
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:498
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:71
   9: core::panicking::panic_bounds_check
             at /checkout/src/libcore/panicking.rs:58
  10: <alloc::vec::Vec<T> as core::ops::index::Index<usize>>::index
             at /checkout/src/liballoc/vec.rs:1555
  11: panic::main
             at src/main.rs:4
  12: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:99
  13: std::rt::lang_start
             at /checkout/src/libstd/panicking.rs:459
             at /checkout/src/libstd/panic.rs:361
             at /checkout/src/libstd/rt.rs:61
  14: main
  15: __libc_start_main
  16: <unknown>
```

<!--
<span class="caption">Listing 9-2: The backtrace generated by a call to
`panic!` displayed when the environment variable `RUST_BACKTRACE` is set</span>
-->

<span class="caption">リスト 9-2: `RUST_BACKTRACE`環境変数をセットした時に表示される、
`panic!`呼び出しが生成するバックトレース</span>

<!--
That’s a lot of output! The exact output you see might be different depending
on your operating system and Rust version. In order to get backtraces with this
information, debug symbols must be enabled. Debug symbols are enabled by
default when using `cargo build` or `cargo run` without the `--release` flag,
as we have here.
-->

出力が多いですね！OS や Rust のバージョンによって、出力の詳細は変わる可能性があります。この情報とともに、
バックトレースを得るには、デバッグシンボルを有効にしなければなりません。デバッグシンボルは、
`--release`オプションなしで`cargo build`や`cargo run`を使用していれば、標準で有効になり、
ここではそうなっています。

<!--
In the output in Listing 9-2, line 11 of the backtrace points to the line in
our project that’s causing the problem: line 4 of *src/main.rs*. If we don’t
want our program to panic, the location pointed to by the first line mentioning
a file we wrote is where we should start investigating to figure out how we got
to this location with values that caused the panic. In Listing 9-1, where
we deliberately wrote code that would panic in order to demonstrate how to use
backtraces, the way to fix the panic is to not request an element at index 99
from a vector that only contains 3 items. When your code panics in the future,
you’ll need to figure out what action the code is taking with what values to
cause the panic and what the code should do instead.
-->

リスト 9-2 の出力で、バックトレースの 11 行目が問題発生箇所を指し示しています：*src/main.rs*の 4 行目です。
プログラムにパニックしてほしくなければ、自分のファイルについて言及している最初の行で示されている箇所が、
どのようにパニックを引き起こす値でこの箇所にたどり着いたか割り出すために調査を開始すべき箇所になります。
バックトレースの使用法を模擬するためにわざとパニックするコードを書いたリスト 9-1 において、
パニックを解消する方法は、3 つしか要素のないベクタの添え字 99 の要素を要求しないことです。
将来コードがパニックしたら、パニックを引き起こすどんな値でコードがどんな動作をしているのかと、
代わりにコードは何をすべきなのかを算出する必要があるでしょう。

<!--
We’ll come back to `panic!` and when we should and should not use `panic!` to
handle error conditions in “To `panic!` or Not to `panic!`” section later
in this chapter. Next, we’ll look at how to recover from an error using
`Result`.
-->

この章の後ほど、「`panic!`するか`panic!`するまいか」節で`panic!`とエラー状態を扱うのに`panic!`を使うべき時と使わぬべき時に戻ってきます。
次は、`Result`を使用してエラーから回復する方法を見ましょう。
