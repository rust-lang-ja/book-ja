<!--
# Enums and Pattern Matching
-->

# Enumとパターンマッチング

<!--
イーナムとカタカナで表記するのも変なので、Enumという表記で統一します
-->

<!--
In this chapter, we’ll look at *enumerations*, also referred to as *enums*.
Enums allow you to define a type by enumerating its possible *variants*. First
we’ll define and use an enum to show how an enum can encode meaning along with
data. Next, we’ll explore a particularly useful enum, called `Option`, which
expresses that a value can be either something or nothing. Then we’ll look at
how pattern matching in the `match` expression makes it easy to run different
code for different values of an enum. Finally, we’ll cover how the `if let`
construct is another convenient and concise idiom available to handle enums in
your code.
-->

この章では、*列挙型*について見ていきます。列挙型は、*enum*とも称されます。enumは、取りうる*列挙子*を列挙することで、
型を定義させてくれます。最初に、enumを定義し、使用して、enumがデータとともに意味をコード化する方法を示します。
次に、特別に有用なenumである`Option`について掘り下げていきましょう。この型は、
値が何かかなんでもないかを表現します。それから、`match`式のパターンマッチングにより、
どうenumの色々な値に対して異なるコードを走らせやすくなるかを見ます。最後に、`if let`文法要素も、
<ruby>如何<rp>(</rp><rt>いか</rt><rp>)</rp></ruby>にenumをコードで扱う際に使用可能な便利で簡潔な慣用句であるかを解説します。
