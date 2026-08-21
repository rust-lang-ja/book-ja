<!--
## Using Message Passing to Transfer Data Between Threads
-->

## メッセージ受け渡しを使ってスレッド間でデータを転送する

<!--
One increasingly popular approach to ensuring safe concurrency is *message
passing*, where threads or actors communicate by sending each other messages
containing data. Here’s the idea in a slogan from [the Go language
documentation](https://golang.org/doc/effective_go.html#concurrency):
“Do not communicate by sharing memory; instead, share memory by communicating.”
-->

人気度を増してきている安全な並行性を保証する一つのアプローチが*メッセージ受け渡し*で、
スレッドやアクターがデータを含むメッセージを相互に送り合うことでやり取りします。
こちらが、[Go言語のドキュメンテーション](https://golang.org/doc/effective_go.html#concurrency)のスローガンにある考えです:
「メモリを共有することでやり取りするな; 代わりにやり取りすることでメモリを共有しろ」

<!--
To accomplish message-sending concurrency, Rust's standard library provides an
implementation of *channels*. A channel is a general programming concept by
which data is sent from one thread to another.
-->

メッセージ送信による並行性を達成するために、Rustの標準ライブラリは*チャンネル*の実装を提供しています。
チャンネルは、あるスレッドから別のスレッドへデータを送信する手段である、普遍的なプログラミング概念です。

<!--
You can imagine a channel in programming as being like a directional channel of
water, such as a stream or a river. If you put something like a rubber duck
into a river, it will travel downstream to the end of the waterway.
-->

プログラミングのチャンネルは、川などの、向きのある水路のように考えることができます（訳注: channelは日常語としては水路を意味します）。
アヒルのおもちゃなどを川に置いたら、水路の終端まで下流に流れていきます。

<!--
6行目、for arriving messagesは本来ならfor messages arrivingのような気がするが、その想定で訳してある
これは自動詞を形容詞のように前からかけているだけと思われる
-->

<!--
A channel has two halves: a transmitter and a receiver. The transmitter half is
the upstream location where you put rubber ducks into the river, and the
receiver half is where the rubber duck ends up downstream. One part of your
code calls methods on the transmitter with the data you want to send, and
another part checks the receiving end for arriving messages. A channel is said
to be *closed* if either the transmitter or receiver half is dropped.
-->

チャンネルは、2分割できます: 転送機と受信機です。転送機はアヒルのおもちゃを川に置く上流になり、
受信機は、アヒルのおもちゃが行き着く下流になります。コードのある箇所が送信したいデータとともに転送機のメソッドを呼び出し、
別の部分がメッセージが到着していないか受信側を調べます。転送機と受信機のどちらかがドロップされると、
チャンネルは*閉じられた*と言います。

<!--
Here, we’ll work up to a program that has one thread to generate values and
send them down a channel, and another thread that will receive the values and
print them out. We’ll be sending simple values between threads using a channel
to illustrate the feature. Once you’re familiar with the technique, you could
use channels for any threads that need to communicate between each other, such
as a chat system or a system where many threads perform parts of a calculation
and send the parts to one thread that aggregates the results.
-->

ここで、1つのスレッドが値を生成し、それをチャンネルに送信し、別のスレッドがその値を受け取り、
出力するプログラムに取り掛かります。チャンネルを使用してスレッド間に単純な値を送り、
機能の説明を行います。一旦そのテクニックに慣れてしまえば、チャットシステムや、
多くのスレッドが計算の一部を担い、結果をまとめる1つのスレッドにその部分を送るようなシステムなど、
任意のスレッドが互いに通信する必要があるシステムに対して、チャンネルを使用できるでしょう。

<!--
First, in Listing 16-6, we’ll create a channel but not do anything with it.
Note that this won’t compile yet because Rust can’t tell what type of values we
want to send over the channel.
-->

まず、リスト16-6において、チャンネルを生成するものの、何もしません。
チャンネル越しにどんな型の値を送りたいのかコンパイラがわからないため、
これはまだコンパイルできないことに注意してください。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

<!--
<span class="caption">Listing 16-6: Creating a channel and assigning the two
halves to `tx` and `rx`</span>
-->

<span class="caption">リスト16-6: チャンネルを生成し、2つの部品を`tx`と`rx`に代入する</span>

<!--
We create a new channel using the `mpsc::channel` function; `mpsc` stands for
*multiple producer, single consumer*. In short, the way Rust’s standard library
implements channels means a channel can have multiple *sending* ends that
produce values but only one *receiving* end that consumes those values. Imagine
multiple streams flowing together into one big river: everything sent down any
of the streams will end up in one river at the end. We’ll start with a single
producer for now, but we’ll add multiple producers when we get this example
working.
-->

`mpsc::channel`関数で新しいチャンネルを生成しています; `mpsc`は*multiple producer, single consumer*を表しています。
簡潔に言えば、Rustの標準ライブラリがチャンネルを実装している方法は、1つのチャンネルが値を生成する複数の*送信*側と、
その値を消費するたった1つの*受信*側を持つことができるということを意味します。
複数の小川が互いに合わさって1つの大きな川になるところを想像してください: 
どの小川を通っても、送られたものは最終的に1つの川に行き着きます。今は、1つの生成器から始めますが、
この例が動作するようになったら、複数の生成器を追加します。

<!--
The `mpsc::channel` function returns a tuple, the first element of which is the
sending end--the transmitter--and the second element is the receiving end--the
receiver. The abbreviations `tx` and `rx` are traditionally used in many fields
for *transmitter* and *receiver* respectively, so we name our variables as such
to indicate each end. We’re using a `let` statement with a pattern that
destructures the tuples; we’ll discuss the use of patterns in `let` statements
and destructuring in Chapter 18. For now, know that using a `let` statement
this way is a convenient approach to extract the pieces of the tuple returned
by `mpsc::channel`.
-->

`mpsc::channel`関数はタプルを返し、1つ目の要素は、送信側（転送機）、2つ目の要素は受信側（受信機）になります。
`tx`と`rx`という略称は、多くの分野で伝統的に*転送機*と*受信機*にそれぞれ使用されているので、
変数をそのように名付けて、各終端を示します。タプルを分配するパターンを伴う`let`文を使用しています;
`let`文でパターンを使用することと分配については、第18章で議論しましょう。今のところは、
このように`let`文を使うと、`mpsc::channel`で返ってくるタプルの部品を抽出するのが便利になるということだけ知っておいてください。

<!--
Let’s move the transmitting end into a spawned thread and have it send one
string so the spawned thread is communicating with the main thread, as shown in
Listing 16-7. This is like putting a rubber duck in the river upstream or
sending a chat message from one thread to another.
-->

立ち上げたスレッドがメインスレッドとやり取りするように、転送機を立ち上げたスレッドに移動し、
1文字列を送らせましょう。リスト16-7のようにですね。川の上流にアヒルのおもちゃを置いたり、
チャットのメッセージをあるスレッドから別のスレッドに送るみたいですね。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

<!--
<span class="caption">Listing 16-7: Moving `tx` to a spawned thread and sending
“hi”</span>
-->

<span class="caption">リスト16-7: `tx`を立ち上げたスレッドに移動し、「やあ」を送る</span>

<!--
Again, we’re using `thread::spawn` to create a new thread and then using `move`
to move `tx` into the closure so the spawned thread owns `tx`. The spawned
thread needs to own the transmitter to be able to send messages through the
channel. The transmitter has a `send` method that takes the value we want to
send. The `send` method returns a `Result<T, E>` type, so if the receiver has
already been dropped and there’s nowhere to send a value, the send operation
will return an error. In this example, we’re calling `unwrap` to panic in case
of an error. But in a real application, we would handle it properly: return to
Chapter 9 to review strategies for proper error handling.
-->

今回も、`thread::spawn`を使用して新しいスレッドを生成し、それから`move`を使用して、
立ち上げたスレッドが`tx`を所有するようにクロージャに`tx`をムーブしています。立ち上げたスレッドは、
メッセージをチャンネルを通して送信できるように、転送機を所有する必要があります。
転送機には、送信したい値を取る`send`メソッドがあります。`send`メソッドは`Result<T, E>`型を返すので、
既に受信機がドロップされ、値を送信する場所がなければ、送信処理はエラーを返します。
この例では、エラーの場合には、パニックするように`unwrap`を呼び出しています。ですが、実際のアプリケーションでは、
ちゃんと扱うでしょう: 第9章に戻ってちゃんとしたエラー処理の方法を再確認してください。

<!--
In Listing 16-8, we’ll get the value from the receiver in the main thread. This
is like retrieving the rubber duck from the water at the end of the river or
receiving a chat message.
-->

リスト16-8において、メインスレッドのチャンネルの受信機から値を得ます。
アヒルのおもちゃを川の終端で水から回収したり、チャットメッセージを受信するみたいですね。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

<!--
<span class="caption">Listing 16-8: Receiving the value “hi” in the main thread
and printing it</span>
-->

<span class="caption">リスト16-8: 「やあ」の値をメインスレッドで受け取り、出力する</span>

<!--
The receiver has two useful methods: `recv` and `try_recv`. We’re using `recv`,
short for *receive*, which will block the main thread’s execution and wait
until a value is sent down the channel. Once a value is sent, `recv` will
return it in a `Result<T, E>`. When the transmitter closes, `recv` will return
an error to signal that no more values will be coming.
-->

受信機には有用なメソッドが2つあります: `recv`と`try_recv`です。
*receive*の省略形である`recv`を使っています。これは、メインスレッドの実行をブロックし、
値がチャンネルを流れてくるまで待機します。一旦値が送信されたら、`recv`はそれを`Result<T, E>`に含んで返します。
転送機が閉じたら、`recv`はエラーを返し、もう値は来ないと通知します。

<!--
The `try_recv` method doesn’t block, but will instead return a `Result<T, E>`
immediately: an `Ok` value holding a message if one is available and an `Err`
value if there aren’t any messages this time. Using `try_recv` is useful if
this thread has other work to do while waiting for messages: we could write a
loop that calls `try_recv` every so often, handles a message if one is
available, and otherwise does other work for a little while until checking
again.
-->

`try_recv`メソッドはブロックせず、代わりに即座に`Result<T, E>`を返します: 
メッセージがあったら、それを含む`Ok`値、今回は何もメッセージがなければ、`Err`値です。
メッセージを待つ間にこのスレッドにすることが他にあれば、`try_recv`は有用です:
`try_recv`を頻繁に呼び出し、メッセージがあったら処理し、それ以外の場合は、
再度チェックするまでちょっとの間、他の作業をするループを書くことができるでしょう。

<!--
We’ve used `recv` in this example for simplicity; we don’t have any other work
for the main thread to do other than wait for messages, so blocking the main
thread is appropriate.
-->

この例では、簡潔性のために`recv`を使用しました; メッセージを待つこと以外にメインスレッドがすべき作業はないので、
メインスレッドをブロックするのは適切です。

<!--
When we run the code in Listing 16-8, we’ll see the value printed from the main
thread:
-->

リスト16-8のコードを実行したら、メインスレッドから値が出力されるところを目撃するでしょう:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
```

<!--
Perfect!
-->

完璧です！

<!--
### Channels and Ownership Transference
-->

### チャンネルと所有権の転送

<!--
The ownership rules play a vital role in message sending because they help you
write safe, concurrent code. Preventing errors in concurrent programming is the
advantage of thinking about ownership throughout your Rust programs. Let’s do
an experiment to show how channels and ownership work together to prevent
problems: we’ll try to use a `val` value in the spawned thread *after* we’ve
sent it down the channel. Try compiling the code in Listing 16-9 to see why
this code isn’t allowed:
-->

安全な並行コードを書く手助けをしてくれるので、所有権規則は、メッセージ送信で重要な役割を担っています。
並行プログラミングでエラーを回避することは、Rustプログラム全体で所有権について考える利点です。
実験をしてチャンネルと所有権がともに動いて、どう問題を回避するかをお見せしましょう:
`val`値を立ち上げたスレッドで、チャンネルに送った*後*に使用を試みます。
リスト16-9のコードのコンパイルを試みて、このコードが許容されない理由を確認してください:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

<!--
<span class="caption">Listing 16-9: Attempting to use `val` after we’ve sent it
down the channel</span>
-->

<span class="caption">リスト16-9: チャンネルに送信後に`val`の使用を試みる</span>

<!--
Here, we try to print `val` after we’ve sent it down the channel via `tx.send`.
Allowing this would be a bad idea: once the value has been sent to another
thread, that thread could modify or drop it before we try to use the value
again. Potentially, the other thread’s modifications could cause errors or
unexpected results due to inconsistent or nonexistent data. However, Rust gives
us an error if we try to compile the code in Listing 16-9:
-->

ここで、`tx.send`経由でチャンネルに送信後に`val`を出力しようとしています。これを許可するのは、悪い考えです:
一旦、値が他のスレッドに送信されたら、再度値を使用しようとする前にそのスレッドが変更したりドロップできてしまいます。
可能性として、その別のスレッドの変更により、矛盾していたり存在しないデータのせいでエラーが発生したり、
予期しない結果になるでしょう。ですが、リスト16-9のコードのコンパイルを試みると、Rustはエラーを返します:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

<!--
Our concurrency mistake has caused a compile time error. The `send` function
takes ownership of its parameter, and when the value is moved, the receiver
takes ownership of it. This stops us from accidentally using the value again
after sending it; the ownership system checks that everything is okay.
-->

並行性のミスがコンパイルエラーを招きました。`send`関数は引数の所有権を奪い、
値がムーブされると、受信側が所有権を得るのです。これにより、送信後に誤って再度値を使用するのを防いでくれます;
所有権システムが、万事問題ないことを確認してくれます。

<!--
### Sending Multiple Values and Seeing the Receiver Waiting
-->

### 複数の値を送信し、受信側が待機するのを確かめる

<!--
The code in Listing 16-8 compiled and ran, but it didn’t clearly show us that
two separate threads were talking to each other over the channel. In Listing
16-10 we’ve made some modifications that will prove the code in Listing 16-8 is
running concurrently: the spawned thread will now send multiple messages and
pause for a second between each message.
-->

リスト16-8のコードはコンパイルでき、動きましたが、2つの個別のスレッドがお互いにチャンネル越しに会話していることは、
明瞭に示されませんでした。リスト16-10において、リスト16-8のコードが並行に動いていることを証明する変更を行いました:
立ち上げたスレッドは、複数のメッセージを送信し、各メッセージ間で、1秒待機します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

<!--
<span class="caption">Listing 16-10: Sending multiple messages and pausing
between each</span>
-->

<span class="caption">リスト16-10: 複数のメッセージを送信し、メッセージ間で停止する</span>

<!--
This time, the spawned thread has a vector of strings that we want to send to
the main thread. We iterate over them, sending each individually, and pause
between each by calling the `thread::sleep` function with a `Duration` value of
1 second.
-->

今回は、メインスレッドに送信したい文字列のベクタを立ち上げたスレッドが持っています。
それらを繰り返し、各々個別に送信し、`Duration`の値1秒とともに`thread::sleep`関数を呼び出すことで、
メッセージ間で停止します。

<!--
In the main thread, we’re not calling the `recv` function explicitly anymore:
instead, we’re treating `rx` as an iterator. For each value received, we’re
printing it. When the channel is closed, iteration will end.
-->

メインスレッドにおいて、最早`recv`関数を明示的に呼んではいません: 代わりに、
`rx`をイテレータとして扱っています。受信した値それぞれを出力します。
チャンネルが閉じられると、繰り返しも終わります。

<!--
When running the code in Listing 16-10, you should see the following output
with a 1-second pause in between each line:
-->

リスト16-10のコードを走らせると、各行の間に1秒の待機をしつつ、以下のような出力を目の当たりにするはずです:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
Got: from
Got: the
Got: thread
```

<!--
Because we don’t have any code that pauses or delays in the `for` loop in the
main thread, we can tell that the main thread is waiting to receive values from
the spawned thread.
-->

メインスレッドの`for`ループには停止したり、遅れせたりするコードは何もないので、
メインスレッドが立ち上げたスレッドから値を受け取るのを待機していることがわかります。

<!--
### Creating Multiple Producers by Cloning the Transmitter
-->

### 転送機をクローンして複数の生成器を作成する

<!--
Earlier we mentioned that `mpsc` was an acronym for *multiple producer,
single consumer*. Let’s put `mpsc` to use and expand the code in Listing 16-10
to create multiple threads that all send values to the same receiver. We can do
so by cloning the transmitter, as shown in Listing 16-11:
-->

`mpsc`は、*mutiple producer, single consumer*の頭字語であると前述しました。
`mpsc`を使い、リスト16-10のコードを拡張して、全ての値を同じ受信機に送信する複数のスレッドを生成しましょう。
転送機をクローンすることでそうすることができます。リスト16-11のようにですね:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

<!--
<span class="caption">Listing 16-11: Sending multiple messages from multiple
producers</span>
-->

<span class="caption">リスト16-11: 複数の生成器から複数のメッセージを送信する</span>

<!--
This time, before we create the first spawned thread, we call `clone` on the
transmitter. This will give us a new transmitter we can pass to the first
spawned thread. We pass the original transmitter to a second spawned thread.
This gives us two threads, each sending different messages to the one receiver.
-->

今回、最初のスレッドを立ち上げる前に、転送機に対して`clone`を呼び出しています。
これにより、最初に立ち上げたスレッドに渡せる新しい転送機が得られます。
元の転送機は、2番目に立ち上げたスレッドに渡します。これにより2つスレッドが得られ、
1つの受信機にそれぞれ異なるメッセージを送信します。

<!--
When you run the code, your output should look something like this:
-->

コードを実行すると、出力は以下のようなものになるはずです:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

<!--
You might see the values in another order, depending on your system. This is
what makes concurrency interesting as well as difficult. If you experiment with
`thread::sleep`, giving it various values in the different threads, each run
will be more nondeterministic and create different output each time.
-->

システム次第で、別の順番で値が出る可能性もあります。並行性が面白いと同時に難しい部分でもあります。
異なるスレッドで色々な値を与えて`thread::sleep`で実験をしたら、走らせるたびにより非決定的になり、
毎回異なる出力をするでしょう。

<!--
Now that we’ve looked at how channels work, let’s look at a different method of
concurrency.
-->

チャンネルの動作方法を見たので、他の並行性に目を向けましょう。
