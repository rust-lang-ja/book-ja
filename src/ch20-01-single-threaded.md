<!--
## Building a Single-Threaded Web Server
-->

## シングルスレッドのWebサーバを構築する

<!--
We’ll start by getting a single-threaded web server working. Before we begin,
let’s look at a quick overview of the protocols involved in building web
servers. The details of these protocols are beyond the scope of this book, but
a brief overview will give you the information you need.
-->

シングルスレッドのWebサーバを動かすところから始めます。始める前に、Webサーバ構築に関係するプロトコルをさっと一覧しましょう。
これらのプロトコルの詳細は、この本の範疇を超えていますが、さっと眺めることで必要な情報が得られるでしょう。

<!--
The two main protocols involved in web servers are *Hypertext Transfer
Protocol* *(HTTP)* and *Transmission Control Protocol* *(TCP)*. Both protocols
are *request-response* protocols, meaning a *client* initiates requests and a
*server* listens to the requests and provides a response to the client. The
contents of those requests and responses are defined by the protocols.
-->

主に2つのプロトコルがWebサーバに関係し、*Hypertext Transfer Protocol* *(HTTP)*(`注釈`: ハイパーテキスト転送プロトコル)と、
*Transmission Control Protocol* *(TCP)*(`注釈`: 伝送制御プロトコル)です。
両者のプロトコルは、*リクエスト・レスポンス*プロトコルであり、つまり、*クライアント*がリクエスト(要求)を初期化し、
*サーバ*はリクエストをリッスンし、クライアントにレスポンス(応答)を提供するということです。
それらのリクエストとレスポンスの中身は、プロトコルで規定されています。

<!--
TCP is the lower-level protocol that describes the details of how information
gets from one server to another but doesn’t specify what that information is.
HTTP builds on top of TCP by defining the contents of the requests and
responses. It’s technically possible to use HTTP with other protocols, but in
the vast majority of cases, HTTP sends its data over TCP. We’ll work with the
raw bytes of TCP and HTTP requests and responses.
-->

TCPは、情報がとあるサーバから別のサーバへどう到達するかの詳細を記述するものの、その情報がなんなのかは指定しない、
より低レベルのプロトコルです。HTTPはリクエストとレスポンスの中身を定義することでTCPの上に成り立っています。
技術的にはHTTPを他のプロトコルとともに使用することができますが、大抵の場合、HTTPはTCPの上にデータを送信します。
TCPとHTTPのリクエストとレスポンスの生のバイトを取り扱います。

<!--
### Listening to the TCP Connection
-->

### TCP接続をリッスンする

<!--
Our web server needs to listen to a TCP connection, so that’s the first part
we’ll work on. The standard library offers a `std::net` module that lets us do
this. Let’s make a new project in the usual fashion:
-->

WebサーバはTCP接続をリッスンするので、そこが最初に取り掛かる部分になります。標準ライブラリは、
`std::net`というこれを行うモジュールを用意しています。通常通り、新しいプロジェクトを作りましょう:

```console
$ cargo new hello
     Created binary (application) `hello` project
$ cd hello
```

<!--
Now enter the code in Listing 20-1 in *src/main.rs* to start. This code will
listen at the local address `127.0.0.1:7878` for incoming TCP streams. When it
gets an incoming stream, it will print `Connection established!`.
-->

さて、リスト20-1のコードを*src/main.rs*に入力して始めてください。このコードは、
TCPストリームを受信するため`127.0.0.1:7878`というローカルアドレスをリッスンします。
入力ストリームを得ると、`Connection established!`と出力します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-01/src/main.rs}}
```

<!--
<span class="caption">Listing 20-1: Listening for incoming streams and printing
a message when we receive a stream</span>
-->

<span class="caption">リスト20-1: 入力ストリームをリッスンし、ストリームを受け付けた時にメッセージを出力する</span>

<!--
Using `TcpListener`, we can listen for TCP connections at the address
`127.0.0.1:7878`. In the address, the section before the colon is an IP address
representing your computer (this is the same on every computer and doesn’t
represent the authors’ computer specifically), and `7878` is the port. We’ve
chosen this port for two reasons: HTTP isn’t normally accepted on this port so
our server is unlikely to conflict with any other web server you might have
running on your machine, and 7878 is *rust* typed on a telephone.
-->

`TcpListener`により、アドレス`127.0.0.1:7878`でTCP接続をリッスンできます。アドレス内で、
コロンの前の区域は、自分のコンピュータを表すIPアドレスで(これはどんなコンピュータでも同じで、
特に著者のコンピュータを表すわけではありません)、`7878`はポートです。このポートを選択した理由は2つあります:
HTTPは通常このポートで受け付けられることがないので、私たちのサーバはおそらく、
あなたのマシンで実行中かもしれない他のwebサーバと混線しないだろうということと、
7878は電話で*rust*と入力されるからです。

<!--
The `bind` function in this scenario works like the `new` function in that it
will return a new `TcpListener` instance. The function is called `bind`
because, in networking, connecting to a port to listen to is known as “binding
to a port.”
-->

この筋書きでの`bind`関数は、新しい`TcpListener`インスタンスを返すという点で`new`関数のような働きをします。
この関数は`bind`と呼ばれます。ネットワークにおいて、リッスンすべきポートに接続することは、
「ポートに束縛する」(binding to a port)こととして知られているからです。

<!--
The `bind` function returns a `Result<T, E>`, which indicates that it’s
possible for binding to fail. For example, connecting to port 80 requires
administrator privileges (nonadministrators can listen only on ports higher
than 1023), so if we tried to connect to port 80 without being an
administrator, binding wouldn’t work. Binding also wouldn’t work, for example,
if we ran two instances of our program and so had two programs listening to the
same port. Because we’re writing a basic server just for learning purposes, we
won’t worry about handling these kinds of errors; instead, we use `unwrap` to
stop the program if errors happen.
-->

`bind`関数は`Result<T, E>`を返し、束縛が失敗することもあることを示しています。例えば、
ポート80に接続するには管理者権限が必要なので(管理者以外は1023より大きい番号のポートしかリッスンできません)管理者にならずにポート80に接続を試みたら、
束縛はうまくいかないでしょう。また例えば、自分のプログラムを2つ同時に立ち上げて2つのプログラムが同じポートをリッスンする場合も、
束縛は機能しないでしょう。学習目的のためだけに基本的なサーバを記述しているので、この種のエラーを扱う心配はしません;
その代わり、`unwrap`を使用してエラーが発生したら、プログラムを停止します。

<!--
The `incoming` method on `TcpListener` returns an iterator that gives us a
sequence of streams (more specifically, streams of type `TcpStream`). A single
*stream* represents an open connection between the client and the server. A
*connection* is the name for the full request and response process in which a
client connects to the server, the server generates a response, and the server
closes the connection. As such, we will read from the `TcpStream` to see what
the client sent and then write our response to the stream to send data back to
the client. Overall, this `for` loop will process each connection in turn and
produce a series of streams for us to handle.
-->

`TcpListener`の`incoming`メソッドは、一連のストリームを与えるイテレータを返します(具体的には、型`TcpStream`のストリーム)。
単独の*ストリーム*がクライアント・サーバ間の開かれた接続を表します。*接続*(connection)は、
クライアントがサーバに接続し、サーバがレスポンスを生成し、サーバが接続を閉じるというリクエストとレスポンス全体の過程の名前です。
そのため、`TcpStream`から読み取ってクライアントが送信した内容を確認し、
それからレスポンスをストリームに書き込んでクライアントにデータを返信します。
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
そのうちの多くは、OS特有です。例を挙げれば、多くのOSには、サポートできる同時に開いた接続数に上限があります;
開かれた接続の一部が閉じられるまでその数字を超えた接続の試行はエラーになります。

<!--
Let’s try running this code! Invoke `cargo run` in the terminal and then load
*127.0.0.1:7878* in a web browser. The browser should show an error message
like “Connection reset,” because the server isn’t currently sending back any
data. But when you look at your terminal, you should see several messages that
were printed when the browser connected to the server!
-->

このコードを試しに実行してみましょう！端末で`cargo run`を呼び出し、それからWebブラウザで*127.0.0.1:7878*をロードしてください。
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

時々、1回のブラウザリクエストで複数のメッセージが出力されるのを目の当たりにするでしょう;
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
重要な要素は、TCP接続へのハンドルを得ることに成功したということです！

<!--
Remember to stop the program by pressing <span class="keystroke">ctrl-c</span>
when you’re done running a particular version of the code. Then restart the
program by invoking the `cargo run` command after you’ve made each set of code
changes to make sure you’re running the newest code.
-->

特定のバージョンのコードを走らせ終わった時に<span class="keystroke">ctrl-c</span>を押して、
プログラムを止めることを忘れないでください。そして、一連のコード変更を行った後に、
`cargo run`コマンドを実行することでプログラムを再起動し、
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
接続を処理する新しい関数を開始します。この新しい`handle_connection`関数において、TCPストリームからデータを読み取り、
ブラウザからデータが送られていることを確認できるように端末に出力します。コードをリスト20-2のように変更してください。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-02/src/main.rs}}
```

<!--
<span class="caption">Listing 20-2: Reading from the `TcpStream` and printing
the data</span>
-->

<span class="caption">リスト20-2: `TcpStream`から読み取り、データを出力する</span>

<!--
We bring `std::io::prelude` and `std::io::BufReader` into scope to get access
to traits and types that let us read from and write to the stream. In the `for`
loop in the `main` function, instead of printing a message that says we made a
connection, we now call the new `handle_connection` function and pass the
`stream` to it.
-->

`std::io::prelude`と`std::io::BufReader`をスコープに導入して、
ストリームから読み書きさせてくれるトレイトと型にアクセスできるようにしています。
`main`関数内の`for`ループで、接続を確立したというメッセージを出力する代わりに、今では、
新しい`handle_connection`関数を呼び出し、`stream`を渡しています。

<!--
In the `handle_connection` function, we create a new `BufReader` instance that
wraps a mutable reference to the `stream`. `BufReader` adds buffering by
managing calls to the `std::io::Read` trait methods for us.
-->

`handle_connection`関数では、`stream`への可変参照をラップする、新しい`BufReader`インスタンスを作成しています。
`BufReader`は、`std::io::Read`トレイトメソッドへの呼び出しを管理することで、バッファリング機能を追加します。

<!--
We create a variable named `http_request` to collect the lines of the request
the browser sends to our server. We indicate that we want to collect these
lines in a vector by adding the `Vec<_>` type annotation.
-->

ブラウザがサーバに送信したリクエストの各行を集めるために、`http_request`という名前の変数を作成しています。
`Vec<_>`型注釈を追加することで、これらの行をベクタに集めたいということを示します。

<!--
`BufReader` implements the `std::io::BufRead` trait, which provides the `lines`
method. The `lines` method returns an iterator of `Result<String,
std::io::Error>` by splitting the stream of data whenever it sees a newline
byte. To get each `String`, we map and `unwrap` each `Result`. The `Result`
might be an error if the data isn’t valid UTF-8 or if there was a problem
reading from the stream. Again, a production program should handle these errors
more gracefully, but we’re choosing to stop the program in the error case for
simplicity.
-->

`BufReader`は`std::io::BufRead`トレイトを実装しており、これは`lines`メソッドを提供しています。
`lines`メソッドは、改行バイトを見つけたらそこでデータのストリームを分割することで、
`Result<String, std::io::Error>`からなるイテレータを返します。
各`String`を得るために、`map`して各`Result`を`unwrap`します。
データが妥当なUTF-8でない場合や、ストリームからの読み込みに問題があった場合などは、
`Result`はエラーになる場合があります。繰り返しになりますが、
実運用のプログラムではこれらのエラーをより丁寧に処理するべきですが、
ここでは簡潔性のために、エラーの場合にはプログラムを停止することを選択しています。

<!--
The browser signals the end of an HTTP request by sending two newline
characters in a row, so to get one request from the stream, we take lines until
we get a line that is the empty string. Once we’ve collected the lines into the
vector, we’re printing them out using pretty debug formatting so we can take a
look at the instructions the web browser is sending to our server.
-->

ブラウザは2個の改行文字列を連続で送信することでHTTPリクエストの終わりを伝えるてくるので、
ストリームから1リクエストを取得するためには、空文字列である行が得られるまで行を取得します。
行をベクタに集めたら、それらをprettyデバッグフォーマットを使用して出力していますので、
webブラウザが私たちのサーバに送信している指示を確認することができます。

<!--
Let’s try this code! Start the program and make a request in a web browser
again. Note that we’ll still get an error page in the browser, but our
program’s output in the terminal will now look similar to this:
-->

このコードを試しましょう！プログラムを開始してWebブラウザで再度リクエストを送ってください。ブラウザではそれでも、
エラーページが得られるでしょうが、端末のプログラムの出力はこんな感じになっていることに注目してください:

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello`
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]
```

<!--
Depending on your browser, you might get slightly different output. Now that
we’re printing the request data, we can see why we get multiple connections
from one browser request by looking at the path after `GET` in the first line
of the request. If the repeated connections are all requesting */*, we know the
browser is trying to fetch */* repeatedly because it’s not getting a response
from our program.
-->

ブラウザによって、少し異なる出力になる可能性があります。今やリクエストデータを出力しているので、
リクエストの最初の行の`GET`の後のパスを見ることで1回のブラウザリクエストから複数の接続が得られる理由が確認できます。
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

### HTTPリクエストを詳しく見る

<!--
HTTP is a text-based protocol, and a request takes this format:
-->

HTTPはテキストベースのプロトコルで、1つの要求はこのようなフォーマットに則っています:

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

<!--
The first line is the *request line* that holds information about what the
client is requesting. The first part of the request line indicates the *method*
being used, such as `GET` or `POST`, which describes how the client is making
this request. Our client used a `GET` request, which means it is asking for
information.
-->

1行目は、クライアントが要求しているものがなんなのかについての情報を保持するリクエスト行です。
リクエスト行の最初の部分は使用されている`GET`や`POST`などの*メソッド*を示し、これは、どのようにクライアントがこの要求を行なっているかを記述します。
クライアントは`GET`リクエストを使用しており、これは情報を求めていることを意味します。

<!--
The next part of the request line is */*, which indicates the *Uniform Resource
Identifier* *(URI)* the client is requesting: a URI is almost, but not quite,
the same as a *Uniform Resource Locator* *(URL)*. The difference between URIs
and URLs isn’t important for our purposes in this chapter, but the HTTP spec
uses the term URI, so we can just mentally substitute URL for URI here.
-->

リクエスト行の次の部分は */* で、これはクライアントが要求している*Uniform Resource Identifier* *(URI)*(`注釈`: 統一資源識別子)を示します:
URIはほぼ*Uniform Resource Locator* *(URL)*(`注釈`: 統一資源位置指定子)と同じですが、完全に同じではありません。
URIとURLの違いは、この章の目的には重要ではありませんが、HTTPの規格はURIという用語を使用しているので、
ここでは脳内でURIをURLと読み替えられます。

<!--
The last part is the HTTP version the client uses, and then the request line
ends in a *CRLF sequence*. (CRLF stands for *carriage return* and *line feed*,
which are terms from the typewriter days!) The CRLF sequence can also be
written as `\r\n`, where `\r` is a carriage return and `\n` is a line feed. The
CRLF sequence separates the request line from the rest of the request data.
Note that when the CRLF is printed, we see a new line start rather than `\r\n`.
-->

最後の部分は、クライアントが使用しているHTTPのバージョンで、それからリクエスト行は*CRLF*で終了します。
(CRLFは*carriage return*と*line feed*(無理に日本語でいえば、キャリッジ(紙を固定するシリンダー)が戻ることと行を(コンピュータに)与えること)を表していて、
これはタイプライター時代からの用語です！)CRLFは`\r\n`とも表記され、`\r`がキャリッジ・リターンで`\n`がライン・フィードです。
CRLFにより、リクエスト行がリクエストデータの残りと区別されています。CRLFを出力すると、
`\r\n`ではなく、新しい行が開始されることに注意してください。

<!--
Looking at the request line data we received from running our program so far,
we see that `GET` is the method, */* is the request URI, and `HTTP/1.1` is the
version.
-->

ここまでプログラムを実行して受け取ったリクエスト行のデータをみると、`GET`がメソッド、*/* が要求URI、
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
We’re going to implement sending data in response to a client request.
Responses have the following format:
-->

クライアントのリクエストに対する返答としてデータの送信を実装します。
レスポンスは、以下のようなフォーマットです:

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

最初の行は、レスポンスで使用されるHTTPバージョン、リクエストの結果を要約する数値ステータス・コード、そしてステータス・コードのテキスト記述を提供する理由句を含む *ステータス行* です。
CRLFシーケンスの後には、任意のヘッダ、別のCRLFシーケンス、そしてレスポンスの本体が続きます。

<!--
Here is an example response that uses HTTP version 1.1, has a status code of
200, an OK reason phrase, no headers, and no body:
-->

こちらがHTTPバージョン1.1を使用し、ステータスコードが200で、OKフレーズ、ヘッダと本体なしの例のレスポンスです:

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

ステータスコード200は、一般的な成功のレスポンスです。テキストは、<ruby>矮小<rp>(</rp><rt>わいしょう</rt><rp>)</rp></ruby>な成功のHTTPレスポンスです。
これを成功したリクエストへの返答としてストリームに書き込みましょう！`handle_connection`関数から、
リクエストデータを出力していた`println!`を除去し、リスト20-3のコードと置き換えてください。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-03/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-3: Writing a tiny successful HTTP response to
the stream</span>
-->

<span class="caption">リスト20-3: ストリームに矮小な成功のHTTPレスポンスを書き込む</span>

<!--
The first new line defines the `response` variable that holds the success
message’s data. Then we call `as_bytes` on our `response` to convert the string
data to bytes. The `write_all` method on `stream` takes a `&[u8]` and sends
those bytes directly down the connection. Because the `write_all` operation
could fail, we use `unwrap` on any error result as before. Again, in a real
application you would add error handling here.
-->

新しい最初の行に成功したメッセージのデータを保持する`response`変数を定義しています。そして、
`response`に対して`as_bytes`を呼び出し、文字列データをバイトに変換します。`stream`の`write_all`メソッドは、
`&[u8]`を取り、接続に直接そのバイトを送信します。
`write_all`処理は失敗することもあるので、以前のようにエラーの結果には`unwrap`を使用します。
今回も、実際のアプリでは、エラー処理をここに追加するでしょう。

<!--
With these changes, let’s run our code and make a request. We’re no longer
printing any data to the terminal, so we won’t see any output other than the
output from Cargo. When you load *127.0.0.1:7878* in a web browser, you should
get a blank page instead of an error. You’ve just hand-coded receiving an HTTP
request and sending a response!
-->

これらの変更とともに、コードを実行し、リクエストをしましょう。最早、端末にどんなデータも出力していないので、
Cargoからの出力以外には何も出力はありません。Webブラウザで*127.0.0.1:7878*をロードすると、
エラーではなく空のページが得られるはずです。
HTTPリクエストの受信とレスポンスの送信を手で実装することができました！

<!--
### Returning Real HTML
-->

### 本物のHTMLを返す

<!--
Let’s implement the functionality for returning more than a blank page. Create
the new file *hello.html* in the root of your project directory, not in the
*src* directory. You can input any HTML you want; Listing 20-4 shows one
possibility.
-->

空のページ以上のものを返す機能を実装しましょう。新しいファイル*hello.html*を*src*ディレクトリではなく、
プロジェクトのルートディレクトリに作成してください。お好きなようにHTMLを書いてください;
リスト20-4は、一つの可能性を示しています。

<!--
<span class="filename">Filename: hello.html</span>
-->

<span class="filename">ファイル名: hello.html</span>

```html
{{#include ../listings/ch20-web-server/listing-20-05/hello.html}}
```

<!--
<span class="caption">Listing 20-4: A sample HTML file to return in a
response</span>
-->

<span class="caption">リスト20-4: レスポンスで返すサンプルのHTMLファイル</span>

<!--
This is a minimal HTML5 document with a heading and some text. To return this
from the server when a request is received, we’ll modify `handle_connection` as
shown in Listing 20-5 to read the HTML file, add it to the response as a body,
and send it.
-->

これは、ヘッドとテキストのある最低限のHTML5ドキュメントです。リクエストを受け付けた際にこれをサーバから返すには、
リスト20-5のように`handle_connection`を変更してHTMLファイルを読み込み、本体としてレスポンスに追加して送ります。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-05/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-5: Sending the contents of *hello.html* as the
body of the response</span>
-->

<span class="caption">リスト20-5: レスポンスの本体として*hello.html*の中身を送る</span>

<!--
We’ve added `fs` to the `use` statement to bring the standard library’s
filesystem module into scope. The code for reading the contents of a file to a
string should look familiar; we used it in Chapter 12 when we read the contents
of a file for our I/O project in Listing 12-4.
-->

`use`文に`fs`を追加して、標準ライブラリのファイルシステムモジュールをスコープに導入しました。
ファイルの中身を文字列として読み込むコードは、馴染みがあるはずです;
リスト12-4でI/Oプロジェクト用にファイルの中身を読み込んだ時に第12章で使用しましたね。

<!--
Next, we use `format!` to add the file’s contents as the body of the success
response. To ensure a valid HTTP response, we add the `Content-Length` header
which is set to the size of our response body, in this case the size of
`hello.html`.
-->

次に`format!`でファイルの中身を成功したレスポンスの本体として追記しています。
妥当なHTTPレスポンスであることを保証するために、`Content-Length`ヘッダを追加し、
私たちのレスポンスボディのサイズに設定します。この場合は`hello.html`のサイズです。

<!--
Run this code with `cargo run` and load *127.0.0.1:7878* in your browser; you
should see your HTML rendered!
-->

このコードを`cargo run`で走らせ、*127.0.0.1:7878*をブラウザでロードしてください;
HTMLが描画されるのが確認できるはずです！

<!--
Currently, we’re ignoring the request data in `http_request` and just sending
back the contents of the HTML file unconditionally. That means if you try
requesting *127.0.0.1:7878/something-else* in your browser, you’ll still get
back this same HTML response. At the moment, our server is very limited and
does not do what most web servers do. We want to customize our responses
depending on the request and only send back the HTML file for a well-formed
request to */*.
-->

現時点では、`http_request`内のリクエストデータは無視し、無条件でHTMLファイルの中身を送り返しているだけです。
これはつまり、ブラウザで*127.0.0.1:7878/something-else*をリクエストしても、
この同じHTMLレスポンスが得られるということです。現時点では、我々のサーバはかなり限定的で、
多くのWebサーバが行うことを行っていません。
リクエストに基づいてレスポンスをカスタマイズし、*/* への合法なリクエストに対してのみHTMLファイルを送り返したいです。

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

現状、このWebサーバはクライアントが何を要求しても、このファイルのHTMLを返します。HTMLファイルを返却する前にブラウザが */* をリクエストしているか確認し、
ブラウザが他のものを要求していたらエラーを返す機能を追加しましょう。このために、
`handle_connection`をリスト20-6のように変更する必要があります。この新しいコードは、
*/* への要求がどんな見た目になるのか我々が知っていることに対して受け取ったリクエストの中身を検査し、`if`と`else`ブロックを追加して、
リクエストを異なる形で扱います。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-06/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-6: Handling requests to */* differently from
other requests</span>
-->

<span class="caption">リスト20-6: */* へのリクエストを他のリクエストとは異なる形で扱う</span>

<!--
We’re only going to be looking at the first line of the HTTP request, so rather
than reading the entire request into a vector, we’re calling `next` to get the
first item from the iterator. The first `unwrap` takes care of the `Option` and
stops the program if the iterator has no items. The second `unwrap` handles the
`Result` and has the same effect as the `unwrap` that was in the `map` added in
Listing 20-2.
-->

HTTPリクエストの最初の行しか見ないので、リクエスト全体をベクタに読むのではなく、
`next`を呼んでイテレータから最初の要素を取得しています。
最初の`unwrap`は`Option`を処理するもので、イテレータに要素が無い場合はプログラムを停止します。
2番目の`unwrap`は`Result`を処理するもので、リスト20-2で追加された`map`の中にあった`unwrap`と同じ効果を持ちます。

<!--
Next, we check the `request_line` to see if it equals the request line of a GET
request to the */* path. If it does, the `if` block returns the contents of our
HTML file.
-->

次に、`request_line`を確認して、*/* パスへのGETリクエストのリクエスト行と等しいか確認します。
もしそうなら、`if`ブロックはHTMLファイルの中身を返します。

<!--
If the `request_line` does *not* equal the GET request to the */* path, it
means we’ve received some other request. We’ll add code to the `else` block in
a moment to respond to all other requests.
-->

`request_line`が */* パスへのGETリクエストと等しく*ない*のなら、何か他のリクエストを受け取ったことになります。
この後すぐ、`else`ブロックに他のリクエストに対応するコードを追加します。

<!--
Run this code now and request *127.0.0.1:7878*; you should get the HTML in
*hello.html*. If you make any other request, such as
*127.0.0.1:7878/something-else*, you’ll get a connection error like those you
saw when running the code in Listing 20-1 and Listing 20-2.
-->

さあ、このコードを走らせて*127.0.0.1:7878*を要求してください; *hello.html*のHTMLが得られるはずです。
*127.0.0.1:7878/something-else*などの他のリクエストを行うと、リスト20-1や20-2のコードを走らせた時に見かけたような接続エラーになるでしょう。

<!--
Now let’s add the code in Listing 20-7 to the `else` block to return a response
with the status code 404, which signals that the content for the request was
not found. We’ll also return some HTML for a page to render in the browser
indicating the response to the end user.
-->

では、`else`ブロックにリスト20-7のコードを追記して、ステータスコード404のレスポンスを返しましょう。
これは、リクエストの中身が見つからなかったことを通知します。エンドユーザへのレスポンスを示し、ページをブラウザに描画するよう、
何かHTMLも返します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-07/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-7: Responding with status code 404 and an
error page if anything other than */* was requested</span>
-->

<span class="caption">リスト20-7: */* 以外の何かが要求されたら、ステータスコード404とエラーページで応答する</span>

<!--
Here, our response has a status line with status code 404 and the reason phrase
`NOT FOUND`. The body of the response will be the HTML in the file *404.html*.
You’ll need to create a *404.html* file next to *hello.html* for the error
page; again feel free to use any HTML you want or use the example HTML in
Listing 20-8.
-->

ここでは、レスポンスにはステータスコード404と理由フレーズ`NOT FOUND`のステータス行があります。
レスポンスの本体は、ファイル*404.html*のHTMLになります。エラーページのために、
*hello.html*の隣に*404.html*ファイルを作成する必要があります; 今回も、ご自由にお好きなHTMLにしたり、
リスト20-8の例のHTMLを使用したりしてください。

<!--
<span class="filename">Filename: 404.html</span>
-->

<span class="filename">ファイル名: 404.html</span>

```html
{{#include ../listings/ch20-web-server/listing-20-07/404.html}}
```

<!--
<span class="caption">Listing 20-8: Sample content for the page to send back
with any 404 response</span>
-->

<span class="caption">リスト20-8: あらゆる404レスポンスでページが送り返す中身のサンプル</span>

<!--
With these changes, run your server again. Requesting *127.0.0.1:7878* should
return the contents of *hello.html*, and any other request, like
*127.0.0.1:7878/foo*, should return the error HTML from *404.html*.
-->

これらの変更とともに、もう一度サーバを実行してください。*127.0.0.1:7878*を要求すると、
*hello.html*の中身が返り、*127.0.0.1:7878/foo*などの他のリクエストには*404.html*からのエラーHTMLが返るはずです。

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

現在、`if`と`else`ブロックには多くの繰り返しがあります: どちらもファイルを読み、ファイルの中身をストリームに書き込んでいます。
唯一の違いは、ステータス行とファイル名だけです。それらの差異を、ステータス行とファイル名の値を変数に代入する個別の`if`と`else`行に引っ張り出して、
コードをより簡潔にしましょう; そうしたら、それらの変数を無条件にコードで使用し、ファイルを読んでレスポンスを書き込めます。
リスト20-9は、大きな`if`と`else`ブロックを置き換えた後の結果のコードを示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-09/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-9: Refactoring the `if` and `else` blocks to
contain only the code that differs between the two cases</span>
-->

<span class="caption">リスト20-9: 2つの場合で異なるコードだけを含むように、`if`と`else`ブロックをリファクタリングする</span>

<!--
Now the `if` and `else` blocks only return the appropriate values for the
status line and filename in a tuple; we then use destructuring to assign these
two values to `status_line` and `filename` using a pattern in the `let`
statement, as discussed in Chapter 18.
-->

これで、`if`と`else`ブロックは、タプルにステータス行とファイル名の適切な値を返すだけになりました; 
それから、分配を使用してこれら2つの値を第18章で議論したように、`let`文のパターンで`status_line`と`filename`に代入しています。

<!--
The previously duplicated code is now outside the `if` and `else` blocks and
uses the `status_line` and `filename` variables. This makes it easier to see
the difference between the two cases, and it means we have only one place to
update the code if we want to change how the file reading and response writing
work. The behavior of the code in Listing 20-9 will be the same as that in
Listing 20-8.
-->

前は重複していたコードは、今では`if`と`else`ブロックの外に出て、`status_line`と`filename`変数を使用しています。
これにより、2つの場合の違いがわかりやすくなり、ファイル読み取りとレスポンス記述の動作法を変更したくなった際に、
1箇所だけコードを更新すればいいようになったことを意味します。リスト20-9のコードの振る舞いは、
リスト20-8と同じです。

<!--
Awesome! We now have a simple web server in approximately 40 lines of Rust code
that responds to one request with a page of content and responds to all other
requests with a 404 response.
-->

素晴らしい！もう、およそ40行のRustコードで、あるリクエストには中身のあるページで応答し、
他のあらゆるリクエストには404レスポンスで応答する単純なWebサーバができました。

<!--
Currently, our server runs in a single thread, meaning it can only serve one
request at a time. Let’s examine how that can be a problem by simulating some
slow requests. Then we’ll fix it so our server can handle multiple requests at
once.
-->

現状、このサーバは、シングルスレッドで実行されます。つまり、1回に1つのリクエストしか捌けないということです。
何か遅いリクエストをシミュレーションすることで、それが問題になる可能性を調査しましょう。
それから1度にサーバが複数のリクエストを扱えるように修正します。
