<!-- ## Unrecoverable Errors with `panic!` -->

## `panic!`で回復不能なエラー

<!-- Sometimes, bad things happen in your code, and there’s nothing you can do about -->
<!-- it. In these cases, Rust has the `panic!` macro. When the `panic!` macro -->
<!-- executes, your program will print a failure message, unwind and clean up the -->
<!-- stack, and then quit. The most common situation this occurs in is when a bug of -->
<!-- some kind has been detected, and it’s not clear to the programmer how to handle -->
<!-- the error. -->

時として、コードで悪いことが起きるものです。そして、それに対してできることは何もありません。
このような場面で、Rustには`panic!`マクロが用意されています。`panic!`マクロが実行されると、
プログラムは失敗のメッセージを表示し、スタックを巻き戻し掃除して、終了します。こうなる最もありふれた場面は、
何らかのバグが検出された時であり、プログラマには、どうエラーを処理すればいいか明確ではありません。

<!--  ### Unwinding the Stack or Aborting in Response to a `panic!` -->
<!--  -->
<!--  By default, when a `panic!` occurs, the program starts *unwinding*, which -->
<!--  means Rust walks back up the stack and cleans up the data from each function -->
<!--  it encounters. But this walking back and cleanup is a lot of work. The -->
<!--  alternative is to immediately *abort*, which ends the program without -->
<!--  cleaning up. Memory that the program was using will then need to be cleaned -->
<!--  up by the operating system. If in your project you need to make the resulting -->
<!--  binary as small as possible, you can switch from unwinding to aborting on -->
<!--  panic by adding `panic = 'abort'` to the appropriate `[profile]` sections in -->
<!--  your *Cargo.toml* file. For example, if you want to abort on panic in release -->
<!--  mode, add this: -->
<!--  -->
<!--  ```toml -->
<!--  [profile.release] -->
<!--  panic = 'abort' -->
<!--  ``` -->

> ### `panic!`に対してスタックを巻き戻すか異常終了するか
>
> 標準では、`panic!`が発生すると、プログラムは*巻き戻し*を始めます。つまり、言語がスタックを遡り、
> 遭遇した各関数のデータを片付けるということです。しかし、この遡りと片付けはすべきことが多くなります。
> 対立案は、即座に異常終了し、片付けをせずにプログラムを終了させることです。そうなると、プログラムが使用していたメモリは、
> OSが片付ける必要があります。プロジェクトにおいて、実行可能ファイルを極力小さくする必要があれば、
> *Cargo.toml*ファイルの適切な`[profile]`欄に`panic = 'abort'`を追記することで、
> パニック時に巻き戻しから異常終了するように切り替えることができます。例として、
> リリースモード時に異常終了するようにしたければ、以下を追記してください:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

<!-- Let’s try calling `panic!` in a simple program: -->

単純なプログラムで`panic!`の呼び出しを試してみましょう:

<!-- <span class="filename">Filename: src/main.rs</span> -->

<span class="filename">ファイル名: src/main.rs</span>

```rust,should_panic
fn main() {
    panic!("crash and burn");  //クラッシュして炎上
}
```

<!-- When you run the program, you’ll see something like this: -->

このプログラムを実行すると、以下のような出力を目の当たりにするでしょう:

```text
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25 secs
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2
('main'スレッドはsrc/main.rs:2の「クラッシュして炎上」でパニックしました)
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```

<!-- The call to `panic!` causes the error message contained in the last three -->
<!-- lines. The first line shows our panic message and the place in our source code -->
<!-- where the panic occurred: *src/main.rs:2* indicates that it’s the second line -->
<!-- of our *src/main.rs* file. -->

`panic!`の呼び出しが、最後の3行に含まれるエラーメッセージを発生させているのです。
1行目にパニックメッセージとソースコード中でパニックが発生した箇所を示唆しています:
*src/main.rs:2*は、*src/main.rs*ファイルの2行目であることを示しています。

<!-- In this case, the line indicated is part of our code, and if we go to that -->
<!-- line, we see the `panic!` macro call. In other cases, the `panic!` call might -->
<!-- be in code that our code calls. The filename and line number reported by the -->
<!-- error message will be someone else’s code where the `panic!` macro is called, -->
<!-- not the line of our code that eventually led to the `panic!` call. We can use -->
<!-- the backtrace of the functions the `panic!` call came from to figure out the -->
<!-- part of our code that is causing the problem. We’ll discuss what a backtrace is -->
<!-- in more detail next. -->

この場合、示唆される行は、自分のコードの一部で、その箇所を見に行けば、`panic!`マクロ呼び出しがあるわけです。
`panic!`呼び出しが、自分のコードが呼び出しているコードの一部になっている可能性もあるわけです。
エラーメッセージで報告されるファイル名と行番号が、結果的に`panic!`呼び出しに導いた自分のコードの行ではなく、
`panic!`マクロが呼び出されている他人のコードになるでしょう。`panic!`呼び出しの発生元である関数のバックトレースを使用して、
問題を起こしている自分のコードの箇所を割り出すことができます。バックトレースがどんなものか、次に議論しましょう。

<!-- ### Using a `panic!` Backtrace -->

### `panic!`バックトレースを使用する

<!-- Let’s look at another example to see what it’s like when a `panic!` call comes -->
<!-- from a library because of a bug in our code instead of from our code calling -->
<!-- the macro directly. Listing 9-1 has some code that attempts to access an -->
<!-- element by index in a vector: -->

別の例を眺めて、自分のコードでマクロを直接呼び出す代わりに、コードに存在するバグにより、
ライブラリで`panic!`呼び出しが発生するとどんな感じなのか確かめてみましょう。リスト9-1は、
添え字でベクタの要素にアクセスを試みるあるコードです:

<!-- <span class="filename">Filename: src/main.rs</span> -->

<span class="filename">ファイル名: src/main.rs</span>

```rust,should_panic
fn main() {
    let v = vec![1, 2, 3];

    v[100];
}
```

<!-- <span class="caption">Listing 9-1: Attempting to access an element beyond the -->
<!-- end of a vector, which will cause a `panic!`</span> -->

<span class="caption">リスト9-1: ベクタの境界を超えて要素へのアクセスを試み、
  `panic!`を発生させる</span>

<!-- Here, we’re attempting to access the hundredth element of our vector, but it -->
<!-- has only three elements. In this situation, Rust will panic. Using `[]` is -->
<!-- supposed to return an element, but if you pass an invalid index, there’s no -->
<!-- element that Rust could return here that would be correct. -->

ここでは、ベクタの100番目の要素にアクセスを試みていますが、ベクタには3つしか要素がありません。
この場面では、Rustはパニックします。`[]`の使用は、要素を返すと想定されるものの、
無効な番号を渡せば、ここでRustが返せて正しいと思われる要素は何もないわけです。

<!-- Other languages, like C, will attempt to give you exactly what you asked for in -->
<!-- this situation, even though it isn’t what you want: you’ll get whatever is at -->
<!-- the location in memory that would correspond to that element in the vector, -->
<!-- even though the memory doesn’t belong to the vector. This is called a *buffer -->
<!-- overread* and can lead to security vulnerabilities if an attacker is able to -->
<!-- manipulate the index in such a way as to read data they shouldn’t be allowed to -->
<!-- that is stored after the array. -->

他の言語(Cなど)では、この場面で欲しいものではないにもかかわらず、まさしく要求したものを返そうとしてきます:
メモリがベクタに属していないにもかかわらず、ベクタ内のその要素に対応するメモリ上の箇所にあるものを何か返してくるのです。
これは、*バッファー外読み出し*(buffer overread; `脚注`: 初めて見かけた表現。Rust独自？)と呼ばれ、
攻撃者が、配列の後に格納された読めるべきでないデータを読み出せるように添え字を操作できたら、
セキュリティ脆弱性につながる可能性があります。

<!-- To protect your program from this sort of vulnerability, if you try to read an -->
<!-- element at an index that doesn’t exist, Rust will stop execution and refuse to -->
<!-- continue. Let’s try it and see: -->

この種の脆弱性からプログラムを保護するために、存在しない番号の要素を読もうとしたら、
Rustは実行を中止し、継続を拒みます。試して確認してみましょう:

```text
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
100', /stable-dist-rustc/build/src/libcollections/vec.rs:1362
('main'スレッドは、/stable-dist-rustc/build/src/libcollections/vec.rs:1362の
「境界外番号: 長さは3なのに、添え字は100です」でパニックしました)
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```

<!-- This error points at a file we didn’t write, *libcollections/vec.rs*. That’s -->
<!-- the implementation of `Vec<T>` in the standard library. The code that gets run -->
<!-- when we use `[]` on our vector `v` is in *libcollections/vec.rs*, and that is -->
<!-- where the `panic!` is actually happening. -->

このエラーは、自分のファイルではない*libcollections/vec.rs*ファイルを指しています。
標準ライブラリの`Vec<T>`の実装です。ベクタ`v`に対して`[]`を使った時に走るコードは、
*libcollections/vec.rs*に存在し、ここで実際に`panic!`が発生しているのです。

<!-- The next note line tells us that we can set the `RUST_BACKTRACE` environment -->
<!-- variable to get a backtrace of exactly what happened to cause the error. A -->
<!-- *backtrace* is a list of all the functions that have been called to get to this -->
<!-- point. Backtraces in Rust work like they do in other languages: the key to -->
<!-- reading the backtrace is to start from the top and read until you see files you -->
<!-- wrote. That’s the spot where the problem originated. The lines above the lines -->
<!-- mentioning your files are code that your code called; the lines below are code -->
<!-- that called your code. These lines might include core Rust code, standard -->
<!-- library code, or crates that you’re using. Let’s try getting a backtrace: -->
<!-- Listing 9-2 shows output similar to what you’ll see: -->

その次の注釈行は、`RUST_BACKTRACE`環境変数をセットして、まさしく何が起き、
エラーが発生したのかのバックトレースを得られることを教えてくれています。
*バックトレース*とは、ここに至るまでに呼び出された全関数の一覧です。Rustのバックトレースも、
他の言語同様に動作します: バックトレースを読むコツは、頭からスタートして自分のファイルを見つけるまで読むことです。
そこが、問題の根源になるのです。自分のファイルを言及している箇所以前は、自分のコードで呼び出したコードになります;
以後は、自分のコードを呼び出しているコードになります。これらの行には、Rustの核となるコード、標準ライブラリのコード、
使用しているクレートなどが含まれるかもしれません。バックトレースを出力してみましょう:
リスト9-2のような出力が得られるでしょう:

```text
$ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 100', /stable-dist-rustc/build/src/libcollections/vec.rs:1392
stack backtrace:
   1:     0x560ed90ec04c - std::sys::imp::backtrace::tracing::imp::write::hf33ae72d0baa11ed
                        at /stable-dist-rustc/build/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x560ed90ee03e - std::panicking::default_hook::{{closure}}::h59672b733cc6a455
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:351
   3:     0x560ed90edc44 - std::panicking::default_hook::h1670459d2f3f8843
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:367
   4:     0x560ed90ee41b - std::panicking::rust_panic_with_hook::hcf0ddb069e7abcd7
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:555
   5:     0x560ed90ee2b4 - std::panicking::begin_panic::hd6eb68e27bdf6140
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:517
   6:     0x560ed90ee1d9 - std::panicking::begin_panic_fmt::abcd5965948b877f8
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:501
   7:     0x560ed90ee167 - rust_begin_unwind
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:477
   8:     0x560ed911401d - core::panicking::panic_fmt::hc0f6d7b2c300cdd9
                        at /stable-dist-rustc/build/src/libcore/panicking.rs:69
   9:     0x560ed9113fc8 - core::panicking::panic_bounds_check::h02a4af86d01b3e96
                        at /stable-dist-rustc/build/src/libcore/panicking.rs:56
  10:     0x560ed90e71c5 - <collections::vec::Vec<T> as core::ops::Index<usize>>::index::h98abcd4e2a74c41
                        at /stable-dist-rustc/build/src/libcollections/vec.rs:1392
  11:     0x560ed90e727a - panic::main::h5d6b77c20526bc35
                        at /home/you/projects/panic/src/main.rs:4
  12:     0x560ed90f5d6a - __rust_maybe_catch_panic
                        at /stable-dist-rustc/build/src/libpanic_unwind/lib.rs:98
  13:     0x560ed90ee926 - std::rt::lang_start::hd7c880a37a646e81
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:436
                        at /stable-dist-rustc/build/src/libstd/panic.rs:361
                        at /stable-dist-rustc/build/src/libstd/rt.rs:57
  14:     0x560ed90e7302 - main
  15:     0x7f0d53f16400 - __libc_start_main
  16:     0x560ed90e6659 - _start
  17:                0x0 - <unknown>
```

<!-- <span class="caption">Listing 9-2: The backtrace generated by a call to -->
<!-- `panic!` displayed when the environment variable `RUST_BACKTRACE` is set</span> -->

<span class="caption">リスト9-2: `RUST_BACKTRACE`環境変数をセットした時に表示される、
`panic!`呼び出しが生成するバックトレース</span>

<!-- That’s a lot of output! The exact output you see might be different depending -->
<!-- on your operating system and Rust version. In order to get backtraces with this -->
<!-- information, debug symbols must be enabled. Debug symbols are enabled by -->
<!-- default when using cargo build or cargo run without the --release flag, as we -->
<!-- have here. -->

出力が多いですね！OSやRustのバージョンによって、出力の詳細は変わる可能性があります。この情報とともに、
バックトレースを得るには、デバッグシンボルを有効にしなければなりません。デバッグシンボルは、
--releaseオプションなしでcargo buildやcargo runを使用していれば、標準で有効になり、
ここではそうなっています。

<!-- In the output in Listing 9-2, line 11 of the backtrace points to the line in -->
<!-- our project that’s causing the problem: *src/main.rs* in line 4. If we don’t -->
<!-- want our program to panic, the location pointed to by the first line mentioning -->
<!-- a file we wrote is where we should start investigating to figure out how we got -->
<!-- to this location with values that caused the panic. In Listing 9-1 where we -->
<!-- deliberately wrote code that would panic in order to demonstrate how to use -->
<!-- backtraces, the way to fix the panic is to not request an element at index 100 -->
<!-- from a vector that only contains three items. When your code panics in the -->
<!-- future, you’ll need to figure out what action the code is taking with what -->
<!-- values that causes the panic and what the code should do instead. -->

リスト9-2の出力で、バックトレースの11行目が問題発生箇所を指し示しています: *src/main.rs*の4行目です。
プログラムにパニックしてほしくなければ、自分のファイルについて言及している最初の行で示されている箇所が、
どのようにパニックを引き起こす値でこの箇所にたどり着いたか割り出すために調査を開始すべき箇所になります。
バックトレースの使用法を模擬するためにわざとパニックするコードを書いたリスト9-1において、
パニックを解消する方法は、3つしか要素のないベクタの100番目の要素を要求しないことです。
将来コードがパニックしたら、パニックを引き起こすどんな値でコードがどんな動作をしているのかと、
代わりにコードは何をすべきなのかを算出する必要があるでしょう。

<!-- We’ll come back to `panic!` and when we should and should not use `panic!` to -->
<!-- handle error conditions later in the chapter. Next, we’ll look at how to -->
<!-- recover from an error using `Result`. -->

また、この章で`panic!`とエラー状態を扱うのに`panic!`を使うべき時と使わぬべき時について触れます。
次は、`Result`を使用してエラーから回復する方法を見ましょう。
