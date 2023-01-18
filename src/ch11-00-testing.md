<!--
# Writing Automated Tests
-->

# 自動テストを書く

<!--
In his 1972 essay “The Humble Programmer,” Edsger W. Dijkstra said that
“Program testing can be a very effective way to show the presence of bugs, but
it is hopelessly inadequate for showing their absence.” That doesn’t mean we
shouldn’t try to test as much as we can!
-->

1972 年のエッセイ「謙虚なプログラマ」でエドガー・W・ダイクストラは以下のように述べています。
「プログラムのテストは、バグの存在を示すには非常に効率的な手法であるが、
バグの不在を示すには望み薄く不適切である」と。これは、できるだけテストを試みるべきではないということではありません。

<!--
Correctness in our programs is the extent to which our code does what we intend
it to do. Rust is designed with a high degree of concern about the correctness
of programs, but correctness is complex and not easy to prove. Rust’s type
system shoulders a huge part of this burden, but the type system cannot catch
every kind of incorrectness. As such, Rust includes support for writing
automated software tests within the language.
-->

プログラムの正当性は、どこまで自分のコードが意図していることをしているかなのです。
Rust は、プログラムの正当性に重きを置いて設計されていますが、
正当性は複雑で、単純に証明することはありません。Rust の型システムは、
この重荷の多くの部分を肩代わりしてくれますが、型システムはあらゆる種類の不当性を捕捉してはくれません。
ゆえに、Rust では、言語内で自動化されたソフトウェアテストを書くことをサポートしているのです。

<!--
As an example, say we write a function called `add_two` that adds 2 to whatever
number is passed to it. This function’s signature accepts an integer as a
parameter and returns an integer as a result. When we implement and compile
that function, Rust does all the type checking and borrow checking that you've
learned so far to ensure that, for instance, we aren’t passing a `String` value
or an invalid reference to this function. But Rust *can’t* check that this
function will do precisely what we intend, which is return the parameter plus 2
rather than, say, the parameter plus 10 or the parameter minus 50! That's where
tests come in.
-->

例として、渡された何かの数値に 2 を足す`add_two`という関数を書くとしましょう。
この関数のシグニチャは、引数に整数を取り、結果として整数を返します。
この関数を実装してコンパイルすると、コンパイラはこれまでに学んできた型チェックと借用チェックを全て行い、
例えば、`String`の値や無効な参照をこの関数に渡していないかなどを確かめるのです。
ところが、コンパイラはプログラマがまさしく意図したことを関数が実行しているかどうかは確かめ*られません*。
つまり、そうですね、引数に 10 を足したり、50 を引いたりするのではなく、引数に 2 を足していることです。
そんな時に、テストは必要になるのです。

<!--
We can write tests that assert, for example, that when we pass `3` to the
`add_two` function, the returned value is `5`. We can run these tests whenever
we make changes to our code to make sure any existing correct behavior has not
changed.
-->

例えば、`add_two`関数に`3`を渡した時に、戻り値は 5 であることをアサーションするようなテストを書くことができます。
コードに変更を加えた際にこれらのテストを走らせ、既存の正当な振る舞いが変わっていないことを確認できます。

<!--
Testing is a complex skill: although we can’t cover every detail about how to
write good tests in one chapter, we’ll discuss the mechanics of Rust’s testing
facilities. We’ll talk about the annotations and macros available to you when
writing your tests, the default behavior and options provided for running your
tests, and how to organize tests into unit tests and integration tests.
-->

テストは、複雑なスキルです：いいテストの書き方をあらゆる方面から講義することは 1 章だけではできないのですが、
Rust のテスト機構のメカニズムについて議論します。テストを書く際に利用可能になるアノテーションとマクロについて、
テストを実行するのに提供されているオプションと標準の動作、さらにテストをユニットテストや統合テストに体系化する方法について語ります。
