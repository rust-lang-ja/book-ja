<!--
## Recoverable Errors with `Result`
-->

## `Result`で回復可能なエラー

<!--
Most errors aren’t serious enough to require the program to stop entirely.
Sometimes, when a function fails, it’s for a reason that you can easily
interpret and respond to. For example, if you try to open a file and that
operation fails because the file doesn’t exist, you might want to create the
file instead of terminating the process.
-->

多くのエラーは、プログラムを完全にストップさせるほど深刻ではありません。時々、関数が失敗した時に、
容易に解釈し、対応できる理由によることがあります。例えば、ファイルを開こうとして、
ファイルが存在しないために処理が失敗したら、プロセスを停止するのではなく、ファイルを作成したいことがあります。

<!--
Recall from [“Handling Potential Failure with `Result`”][handle_failure]
in Chapter 2 that the `Result` enum is defined as having two
variants, `Ok` and `Err`, as follows:
-->

第2章の[「`Result`で失敗の可能性を扱う」][handle_failure]で`Result` enumが以下のように、
`Ok`と`Err`の2列挙子からなるよう定義されていることを思い出してください:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

<!--
The `T` and `E` are generic type parameters: we’ll discuss generics in more
detail in Chapter 10. What you need to know right now is that `T` represents
the type of the value that will be returned in a success case within the `Ok`
variant, and `E` represents the type of the error that will be returned in a
failure case within the `Err` variant. Because `Result` has these generic type
parameters, we can use the `Result` type and the functions defined on it in
many different situations where the successful value and error value we want to
return may differ.
-->

`T`と`E`は、ジェネリックな型引数です: ジェネリクスについて詳しくは、第10章で議論します。
たった今知っておく必要があることは、`T`が成功した時に`Ok`列挙子に含まれて返される値の型を表すことと、
`E`が失敗した時に`Err`列挙子に含まれて返されるエラーの型を表すことです。`Result`はこのようなジェネリックな型引数を含むので、
`Result`型とその上に定義されている関数を、成功した時とエラーの時に返したい値が異なるような様々な場面で使用できるのです。

<!--
Let’s call a function that returns a `Result` value because the function could
fail. In Listing 9-3 we try to open a file.
-->

関数が失敗する可能性があるために`Result`値を返す関数を呼び出しましょう: リスト9-3では、
ファイルを開こうとしています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

<!--
<span class="caption">Listing 9-3: Opening a file</span>
-->

<span class="caption">リスト9-3: ファイルを開く</span>

<!--
The return type of `File::open` is a `Result<T, E>`. The generic parameter `T`
has been filled in by the implementation of `File::open` with the type of the
success value, `std::fs::File`, which is a file handle. The type of `E` used in
the error value is `std::io::Error`. This return type means the call to
`File::open` might succeed and return a file handle that we can read from or
write to. The function call also might fail: for example, the file might not
exist, or we might not have permission to access the file. The `File::open`
function needs to have a way to tell us whether it succeeded or failed and at
the same time give us either the file handle or error information. This
information is exactly what the `Result` enum conveys.
-->

`File::open`の戻り値の型は、`Result<T, E>`です。ジェネリック引数の`T`は、
`File::open`の実装によって成功値の型`std::fs::File`で埋められていて、これはファイルハンドルです。
エラー値で使用されている`E`の型は、`std::io::Error`です。
この戻り値型は、`File::open`の呼び出しが成功し、読み込みと書き込みを行えるファイルハンドルを返す可能性があることを意味します。
また、関数呼び出しは失敗もする可能性があります: 例えば、ファイルが存在しない可能性、ファイルへのアクセス権限がない可能性です。
`File::open`には成功したか失敗したかを知らせる方法とファイルハンドルまたは、エラー情報を与える方法が必要なのです。
この情報こそが`Result` enumが伝達するものなのです。

<!--
In the case where `File::open` succeeds, the value in the variable
`greeting_file_result` will be an instance of `Ok` that contains a file handle.
In the case where it fails, the value in `greeting_file_result` will be an
instance of `Err` that contains more information about the kind of error that
happened.
-->

`File::open`が成功した場合、変数`greeting_file_result`の値はファイルハンドルを含む`Ok`インスタンスになります。
失敗した場合には、発生したエラーの種類に関する情報をより多く含む`Err`インスタンスが`greeting_file_result`の値になります。

<!--
We need to add to the code in Listing 9-3 to take different actions depending
on the value `File::open` returns. Listing 9-4 shows one way to handle the
`Result` using a basic tool, the `match` expression that we discussed in
Chapter 6.
-->

リスト9-3のコードに追記をして`File::open`が返す値に応じて異なる動作をする必要があります。
リスト9-4に基礎的な道具を使って`Result`を扱う方法を一つ示しています。第6章で議論した`match`式です。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

<!--
<span class="caption">Listing 9-4: Using a `match` expression to handle the
`Result` variants that might be returned</span>
-->

<span class="caption">リスト9-4: `match`式を使用して返却される可能性のある`Result`列挙子を処理する</span>

<!--
Note that, like the `Option` enum, the `Result` enum and its variants have been
brought into scope by the prelude, so we don’t need to specify `Result::`
before the `Ok` and `Err` variants in the `match` arms.
-->

`Option` enumのように、`Result` enumとその列挙子は、preludeでスコープ内に持ち込まれているので、
`match`アーム内で`Ok`と`Err`列挙子の前に`Result::`を指定する必要がないことに注目してください。

<!--
When the result is `Ok`, this code will return the inner `file` value out of
the `Ok` variant, and we then assign that file handle value to the variable
`greeting_file`. After the `match`, we can use the file handle for reading or
writing.
-->

このコードは、結果が`Ok`の場合は、`Ok`列挙子から中身の`file`値を返し、
それからそのファイルハンドル値を変数`greeting_file`に代入しています。
`match`の後には、ファイルハンドルを使用して読み込んだり書き込むことができるわけです。

<!--
The other arm of the `match` handles the case where we get an `Err` value from
`File::open`. In this example, we’ve chosen to call the `panic!` macro. If
there’s no file named *hello.txt* in our current directory and we run this
code, we’ll see the following output from the `panic!` macro:
-->

`match`のもう一つのアームは、`File::open`から`Err`値が得られたケースを処理しています。
この例では、`panic!`マクロを呼び出すことを選択しています。カレントディレクトリに*hello.txt*というファイルがなく、
このコードを走らせたら、`panic!`マクロからの以下のような出力を目の当たりにするでしょう:

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

<!--
As usual, this output tells us exactly what has gone wrong.
-->

通常通り、この出力は、一体何がおかしくなったのかを物語っています。

<!--
### Matching on Different Errors
-->

### 色々なエラーにマッチする

<!--
The code in Listing 9-4 will `panic!` no matter why `File::open` failed.
However, we want to take different actions for different failure reasons: if
`File::open` failed because the file doesn’t exist, we want to create the file
and return the handle to the new file. If `File::open` failed for any other
reason—for example, because we didn’t have permission to open the file—we still
want the code to `panic!` in the same way as it did in Listing 9-4. For this we
add an inner `match` expression, shown in Listing 9-5.
-->

リスト9-4のコードは、`File::open`が失敗した理由にかかわらず`panic!`します。
ですが、失敗理由によって動作を変えたいとしましょう: ファイルが存在しないために`File::open`が失敗したら、
ファイルを作成し、その新しいファイルへのハンドルを返したいです。他の理由(例えばファイルを開く権限がなかったなど)で、
`File::open`が失敗したら、リスト9-4のようにコードには`panic!`してほしいのです。
このために、リスト9-5で示すように内側の`match`式を追加します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

<!--
<span class="caption">Listing 9-5: Handling different kinds of errors in
different ways</span>
-->

<span class="caption">リスト9-5: 色々な種類のエラーを異なる方法で扱う</span>

<!--
The type of the value that `File::open` returns inside the `Err` variant is
`io::Error`, which is a struct provided by the standard library. This struct
has a method `kind` that we can call to get an `io::ErrorKind` value. The enum
`io::ErrorKind` is provided by the standard library and has variants
representing the different kinds of errors that might result from an `io`
operation. The variant we want to use is `ErrorKind::NotFound`, which indicates
the file we’re trying to open doesn’t exist yet. So we match on
`greeting_file_result`, but we also have an inner match on `error.kind()`.
-->

`File::open`が`Err`列挙子に含めて返す値の型は、`io::Error`であり、これは標準ライブラリで提供されている構造体です。
この構造体には、呼び出すと`io::ErrorKind`値が得られる`kind`メソッドがあります。`io::ErrorKind`というenumは、
標準ライブラリで提供されていて、`io`処理の結果発生する可能性のある色々な種類のエラーを表す列挙子があります。
使用したい列挙子は、`ErrorKind::NotFound`で、これは開こうとしているファイルがまだ存在しないことを示唆します。
そこで、`greeting_file_result`に対してマッチし、さらに`error.kind()`に対する内側のマッチも持たせています。

<!--
The condition we want to check in the inner match is whether the value returned
by `error.kind()` is the `NotFound` variant of the `ErrorKind` enum. If it is,
we try to create the file with `File::create`. However, because `File::create`
could also fail, we need a second arm in the inner `match` expression. When the
file can’t be created, a different error message is printed. The second arm of
the outer `match` stays the same, so the program panics on any error besides
the missing file error.
-->

内側のマッチで精査したい条件は、`error.kind()`により返る値が、`ErrorKind` enumの`NotFound`列挙子であるかということです。
もしそうなら、`File::create`でファイル作成を試みます。ところが、`File::create`も失敗する可能性があるので、
内側の`match`式の2番目のアームが必要なのです。ファイルを作成できない場合、異なるエラーメッセージが出力されます。
外側の`match`の2番目のアームは同じままなので、ファイルが存在しないエラー以外ならプログラムはパニックします。

<!--
> ### Alternatives to Using `match` with `Result<T, E>`
>
> That’s a lot of `match`! The `match` expression is very useful but also very
> much a primitive. In Chapter 13, you’ll learn about closures, which are used
> with many of the methods defined on `Result<T, E>`. These methods can be more
> concise than using `match` when handling `Result<T, E>` values in your code.
>
> For example, here’s another way to write the same logic as shown in Listing
> 9-5, this time using closures and the `unwrap_or_else` method:
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 --
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Problem creating the file: {:?}", error);
>             })
>         } else {
>             panic!("Problem opening the file: {:?}", error);
>         }
>     });
> }
> ```
>
> Although this code has the same behavior as Listing 9-5, it doesn’t contain
> any `match` expressions and is cleaner to read. Come back to this example
> after you’ve read Chapter 13, and look up the `unwrap_or_else` method in the
> standard library documentation. Many more of these methods can clean up huge
> nested `match` expressions when you’re dealing with errors.
-->

> ### `Result<T, E>`に対する`match`の使用の代わりとなる方法
>
> `match`がたくさん出てきましたね！`match`式は非常に有用ですが、非常に原始的でもあります。
> 第13章でクロージャについて学習しますが、これは`Result<T, E>`上に定義されているメソッドの多くで使用することができます。
> コード内で`Result<T, E>`値を扱うときには、こうしたメソッドを使ったほうがより簡潔になります。
>
> 例えば、以下はリスト9-5に示したものと同じロジックを書く例ですが、
> 今度はクロージャと`unwrap_or_else`メソッドを使用しています:
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Problem creating the file: {:?}", error);
>             })
>         } else {
>             panic!("Problem opening the file: {:?}", error);
>         }
>     });
> }
> ```
>
> このコードはリスト9-5と同じ動作をしますが、`match`式をまったく含んでおらず、より読みやすいです。
> 第13章を読み終えたら、この例に戻ってきて、標準ライブラリドキュメント内で`unwrap_or_else`を探してみてください。
> エラーに対処するときには、これらのメソッドを多用することで、巨大なネストされた`match`式を整理することができます。

<!--
### Shortcuts for Panic on Error: `unwrap` and `expect`
-->

### エラー時にパニックするショートカット: `unwrap`と`expect`

<!--
Using `match` works well enough, but it can be a bit verbose and doesn’t always
communicate intent well. The `Result<T, E>` type has many helper methods
defined on it to do various, more specific tasks. The `unwrap` method is a
shortcut method implemented just like the `match` expression we wrote in
Listing 9-4. If the `Result` value is the `Ok` variant, `unwrap` will return
the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will
call the `panic!` macro for us. Here is an example of `unwrap` in action:
-->

`match`の使用は、十分に仕事をしてくれますが、いささか冗長になり得る上、必ずしも意図をよく伝えるとは限りません。
`Result<T, E>`型には、様々な特定の作業をするヘルパーメソッドが多く定義されています。
`unwrap`メソッドは、リスト9-4で書いた`match`式と同じように実装された短絡メソッドです。
`Result`値が`Ok`列挙子なら、`unwrap`は`Ok`の中身を返します。`Result`が`Err`列挙子なら、
`unwrap`は`panic!`マクロを呼んでくれます。こちらが実際に動作している`unwrap`の例です:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

<!--
If we run this code without a *hello.txt* file, we’ll see an error message from
the `panic!` call that the `unwrap` method makes:
-->

このコードを*hello.txt*ファイルなしで走らせたら、`unwrap`メソッドが行う`panic!`呼び出しからのエラーメッセージを目の当たりにするでしょう:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-04-unwrap
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at src/main.rs:4:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
('main'スレッドは、src/main.rs:4:49でパニックしました:
`Err`値に対して`Result::unwrap()`が呼び出されました: Os { code: 2, kind: NotFound, message: "そのようなファイルやディレクトリはありません" })
```

<!--
Similarly, the `expect` method lets us also choose the `panic!` error message.
Using `expect` instead of `unwrap` and providing good error messages can convey
your intent and make tracking down the source of a panic easier. The syntax of
`expect` looks like this:
-->

同様に、`expect`メソッドは、`panic!`のエラーメッセージも選択させてくれます。
`unwrap`の代わりに`expect`を使用して、いいエラーメッセージを提供すると、意図を伝え、
パニックの原因をたどりやすくしてくれます。`expect`の表記はこんな感じです:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

<!--
We use `expect` in the same way as `unwrap`: to return the file handle or call
the `panic!` macro. The error message used by `expect` in its call to `panic!`
will be the parameter that we pass to `expect`, rather than the default
`panic!` message that `unwrap` uses. Here’s what it looks like:
-->

`expect`を`unwrap`と同じように使用してます: ファイルハンドルを返したり、`panic!`マクロを呼び出しています。
`expect`が`panic!`呼び出しで使用するエラーメッセージは、`unwrap`が使用するデフォルトの`panic!`メッセージではなく、
`expect`に渡した引数になります。以下のようになります:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-05-expect
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at src/main.rs:5:10:
hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
('main'スレッドは、src/main.rs:5:10でパニックしました:
hello.txtがこのプロジェクトに含まれているべきです: Os { code: 2, kind: NotFound, message: "そのようなファイルやディレクトリはありません" })
```

<!--
In production-quality code, most Rustaceans choose `expect` rather than
`unwrap` and give more context about why the operation is expected to always
succeed. That way, if your assumptions are ever proven wrong, you have more
information to use in debugging.
-->

製品レベルの品質のコードでは、多くのRustaceanは`unwrap`よりむしろ`expect`を選択し、
なぜその操作が常に成功すると想定されるのかについてより多くの文脈情報を提供します。
そうすることで、万一あなたの仮定が誤っていたと判明した場合には、
デバッグ時に利用するためのより多くの情報が得られます。

<!--
### Propagating Errors
-->

### エラーを委譲する

<!--
When a function’s implementation calls something that might fail, instead of
handling the error within the function itself, you can return the error to the
calling code so that it can decide what to do. This is known as *propagating*
the error and gives more control to the calling code, where there might be more
information or logic that dictates how the error should be handled than what
you have available in the context of your code.
-->

関数の実装が失敗する可能性のある何かを呼び出す際、その関数自体の中でエラーを処理する代わりに、
呼び出し元がどうするかを決められるようにエラーを返すことができます。これはエラーの*委譲*として認知され、
自分のコードの文脈で利用可能なものよりも、
エラーの処理法を規定する情報やロジックがより多くある呼び出し元のコードに制御を明け渡します。

<!--
For example, Listing 9-6 shows a function that reads a username from a file. If
the file doesn’t exist or can’t be read, this function will return those errors
to the code that called the function.
-->

例えば、リスト9-6の関数は、ファイルからユーザ名を読み取ります。ファイルが存在しなかったり、読み込みできなければ、
この関数はそのようなエラーを呼び出し元のコードに返します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

<!--
<span class="caption">Listing 9-6: A function that returns errors to the
calling code using `match`</span>
-->

<span class="caption">リスト9-6: `match`でエラーを呼び出し元のコードに返す関数</span>

<!--
This function can be written in a much shorter way, but we’re going to start by
doing a lot of it manually in order to explore error handling; at the end,
we’ll show the shorter way. Let’s look at the return type of the function
first: `Result<String, io::Error>`. This means the function is returning a
value of the type `Result<T, E>` where the generic parameter `T` has been
filled in with the concrete type `String`, and the generic type `E` has been
filled in with the concrete type `io::Error`.
-->

この関数はもっと短く書くことができますが、エラー処理について詳しく見るために、
まずはエラー処理をたくさん手動で書くことから始めましょう; より短い方法は最後に示します。
まずは、関数の戻り値型に注目しましょう: `Result<String, io::Error>`です。つまり、この関数は、
`Result<T, E>`型の値を返しているということです。ここでジェネリック引数の`T`は、具体型`String`で埋められ、
ジェネリック引数の`E`は具体型`io::Error`で埋められています。

<!--
If this function succeeds without any problems, the code that calls this
function will receive an `Ok` value that holds a `String`—the username that
this function read from the file. If this function encounters any problems, the
calling code will receive an `Err` value that holds an instance of `io::Error`
that contains more information about what the problems were. We chose
`io::Error` as the return type of this function because that happens to be the
type of the error value returned from both of the operations we’re calling in
this function’s body that might fail: the `File::open` function and the
`read_to_string` method.
-->

この関数が何の問題もなく成功すれば、
この関数を呼び出したコードは、`String`(関数がファイルから読み取ったユーザ名)を保持する`Ok`値を受け取ります。
この関数が何か問題に行き当たったら、呼び出し元のコードは`io::Error`のインスタンスを保持する`Err`値を受け取り、
この`io::Error`は問題の内容に関する情報をより多く含んでいます。関数の戻り値の型に`io::Error`を選んだのは、
この関数本体で呼び出している失敗する可能性のある処理が両方とも偶然この型をエラー値として返すからです:
`File::open`関数と`read_to_string`メソッドです。

<!--
The body of the function starts by calling the `File::open` function. Then we
handle the `Result` value with a `match` similar to the `match` in Listing 9-4.
If `File::open` succeeds, the file handle in the pattern variable `file`
becomes the value in the mutable variable `username_file` and the function
continues. In the `Err` case, instead of calling `panic!`, we use the `return`
keyword to return early out of the function entirely and pass the error value
from `File::open`, now in the pattern variable `e`, back to the calling code as
this function’s error value.
-->

関数の本体は、`File::open`関数を呼び出すところから始まります。
そして、リスト9-4の`match`に似た`match`で`Result`値を扱います。
`File::open`が成功すれば、パターン変数`file`にあるファイルハンドルは可変変数`username_file`内の値となり、
関数は継続します。
`Err`ケースでは`panic!`を呼び出す代わりに、関数から完全に早期リターンしてこの関数のエラー値として、
`File::open`から得たエラー値、これはパターン変数`e`内にありますが、これを呼び出し元に渡し戻すために`return`キーワードを使用します。

<!--
So if we have a file handle in `username_file`, the function then creates a new
`String` in variable `username` and calls the `read_to_string` method on
the file handle in `username_file` to read the contents of the file into
`username`. The `read_to_string` method also returns a `Result` because it
might fail, even though `File::open` succeeded. So we need another `match` to
handle that `Result`: if `read_to_string` succeeds, then our function has
succeeded, and we return the username from the file that’s now in `username`
wrapped in an `Ok`. If `read_to_string` fails, we return the error value in the
same way that we returned the error value in the `match` that handled the
return value of `File::open`. However, we don’t need to explicitly say
`return`, because this is the last expression in the function.
-->

`username_file`にファイルハンドルが得られたら、関数は次に変数`username`に新規`String`を生成し、
`username_file`のファイルハンドルに対して`read_to_string`を呼び出して、ファイルの中身を`username`に読み出します。
`File::open`が成功しても、失敗する可能性があるので、`read_to_string`メソッドも、
`Result`を返却します。その`Result`を処理するために別の`match`が必要になります: `read_to_string`が成功したら、
関数は成功し、今は`Ok`に包まれた`username`に入っているファイルのユーザ名を返却します。`read_to_string`が失敗したら、
`File::open`の戻り値を扱った`match`でエラー値を返したように、エラー値を返します。
しかし、明示的に`return`を述べる必要はありません。これが関数の最後の式だからです。

<!--
The code that calls this code will then handle getting either an `Ok` value
that contains a username or an `Err` value that contains an `io::Error`. It’s
up to the calling code to decide what to do with those values. If the calling
code gets an `Err` value, it could call `panic!` and crash the program, use a
default username, or look up the username from somewhere other than a file, for
example. We don’t have enough information on what the calling code is actually
trying to do, so we propagate all the success or error information upward for
it to handle appropriately.
-->

そうしたら、呼び出し元のコードは、ユーザ名を含む`Ok`値か、`io::Error`を含む`Err`値を得て扱います。
それらの値をどうするかを決めるのは、呼び出し元のコードに委ねられます。呼び出しコードが`Err`値を得たら、
例えば、`panic!`を呼び出してプログラムをクラッシュさせたり、デフォルトのユーザ名を使ったり、
ファイル以外の場所からユーザ名を検索したりできるでしょう。呼び出し元のコードが実際に何をしようとするかについて、
十分な情報がないので、成功や失敗情報を全て委譲して適切に扱えるようにするのです。

<!--
This pattern of propagating errors is so common in Rust that Rust provides the
question mark operator `?` to make this easier.
-->

Rustにおいて、この種のエラー委譲は非常に一般的なので、Rustにはこれをしやすくする`?`演算子が用意されています。

<!--
#### A Shortcut for Propagating Errors: the `?` Operator
-->

#### エラー委譲のショートカット: `?`演算子

<!--
Listing 9-7 shows an implementation of `read_username_from_file` that has the
same functionality as in Listing 9-6, but this implementation uses the
`?` operator.
-->

リスト9-7はリスト9-6にあったのと同じ機能を持つ`read_username_from_file`の実装ですが、
こちらは`?`演算子を使用しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

<!--
<span class="caption">Listing 9-7: A function that returns errors to the
calling code using the `?` operator</span>
-->

<span class="caption">リスト9-7: `?`演算子でエラーを呼び出し元に返す関数</span>

<!--
The `?` placed after a `Result` value is defined to work in almost the same way
as the `match` expressions we defined to handle the `Result` values in Listing
9-6. If the value of the `Result` is an `Ok`, the value inside the `Ok` will
get returned from this expression, and the program will continue. If the value
is an `Err`, the `Err` will be returned from the whole function as if we had
used the `return` keyword so the error value gets propagated to the calling
code.
-->

`Result`値の直後に置かれた`?`は、リスト9-6で`Result`値を処理するために定義した`match`式とほぼ同じように動作します。
`Result`の値が`Ok`なら、`Ok`の中身がこの式から返ってきて、プログラムは継続します。値が`Err`なら、
`return`キーワードを使ったかのように関数全体から`Err`が返ってくるので、
エラー値は呼び出し元のコードに委譲されます。

<!--
There is a difference between what the `match` expression from Listing 9-6 does
and what the `?` operator does: error values that have the `?` operator called
on them go through the `from` function, defined in the `From` trait in the
standard library, which is used to convert values from one type into another.
When the `?` operator calls the `from` function, the error type received is
converted into the error type defined in the return type of the current
function. This is useful when a function returns one error type to represent
all the ways a function might fail, even if parts might fail for many different
reasons.
-->

リスト9-6の`match`式がやっていることと`?`演算子がやっていることには、違いがあります:
`?`演算子が呼ばれる対象となったエラー値は、
標準ライブラリの`From`トレイトで定義され、値を別の型に変換する`from`関数を通ることです。
`?`演算子が`from`関数を呼び出すと、受け取ったエラー型が現在の関数の戻り値型で定義されているエラー型に変換されます。これは、
個々がいろんな理由で失敗する可能性があるのにも関わらず、関数が失敗する可能性を全て一つのエラー型で表現して返す時に有用です。

<!--
For example, we could change the `read_username_from_file` function in Listing
9-7 to return a custom error type named `OurError` that we define. If we also
define `impl From<io::Error> for OurError` to construct an instance of
`OurError` from an `io::Error`, then the `?` operator calls in the body of
`read_username_from_file` will call `from` and convert the error types without
needing to add any more code to the function.
-->

例えば、リスト9-7の`read_username_from_file`関数を、`OurError`という名前の自分で定義したカスタムエラー型を返すように変更することができます。
さらに、`io::Error`から`OutError`のインスタンスを構築するための`impl From<io::Error> for OurError`を定義すれば、
`read_username_from_file`の本体の中の`?`演算子呼び出しは`from`を呼び出してくれるので、
関数にコードを追加する必要なくエラー型を変換してくれます。

<!--
In the context of Listing 9-7, the `?` at the end of the `File::open` call will
return the value inside an `Ok` to the variable `username_file`. If an error
occurs, the `?` operator will return early out of the whole function and give
any `Err` value to the calling code. The same thing applies to the `?` at the
end of the `read_to_string` call.
-->

リスト9-7の文脈では、`File::open`呼び出し末尾の`?`は`Ok`の中身を変数`username_file`に返します。
エラーが発生したら、`?`演算子により関数全体から早期リターンし、あらゆる`Err`値を呼び出し元に与えます。
同じ法則が`read_to_string`呼び出し末尾の`?`にも適用されます。

<!--
The `?` operator eliminates a lot of boilerplate and makes this function’s
implementation simpler. We could even shorten this code further by chaining
method calls immediately after the `?`, as shown in Listing 9-8.
-->

`?`演算子により定型コードの多くが排除され、この関数の実装を単純にしてくれます。
リスト9-8で示したように、`?`の直後のメソッド呼び出しを連結することでさらにこのコードを短くすることさえもできます。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

<!--
<span class="caption">Listing 9-8: Chaining method calls after the `?`
operator</span>
-->

<span class="caption">リスト9-8: `?`演算子の後のメソッド呼び出しを連結する</span>

<!--
We’ve moved the creation of the new `String` in `username` to the beginning of
the function; that part hasn’t changed. Instead of creating a variable
`username_file`, we’ve chained the call to `read_to_string` directly onto the
result of `File::open("hello.txt")?`. We still have a `?` at the end of the
`read_to_string` call, and we still return an `Ok` value containing `username`
when both `File::open` and `read_to_string` succeed rather than returning
errors. The functionality is again the same as in Listing 9-6 and Listing 9-7;
this is just a different, more ergonomic way to write it.
-->

`username`の新規`String`の生成を関数の冒頭に移動しました; その部分は変化していません。
変数`username_file`を生成する代わりに、
`read_to_string`の呼び出しを直接`File::open("hello.txt")?`の結果に連結させました。
それでも、`read_to_string`呼び出しの末尾には`?`があり、`File::open`と`read_to_string`両方が成功したら、
エラーを返すというよりもそれでも、`username`を含む`Ok`値を返します。機能もまたリスト9-6及び、9-7と同じです;
ただ単に異なるバージョンのよりエルゴノミックな書き方なのです。

<!--
Listing 9-9 shows a way to make this even shorter using `fs::read_to_string`.
-->

これを`fs::read_to_string`を使用してさらに短くする方法をリスト9-9に示します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

<!--
<span class="caption">Listing 9-9: Using `fs::read_to_string` instead of
opening and then reading the file</span>
-->

<span class="caption">リスト9-9: ファイルを開いて読む代わりに`fs::read_to_string`を使用する</span>

<!--
Reading a file into a string is a fairly common operation, so the standard
library provides the convenient `fs::read_to_string` function that opens the
file, creates a new `String`, reads the contents of the file, puts the contents
into that `String`, and returns it. Of course, using `fs::read_to_string`
doesn’t give us the opportunity to explain all the error handling, so we did it
the longer way first.
-->

ファイルを文字列に読み込むことは非常によくある操作なので、標準ライブラリは、
ファイルを開き、新しい`String`を作成し、ファイルの内容を読み込み、内容をその`String`に格納し、それを返す、
便利な`fs::read_to_string`関数を提供しています。そうそう、`fs::read_to_string`を使用してしまうと、
エラー処理のすべてについて説明する機会がなくなってしまうので、長ったらしい方法を先に使ったのでした。

<!--
#### Where The `?` Operator Can Be Used
-->

#### `?`演算子が使用できる場所

<!--
The `?` operator can only be used in functions whose return type is compatible
with the value the `?` is used on. This is because the `?` operator is defined
to perform an early return of a value out of the function, in the same manner
as the `match` expression we defined in Listing 9-6. In Listing 9-6, the
`match` was using a `Result` value, and the early return arm returned an
`Err(e)` value. The return type of the function has to be a `Result` so that
it’s compatible with this `return`.
-->

`?`演算子は、`?`を使用する対象の値と戻り値の型に互換性がある関数でしか使用できません。
というのも`?`演算子は、リスト9-6で定義した`match`式と同様に関数から値の早期リターンを実行するように定義されているからです。
リスト9-6では、`match`は`Result`値を使用して、早期リターンする側のアームは`Err(e)`値を返していました。
関数の戻り値型はこの`return`と互換性を保てるように、`Result`でなければならないのです。

<!--
In Listing 9-10, let’s look at the error we’ll get if we use the `?` operator
in a `main` function with a return type incompatible with the type of the value
we use `?` on:
-->

リスト9-10で、`?`を使用する対象の値の型と戻り値の型に互換性がない`main`関数で、
`?`演算子を使用した場合に得られるエラーを見てみましょう:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

<!--
<span class="caption">Listing 9-10: Attempting to use the `?` in the `main`
function that returns `()` won’t compile</span>
-->

<span class="caption">リスト9-10: `()`を返す`main`関数内で`?`を使用しようとするとコンパイルが通らない</span>

<!--
This code opens a file, which might fail. The `?` operator follows the `Result`
value returned by `File::open`, but this `main` function has the return type of
`()`, not `Result`. When we compile this code, we get the following error
message:
-->

このコードはファイルを開きますが、これは失敗する可能性があります。
`?`演算子は`File::open`によって返される`Result`値を確認して対処しますが、この`main`関数は`Result`ではなく`()`の戻り値型を持っています。 
このコードをコンパイルすると、以下のようなエラーメッセージが得られます:

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

<!--
This error points out that we’re only allowed to use the `?` operator in a
function that returns `Result`, `Option`, or another type that implements
`FromResidual`.
-->

このエラーは、`?`演算子は`Result`、`Option`、またはその他`FromResidual`を実装する型を返す関数でしか使用が許可されないと指摘しています。

<!--
To fix the error, you have two choices. One choice is to change the return type
of your function to be compatible with the value you’re using the `?` operator
on as long as you have no restrictions preventing that. The other technique is
to use a `match` or one of the `Result<T, E>` methods to handle the `Result<T,
E>` in whatever way is appropriate.
-->

このエラーを修正する方法としては、二つの選択肢があります。
一つは、それを阻害する制限がない場合は、関数の戻り値型を、`?`演算子を使用する対象の値と互換性があるような型に変更することです。
もう一つの手法は、`match`または`Result<T, E>`のメソッドのいずれかを使用して、何らかの適切な方法で`Result<T, E>`を処理することです。

<!--
The error message also mentioned that `?` can be used with `Option<T>` values
as well. As with using `?` on `Result`, you can only use `?` on `Option` in a
function that returns an `Option`. The behavior of the `?` operator when called
on an `Option<T>` is similar to its behavior when called on a `Result<T, E>`:
if the value is `None`, the `None` will be returned early from the function at
that point. If the value is `Some`, the value inside the `Some` is the
resulting value of the expression and the function continues. Listing 9-11 has
an example of a function that finds the last character of the first line in the
given text:
-->

エラーメッセージは、`?`は`Option<T>`値にも使用できることに言及しています。
`Result`に`?`を使用する場合と同様に、`Option`を返す関数の中でのみ、`Option`に`?`を使用することができます。
`Option<T>`に対して呼ばれた場合の`?`演算子の挙動は、`Result<T, E>`に対して呼ばれた場合の挙動と似ています:
もしその値が`None`ならば、その時点で`None`が関数から早期リターンされます。
もしその値が`Some`ならば、`Some`の内側の値がその式の結果の値となり、関数は継続します。
リスト9-11は、与えられたテキストの最初の行の最後の文字を探索する関数の例です:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

<!--
<span class="caption">Listing 9-11: Using the `?` operator on an `Option<T>`
value</span>
-->

<span class="caption">リスト9-11: `Option<T>`値に対する`?`演算子の使用</span>

<!--
This function returns `Option<char>` because it’s possible that there is a
character there, but it’s also possible that there isn’t. This code takes the
`text` string slice argument and calls the `lines` method on it, which returns
an iterator over the lines in the string. Because this function wants to
examine the first line, it calls `next` on the iterator to get the first value
from the iterator. If `text` is the empty string, this call to `next` will
return `None`, in which case we use `?` to stop and return `None` from
`last_char_of_first_line`. If `text` is not the empty string, `next` will
return a `Some` value containing a string slice of the first line in `text`.
-->

この関数は、文字はあるかもしれないし、ないかもしれないので、`Option<char>`を返します。
このコードは`text`文字列スライス引数を受け取り、それに対して`lines`メソッドを呼び出します。
これは文字列内の行を走査するイテレータを返します。この関数では最初の行を確認したいので、
イテレータに対して`next`を呼び出してイテレータから最初の値を取得します。
`text`が空文字列の場合は、この`next`呼び出しは`None`を返すでしょう。
その場合は`?`を使用して停止し、`last_char_of_first_line`から`None`を返します。
`text`が空文字列でない場合は、`next`は`text`内の最初の行の文字列スライスを含む`Some`値を返すでしょう。

<!--
The `?` extracts the string slice, and we can call `chars` on that string slice
to get an iterator of its characters. We’re interested in the last character in
this first line, so we call `last` to return the last item in the iterator.
This is an `Option` because it’s possible that the first line is the empty
string, for example if `text` starts with a blank line but has characters on
other lines, as in `"\nhi"`. However, if there is a last character on the first
line, it will be returned in the `Some` variant. The `?` operator in the middle
gives us a concise way to express this logic, allowing us to implement the
function in one line. If we couldn’t use the `?` operator on `Option`, we’d
have to implement this logic using more method calls or a `match` expression.
-->

`?`が文字列スライスを抽出するので、その文字列スライスに対して`chars`を呼び出して、含まれる文字からなるイテレータを得ることができます。
最初の行の最後の文字に関心があるので、このイテレータの最後の要素を返すために`last`を呼び出します。
例えば`text`が空行で始まるが他の行には文字が含まれる`"\nhi"`のような文字列の場合、
最初の行が空文字列である可能性があるので、この戻り値は`Option`になっています。
しかしながら、最初の行の最後の文字がある場合は、それを`Some`列挙子に入れて返します。
中間の`?`演算子はこのロジックを簡潔に表現する方法を提供し、そのためこの関数を一行で実装することができています。
もし`?`演算子が`Option`に使用できなかったなら、より多くのメソッド呼び出しや`match`式を使用してこのロジックを実装する必要があったでしょう。

<!--
Note that you can use the `?` operator on a `Result` in a function that returns
`Result`, and you can use the `?` operator on an `Option` in a function that
returns `Option`, but you can’t mix and match. The `?` operator won’t
automatically convert a `Result` to an `Option` or vice versa; in those cases,
you can use methods like the `ok` method on `Result` or the `ok_or` method on
`Option` to do the conversion explicitly.
-->

`Result`を返す関数内では`Result`に対して`?`演算子を使用でき、`Option`を返す関数内では`Option`に対して`?`演算子を使用できますが、
混ぜて組み合わせることはできないことに注意してください。
`?`演算子は、`Result`を`Option`に、またはその逆に、自動的に変換することはしません;
そのような場合には明示的に変換を行うために、`Result`の`ok`メソッドや、`Option`の`ok_or`メソッドのようなメソッドを使用することができます。

<!--
So far, all the `main` functions we’ve used return `()`. The `main` function is
special because it’s the entry and exit point of executable programs, and there
are restrictions on what its return type can be for the programs to behave as
expected.
-->

今までに使ってきた`main`関数はすべて`()`を返してきました。
`main`関数は、実行可能なプログラムの開始および終了点であることから特別扱いされており、
プログラムが期待通りに振る舞うために、その戻り値型として指定できる型に制限があります。

<!--
Luckily, `main` can also return a `Result<(), E>`. Listing 9-12 has the
code from Listing 9-10 but we’ve changed the return type of `main` to be
`Result<(), Box<dyn Error>>` and added a return value `Ok(())` to the end. This
code will now compile:
-->

幸運なことに、`main`は`Result<(), E>`を返すこともできます。
リスト9-12はリスト9-10のコードを含んでいますが、`main`の戻り値型を`Result<(), Box<dyn Error>>`に変更し、
最後に戻り値`Ok(())`を追加しています。
これでこのコードはコンパイルできるでしょう:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

<!--
<span class="caption">Listing 9-12: Changing `main` to return `Result<(), E>`
allows the use of the `?` operator on `Result` values</span>
-->

<span class="caption">リスト9-12: `Result<(), E>`を返すように`main`を変更することで、`Result`値に対する`?`演算子が使用可能になる</span>

<!--
The `Box<dyn Error>` type is a *trait object*, which we’ll talk about in the
[“Using Trait Objects that Allow for Values of Different
Types”][trait-objects] section in Chapter 17. For now, you can
read `Box<dyn Error>` to mean “any kind of error.” Using `?` on a `Result`
value in a `main` function with the error type `Box<dyn Error>` is allowed,
because it allows any `Err` value to be returned early. Even though the body of
this `main` function will only ever return errors of type `std::io::Error`, by
specifying `Box<dyn Error>`, this signature will continue to be correct even if
more code that returns other errors is added to the body of `main`.
-->

`Box<dyn Error>`型は*トレイトオブジェクト*ですが、これについては第17章の[「トレイトオブジェクトで異なる型の値を許容する」][trait-objects]節で話します。
今のところは、`Box<dyn Error>`は「任意の種類のエラー」を意味するものと理解してください。
エラー型`Box<dyn Error>`を持つ`main`関数の中では、任意の`Err`値を早期リターンすることができるので、`Result`値に対する`?`の使用が許可されます。
この`main`関数の本体は型`std::io::Error`のエラーしか返しませんが、それでも`Box<dyn Error>`を指定することにより、
他のエラーを返すコードが`main`の本体に追加された場合であっても、このシグネチャは正しいままであり続けるでしょう。

<!--
When a `main` function returns a `Result<(), E>`, the executable will
exit with a value of `0` if `main` returns `Ok(())` and will exit with a
nonzero value if `main` returns an `Err` value. Executables written in C return
integers when they exit: programs that exit successfully return the integer
`0`, and programs that error return some integer other than `0`. Rust also
returns integers from executables to be compatible with this convention.
-->

`main`関数が`Result<(), E>`を返す場合、その実行可能ファイルは、
`main`が`Ok(())`を返す場合には値`0`で終了し、`main`が`Err`値を返す場合は非ゼロ値で終了するでしょう。
Cで書かれた実行可能ファイルは終了時に整数を返します:
正常終了するプログラムは整数`0`を返し、エラーが発生したプログラムは`0`以外の整数を返します。
Rustもこの慣例に従い、実行可能ファイルから整数を返します。

<!--
The `main` function may return any types that implement [the
`std::process::Termination` trait][termination], which contains
a function `report` that returns an `ExitCode`. Consult the standard library
documentation for more information on implementing the `Termination` trait for
your own types.
-->

`main`関数は[`std::process::Termination`トレイト][termination]を実装する任意の型を返すことができます。
このトレイトには`ExitCode`を返す`report`関数が含まれます。
自身の型に対する`Termination`トレイトの実装に関するさらなる情報については、
標準ライブラリドキュメントを確認してください。

<!--
Now that we’ve discussed the details of calling `panic!` or returning `Result`,
let’s return to the topic of how to decide which is appropriate to use in which
cases.
-->

さて、`panic!`呼び出しや`Result`を返す詳細について議論し終えたので、
どんな場合にどちらを使うのが適切か決める方法についての話に戻りましょう。

<!--
[handle_failure]: ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-result
[trait-objects]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[termination]: ../std/process/trait.Termination.html
-->

[handle_failure]: ch02-00-guessing-game-tutorial.html#resultで失敗の可能性を扱う
[trait-objects]: ch17-02-trait-objects.html#トレイトオブジェクトで異なる型の値を許容する
[termination]: https://doc.rust-lang.org/std/process/trait.Termination.html
