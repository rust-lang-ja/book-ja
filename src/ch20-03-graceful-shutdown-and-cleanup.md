<!--
## Graceful Shutdown and Cleanup
-->

## 正常なシャットダウンと片付け

<!--
The code in Listing 20-20 is responding to requests asynchronously through the
use of a thread pool, as we intended. We get some warnings about the `workers`,
`id`, and `thread` fields that we’re not using in a direct way that reminds us
we’re not cleaning up anything. When we use the less elegant <span
class="keystroke">ctrl-c</span> method to halt the main thread, all other
threads are stopped immediately as well, even if they’re in the middle of
serving a request.
-->

リスト20-20のコードは、意図した通り、スレッドプールの使用を通してリクエストに非同期に応答できます。
直接使用していない`workers`、`id`、`thread`フィールドについて警告が出ます。この警告は、現在のコードは何も片付けていないことを思い出させてくれます。
優美さに欠ける<span class="keystroke">ctrl-c</span>を使用してメインスレッドを停止させる方法を使用すると、
リクエストの処理中であっても、他のスレッドも停止します。

<!--
Next, then, we’ll implement the `Drop` trait to call `join` on each of the
threads in the pool so they can finish the requests they’re working on before
closing. Then we’ll implement a way to tell the threads they should stop
accepting new requests and shut down. To see this code in action, we’ll modify
our server to accept only two requests before gracefully shutting down its
thread pool.
-->

では次に、閉じる前に取り掛かっているリクエストを完了できるように、プールの各スレッドに対して`join`を呼び出す`Drop`トレイトを実装します。
そして、スレッドに新しいリクエストの受付を停止し、終了するように教える方法を実装します。
このコードが動いているのを確かめるために、サーバを変更して正常にスレッドプールを終了する前に2つしかリクエストを受け付けないようにします。

<!--
### Implementing the `Drop` Trait on `ThreadPool`
-->

### `ThreadPool`に`Drop`トレイトを実装する

<!--
Let’s start with implementing `Drop` on our thread pool. When the pool is
dropped, our threads should all join to make sure they finish their work.
Listing 20-22 shows a first attempt at a `Drop` implementation; this code won’t
quite work yet.
-->

スレッドプールに`Drop`を実装するところから始めましょう。プールがドロップされると、
スレッドは全てjoinして、作業を完了するのを確かめるべきです。リスト20-22は、`Drop`実装の最初の試みを表示しています;
このコードはまだ完全には動きません。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/listing-20-22/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-22: Joining each thread when the thread pool
goes out of scope</span>
-->

<span class="caption">リスト20-22: スレッドプールがスコープを抜けた時にスレッドをjoinさせる</span>

<!--
First, we loop through each of the thread pool `workers`. We use `&mut` for
this because `self` is a mutable reference, and we also need to be able to
mutate `worker`. For each worker, we print a message saying that this
particular worker is shutting down, and then we call `join` on that worker’s
thread. If the call to `join` fails, we use `unwrap` to make Rust panic and go
into an ungraceful shutdown.
-->

まず、スレッドプール`workers`それぞれを走査します。`self`は可変参照であり、`worker`を可変化できる必要もあるので、
これには`&mut`を使用しています。ワーカーそれぞれに対して、特定のワーカーを終了する旨のメッセージを出力し、
それから`join`をワーカースレッドに対して呼び出しています。`join`の呼び出しが失敗したら、
`unwrap`を使用してRustをパニックさせ、正常でないシャットダウンに移行します。

<!--
Here is the error we get when we compile this code:
-->

こちらが、このコードをコンパイルする際に出るエラーです:

```console
{{#include ../listings/ch20-web-server/listing-20-22/output.txt}}
```

<!--
The error tells us we can’t call `join` because we only have a mutable borrow
of each `worker` and `join` takes ownership of its argument. To solve this
issue, we need to move the thread out of the `Worker` instance that owns
`thread` so `join` can consume the thread. We did this in Listing 17-15: if
`Worker` holds an `Option<thread::JoinHandle<()>>` instead, we can call the
`take` method on the `Option` to move the value out of the `Some` variant and
leave a `None` variant in its place. In other words, a `Worker` that is running
will have a `Some` variant in `thread`, and when we want to clean up a
`Worker`, we’ll replace `Some` with `None` so the `Worker` doesn’t have a
thread to run.
-->

各`worker`の可変参照しかなく、`join`は引数の所有権を奪うためにこのエラーは`join`を呼び出せないと教えてくれています。
この問題を解決するには、`join`がスレッドを消費できるように、`thread`を所有する`Worker`インスタンスからスレッドをムーブする必要があります。
これをリスト17-15では行いました: `Worker`が代わりに`Option<thread::JoinHandle<()>>`を保持していれば、
`Option`に対して`take`メソッドを呼び出し、`Some`列挙子から値をムーブし、その場所に`None`列挙子を残すことができます。
言い換えれば、実行中の`Worker`には`thread`に`Some`列挙子があり、`Worker`を片付けたい時には、
`Worker`が実行するスレッドがないように`Some`を`None`で置き換えるのです。

<!--
So we know we want to update the definition of `Worker` like this:
-->

従って、`Worker`の定義を以下のように更新したいことがわかります:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/no-listing-04-update-worker-definition/src/lib.rs:here}}
```

<!--
Now let’s lean on the compiler to find the other places that need to change.
Checking this code, we get two errors:
-->

さて、コンパイラを頼りにして他に変更する必要がある箇所を探しましょう。このコードをチェックすると、
2つのエラーが出ます:

```console
{{#include ../listings/ch20-web-server/no-listing-04-update-worker-definition/output.txt}}
```

<!--
Let’s address the second error, which points to the code at the end of
`Worker::new`; we need to wrap the `thread` value in `Some` when we create a
new `Worker`. Make the following changes to fix this error:
-->

2番目のエラーを扱いましょう。これは、`Worker::new`の最後のコードを指しています; 新しい`Worker`を作成する際に、
`Some`に`thread`の値を包む必要があります。このエラーを修正するために以下の変更を行なってください:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/no-listing-05-fix-worker-new/src/lib.rs:here}}
```

<!--
The first error is in our `Drop` implementation. We mentioned earlier that we
intended to call `take` on the `Option` value to move `thread` out of `worker`.
The following changes will do so:
-->

最初のエラーは`Drop`実装内にあります。先ほど、`Option`値に対して`take`を呼び出し、
`thread`を`worker`からムーブする意図があることに触れました。以下の変更がそれを行います:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch20-web-server/no-listing-06-fix-threadpool-drop/src/lib.rs:here}}
```

<!--
As discussed in Chapter 17, the `take` method on `Option` takes the `Some`
variant out and leaves `None` in its place. We’re using `if let` to destructure
the `Some` and get the thread; then we call `join` on the thread. If a worker’s
thread is already `None`, we know that worker has already had its thread
cleaned up, so nothing happens in that case.
-->

第17章で議論したように、`Option`の`take`メソッドは、`Some`列挙子を取り出し、その箇所に`None`を残します。
`if let`を使用して`Some`を分配し、スレッドを得ています; そして、スレッドに対して`join`を呼び出します。
ワーカーのスレッドが既に`None`なら、ワーカーはスレッドを既に片付け済みであることがわかるので、
その場合には何も起きません。

<!--
### Signaling to the Threads to Stop Listening for Jobs
-->

### スレッドに仕事をリッスンするのを止めるよう通知する

<!--
With all the changes we’ve made, our code compiles without any warnings.
However, the bad news is this code doesn’t function the way we want it to yet.
The key is the logic in the closures run by the threads of the `Worker`
instances: at the moment, we call `join`, but that won’t shut down the threads
because they `loop` forever looking for jobs. If we try to drop our
`ThreadPool` with our current implementation of `drop`, the main thread will
block forever waiting for the first thread to finish.
-->

これらの変更によって、コードは警告なしでコンパイルできます。ですが悪い知らせは、このコードが期待したようにはまだ機能しないことです。
鍵は、`Worker`インスタンスのスレッドで実行されるクロージャのロジックです: 現時点で`join`を呼び出していますが、
仕事を求めて永遠に`loop`するので、スレッドを終了しません。現在の`drop`の実装で`ThreadPool`をドロップしようとしたら、
最初のスレッドが完了するのを待機してメインスレッドは永遠にブロックされるでしょう。

<!--
To fix this problem, we’ll need a change in the `ThreadPool` `drop`
implementation and then a change in the `Worker` loop.
-->

この問題を修正するためには、`ThreadPool`の`drop`実装に対して変更を加え、
その後`Worker`ループに対して変更を加える必要があるでしょう。

<!--
First, we’ll change the `ThreadPool` `drop` implementation to explicitly drop
the `sender` before waiting for the threads to finish. Listing 20-23 shows the
changes to `ThreadPool` to explicitly drop `sender`. We use the same `Option`
and `take` technique as we did with the thread to be able to move `sender` out
of `ThreadPool`:
-->

まず、スレッドが完了するのを待つ前に明示的に`sender`をドロップするように、
`ThreadPool`の`drop`実装を変更します。リスト20-23は
明示的に`sender`をドロップするような`ThreadPool`に対する変更を示しています。
`ThreadPool`から`sender`をムーブできるようにするために、
スレッドで使ったのと同じ`Option`と`take`のテクニックを使用します:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground,not_desired_behavior
{{#rustdoc_include ../listings/ch20-web-server/listing-20-23/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-23: Explicitly drop `sender` before joining
the worker threads</span>
-->

<span class="caption">リスト20-23: ワーカースレッドをjoinする前に明示的に`sender`をドロップする</span>

<!--
Dropping `sender` closes the channel, which indicates no more messages will be
sent. When that happens, all the calls to `recv` that the workers do in the
infinite loop will return an error. In Listing 20-24, we change the `Worker`
loop to gracefully exit the loop in that case, which means the threads will
finish when the `ThreadPool` `drop` implementation calls `join` on them.
-->

`sender`をドロップすると、チャンネルは閉じられ、それ以上メッセージが送られないことを示します。
これが発生すると、ワーカーが無限ループ内で行うすべての`recv`への呼び出しは失敗し、エラーを返すでしょう。
リスト20-24では、そのような場合に正常にループを脱出するように`Worker`ループを変更します。
正常とは、`ThreadPool`の`drop`実装がスレッドに対して`join`を呼び出すときに、スレッドが完了するようにします。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-24/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-24: Explicitly break out of the loop when
`recv` returns an error</span>
-->

<span class="caption">リスト20-24: `recv`がエラーを返したときに明示的にループを抜ける</span>

<!--
To see this code in action, let’s modify `main` to accept only two requests
before gracefully shutting down the server, as shown in Listing 20-25.
-->

このコードが動いているところを確認するために、`main`を変更してサーバを正常に閉じる前に2つしかリクエストを受け付けないようにしましょう。
リスト20-25のようにですね。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/listing-20-25/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-25: Shut down the server after serving two
requests by exiting the loop</span>
-->

<span class="caption">リスト20-25: ループを抜けることで、2つのリクエストを処理した後にサーバを閉じる</span>

<!--
You wouldn’t want a real-world web server to shut down after serving only two
requests. This code just demonstrates that the graceful shutdown and cleanup is
in working order.
-->

現実世界のWebサーバには、たった2つリクエストを受け付けた後にシャットダウンしてほしくはないでしょう。
このコードは、単に正常なシャットダウンとクリーンアップが正しく機能することを示すだけです。

<!--
The `take` method is defined in the `Iterator` trait and limits the iteration
to the first two items at most. The `ThreadPool` will go out of scope at the
end of `main`, and the `drop` implementation will run.
-->

`take`メソッドは、`Iterator`トレイトで定義されていて、最大でも繰り返しを最初の2つの要素だけに制限します。
`ThreadPool`は`main`の末端でスコープを抜け、`drop`実装が実行されます。

<!--
Start the server with `cargo run`, and make three requests. The third request
should error, and in your terminal you should see output similar to this:
-->

`cargo run`でサーバを開始し、3つリクエストを行なってください。3番目のリクエストはエラーになるはずで、
端末にはこのような出力が目撃できるはずです:

<!-- manual-regeneration
cd listings/ch20-web-server/listing-20-25
cargo run
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
third request will error because server will have shut down
copy output below
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.0s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 3 got a job; executing.
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

<!--
You might see a different ordering of workers and messages printed. We can see
how this code works from the messages: workers 0 and 3 got the first two
requests. The server stopped accepting connections after the second connection,
and the `Drop` implementation on `ThreadPool` starts executing before worker 3
even starts its job. Dropping the `sender` disconnects all the workers and
tells them to shut down. The workers each print a message when they disconnect,
and then the thread pool calls `join` to wait for each worker thread to finish.
-->

ワーカーとメッセージの順番は異なる可能性があります。どうやってこのコードが動くのかメッセージからわかります:
ワーカー0と3が最初の2つのリクエストを受け付けます。
2回目の接続の後は、サーバは接続を受け付けるのをやめ、ワーカー3がジョブを開始する前に`ThreadPool`の`Drop`実装が実行を開始します。
`sender`をドロップするとすべてのワーカーは切断され、それらに停止するように指示します。
ワーカーはそれぞれ切断された時にメッセージを出力し、それからスレッドプールは各ワーカースレッドが完了するのを待つために`join`を呼び出します。

<!--
Notice one interesting aspect of this particular execution: the `ThreadPool`
dropped the `sender`, and before any worker received an error, we tried to join
worker 0. Worker 0 had not yet gotten an error from `recv`, so the main thread
blocked waiting for worker 0 to finish. In the meantime, worker 3 received a
job and then all threads received an error. When worker 0 finished, the main
thread waited for the rest of the workers to finish. At that point, they had
all exited their loops and stopped.
-->

この特定の実行のある面白い側面に注目してください: `ThreadPool`は`sender`をドロップしますが、
どのワーカーがそのエラーを受け取るよりも前に、ワーカー0のjoinを試みています。ワーカー0はまだ`recv`からエラーを受け取っていなかったので、
メインスレッドはワーカー0が完了するまで待機してブロックされます。その間に、ワーカー3はジョブを受け取り、
その後すべてのスレッドはエラーを受け取ります。
ワーカー0が完了したら、メインスレッドは残りのワーカーが完了するのを待機します。
その時点で全ワーカーはループを脱出し、停止したのです。

<!--
Congrats! We’ve now completed our project; we have a basic web server that uses
a thread pool to respond asynchronously. We’re able to perform a graceful
shutdown of the server, which cleans up all the threads in the pool.
-->

おめでとうございます！プロジェクトを完成させました; スレッドプールを使用して非同期に応答する基本的なWebサーバができました。
サーバの正常なシャットダウンを行うことができ、プールの全スレッドを片付けます。

<!--
Here’s the full code for reference:
-->

参考までに、こちらが全コードです:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/no-listing-07-final-code/src/main.rs}}
```

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-07-final-code/src/lib.rs}}
```

<!--
We could do more here! If you want to continue enhancing this project, here are
some ideas:
-->

ここでできることはまだあるでしょう！よりこのプロジェクトを改善したいのなら、こちらがアイディアの一部です:

<!--
* Add more documentation to `ThreadPool` and its public methods.
* Add tests of the library’s functionality.
* Change calls to `unwrap` to more robust error handling.
* Use `ThreadPool` to perform some task other than serving web requests.
* Find a thread pool crate on [crates.io](https://crates.io/) and implement a
  similar web server using the crate instead. Then compare its API and
  robustness to the thread pool we implemented.
-->

* `ThreadPool`とその公開メソッドにもっとドキュメンテーションを追加する。
* ライブラリの機能のテストを追加する。
* `unwrap`の呼び出しをもっと頑健なエラー処理に変更する。
* `ThreadPool`を使用してWebリクエスト以外のなんらかの作業を行う。
* [crates.io](https://crates.io/) でスレッドプールのクレートを探して、そのクレートを代わりに使用して似たWebサーバを実装する。
  そして、APIと頑健性を我々が実装したものと比較する。

<!--
## Summary
-->

## まとめ

<!--
Well done! You’ve made it to the end of the book! We want to thank you for
joining us on this tour of Rust. You’re now ready to implement your own Rust
projects and help with other peoples’ projects. Keep in mind that there is a
welcoming community of other Rustaceans who would love to help you with any
challenges you encounter on your Rust journey.
-->

よくやりました！本の最後に到達しました！Rustのツアーに参加していただき、感謝の辞を述べたいです。
もう、ご自身のRustプロジェクトや他の方のプロジェクトのお手伝いをする準備ができています。
あなたがこれからのRustの旅で遭遇する、あらゆる困難の手助けを是非とも行いたいRustaceanたちの温かいコミュニティがあることを心に留めておいてくださいね。
