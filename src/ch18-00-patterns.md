<!--
# Patterns and Matching
-->

# パターンとマッチング

<!--
*Patterns* are a special syntax in Rust for matching against the structure of
types, both complex and simple. Using patterns in conjunction with `match`
expressions and other constructs gives you more control over a program’s
control flow. A pattern consists of some combination of the following:
-->

*パターン*は、型の構造にマッチするためのRustの特別な記法で、複合的なものから単純なものまで様々です。
`match`式や他の構文と組み合わせてパターンを使用すると、
プログラムの制御フローをよりコントロールできます。パターンは、以下を組み合わせることで構成されます:

<!--
* Literals
* Destructured arrays, enums, structs, or tuples
* Variables
* Wildcards
* Placeholders
-->

* リテラル
* 分配された配列、enum、構造体、タプル
* 変数
* ワイルドカード
* プレースホルダー

<!--
Some example patterns include `x`, `(a, 3)`, and `Some(Color::Red)`. In the
contexts in which patterns are valid, these components describe the shape of
data. Our program then matches values against the patterns to determine whether
it has the correct shape of data to continue running a particular piece of code.
-->

パターンの例としては`x`や`(a, 3)`や`Some(Color::Red)`などが含まれます。
有効なパターンの文脈では、これらの構成要素はデータの形を記述します。
プログラムは値をパターンとマッチングして、特定のコードを実行し続けるために正しいデータの形を持っているかどうかを判断します。

<!--
To use a pattern, we compare it to some value. If the pattern matches the
value, we use the value parts in our code. Recall the `match` expressions in
Chapter 6 that used patterns, such as the coin-sorting machine example. If the
value fits the shape of the pattern, we can use the named pieces. If it
doesn’t, the code associated with the pattern won’t run.
-->

パターンを使用するには、なんらかの値と比較します。パターンが値に合致したら、コードで値の部分を使用します。
コイン並び替えマシンの例のような第6章でパターンを使用した`match`式を思い出してください。
値がパターンの形に当てはまったら、名前のある部品を使用できます。当てはまらなければ、
パターンに紐づいたコードは実行されません。

<!--
This chapter is a reference on all things related to patterns. We’ll cover the
valid places to use patterns, the difference between refutable and irrefutable
patterns, and the different kinds of pattern syntax that you might see. By the
end of the chapter, you’ll know how to use patterns to express many concepts in
a clear way.
-->

この章は、パターンに関連するあらゆるものの参考文献です。パターンを使用するのが合法な箇所、
<ruby>論駁<rp>(</rp><rt>ろんばく</rt><rp>)</rp></ruby>可能と論駁不可能なパターンの違い、
目撃する可能性のある色々な種類のパターン記法を講義します。章の終わりまでに、
パターンを使用して多くの概念をはっきり表現する方法を知るでしょう。
