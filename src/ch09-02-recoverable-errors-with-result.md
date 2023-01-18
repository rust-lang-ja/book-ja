<!--
## Recoverable Errors with `Result`
-->

## `Result`で回復可能なエラー

<!--
Most errors aren’t serious enough to require the program to stop entirely.
Sometimes, when a function fails, it’s for a reason that we can easily
interpret and respond to. For example, if you try to open a file and that
operation fails because the file doesn’t exist, we might want to create the
file instead of terminating the process.
-->

多くのエラーは、プログラムを完全にストップさせるほど深刻ではありません。時々、関数が失敗した時に、
容易に解釈し、対応できる理由によることがあります。例えば、ファイルを開こうとして、
ファイルが存在しないために処理が失敗したら、プロセスを停止するのではなく、ファイルを作成したいことがあります。

<!--
Recall from “[Handling Potential Failure with the `Result`
Type][handle_failure]” in Chapter 2 that the `Result` enum is
defined as having two variants, `Ok` and `Err`, as follows:
-->

第 2 章の[「`Result`型で失敗する可能性に対処する」][handle_failure]で`Result` enumが以下のように、
`Ok`と`Err`の 2 列挙子からなるよう定義されていることを思い出してください：

[handle_failure]: ch02-00-guessing-game-tutorial.html#result型で失敗の可能性を扱う

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
parameters, we can use the `Result` type and the functions that the standard
library has defined on it in many different situations where the successful
value and error value we want to return may differ.
-->

`T`と`E`は、ジェネリックな型引数です：ジェネリクスについて詳しくは、第 10 章で議論します。
たった今知っておく必要があることは、`T`が成功した時に`Ok`列挙子に含まれて返される値の型を表すことと、
`E`が失敗した時に`Err`列挙子に含まれて返されるエラーの型を表すことです。`Result`はこのようなジェネリックな型引数を含むので、
標準ライブラリ上に定義されている`Result`型や関数などを、成功した時とエラーの時に返したい値が異なるような様々な場面で使用できるのです。

<!--
Let’s call a function that returns a `Result` value because the function could
fail. In Listing 9-3 we try to open a file.
-->

関数が失敗する可能性があるために`Result`値を返す関数を呼び出しましょう：リスト 9-3 では、
ファイルを開こうとしています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

<!--
<span class="caption">Listing 9-3: Opening a file</span>
-->

<span class="caption">リスト 9-3: ファイルを開く</span>

<!--
How do we know `File::open` returns a `Result`? We could look at the standard
library API documentation, or we could ask the compiler! If we give `f` a type
annotation that we know is *not* the return type of the function and then try
to compile the code, the compiler will tell us that the types don’t match. The
error message will then tell us what the type of `f` *is*. Let’s try it! We
know that the return type of `File::open` isn’t of type `u32`, so let’s change
the `let f` statement to this:
-->

`File::open`が`Result`を返すとどう知るのでしょうか？標準ライブラリの API ドキュメントを参照することもできますし、
コンパイラに尋ねることもできます！`f`に関数の戻り値では*ない*と判明している型注釈を与えて、
コードのコンパイルを試みれば、コンパイラは型が合わないと教えてくれるでしょう。そして、エラーメッセージは、
`f`の*実際の*型を教えてくれるでしょう。試してみましょう！`File::open`の戻り値の型は`u32`ではないと判明しているので、
`let f`文を以下のように変更しましょう：

```rust,ignore
let f: u32 = File::open("hello.txt");
```

<!--
Attempting to compile now gives us the following output:
-->

これでコンパイルしようとすると、以下のような出力が得られます：

```text
error[E0308]: mismatched types
(エラー: 型が合いません)
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
`std::result::Result`
  |
  = note: expected type `u32`
  (注釈：予期した型は`u32`です)
             found type `std::result::Result<std::fs::File, std::io::Error>`
  (実際の型は`std::result::Result<std::fs::File, std::io::Error>`です)
```

<!--
This tells us the return type of the `File::open` function is a `Result<T, E>`.
The generic parameter `T` has been filled in here with the type of the success
value, `std::fs::File`, which is a file handle. The type of `E` used in the
error value is `std::io::Error`.
-->

これにより、`File::open`関数の戻り値の型は、`Result<T, E>`であることがわかります。ジェネリック引数の`T`は、
ここでは成功値の型`std::fs::File`で埋められていて、これはファイルハンドルです。
エラー値で使用されている`E`の型は、`std::io::Error`です。

<!--
This return type means the call to `File::open` might succeed and return a file
handle that we can read from or write to. The function call also might fail:
for example, the file might not exist, or we might not have permission to
access the file. The `File::open` function needs to have a way to tell us
whether it succeeded or failed and at the same time give us either the file
handle or error information. This information is exactly what the `Result` enum
conveys.
-->

この戻り値型は、`File::open`の呼び出しが成功し、読み込みと書き込みを行えるファイルハンドルを返す可能性があることを意味します。
また、関数呼び出しは失敗もする可能性があります：例えば、ファイルが存在しない可能性、ファイルへのアクセス権限がない可能性です。
`File::open`には成功したか失敗したかを知らせる方法とファイルハンドルまたは、エラー情報を与える方法が必要なのです。
この情報こそが`Result` enum が伝達するものなのです。

<!--
In the case where `File::open` succeeds, the value in the variable `f` will be
an instance of `Ok` that contains a file handle. In the case where it fails,
the value in `f` will be an instance of `Err` that contains more information
about the kind of error that happened.
-->

`File::open`が成功した場合、変数`f`の値はファイルハンドルを含む`Ok`インスタンスになります。
失敗した場合には、発生したエラーの種類に関する情報をより多く含む`Err`インスタンスが`f`の値になります。

<!--
We need to add to the code in Listing 9-3 to take different actions depending
on the value `File::open` returns. Listing 9-4 shows one way to handle the
`Result` using a basic tool, the `match` expression that we discussed in
Chapter 6.
-->

リスト 9-3 のコードに追記をして`File::open`が返す値に応じて異なる動作をする必要があります。
リスト 9-4 に基礎的な道具を使って`Result`を扱う方法を一つ示しています。第 6 章で議論した`match`式です。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            // ファイルを開く際に問題がありました
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
```

<!--
<span class="caption">Listing 9-4: Using a `match` expression to handle the
`Result` variants that might be returned</span>
-->

<span class="caption">リスト 9-4: `match`式を使用して返却される可能性のある`Result`列挙子を処理する</span>

<!--
Note that, like the `Option` enum, the `Result` enum and its variants have been
imported in the prelude, so we don’t need to specify `Result::` before the `Ok`
and `Err` variants in the `match` arms.
-->

`Option` enum のように、`Result` enum とその列挙子は、初期化処理でインポートされているので、
`match`アーム内で`Ok`と`Err`列挙子の前に`Result::`を指定する必要がないことに注目してください。

<!--
Here we tell Rust that when the result is `Ok`, return the inner `file` value
out of the `Ok` variant, and we then assign that file handle value to the
variable `f`. After the `match`, we can use the file handle for reading or
writing.
-->

ここでは、結果が`Ok`の時に、`Ok`列挙子から中身の`file`値を返すように指示し、
それからそのファイルハンドル値を変数`f`に代入しています。`match`の後には、
ファイルハンドルを使用して読み込んだり書き込むことができるわけです。

<!--
The other arm of the `match` handles the case where we get an `Err` value from
`File::open`. In this example, we’ve chosen to call the `panic!` macro. If
there’s no file named *hello.txt* in our current directory and we run this
code, we’ll see the following output from the `panic!` macro:
-->

`match`のもう一つのアームは、`File::open`から`Err`値が得られたケースを処理しています。
この例では、`panic!`マクロを呼び出すことを選択しています。カレントディレクトリに*hello.txt*というファイルがなく、
このコードを走らせたら、`panic!`マクロからの以下のような出力を目の当たりにするでしょう：

```text
thread 'main' panicked at 'There was a problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:9:12
('main'スレッドは、src/main.rs:9:12 の「ファイルを開く際に問題がありました：Error{ repr:
Os { code: 2, message: "そのような名前のファイルまたはディレクトリはありません"}}」でパニックしました)
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
The code in Listing 9-4 will `panic!` no matter why `File::open` failed. What
we want to do instead is take different actions for different failure reasons:
if `File::open` failed because the file doesn’t exist, we want to create the
file and return the handle to the new file. If `File::open` failed for any
other reason-for example, because we didn’t have permission to open the file-we
still want the code to `panic!` in the same way as it did in Listing 9-4. Look
at Listing 9-5, which adds another arm to the `match`.
-->

リスト 9-4 のコードは、`File::open`が失敗した理由にかかわらず`panic!`します。代わりにしたいことは、
失敗理由によって動作を変えることです：ファイルが存在しないために`File::open`が失敗したら、
ファイルを作成し、その新しいファイルへのハンドルを返したいです。他の理由 (例えばファイルを開く権限がなかったなど) で、
`File::open`が失敗したら、リスト 9-4 のようにコードには`panic!`してほしいのです。
リスト 9-5 を眺めてください。ここでは`match`に別のアームを追加しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        //ファイルを作成しようとしましたが、問題がありました
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
}
```

<!--
<span class="caption">Listing 9-5: Handling different kinds of errors in
different ways</span>
-->

<span class="caption">リスト 9-5: 色々な種類のエラーを異なる方法で扱う</span>

<!--
The type of the value that `File::open` returns inside the `Err` variant is
`io::Error`, which is a struct provided by the standard library. This struct
has a method `kind` that we can call to get an `io::ErrorKind` value. The enum
`io::ErrorKind` is provided by the standard library and has variants
representing the different kinds of errors that might result from an `io`
operation. The variant we want to use is `ErrorKind::NotFound`, which indicates
the file we’re trying to open doesn’t exist yet.
-->

`File::open`が`Err`列挙子に含めて返す値の型は、`io::Error`であり、これは標準ライブラリで提供されている構造体です。
この構造体には、呼び出すと`io::ErrorKind`値が得られる`kind`メソッドがあります。`io::ErrorKind`という enum は、
標準ライブラリで提供されていて、`io`処理の結果発生する可能性のある色々な種類のエラーを表す列挙子があります。
使用したい列挙子は、`ErrorKind::NotFound`で、これは開こうとしているファイルがまだ存在しないことを示唆します。

<!--
The condition `if error.kind() == ErrorKind::NotFound` is called a *match
guard*: it’s an extra condition on a `match` arm that further refines the arm’s
pattern. This condition must be true for that arm’s code to be run; otherwise,
the pattern matching will move on to consider the next arm in the `match`. The
`ref` in the pattern is needed so `error` is not moved into the guard condition
but is merely referenced by it. The reason you use `ref` to create a reference
in a pattern instead of `&` will be covered in detail in Chapter 18. In short,
in the context of a pattern, `&` matches a reference and gives you its value,
but `ref` matches a value and gives you a reference to it.
-->

`if error.kind() == ErrorKind::Notfound`という条件式は、*マッチガード*と呼ばれます：
アームのパターンをさらに洗練する`match`アーム上のおまけの条件式です。この条件式は、
そのアームのコードが実行されるには真でなければいけないのです; そうでなければ、
パターンマッチングは継続し、`match`の次のアームを考慮します。パターンの`ref`は、
`error`がガード条件式にムーブされないように必要ですが、ただ単にガード式に参照されます。
`ref`を使用して`&`の代わりにパターン内で参照を作っている理由は、第 18 章で詳しく講義します。
手短に言えば、パターンの文脈において、`&`は参照にマッチし、その値を返しますが、
`ref`は値にマッチし、それへの参照を返すということなのです。

<!--
The condition we want to check in the match guard is whether the value returned
by `error.kind()` is the `NotFound` variant of the `ErrorKind` enum. If it is,
we try to create the file with `File::create`. However, because `File::create`
could also fail, we need to add an inner `match` expression as well. When the
file can’t be opened, a different error message will be printed. The last arm
of the outer `match` stays the same so the program panics on any error besides
the missing file error.
-->

マッチガードで精査したい条件は、`error.kind()`により返る値が、`ErrorKind` enum の`NotFound`列挙子であるかということです。
もしそうなら、`File::create`でファイル作成を試みます。ところが、`File::create`も失敗する可能性があるので、
内部にも`match`式を追加する必要があるのです。ファイルが開けないなら、異なるエラーメッセージが出力されるでしょう。
外側の`match`の最後のアームは同じままなので、ファイルが存在しないエラー以外ならプログラムはパニックします。

<!--
### Shortcuts for Panic on Error: `unwrap` and `expect`
-->

### エラー時にパニックするショートカット：`unwrap`と`expect`

<!--
Using `match` works well enough, but it can be a bit verbose and doesn’t always
communicate intent well. The `Result<T, E>` type has many helper methods
defined on it to do various tasks. One of those methods, called `unwrap`, is a
shortcut method that is implemented just like the `match` expression we wrote
in Listing 9-4. If the `Result` value is the `Ok` variant, `unwrap` will return
the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will
call the `panic!` macro for us. Here is an example of `unwrap` in action:
-->

`match`の使用は、十分に仕事をしてくれますが、いささか冗長になり得る上、必ずしも意図をよく伝えるとは限りません。
`Result<T, E>`型には、色々な作業をするヘルパーメソッドが多く定義されています。それらの関数の一つは、
`unwrap`と呼ばれますが、リスト 9-4 で書いた`match`式と同じように実装された短絡メソッドです。
`Result`値が`Ok`列挙子なら、`unwrap`は`Ok`の中身を返します。`Result`が`Err`列挙子なら、
`unwrap`は`panic!`マクロを呼んでくれます。こちらが実際に動作している`unwrap`の例です：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

<!--
If we run this code without a *hello.txt* file, we’ll see an error message from
the `panic!` call that the `unwrap` method makes:
-->

このコードを*hello.txt*ファイルなしで走らせたら、`unwrap`メソッドが行う`panic!`呼び出しからのエラーメッセージを目の当たりにするでしょう：

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
('main'スレッドは、src/libcore/result.rs:906:4 の
「`Err`値に対して`Result::unwrap()`が呼び出されました：Error{
repr: Os { code: 2, message: "そのようなファイルまたはディレクトリはありません" } }」でパニックしました)
```

<!--
Another method, `expect`, which is similar to `unwrap`, lets us also choose the
`panic!` error message. Using `expect` instead of `unwrap` and providing good
error messages can convey your intent and make tracking down the source of a
panic easier. The syntax of `expect` looks like this:
-->

別のメソッド`expect`は、`unwrap`に似ていますが、`panic!`のエラーメッセージも選択させてくれます。
`unwrap`の代わりに`expect`を使用して、いいエラーメッセージを提供すると、意図を伝え、
パニックの原因をたどりやすくしてくれます。`expect`の表記はこんな感じです：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,should_panic
use std::fs::File;

fn main() {
    // hello.txt を開くのに失敗しました
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

<!--
We use `expect` in the same way as `unwrap`: to return the file handle or call
the `panic!` macro. The error message used by `expect` in its call to `panic!`
will be the parameter that we pass to `expect`, rather than the default
`panic!` message that `unwrap` uses. Here’s what it looks like:
-->

`expect`を`unwrap`と同じように使用してます：ファイルハンドルを返したり、`panic!`マクロを呼び出しています。
`expect`が`panic!`呼び出しで使用するエラーメッセージは、`unwrap`が使用するデフォルトの`panic!`メッセージではなく、
`expect`に渡した引数になります。以下のようになります：

```text
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

<!--
Because this error message starts with the text we specified, `Failed to open
hello.txt`, it will be easier to find where in the code this error message is
coming from. If we use `unwrap` in multiple places, it can take more time to
figure out exactly which `unwrap` is causing the panic because all `unwrap`
calls that panic print the same message.
-->

このエラーメッセージは、指定したテキストの`hello.txtを開くのに失敗しました`で始まっているので、
コード内のどこでエラーメッセージが出力されたのかより見つけやすくなるでしょう。複数箇所で`unwrap`を使用していたら、
ズバリどの`unwrap`がパニックを引き起こしているのか理解するのは、より時間がかかる可能性があります。
パニックする`unwrap`呼び出しは全て、同じメッセージを出力するからです。

<!--
### Propagating Errors
-->

### エラーを委譲する

<!--
When you’re writing a function whose implementation calls something that might
fail, instead of handling the error within this function, you can return the
error to the calling code so that it can decide what to do. This is known as
*propagating* the error and gives more control to the calling code, where there
might be more information or logic that dictates how the error should be
handled than what you have available in the context of your code.
-->

失敗する可能性のある何かを呼び出す実装をした関数を書く際、関数内でエラーを処理する代わりに、
呼び出し元がどうするかを決められるようにエラーを返すことができます。これはエラーの*委譲*として認知され、
自分のコードの文脈で利用可能なものよりも、
エラーの処理法を規定する情報やロジックがより多くある呼び出し元のコードに制御を明け渡します。

<!--
For example, Listing 9-6 shows a function that reads a username from a file. If
the file doesn’t exist or can’t be read, this function will return those errors
to the code that called this function.
-->

例えば、リスト 9-6 の関数は、ファイルからユーザ名を読み取ります。ファイルが存在しなかったり、読み込みできなければ、
この関数はそのようなエラーを呼び出し元のコードに返します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

<!--
<span class="caption">Listing 9-6: A function that returns errors to the
calling code using `match`</span>
-->

<span class="caption">リスト 9-6: `match`でエラーを呼び出し元のコードに返す関数</span>

<!--
Look at the return type of the function first: `Result<String, io::Error>`.
This means the function is returning a value of the type `Result<T, E>` where
the generic parameter `T` has been filled in with the concrete type `String`
and the generic type `E` has been filled in with the concrete type `io::Error`.
If this function succeeds without any problems, the code that calls this
function will receive an `Ok` value that holds a `String`—the username that
this function read from the file. If this function encounters any problems, the
code that calls this function will receive an `Err` value that holds an
instance of `io::Error` that contains more information about what the problems
were. We chose `io::Error` as the return type of this function because that
happens to be the type of the error value returned from both of the operations
we’re calling in this function’s body that might fail: the `File::open`
function and the `read_to_string` method.
-->

まずは、関数の戻り値型に注目してください：`Result<String, io::Error>`です。つまり、この関数は、
`Result<T, E>`型の値を返しているということです。ここでジェネリック引数の`T`は、具体型`String`で埋められ、
ジェネリック引数の`E`は具体型`io::Error`で埋められています。この関数が何の問題もなく成功すれば、
この関数を呼び出したコードは、`String`(関数がファイルから読み取ったユーザ名) を保持する`Ok`値を受け取ります。
この関数が何か問題に行き当たったら、呼び出し元のコードは`io::Error`のインスタンスを保持する`Err`値を受け取り、
この`io::Error`は問題の内容に関する情報をより多く含んでいます。関数の戻り値の型に`io::Error`を選んだのは、
この関数本体で呼び出している失敗する可能性のある処理が両方とも偶然この型をエラー値として返すからです：
`File::open`関数と`read_to_string`メソッドです。

<!--
The body of the function starts by calling the `File::open` function. Then we
handle the `Result` value returned with a `match` similar to the `match` in
Listing 9-4, only instead of calling `panic!` in the `Err` case, we return
early from this function and pass the error value from `File::open` back to the
calling code as this function’s error value. If `File::open` succeeds, we store
the file handle in the variable `f` and continue.
-->

関数の本体は、`File::open`関数を呼び出すところから始まります。そして、リスト 9-4 の`match`に似た`match`で返ってくる`Result`値を扱い、
`Err`ケースに`panic!`を呼び出すだけの代わりに、この関数から早期リターンしてこの関数のエラー値として、
`File::open`から得たエラー値を呼び出し元に渡し戻します。`File::open`が成功すれば、
ファイルハンドルを変数`f`に保管して継続します。

<!--
Then we create a new `String` in variable `s` and call the `read_to_string`
method on the file handle in `f` to read the contents of the file into `s`. The
`read_to_string` method also returns a `Result` because it might fail, even
though `File::open` succeeded. So we need another `match` to handle that
`Result`: if `read_to_string` succeeds, then our function has succeeded, and we
return the username from the file that’s now in `s` wrapped in an `Ok`. If
`read_to_string` fails, we return the error value in the same way that we
returned the error value in the `match` that handled the return value of
`File::open`. However, we don’t need to explicitly say `return`, because this
is the last expression in the function.
-->

さらに、変数`s`に新規`String`を生成し、`f`のファイルハンドルに対して`read_to_string`を呼び出して、
ファイルの中身を`s`に読み出します。`File::open`が成功しても、失敗する可能性があるので、`read_to_string`メソッドも、
`Result`を返却します。その`Result`を処理するために別の`match`が必要になります：`read_to_string`が成功したら、
関数は成功し、今は`Ok`に包まれた`s`に入っているファイルのユーザ名を返却します。`read_to_string`が失敗したら、
`File::open`の戻り値を扱った`match`でエラー値を返したように、エラー値を返します。
しかし、明示的に`return`を述べる必要はありません。これが関数の最後の式だからです。

<!--
The code that calls this code will then handle getting either an `Ok` value
that contains a username or an `Err` value that contains an `io::Error`. We
don’t know what the calling code will do with those values. If the calling code
gets an `Err` value, it could call `panic!` and crash the program, use a
default username, or look up the username from somewhere other than a file, for
example. We don’t have enough information on what the calling code is actually
trying to do, so we propagate all the success or error information upwards for
it to handle appropriately.
-->

そうしたら、呼び出し元のコードは、ユーザ名を含む`Ok`値か、`io::Error`を含む`Err`値を得て扱います。
呼び出し元のコードがそれらの値をどうするかはわかりません。呼び出しコードが`Err`値を得たら、
例えば、`panic!`を呼び出してプログラムをクラッシュさせたり、デフォルトのユーザ名を使ったり、
ファイル以外の場所からユーザ名を検索したりできるでしょう。呼び出し元のコードが実際に何をしようとするかについて、
十分な情報がないので、成功や失敗情報を全て委譲して適切に扱えるようにするのです。

<!--
This pattern of propagating errors is so common in Rust that Rust provides the
question mark operator `?` to make this easier.
-->

Rust において、この種のエラー委譲は非常に一般的なので、Rust にはこれをしやすくする`?`演算子が用意されています。

<!--
#### A Shortcut for Propagating Errors: the `?` operator
-->

#### エラー委譲のショートカット：`?`演算子

<!--
Listing 9-7 shows an implementation of `read_username_from_file` that has the
same functionality as it had in Listing 9-6, but this implementation uses the
`?` operator:
-->

リスト 9-7 もリスト 9-6 と同じ機能を有する`read_username_from_file`の実装ですが、
こちらは`?`演算子を使用しています：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

<!--
<span class="caption">Listing 9-7: A function that returns errors to the
calling code using the `?` operator</span>
-->

<span class="caption">リスト 9-7: `?`演算子でエラーを呼び出し元に返す関数</span>

<!--
The `?` placed after a `Result` value is defined to work in almost the same way
as the `match` expressions we defined to handle the `Result` values in Listing
9-6. If the value of the `Result` is an `Ok`, the value inside the `Ok` will
get returned from this expression, and the program will continue. If the value
is an `Err`, the value inside the `Err` will be returned from the whole
function as if we had used the `return` keyword so the error value gets
propagated to the calling code.
-->

`Result`値の直後に置かれた`?`は、リスト 9-6 で`Result`値を処理するために定義した`match`式とほぼ同じように動作します。
`Result`の値が`Ok`なら、`Ok`の中身がこの式から返ってきて、プログラムは継続します。値が`Err`なら、
`return`キーワードを使ったかのように関数全体から`Err`の中身が返ってくるので、
エラー値は呼び出し元のコードに委譲されます。

<!--
There is a difference between what the `match` expression from Listing 9-6 and
the `?` operator do: error values used with `?` go through the `from` function,
defined in the `From` trait in the standard library, which is used to convert
errors from one type into another. When the `?` operator calls the `from`
function, the error type received is converted into the error type defined in
the return type of the current function. This is useful when a function returns
one error type to represent all the ways a function might fail, even if parts
might fail for many different reasons. As long as each error type implements
the `from` function to define how to convert itself to the returned error type,
the `?` operator takes care of the conversion automatically.
-->

リスト 9-6 の`match`式と`?`演算子には違いがあります：`?`を使ったエラー値は、
標準ライブラリの`From`トレイトで定義され、エラーの型を別のものに変換する`from`関数を通ることです。
`?`演算子が`from`関数を呼び出すと、受け取ったエラー型が現在の関数の戻り値型で定義されているエラー型に変換されます。これは、
個々がいろんな理由で失敗する可能性があるのにも関わらず、関数が失敗する可能性を全て一つのエラー型で表現して返す時に有用です。
各エラー型が`from`関数を実装して返り値のエラー型への変換を定義している限り、
`?`演算子が変換の面倒を自動的に見てくれます。

<!--
In the context of Listing 9-7, the `?` at the end of the `File::open` call will
return the value inside an `Ok` to the variable `f`. If an error occurs, the
`?` operator will return early out of the whole function and give any `Err`
value to the calling code. The same thing applies to the `?` at the end of the
`read_to_string` call.
-->

リスト 9-7 の文脈では、`File::open`呼び出し末尾の`?`は`Ok`の中身を変数`f`に返します。
エラーが発生したら、`?`演算子により関数全体から早期リターンし、あらゆる`Err`値を呼び出し元に与えます。
同じ法則が`read_to_string`呼び出し末尾の`?`にも適用されます。

<!--
The `?` operator eliminates a lot of boilerplate and makes this function’s
implementation simpler. We could even shorten this code further by chaining
method calls immediately after the `?`, as shown in Listing 9-8.
-->

`?`演算子により定型コードの多くが排除され、この関数の実装を単純にしてくれます。
リスト 9-8 で示したように、`?`の直後のメソッド呼び出しを連結することでさらにこのコードを短くすることさえもできます。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

<!--
<span class="caption">Listing 9-8: Chaining method calls after the `?`
operator</span>
-->

<span class="caption">リスト 9-8: `?`演算子の後のメソッド呼び出しを連結する</span>

<!--
We’ve moved the creation of the new `String` in `s` to the beginning of the
function; that part hasn’t changed. Instead of creating a variable `f`, we’ve
chained the call to `read_to_string` directly onto the result of
`File::open("hello.txt")?`. We still have a `?` at the end of the
`read_to_string` call, and we still return an `Ok` value containing the
username in `s` when both `File::open` and `read_to_string` succeed rather than
returning errors. The functionality is again the same as in Listing 9-6 and
Listing 9-7; this is just a different, more ergonomic way to write it.
-->

`s`の新規`String`の生成を関数の冒頭に移動しました; その部分は変化していません。変数`f`を生成する代わりに、
`read_to_string`の呼び出しを直接`File::open("hello.txt")?`の結果に連結させました。
それでも、`read_to_string`呼び出しの末尾には`?`があり、`File::open`と`read_to_string`両方が成功したら、
エラーを返すというよりもそれでも、`s`にユーザ名を含む`Ok`値を返します。機能もまたリスト 9-6 及び、9-7 と同じです;
ただ単に異なるバージョンのよりエルゴノミックな書き方なのです。

<!--
#### `?` Operator Can Only Be Used in Functions That Return `Result`
-->

#### `?`演算子は、`Result`を返す関数でしか使用できない

<!--
The `?` operator can only be used in functions that have a return type of
`Result`, because it is defined to work in the same way as the `match`
expression we defined in Listing 9-6. The part of the `match` that requires a
return type of `Result` is `return Err(e)`, so the return type of the function
must be a `Result` to be compatible with this `return`.
-->

`?`演算子は戻り値に`Result`を持つ関数でしか使用できません。というのも、リスト 9-6 で定義した`match`式と同様に動作するよう、
定義されているからです。`Result`の戻り値型を要求する`match`の部品は、`return Err(e)`なので、
関数の戻り値はこの`return`と互換性を保つために`Result`でなければならないのです。

<!--
Let’s look at what happens if we use the `?` operator in the `main` function,
which you’ll recall has a return type of `()`:
-->

`main`関数で`?`演算子を使用したらどうなるか見てみましょう。`main`関数は、戻り値が`()`でしたね：

```rust,ignore
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

<!--
When we compile this code, we get the following error message:
-->

このコードをコンパイルすると、以下のようなエラーメッセージが得られます：

```text
error[E0277]: the trait bound `(): std::ops::Try` is not satisfied
(エラー: `(): std::ops::Try`というトレイト境界が満たされていません)
 --> src/main.rs:4:13
  |
4 |     let f = File::open("hello.txt")?;
  |             ------------------------
  |             |
  |             the `?` operator can only be used in a function that returns
  `Result` (or another type that implements `std::ops::Try`)
  |             in this macro invocation
  |             (このマクロ呼び出しの`Result`(かまたは`std::ops::Try`を実装する他の型) を返す関数でしか`?`演算子は使用できません)
  |
  = help: the trait `std::ops::Try` is not implemented for `()`
  (助言：`std::ops::Try`トレイトは`()`には実装されていません)
  = note: required by `std::ops::Try::from_error`
  (注釈：`std::ops::Try::from_error`で要求されています)
```

<!--
This error points out that we’re only allowed to use the `?` operator in a
function that returns `Result`. In functions that don’t return `Result`, when
you call other functions that return `Result`, you’ll need to use a `match` or
one of the `Result` methods to handle the `Result` instead of using the `?`
operator to potentially propagate the error to the calling code.
-->

このエラーは、`?`演算子は`Result`を返す関数でしか使用が許可されないと指摘しています。
`Result`を返さない関数では、`Result`を返す別の関数を呼び出した時、
`?`演算子を使用してエラーを呼び出し元に委譲する可能性を生み出す代わりに、`match`か`Result`のメソッドのどれかを使う必要があるでしょう。

<!--
Now that we’ve discussed the details of calling `panic!` or returning `Result`,
let’s return to the topic of how to decide which is appropriate to use in which
cases.
-->

さて、`panic!`呼び出しや`Result`を返す詳細について議論し終えたので、
どんな場合にどちらを使うのが適切か決める方法についての話に戻りましょう。
