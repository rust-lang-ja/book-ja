<!--
## Building a Single-Threaded Web Server
-->

## シングルスレッドの Web サーバを構築する

<!--
We’ll start by getting a single-threaded web server working. Before we begin,
let’s look at a quick overview of the protocols involved in building web
servers. The details of these protocols are beyond the scope of this book, but
a brief overview will give you the information you need.
-->

シングルスレッドの Web サーバを動かすところから始めます。始める前に、Web サーバ構築に関係するプロトコルをさっと一覧しましょう。
これらのプロトコルの詳細は、この本の範疇を超えていますが、さっと眺めることで必要な情報が得られるでしょう。

<!--
The two main protocols involved in web servers are the *Hypertext Transfer
Protocol* *(HTTP)* and the *Transmission Control Protocol* *(TCP)*. Both
protocols are *request-response* protocols, meaning a *client* initiates
requests and a *server* listens to the requests and provides a response to the
client. The contents of those requests and responses are defined by the
protocols.
-->

主に 2 つのプロトコルが Web サーバに関係し、*Hypertext Transfer Protocol* *(HTTP)*(`注釈`: ハイパーテキスト転送プロトコル) と、
*Transmission Control Protocol* *(TCP)*(`注釈`: 伝送制御プロトコル) です。
両者のプロトコルは、*リクエスト・レスポンス*プロトコルであり、つまり、*クライアント*がリクエスト (要求) を初期化し、
*サーバ*はリクエストをリッスンし、クライアントにレスポンス (応答) を提供するということです。
それらのリクエストとレスポンスの中身は、プロトコルで規定されています。

<!--
TCP is the lower-level protocol that describes the details of how information
gets from one server to another but doesn’t specify what that information is.
HTTP builds on top of TCP by defining the contents of the requests and
responses. It’s technically possible to use HTTP with other protocols, but in
the vast majority of cases, HTTP sends its data over TCP. We’ll work with the
raw bytes of TCP and HTTP requests and responses.
-->

TCP は、情報がとあるサーバから別のサーバへどう到達するかの詳細を記述するものの、その情報がなんなのかは指定しない、
より低レベルのプロトコルです。HTTP はリクエストとレスポンスの中身を定義することで TCP の上に成り立っています。
技術的には HTTP を他のプロトコルとともに使用することができますが、大抵の場合、HTTP は TCP の上にデータを送信します。
TCP と HTTP のリクエストとレスポンスの生のバイトを取り扱います。

<!--
### Listening to the TCP Connection
-->

### TCP 接続をリッスンする

<!--
Our web server needs to listen to a TCP connection, so that’s the first part
we’ll work on. The standard library offers a `std::net` module that lets us do
this. Let’s make a new project in the usual fashion:
-->

Web サーバは TCP 接続をリッスンするので、そこが最初に取り掛かる部分になります。標準ライブラリは、
`std::net`というこれを行うモジュールを用意しています。通常通り、新しいプロジェクトを作りましょう：

```text
$ cargo new hello --bin
     Created binary (application) `hello` project
$ cd hello
```

<!--
Now enter the code in Listing 20-1 in *src/main.rs* to start. This code will
listen at the address `127.0.0.1:7878` for incoming TCP streams. When it gets
an incoming stream, it will print `Connection established!`.
-->

さて、リスト 20-1 のコードを*src/main.rs*に入力して始めてください。このコードは、
TCP ストリームを受信するため`127.0.0.1:7878`というアドレスをリッスンします。
入力ストリームを得ると、`Connection established!`と出力します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,no_run
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // 接続が確立しました
        println!("Connection established!");
    }
}
```

<!--
<span class="caption">Listing 20-1: Listening for incoming streams and printing
a message when we receive a stream</span>
-->

<span class="caption">リスト 20-1: 入力ストリームをリッスンし、ストリームを受け付けた時にメッセージを出力する</span>

<!--
Using `TcpListener`, we can listen for TCP connections at the address
`127.0.0.1:7878`. In the address, the section before the colon is an IP address
representing your computer (this is the same on every computer and doesn’t
represent the authors’ computer specifically), and `7878` is the port. We’ve
chosen this port for two reasons: HTTP is normally accepted on this port, and
7878 is “rust” typed on a telephone.
-->

`TcpListener`により、アドレス`127.0.0.1:7878`で TCP 接続をリッスンできます。アドレス内で、
コロンの前の区域は、自分のコンピュータを表す IP アドレスで (これはどんなコンピュータでも同じで、
特に著者のコンピュータを表すわけではありません)、`7878`はポートです。このポートを選択した理由は 2 つあります：
HTTP は通常このポートで受け付けられることと、7878 は電話で“rust”と入力されるからです。

<!--
The `bind` function in this scenario works like the `new` function in that it
will return a new `TcpListener` instance. The reason the function is called
`bind` is that in networking, connecting to a port to listen to is known as
“binding to a port.”
-->

この筋書きでの`bind`関数は、新しい`TcpListener`インスタンスを返すという点で`new`関数のような働きをします。
この関数が`bind`と呼ばれている理由は、ネットワークにおいて、リッスンすべきポートに接続することは、
「ポートに束縛する」(binding to a port) こととして知られているからです。

<!--
The `bind` function returns a `Result<T, E>`, which indicates that binding
might fail. For example, connecting to port 80 requires administrator
privileges (nonadministrators can listen only on ports higher than 1024), so if
we tried to connect to port 80 without being an administrator, binding wouldn’t
work. As another example, binding wouldn’t work if we ran two instances of our
program and so had two programs listening to the same port. Because we’re
writing a basic server just for learning purposes, we won’t worry about
handling these kinds of errors; instead, we use `unwrap` to stop the program if
errors happen.
-->

`bind`関数は`Result<T, E>`を返し、束縛が失敗することもあることを示しています。例えば、
ポート 80 に接続するには管理者権限が必要なので (管理者以外はポート 1024 以上しかリッスンできません) 管理者にならずにポート 80 に接続を試みたら、
束縛はうまくいかないでしょう。また、別の例として自分のプログラムを 2 つ同時に立ち上げて 2 つのプログラムが同じポートをリッスンしたら、
束縛は機能しないでしょう。学習目的のためだけに基本的なサーバを記述しているので、この種のエラーを扱う心配はしません;
その代わり、`unwrap`を使用してエラーが発生したら、プログラムを停止します。

<!--
The `incoming` method on `TcpListener` returns an iterator that gives us a
sequence of streams (more specifically, streams of type `TcpStream`). A single
*stream* represents an open connection between the client and the server. A
*connection* is the name for the full request and response process in which a
client connects to the server, the server generates a response, and the server
closes the connection. As such, `TcpStream` will read from itself to see what
the client sent and then allow us to write our response to the stream. Overall,
this `for` loop will process each connection in turn and produce a series of
streams for us to handle.
-->

`TcpListener`の`incoming`メソッドは、一連のストリームを与えるイテレータを返します (具体的には、型`TcpStream`のストリーム)。
単独の*ストリーム*がクライアント・サーバ間の開かれた接続を表します。*接続*(connection) は、
クライアントがサーバに接続し、サーバがレスポンスを生成し、サーバが接続を閉じるというリクエストとレスポンス全体の過程の名前です。
そのため、`TcpStream`は自身を読み取って、クライアントが送信したことを確認し、それからレスポンスをストリームに記述させてくれます。
総括すると、この`for`ループは各接続を順番に処理し、我々が扱えるように一連のストリームを生成します。

<!--
For now, our handling of the stream consists of calling `unwrap` to terminate
our program if the stream has any errors; if there aren’t any errors, the
program prints a message. We’ll add more functionality for the success case in
the next listing. The reason we might receive errors from the `incoming` method
when a client connects to the server is that we’re not actually iterating over
connections. Instead, we’re iterating over *connection attempts*. The
connection might not be successful for a number of reasons, many of them
operating system specific. For example, many operating systems have a limit to
the number of simultaneous open connections they can support; new connection
attempts beyond that number will produce an error until some of the open
connections are closed.
-->

とりあえず、ストリームの扱いは、`unwrap`を呼び出してストリームにエラーがあった場合にプログラムを停止することから構成されています;
エラーがなければ、プログラムはメッセージを出力します。次のリストで成功した時にさらに多くの機能を追加します。
クライアントがサーバに接続する際に`incoming`メソッドからエラーを受け取る可能性がある理由は、
実際には接続を走査していないからです。代わりに*接続の試行*を走査しています。接続は多くの理由で失敗する可能性があり、
そのうちの多くは、OS 特有です。例を挙げれば、多くの OS には、サポートできる同時に開いた接続数に上限があります;
開かれた接続の一部が閉じられるまでその数字を超えた接続の試行はエラーになります。

<!--
Let’s try running this code! Invoke `cargo run` in the terminal and then load
*127.0.0.1:7878* in a web browser. The browser should show an error message
like “Connection reset,” because the server isn’t currently sending back any
data. But when you look at your terminal, you should see several messages that
were printed when the browser connected to the server!
-->

このコードを試しに実行してみましょう！端末で`cargo run`を呼び出し、それから Web ブラウザで*127.0.0.1:7878*をロードしてください。
ブラウザは、「接続がリセットされました」などのエラーメッセージを表示するはずです。サーバが現状、何もデータを返してこないからです。
ですが、端末に目を向ければ、ブラウザがサーバに接続した際にいくつかメッセージが出力されるのを目の当たりにするはずです。

```text
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```

<!--
Sometimes, you’ll see multiple messages printed for one browser request; the
reason might be that the browser is making a request for the page as well as a
request for other resources, like the *favicon.ico* icon that appears in the
browser tab.
-->

時々、1 回のブラウザリクエストで複数のメッセージが出力されるのを目の当たりにするでしょう;
その理由は、ブラウザがページだけでなく、
ブラウザのタブに出現する*favicon.ico*アイコンなどの他のリソースにもリクエストを行なっているということかもしれません。

<!--
It could also be that the browser is trying to connect to the server multiple
times because the server isn’t responding with any data. When `stream` goes out
of scope and is dropped at the end of the loop, the connection is closed as
part of the `drop` implementation. Browsers sometimes deal with closed
connections by retrying, because the problem might be temporary. The important
factor is that we’ve successfully gotten a handle to a TCP connection!
-->

サーバが何もデータを送り返してこないので、ブラウザがサーバに何度も接続を試みているということである可能性もあるでしょう。
`stream`がスコープを抜け、ループの最後でドロップされると、接続は`drop`実装の一部として閉じられます。
ブラウザは、再試行することで閉じられた接続を扱うことがあります。問題が一時的なものである可能性があるからです。
重要な要素は、TCP 接続へのハンドルを得ることに成功したということです！

<!--
Remember to stop the program by pressing <span class="keystroke">ctrl-c</span>
when you’re done running a particular version of the code. Then restart `cargo
run` after you’ve made each set of code changes to make sure you’re running the
newest code.
-->

特定のバージョンのコードを走らせ終わった時に<span class="keystroke">ctrl-c</span>を押して、
プログラムを止めることを忘れないでください。そして、一連のコード変更を行った後に`cargo run`を再起動し、
最新のコードを実行していることを確かめてください。

<!--
### Reading the Request
-->

### リクエストを読み取る

<!--
Let’s implement the functionality to read the request from the browser! To
separate the concerns of first getting a connection and then taking some action
with the connection, we’ll start a new function for processing connections. In
this new `handle_connection` function, we’ll read data from the TCP stream and
print it so we can see the data being sent from the browser. Change the code to
look like Listing 20-2.
-->

ブラウザからリクエストを読み取る機能を実装しましょう！まず接続を得、それから接続に対して何らかの行動を行う責任を分離するために、
接続を処理する新しい関数を開始します。この新しい`handle_connection`関数において、TCP ストリームからデータを読み取り、
ブラウザからデータが送られていることを確認できるように端末に出力します。コードをリスト 20-2 のように変更してください。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,no_run
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
```

<!--
<span class="caption">Listing 20-2: Reading from the `TcpStream` and printing
the data</span>
-->

<span class="caption">リスト 20-2: `TcpStream`から読み取り、データを出力する</span>

<!--
We bring `std::io::prelude` into scope to get access to certain traits that let
us read from and write to the stream. In the `for` loop in the `main` function,
instead of printing a message that says we made a connection, we now call the
new `handle_connection` function and pass the `stream` to it.
-->

`std::io::prelude`をスコープに導入して、ストリームから読み書きさせてくれる特定のトレイトにアクセスできるようにしています。
`main`関数内の`for`ループで、接続を確立したというメッセージを出力する代わりに、今では、
新しい`handle_connection`関数を呼び出し、`stream`を渡しています。

<!--
In the `handle_connection` function, we’ve made the `stream` parameter mutable.
The reason is that the `TcpStream` instance keeps track of what data it returns
to us internally. It might read more data than we asked for and save that data
for the next time we ask for data. It therefore needs to be `mut` because its
internal state might change; usually, we think of “reading” as not needing
mutation, but in this case we need the `mut` keyword.
-->

`handle_connection`関数において、`stream`引数を可変にしました。理由は、
`TcpStream`インスタンスが内部で返すデータを追いかけているからです。要求した以上のデータを読み取り、
次回データを要求した時のためにそのデータを保存する可能性があります。故に、内部の状態が変化する可能性があるので、
`mut`にする必要があるのです; 普通、「読み取り」に可変化は必要ないと考えてしまいますが、この場合、`mut`キーワードが必要です。

<!--
Next, we need to actually read from the stream. We do this in two steps: first,
we declare a `buffer` on the stack to hold the data that is read in. We’ve made
the buffer 1024 bytes in size, which is big enough to hold the data of a basic
request and sufficient for our purposes in this chapter. If we wanted to handle
requests of an arbitrary size, buffer management would need to be more
complicated; we’ll keep it simple for now. We pass the buffer to `stream.read`,
which will read bytes from the `TcpStream` and put them in the buffer.
-->

次に、実際にストリームから読み取る必要があります。これを 2 つの手順で行います：まず、
スタックに読み取ったデータを保持する`buffer`を宣言します。バッファーのサイズは 1024 バイトにしました。
これは、基本的なリクエストには十分な大きさでこの章の目的には必要十分です。任意のサイズのリクエストを扱いたければ、
バッファーの管理はもっと複雑にする必要があります; 今は、単純に保っておきます。このバッファーを`stream.read`に渡し、
これが`TcpStream`からバイトを読み取ってバッファーに置きます。

<!--
Second, we convert the bytes in the buffer to a string and print that string.
The `String::from_utf8_lossy` function takes a `&[u8]` and produces a `String`
from it. The “lossy” part of the name indicates the behavior of this function
when it sees an invalid UTF-8 sequence: it will replace the invalid sequence
with `�`, the `U+FFFD REPLACEMENT CHARACTER`. You might see replacement
characters for characters in the buffer that aren’t filled by request data.
-->

2 番目にバッファーのバイトを文字列に変換し、その文字列を出力します。`String::from_utf8_lossy`関数は、
`&[u8]`を取り、`String`を生成します。名前の“lossy”の箇所は、無効な UTF-8 シーケンスを目の当たりにした際のこの関数の振る舞いを示唆しています：
無効なシーケンスを`�`、`U+FFFD REPLACEMENT CHARACTER`で置き換えます。
リクエストデータによって埋められなかったバッファーの部分 (`訳注` バッファーとして 1024 バイトの領域を用意しているが、リクエストデータは 1024 バイト存在しないことがほとんどなので変数 `buffer` の後ろ部分が埋められないまま放置されることを意図していると思われる) は、置換文字が表示される場合があります。

<!--
Let’s try this code! Start the program and make a request in a web browser
again. Note that we’ll still get an error page in the browser, but our
program’s output in the terminal will now look similar to this:
-->

このコードを試しましょう！プログラムを開始して Web ブラウザで再度リクエストを送ってください。ブラウザではそれでも、
エラーページが得られるでしょうが、端末のプログラムの出力はこんな感じになっていることに注目してください：

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42 secs
     Running `target/debug/hello`
Request: GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:52.0) Gecko/20100101
Firefox/52.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1
������������������������������������
```

<!--
Depending on your browser, you might get slightly different output. Now that
we’re printing the request data, we can see why we get multiple connections
from one browser request by looking at the path after `Request: GET`. If the
repeated connections are all requesting */*, we know the browser is trying to
fetch */* repeatedly because it’s not getting a response from our program.
-->

ブラウザによって、少し異なる出力になる可能性があります。今やリクエストデータを出力しているので、
`Request: GET`の後のパスを見ることで 1 回のブラウザリクエストから複数の接続が得られる理由が確認できます。
繰り返される接続が全て */* を要求しているなら、ブラウザは、我々のプログラムからレスポンスが得られないので、
繰り返し */* をフェッチしようとしていることがわかります。

<!--
Let’s break down this request data to understand what the browser is asking of
our program.
-->

このリクエストデータを噛み砕いて、ブラウザが我々のプログラムに何を要求しているかを理解しましょう。

<!--
### A Closer Look at an HTTP Request
-->

### HTTP リクエストを詳しく見る

<!--
HTTP is a text-based protocol, and a request takes this format:
-->

HTTP はテキストベースのプロトコルで、1 つの要求はこのようなフォーマットに則っています：

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

<!--
The first line is the *request line* that holds information about what the
client is requesting. The first part of the request line indicates the *method*
being used, such as `GET` or `POST`, which describes how the client is making
this request. Our client used a `GET` request.
-->

1 行目は、クライアントが要求しているものがなんなのかについての情報を保持するリクエスト行です。
リクエスト行の最初の部分は使用されている`GET`や`POST`などの*メソッド*を示し、これは、どのようにクライアントがこの要求を行なっているかを記述します。
クライアントは`GET`リクエストを使用しました。

<!--
The next part of the request line is */*, which indicates the *Uniform Resource
Identifier* *(URI)* the client is requesting: a URI is almost, but not quite,
the same as a *Uniform Resource Locator* *(URL)*. The difference between URIs
and URLs isn’t important for our purposes in this chapter, but the HTTP spec
uses the term URI, so we can just mentally substitute URL for URI here.
-->

リクエスト行の次の部分は */* で、これはクライアントが要求している*Uniform Resource Identifier* *(URI)*(`注釈`: 統一資源識別子) を示します：
URI はほぼ*Uniform Resource Locator* *(URL)*(`注釈`: 統一資源位置指定子) と同じですが、完全に同じではありません。
URI と URL の違いは、この章の目的には重要ではありませんが、HTTP の規格は URI という用語を使用しているので、
ここでは脳内で URI を URL と読み替えられます。

<!--
The last part is the HTTP version the client uses, and then the request line
ends in a *CRLF sequence*. (CRLF stands for *carriage return* and *line feed*,
which are terms from the typewriter days!) The CRLF sequence can also be
written as `\r\n`, where `\r` is a carriage return and `\n` is a line feed. The
CRLF sequence separates the request line from the rest of the request data.
Note that when the CRLF is printed, we see a new line start rather than `\r\n`.
-->

最後の部分は、クライアントが使用している HTTP のバージョンで、それからリクエスト行は*CRLF*で終了します。
(CRLF は*carriage return*と*line feed*(無理に日本語でいえば、キャリッジ (紙を固定するシリンダー) が戻ることと行を (コンピュータに) 与えること) を表していて、
これはタイプライター時代からの用語です！)CRLF は`\r\n`とも表記され、`\r`がキャリッジ・リターンで`\n`がライン・フィードです。
CRLF により、リクエスト行がリクエストデータの残りと区別されています。CRLF を出力すると、
`\r\n`ではなく、新しい行が開始されることに注意してください。

<!--
Looking at the request line data we received from running our program so far,
we see that `GET` is the method, */* is the request URI, and `HTTP/1.1` is the
version.
-->

ここまでプログラムを実行して受け取ったリクエスト行のデータをみると、`GET`がメソッド、*/* が要求 URI、
`HTTP/1.1`がバージョンであることが確認できます。

<!--
After the request line, the remaining lines starting from `Host:` onward are
headers. `GET` requests have no body.
-->

リクエスト行の後に、`Host:`以下から始まる残りの行は、ヘッダです。`GET`リクエストには、本体（`訳注`:`message-body`のこと）がありません。

<!--
Try making a request from a different browser or asking for a different
address, such as *127.0.0.1:7878/test*, to see how the request data changes.
-->

試しに他のブラウザからリクエストを送ったり、*127.0.0.1:7878/test*などの異なるアドレスを要求してみて、どうリクエストデータが変わるか確認してください。

<!--
Now that we know what the browser is asking for, let’s send back some data!
-->

さて、ブラウザが要求しているものがわかったので、何かデータを返しましょう！

<!--
### Writing a Response
-->

### レスポンスを記述する

<!--
Now we’ll implement sending data in response to a client request. Responses
have the following format:
-->

さて、クライアントのリクエストに対する返答としてデータの送信を実装します。レスポンスは、以下のようなフォーマットです：

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

<!--
The first line is a *status line* that contains the HTTP version used in the
response, a numeric status code that summarizes the result of the request, and
a reason phrase that provides a text description of the status code. After the
CRLF sequence are any headers, another CRLF sequence, and the body of the
response.
-->

最初の行は、レスポンスで使用される HTTP バージョン、リクエストの結果を要約する数値ステータス・コード、そしてステータス・コードのテキスト記述を提供する理由句を含む *ステータス行* です。
CRLF シーケンスの後には、任意のヘッダ、別の CRLF シーケンス、そしてレスポンスの本体が続きます。

<!--
Here is an example response that uses HTTP version 1.1, has a status code of
200, an OK reason phrase, no headers, and no body:
-->

こちらが HTTP バージョン 1.1 を使用し、ステータスコードが 200 で、OK フレーズ、ヘッダと本体なしの例のレスポンスです：

```text
HTTP/1.1 200 OK\r\n\r\n
```

<!--
The status code 200 is the standard success response. The text is a tiny
successful HTTP response. Let’s write this to the stream as our response to a
successful request! From the `handle_connection` function, remove the
`println!` that was printing the request data and replace it with the code in
Listing 20-3.
-->

ステータスコード 200 は、一般的な成功のレスポンスです。テキストは、<ruby>矮小<rp>(</rp><rt>わいしょう</rt><rp>)</rp></ruby>な成功の HTTP レスポンスです。
これを成功したリクエストへの返答としてストリームに書き込みましょう！`handle_connection`関数から、
リクエストデータを出力していた`println!`を除去し、リスト 20-3 のコードと置き換えてください。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

<!--
<span class="caption">Listing 20-3: Writing a tiny successful HTTP response to
the stream</span>
-->

<span class="caption">リスト 20-3: ストリームに矮小な成功の HTTP レスポンスを書き込む</span>

<!--
The first new line defines the `response` variable that holds the success
message’s data. Then we call `as_bytes` on our `response` to convert the string
data to bytes. The `write` method on `stream` takes a `&[u8]` and sends those
bytes directly down the connection.
-->

新しい最初の行に成功したメッセージのデータを保持する`response`変数を定義しています。そして、
`response`に対して`as_bytes`を呼び出し、文字列データをバイトに変換します。`stream`の`write`メソッドは、
`&[u8]`を取り、接続に直接そのバイトを送信します。

<!--
Because the `write` operation could fail, we use `unwrap` on any error result
as before. Again, in a real application you would add error handling here.
Finally, `flush` will wait and prevent the program from continuing until all
the bytes are written to the connection; `TcpStream` contains an internal
buffer to minimize calls to the underlying operating system.
-->

`write`処理は失敗することもあるので、以前のようにエラーの結果には`unwrap`を使用します。
今回も、実際のアプリでは、エラー処理をここに追加するでしょう。最後に`flush`は待機し、
バイトが全て接続に書き込まれるまでプログラムが継続するのを防ぎます; `TcpStream`は内部にバッファーを保持して、
元となる OS への呼び出しを最小化します。

<!--
With these changes, let’s run our code and make a request. We’re no longer
printing any data to the terminal, so we won’t see any output other than the
output from Cargo. When you load *127.0.0.1:7878* in a web browser, you should
get a blank page instead of an error. You’ve just hand-coded an HTTP request
and response!
-->

これらの変更とともに、コードを実行し、リクエストをしましょう。最早、端末にどんなデータも出力していないので、
Cargo からの出力以外には何も出力はありません。Web ブラウザで*127.0.0.1:7878*をロードすると、
エラーではなく空のページが得られるはずです。HTTP リクエストとレスポンスを手で実装したばかりなのです！

<!--
### Returning Real HTML
-->

### 本物の HTML を返す

<!--
Let’s implement the functionality for returning more than a blank page. Create
a new file, *hello.html*, in the root of your project directory, not in the
*src* directory. You can input any HTML you want; Listing 20-4 shows one
possibility.
-->

空のページ以上のものを返す機能を実装しましょう。新しいファイル*hello.html*を*src*ディレクトリではなく、
プロジェクトのルートディレクトリに作成してください。お好きなように HTML を書いてください;
リスト 20-4 は、一つの可能性を示しています。

<!--
<span class="filename">Filename: hello.html</span>
-->

<span class="filename">ファイル名：hello.html</span>

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
<!--
やあ！
-->
    <h1>Hello!</h1>
<!--
Rust からやあ
-->
    <p>Hi from Rust</p>
  </body>
</html>
```

<!--
<span class="caption">Listing 20-4: A sample HTML file to return in a
response</span>
-->

<span class="caption">リスト 20-4: レスポンスで返すサンプルの HTML ファイル</span>

<!--
This is a minimal HTML5 document with a heading and some text. To return this
from the server when a request is received, we’ll modify `handle_connection` as
shown in Listing 20-5 to read the HTML file, add it to the response as a body,
and send it.
-->

これは、ヘッドとテキストのある最低限の HTML5 ドキュメントです。リクエストを受け付けた際にこれをサーバから返すには、
リスト 20-5 のように`handle_connection`を変更して HTML ファイルを読み込み、本体としてレスポンスに追加して送ります。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
use std::fs::File;
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let mut file = File::open("hello.html").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

<!--
<span class="caption">Listing 20-5: Sending the contents of *hello.html* as the
body of the response</span>
-->

<span class="caption">リスト 20-5: レスポンスの本体として*hello.html*の中身を送る</span>

<!--
We’ve added a line at the top to bring the standard library’s `File` into
scope. The code for opening a file and reading the contents should look
familiar; we used it in Chapter 12 when we read the contents of a file for our
I/O project in Listing 12-4.
-->

先頭に行を追加して標準ライブラリの`File`をスコープに導入しました。ファイルを開き、中身を読み込むコードは、
馴染みがあるはずです; リスト12-4でI/Oプロジェクト用にファイルの中身を読み込んだ時に第12章で使用しましたね。

<!--
Next, we use `format!` to add the file’s contents as the body of the success
response.
-->

次に`format!`でファイルの中身を成功したレスポンスの本体として追記しています。

<!--
Run this code with `cargo run` and load *127.0.0.1:7878* in your browser; you
should see your HTML rendered!
-->

このコードを`cargo run`で走らせ、*127.0.0.1:7878*をブラウザでロードしてください;
HTML が描画されるのが確認できるはずです！

<!--
Currently, we’re ignoring the request data in `buffer` and just sending back
the contents of the HTML file unconditionally. That means if you try requesting
*127.0.0.1:7878/something-else* in your browser, you’ll still get back this
same HTML response. Our server is very limited and is not what most web servers
do. We want to customize our responses depending on the request and only send
back the HTML file for a well-formed request to */*.
-->

現時点では、`buffer`内のリクエストデータは無視し、無条件で HTML ファイルの中身を送り返しているだけです。
これはつまり、ブラウザで*127.0.0.1:7878/something-else*をリクエストしても、
この同じ HTML レスポンスが得られるということです。我々のサーバはかなり限定的で、多くの Web サーバとは異なっています。
リクエストに基づいてレスポンスをカスタマイズし、*/* への合法なリクエストに対してのみ HTML ファイルを送り返したいです。

<!--
### Validating the Request and Selectively Responding
-->

### リクエストにバリデーションをかけ、選択的にレスポンスを返す

<!--
Right now, our web server will return the HTML in the file no matter what the
client requested. Let’s add functionality to check that the browser is
requesting */* before returning the HTML file and return an error if the
browser requests anything else. For this we need to modify `handle_connection`,
as shown in Listing 20-6. This new code checks the content of the request
received against what we know a request for */* looks like and adds `if` and
`else` blocks to treat requests differently.
-->

現状、この Web サーバはクライアントが何を要求しても、このファイルの HTML を返します。HTML ファイルを返却する前にブラウザが */* をリクエストしているか確認し、
ブラウザが他のものを要求していたらエラーを返す機能を追加しましょう。このために、
`handle_connection`をリスト 20-6 のように変更する必要があります。この新しいコードは、
*/* への要求がどんな見た目になるのか我々が知っていることに対して受け取ったリクエストの中身を検査し、`if`と`else`ブロックを追加して、
リクエストを異なる形で扱います。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let mut file = File::open("hello.html").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // 何か他の要求
        // some other request
    }
}
```

<!--
<span class="caption">Listing 20-6: Matching the request and handling requests
to */* differently than other requests</span>
-->

<span class="caption">リスト 20-6: リクエストをマッチさせ、*/* へのリクエストを他のリクエストとは異なる形で扱う</span>

<!--
First, we hardcode the data corresponding to the */* request into the `get`
variable. Because we’re reading raw bytes into the buffer, we transform `get`
into a byte string by adding the `b""` byte string syntax at the start of the
content data. Then we check whether `buffer` starts with the bytes in `get`. If
it does, it means we’ve received a well-formed request to */*, which is the
success case we’ll handle in the `if` block that returns the contents of our
HTML file.
-->

まず、*/* リクエストに対応するデータを`get`変数にハードコードしています。生のバイトをバッファーに読み込んでいるので、
`b""`バイト文字列記法を中身のデータの先頭に追記することで、`get`をバイト文字列に変換しています。
そして、`buffer`が`get`のバイトから始まっているか確認します。もしそうなら、*/* への合法なリクエストを受け取ったことを意味し、
これが、HTML ファイルの中身を返す`if`ブロックで扱う成功した場合になります。

<!--
If `buffer` does *not* start with the bytes in `get`, it means we’ve received
some other request. We’ll add code to the `else` block in a moment to respond
to all other requests.
-->

`buffer`が`get`のバイトで始まら*ない*のなら、何か他のリクエストを受け取ったことになります。
この後すぐ、`else`ブロックに他のリクエストに対応するコードを追加します。

<!--
Run this code now and request *127.0.0.1:7878*; you should get the HTML in
*hello.html*. If you make any other request, such as
*127.0.0.1:7878/something-else*, you’ll get a connection error like you
saw when running the code in Listing 20-1 and Listing 20-2.
-->

さあ、このコードを走らせて*127.0.0.1:7878*を要求してください; *hello.html*の HTML が得られるはずです。
*127.0.0.1:7878/something-else*などの他のリクエストを行うと、リスト 20-1 や 20-2 のコードを走らせた時に見かけた接続エラーになるでしょう。

<!--
Now let’s add the code in Listing 20-7 to the `else` block to return a response
with the status code 404, which signals that the content for the request was
not found. We’ll also return some HTML for a page to render in the browser
indicating the response to the end user.
-->

では、`else`ブロックにリスト 20-7 のコードを追記して、ステータスコード 404 のレスポンスを返しましょう。
これは、リクエストの中身が見つからなかったことを通知します。エンドユーザへのレスポンスを示し、ページをブラウザに描画するよう、
何か HTML も返します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
# fn handle_connection(mut stream: TcpStream) {
# if true {
// --snip--

} else {
    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let mut file = File::open("404.html").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
# }
```

<!--
<span class="caption">Listing 20-7: Responding with status code 404 and an
error page if anything other than */* was requested</span>
-->

<span class="caption">リスト 20-7: */* 以外の何かが要求されたら、ステータスコード 404 とエラーページで応答する</span>

<!--
Here, our response has a status line with status code 404 and the reason
phrase `NOT FOUND`. We’re still not returning headers, and the body of the
response will be the HTML in the file *404.html*. You’ll need to create a
*404.html* file next to *hello.html* for the error page; again feel free to use
any HTML you want or use the example HTML in Listing 20-8.
-->

ここでは、レスポンスにはステータスコード 404 と理由フレーズ`NOT FOUND`のステータス行があります。
それでもヘッダは返さず、レスポンスの本体は、ファイル*404.html*の HTML になります。エラーページのために、
*hello.html*の隣に*404.html*ファイルを作成する必要があります; 今回も、ご自由にお好きな HTML にしたり、
リスト 20-8 の例の HTML を使用したりしてください。

<!--
<span class="filename">Filename: 404.html</span>
-->

<span class="filename">ファイル名：404.html</span>

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
<!--
ああ！
-->
    <h1>Oops!</h1>
<!--
すいません。要求しているものが理解できません
-->
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>
```

<!--
<span class="caption">Listing 20-8: Sample content for the page to send back
with any 404 response</span>
-->

<span class="caption">リスト 20-8: あらゆる 404 レスポンスでページが送り返す中身のサンプル</span>

<!--
With these changes, run your server again. Requesting *127.0.0.1:7878*
should return the contents of *hello.html*, and any other request, like
*127.0.0.1:7878/foo*, should return the error HTML from *404.html*.
-->

これらの変更とともに、もう一度サーバを実行してください。*127.0.0.1:7878*を要求すると、
*hello.html*の中身が返り、*127.0.0.1:7878/foo*などの他のリクエストには*404.html*からのエラーHTML が返るはずです。

<!--
### A Touch of Refactoring
-->

### リファクタリングの触り

<!--
At the moment the `if` and `else` blocks have a lot of repetition: they’re both
reading files and writing the contents of the files to the stream. The only
differences are the status line and the filename. Let’s make the code more
concise by pulling out those differences into separate `if` and `else` lines
that will assign the values of the status line and the filename to variables;
we can then use those variables unconditionally in the code to read the file
and write the response. Listing 20-9 shows the resulting code after replacing
the large `if` and `else` blocks.
-->

現在、`if`と`else`ブロックには多くの繰り返しがあります：どちらもファイルを読み、ファイルの中身をストリームに書き込んでいます。
唯一の違いは、ステータス行とファイル名だけです。それらの差異を、ステータス行とファイル名の値を変数に代入する個別の`if`と`else`行に引っ張り出して、
コードをより簡潔にしましょう; そうしたら、それらの変数を無条件にコードで使用し、ファイルを読んでレスポンスを書き込めます。
リスト 20-9 は、大きな`if`と`else`ブロックを置き換えた後の結果のコードを示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
// --snip--

fn handle_connection(mut stream: TcpStream) {
#     let mut buffer = [0; 1024];
#     stream.read(&mut buffer).unwrap();
#
#     let get = b"GET / HTTP/1.1\r\n";
    // --snip--

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

<!--
<span class="caption">Listing 20-9: Refactoring the `if` and `else` blocks to
contain only the code that differs between the two cases</span>
-->

<span class="caption">リスト 20-9: 2 つの場合で異なるコードだけを含むように、`if`と`else`ブロックをリファクタリングする</span>

<!--
Now the `if` and `else` blocks only return the appropriate values for the
status line and filename in a tuple; we then use destructuring to assign these
two values to `status_line` and `filename` using a pattern in the `let`
statement, as discussed in Chapter 18.
-->

これで、`if`と`else`ブロックは、タプルにステータス行とファイル名の適切な値を返すだけになりました; 
それから、分配を使用してこれら 2 つの値を第 18 章で議論したように、`let`文のパターンで`status_line`と`filename`に代入しています。

<!--
The previously duplicated code is now outside the `if` and `else` blocks and
uses the `status_line` and `filename` variables. This makes it easier to see
the difference between the two cases, and it means we have only one place to
update the code if we want to change how the file reading and response writing
work. The behavior of the code in Listing 20-9 will be the same as that in
Listing 20-8.
-->

前は重複していたコードは、今では`if`と`else`ブロックの外に出て、`status_line`と`filename`変数を使用しています。
これにより、2 つの場合の違いがわかりやすくなり、ファイル読み取りとレスポンス記述の動作法を変更したくなった際に、
1 箇所だけコードを更新すればいいようになったことを意味します。リスト 20-9 のコードの振る舞いは、
リスト 20-8 と同じです。

<!--
Awesome! We now have a simple web server in approximately 40 lines of Rust code
that responds to one request with a page of content and responds to all other
requests with a 404 response.
-->

素晴らしい！もう、およそ 40 行の Rust コードで、あるリクエストには中身のあるページで応答し、
他のあらゆるリクエストには 404 レスポンスで応答する単純な Web サーバができました。

<!--
Currently, our server runs in a single thread, meaning it can only serve one
request at a time. Let’s examine how that can be a problem by simulating some
slow requests. Then we'll fix it so our server can handle multiple requests at
once.
-->

現状、このサーバは、シングルスレッドで実行されます。つまり、1 回に 1 つのリクエストしか捌けないということです。
何か遅いリクエストをシミュレーションすることで、それが問題になる可能性を調査しましょう。
それから 1 度にサーバが複数のリクエストを扱えるように修正します。
