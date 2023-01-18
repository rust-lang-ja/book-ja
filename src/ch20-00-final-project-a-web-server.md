<!--
# Final Project: Building a Multithreaded Web Server
-->

# 最後のプロジェクト：マルチスレッドの Web サーバを構築する

<!--
It’s been a long journey, but we’ve reached the end of the book. In this
chapter, we’ll build one more project together to demonstrate some of the
concepts we covered in the final chapters, as well as recap some earlier
lessons.
-->

長い旅でしたが、本の末端に到達しました。この章では、共にもう一つプロジェクトを構築して最後の方の章で講義した概念の一部をデモしつつ、
それより前の方で学習した内容を思い出してもらいます。

<!--
For our final project, we’ll make a web server that says “hello” and looks like
Figure 20-1 in a web browser.
-->

最後のプロジェクトでは、`hello`と話す Web サーバを作り、Web ブラウザでは、図 20-1 のような見た目になります。

![hello from rust](img/trpl20-01.png)

<!--
<span class="caption">Figure 20-1: Our final shared project</span>
-->

<span class="caption">図 20-1: 最後の共有されたプロジェクト</span>

<!--
Here is the plan to build the web server:
-->

こちらが Web サーバを構築するプランです：

<!--
1. Learn a bit about TCP and HTTP.
2. Listen for TCP connections on a socket.
3. Parse a small number of HTTP requests.
4. Create a proper HTTP response.
5. Improve the throughput of our server with a thread pool.
-->

1. TCP と HTTP について少し学ぶ。
2. ソケットで TCP 接続をリッスンする。
3. 少量の HTTP リクエストを構文解析する。
4. 適切な HTTP レスポンスを生成する。
5. スレッドプールでサーバのスループットを強化する。

<!--
But before we get started, we should mention one detail: the method we’ll use
won’t be the best way to build a web server with Rust. A number of
production-ready crates are available on *https://crates.io/* that provide more
complete web server and thread pool implementations than we’ll build.
-->

ですが、取り掛かる前に、ある小さな事実に触れなければなりません：
わたしたちがこれから行うやり方は、Rust で Web サーバを構築する最善の方法ではないだろうということです。
これから構築するよりもより完全な Web サーバとスレッドプールの実装を提供する製品利用可能な多くのクレートが、
*https://crates.io/* で利用可能なのです。

<!--
However, our intention in this chapter is to help you learn, not to take the
easy route. Because Rust is a systems programming language, we can choose the
level of abstraction we want to work with and can go to a lower level than is
possible or practical in other languages. We’ll write the basic HTTP server and
thread pool manually so you can learn the general ideas and techniques behind
the crates you might use in the future.
-->

しかしながら、この章での意図は、学習を手助けすることであり、簡単なやり方を選ぶことではありません。
Rust はシステムプログラミング言語なので、取りかかる抽象度を選ぶことができ、
他の言語で可能だったり実践的だったりするよりも低レベルまで行くことができます。一般的な考えと将来使う可能性のあるクレートの背後にある技術を学べるように、
手動で基本的な HTTP サーバとスレッドプールを書きます。
