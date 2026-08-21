<!--
## Storing Keys with Associated Values in Hash Maps
-->

## キーとそれに紐づいた値をハッシュマップに格納する

<!--
The last of our common collections is the *hash map*. The type `HashMap<K, V>`
stores a mapping of keys of type `K` to values of type `V` using a
*hashing function*, which determines how it places these keys and values into
memory. Many programming languages support this kind of data structure, but
they often use a different name, such as hash, map, object, hash table,
dictionary, or associative array, just to name a few.
-->

一般的なコレクションのトリを飾るのは、*ハッシュマップ*です。型`HashMap<K, V>`は、
`K`型のキーと`V`型の値の対応関係を*ハッシュ関数*を使用して保持します。
ハッシュ関数は、キーと値のメモリ配置方法を決めるものです。多くのプログラミング言語でもこの種のデータ構造はサポートされていますが、
しばしば名前が違います。hash、map、object、ハッシュテーブル、辞書、連想配列など、枚挙に<ruby>暇<rp>(</rp><rt>いとま</rt><rp>)</rp></ruby>はありません。

<!--
Hash maps are useful when you want to look up data not by using an index, as
you can with vectors, but by using a key that can be of any type. For example,
in a game, you could keep track of each team’s score in a hash map in which
each key is a team’s name and the values are each team’s score. Given a team
name, you can retrieve its score.
-->

ハッシュマップは、ベクタのように番号ではなく、どんな型にもなりうるキーを使ってデータを参照したいときに有用です。
例えば、ゲームにおいて、各チームのスコアをハッシュマップで追いかけることができます。ここで、各キーはチーム名、
値が各チームのスコアになります。チーム名が与えられれば、スコアを扱うことができるわけです。

<!--
We’ll go over the basic API of hash maps in this section, but many more goodies
are hiding in the functions defined on `HashMap<K, V>` by the standard library.
As always, check the standard library documentation for more information.
-->

この節でハッシュマップの基礎的なAPIを見ていきますが、より多くのグッズが標準ライブラリにより、
`HashMap<K, V>`上に定義された関数に隠されています。いつものように、
もっと情報が欲しければ、標準ライブラリのドキュメントをチェックしてください。

<!--
### Creating a New Hash Map
-->

### 新規ハッシュマップを生成する

<!--
One way to create an empty hash map is using `new` and adding elements with
`insert`. In Listing 8-20, we’re keeping track of the scores of two teams whose
names are *Blue* and *Yellow*. The Blue team starts with 10 points, and the
Yellow team starts with 50.
-->

空のハッシュマップを作成する方法のひとつは`new`を使用する方法で、その後要素を`insert`で追加することができます。
リスト8-20では、名前が*ブルー*と*イエロー*の2チームのスコアを追いかけています。
ブルーチームは10点から、イエローチームは50点から始まります。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-20: Creating a new hash map and inserting some
keys and values</span>
-->

<span class="caption">リスト8-20: ハッシュマップを生成してキーと値を挿入する</span>

<!--
Note that we need to first `use` the `HashMap` from the collections portion of
the standard library. Of our three common collections, this one is the least
often used, so it’s not included in the features brought into scope
automatically in the prelude. Hash maps also have less support from the
standard library; there’s no built-in macro to construct them, for example.
-->

最初に標準ライブラリのコレクション部分から`HashMap`を`use`する必要があることに注意してください。
今までの3つの一般的なコレクションの内、これが最も使用頻度が低いので、初期化処理で自動的にスコープに導入される機能には含まれていません。
また、標準ライブラリからのサポートもハッシュマップは少ないです; 例えば、生成するための組み込みマクロがありません。

<!--
Just like vectors, hash maps store their data on the heap. This `HashMap` has
keys of type `String` and values of type `i32`. Like vectors, hash maps are
homogeneous: all of the keys must have the same type as each other, and all of
the values must have the same type.
-->

ベクタと全く同様に、ハッシュマップはデータをヒープに保持します。この`HashMap`はキーが`String`型、
値は`i32`型です。ベクタのように、ハッシュマップは均質です: すべてのキーは互いに同じ型でなければならず、
値も全て同じ型でなければなりません。

<!--
### Accessing Values in a Hash Map
-->

### ハッシュマップの値にアクセスする

<!--
We can get a value out of the hash map by providing its key to the `get`
method, as shown in Listing 8-21.
-->

リスト8-21に示したように、キーを`get`メソッドに提供することで、ハッシュマップから値を取り出すことができます。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-21: Accessing the score for the Blue team
stored in the hash map</span>
-->

<span class="caption">リスト8-21: ハッシュマップに保持されたブルーチームのスコアにアクセスする</span>

<!--
Here, `score` will have the value that’s associated with the Blue team, and the
result will be `10`. The `get` method returns an `Option<&V>`; if there’s no
value for that key in the hash map, `get` will return `None`. This program
handles the `Option` by calling `copied` to get an `Option<i32>` rather than an
`Option<&i32>`, then `unwrap_or` to set `score` to zero if `scores` doesn't
have an entry for the key.
-->

ここで、`score`はブルーチームに紐づけられた値になり、結果は`10`となるでしょう。
`get`メソッドは`Option<&V>`を返します;
キーに対応する値がハッシュマップになかったら、`get`は`None`を返すでしょう。
このプログラムはこの`Option`に対して、`Option<&i32>`ではなく`Option<i32>`を得るために`copied`を呼び出し、
その後、`scores`がそのキーに対応するエントリを持たない場合は`score`を0に設定するために、`unwrap_or`を呼び出して対処します。

<!--
We can iterate over each key/value pair in a hash map in a similar manner as we
do with vectors, using a `for` loop:
-->

ベクタのように、`for`ループでハッシュマップのキーと値のペアを走査することができます:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

<!--
This code will print each pair in an arbitrary order:
-->

このコードは、各ペアを任意の順番で出力します:

```text
Yellow: 50
Blue: 10
```

<!--
### Hash Maps and Ownership
-->

### ハッシュマップと所有権

<!--
For types that implement the `Copy` trait, like `i32`, the values are copied
into the hash map. For owned values like `String`, the values will be moved and
the hash map will be the owner of those values, as demonstrated in Listing 8-22.
-->

`i32`のような`Copy`トレイトを実装する型について、値はハッシュマップにコピーされます。
`String`のような所有権のある値なら、値はムーブされ、リスト8-22でデモされているように、
ハッシュマップはそれらの値の所有者になるでしょう。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-22: Showing that keys and values are owned by
the hash map once they’re inserted</span>
-->

<span class="caption">リスト8-22: 一旦挿入されたら、キーと値はハッシュマップに所有されることを示す</span>

<!--
We aren’t able to use the variables `field_name` and `field_value` after
they’ve been moved into the hash map with the call to `insert`.
-->

`insert`を呼び出して`field_name`と`field_value`がハッシュマップにムーブされた後は、
これらの変数を使用することは叶いません。

<!--
If we insert references to values into the hash map, the values won’t be moved
into the hash map. The values that the references point to must be valid for at
least as long as the hash map is valid. We’ll talk more about these issues in
the [“Validating References with
Lifetimes”][validating-references-with-lifetimes] section in
Chapter 10.
-->

値への参照をハッシュマップに挿入したら、値はハッシュマップにムーブされません。参照が指している値は、
最低でもハッシュマップが有効な間は、有効でなければなりません。これらの問題について詳細には、
第10章の[「ライフタイムで参照を有効化する」][validating-references-with-lifetimes]節で語ります。

<!--
### Updating a Hash Map
-->

### ハッシュマップを更新する

<!--
Although the number of key and value pairs is growable, each unique key can
only have one value associated with it at a time (but not vice versa: for
example, both the Blue team and the Yellow team could have value 10 stored in
the `scores` hash map).
-->

キーと値のペアの数は伸長可能ですが、それぞれの一意なキーには同時に1つの値しか紐づけることができません（ただし逆は可能です:
例えば、`scores`ハッシュマップ内にブルーチームとイエローチームはともに値10を保存することができます）。

<!--
When you want to change the data in a hash map, you have to decide how to
handle the case when a key already has a value assigned. You could replace the
old value with the new value, completely disregarding the old value. You could
keep the old value and ignore the new value, only adding the new value if the
key *doesn’t* already have a value. Or you could combine the old value and the
new value. Let’s look at how to do each of these!
-->

ハッシュマップ内のデータを変えたい時は、すでにキーに値が紐づいている場合の扱い方を決めなければなりません。
古い値を新しい値で置き換えて、古い値を完全に無視することもできます。古い値を保持して、
新しい値を無視し、キーにまだ値が*ない*場合に新しい値を追加するだけにすることもできます。
あるいは、古い値と新しい値を組み合わせることもできます。各方法について見ていきましょう！

<!--
#### Overwriting a Value
-->

#### 値を上書きする

<!--
If we insert a key and a value into a hash map and then insert that same key
with a different value, the value associated with that key will be replaced.
Even though the code in Listing 8-23 calls `insert` twice, the hash map will
only contain one key/value pair because we’re inserting the value for the Blue
team’s key both times.
-->

キーと値をハッシュマップに挿入し、同じキーを異なる値で挿入したら、そのキーに紐づけられている値は置換されます。
リスト8-23のコードは、`insert`を二度呼んでいるものの、ハッシュマップには一つのキーと値の組しか含まれません。
なぜなら、ブルーチームキーに対する値を2回とも挿入しているからです。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-23: Replacing a value stored with a particular
key</span>
-->

<span class="caption">リスト8-23: 特定のキーで保持された値を置き換える</span>

<!--
This code will print `{"Blue": 25}`. The original value of `10` has been
overwritten.
-->

このコードは、`{"Blue": 25}`と出力するでしょう。`10`という元の値は上書きされたのです。

<!-- Old headings. Do not remove or links may break. -->
<!--
<a id="only-inserting-a-value-if-the-key-has-no-value"></a>
-->

<!--
#### Adding a Key and Value Only If a Key Isn’t Present
-->

#### キーが存在しない場合のみキーと値を追加する

<!--
It’s common to check whether a particular key already exists in the hash map
with a value then take the following actions: if the key does exist in the hash
map, the existing value should remain the way it is. If the key doesn’t exist,
insert it and a value for it.
-->

特定のキーがハッシュマップ内に値とともに存在しているか確認して、以下のアクションを取ることは一般的でしょう:
キーがハッシュマップに存在する場合は、既存の値はそのままにします。
キーが存在しない場合は、そのキーとそれに対応する値を挿入します。

<!--
Hash maps have a special API for this called `entry` that takes the key you
want to check as a parameter. The return value of the `entry` method is an enum
called `Entry` that represents a value that might or might not exist. Let’s say
we want to check whether the key for the Yellow team has a value associated
with it. If it doesn’t, we want to insert the value 50, and the same for the
Blue team. Using the `entry` API, the code looks like Listing 8-24.
-->

ハッシュマップには、これを行う`entry`と呼ばれる特別なAPIがあり、これは、引数としてチェックしたいキーを取ります。
この`entry`メソッドの戻り値は、`Entry`と呼ばれるenumであり、これは存在したりしなかったりする可能性のある値を表します。
イエローチームに対するキーに値が紐づけられているか否か確認したくなったとしましょう。存在しなかったら、
50という値を挿入したく、ブルーチームに対しても同様です。`entry`APIを使用すれば、コードはリスト8-24のようになります。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-24: Using the `entry` method to only insert if
the key does not already have a value</span>
-->

<span class="caption">リスト8-24: `entry`メソッドを使ってキーに値がない場合だけ挿入する</span>

<!--
The `or_insert` method on `Entry` is defined to return a mutable reference to
the value for the corresponding `Entry` key if that key exists, and if not,
inserts the parameter as the new value for this key and returns a mutable
reference to the new value. This technique is much cleaner than writing the
logic ourselves and, in addition, plays more nicely with the borrow checker.
-->

`Entry`上の`or_insert`メソッドは、対応する`Entry`キーが存在した時にそのキーに対する値への可変参照を返すために定義されており、
もしなかったら、引数をこのキーの新しい値として挿入し、新しい値への可変参照を返します。このテクニックの方が、
そのロジックを自分で書くよりもはるかに綺麗な上に、borrow checkerとも親和性が高くなります。

<!--
Running the code in Listing 8-24 will print `{"Yellow": 50, "Blue": 10}`. The
first call to `entry` will insert the key for the Yellow team with the value
50 because the Yellow team doesn’t have a value already. The second call to
`entry` will not change the hash map because the Blue team already has the
value 10.
-->

リスト8-24のコードを実行すると、`{"Yellow": 50, "Blue": 10}`と出力するでしょう。
最初の`entry`呼び出しは、まだイエローチームに対する値がないので、値50でイエローチームのキーを挿入します。
`entry`の2回目の呼び出しはハッシュマップを変更しません。なぜなら、ブルーチームには既に10という値があるからです。

<!--
#### Updating a Value Based on the Old Value
-->

#### 古い値に基づいて値を更新する

<!--
Another common use case for hash maps is to look up a key’s value and then
update it based on the old value. For instance, Listing 8-25 shows code that
counts how many times each word appears in some text. We use a hash map with
the words as keys and increment the value to keep track of how many times we’ve
seen that word. If it’s the first time we’ve seen a word, we’ll first insert
the value 0.
-->

ハッシュマップの別の一般的なユースケースは、キーの値を探し、古い値に基づいてそれを更新することです。
例えば、リスト8-25は、各単語があるテキストに何回出現するかを数え上げるコードを示しています。
キーに単語を入れたハッシュマップを使用し、その単語を何回見かけたか追いかけるために値を増やします。
ある単語を見かけたのが最初だったら、まず0という値を挿入します。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-25: Counting occurrences of words using a hash
map that stores words and counts</span>
-->

<span class="caption">リスト8-25: 単語とカウントを保持するハッシュマップを使って単語の出現数をカウントする</span>

<!--
This code will print `{"world": 2, "hello": 1, "wonderful": 1}`. You might see
the same key/value pairs printed in a different order: recall from the
[“Accessing Values in a Hash Map”][access] section that
iterating over a hash map happens in an arbitrary order.
-->

このコードは、`{"world": 2, "hello": 1, "wonderful": 1}`と出力するでしょう。
同じキー/値ペアが、異なる順で印字されるかもしれません:
[「ハッシュマップの値にアクセスする」][access]の節で説明した、
ハッシュマップの走査は任意の順で起こるということを思い出してください。

<!--
The `split_whitespace` method returns an iterator over sub-slices, separated by
whitespace, of the value in `text`. The `or_insert` method returns a mutable
reference (`&mut V`) to the value for the specified key. Here we store that
mutable reference in the `count` variable, so in order to assign to that value,
we must first dereference `count` using the asterisk (`*`). The mutable
reference goes out of scope at the end of the `for` loop, so all of these
changes are safe and allowed by the borrowing rules.
-->

`split_whitespace`メソッドは、`text`の値から、ホワイトスペースによって区切られた部分スライスを走査するイテレータを返します。
`or_insert`関数は、指定されたキーに対する値への可変参照(`&mut V`)を返すのです。
ここでその可変参照を`count`変数に保持しているので、その値に代入するには、
まずアスタリスク(`*`)で`count`を参照外ししなければならないのです。この可変参照は、
`for`ループの終端でスコープを抜けるので、これらの変更は全て安全であり、借用規則により許可されるのです。

<!--
### Hashing Functions
-->

### ハッシュ関数

<!--
By default, `HashMap` uses a hashing function called *SipHash* that can provide
resistance to Denial of Service (DoS) attacks involving hash
tables[^siphash]. This is not the fastest hashing algorithm
available, but the trade-off for better security that comes with the drop in
performance is worth it. If you profile your code and find that the default
hash function is too slow for your purposes, you can switch to another function
by specifying a different hasher. A *hasher* is a type that implements the
`BuildHasher` trait. We’ll talk about traits and how to implement them in
Chapter 10. You don’t necessarily have to implement your own hasher from
scratch; [crates.io](https://crates.io/) has libraries shared by
other Rust users that provide hashers implementing many common hashing
algorithms.
-->

`HashMap`はデフォルトでは、ハッシュテーブルに関するサービス拒否(DoS)攻撃に対する耐性を提供する、*SipHash*と呼ばれるハッシュ関数を使用します[^siphash]。
これは、利用可能な最速のハッシュアルゴリズムではありませんが、パフォーマンスの欠落と引き換えに安全性を得るというトレードオフは、
価値があります。自分のコードをプロファイリングして、自分の目的では標準のハッシュ関数は遅すぎると判明したら、
異なるhasherを指定することで別の関数に切り替えることができます。*hasher*とは、
`BuildHasher`トレイトを実装する型のことです。トレイトについてとその実装方法については、第10章で語ります。
必ずしも独自のhasherを1から作り上げる必要はありません; [crates.io](https://crates.io/)には、
他のRustユーザによって共有された多くの一般的なハッシュアルゴリズムを実装したhasherを提供するライブラリがあります。

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

<!--
## Summary
-->

## まとめ

<!--
Vectors, strings, and hash maps will provide a large amount of functionality
necessary in programs when you need to store, access, and modify data. Here are
some exercises you should now be equipped to solve:
-->

ベクタ、文字列、ハッシュマップはデータを保持し、アクセスし、変更する必要のあるプログラムで必要になる、
多くの機能を提供してくれるでしょう。今なら解決可能なはずの練習問題を用意しました:

<!--
* Given a list of integers, use a vector and return the median (when sorted,
  the value in the middle position) and mode (the value that occurs most often;
  a hash map will be helpful here) of the list.
* Convert strings to pig latin. The first consonant of each word is moved to
  the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words
  that start with a vowel have “hay” added to the end instead (“apple” becomes
  “apple-hay”). Keep in mind the details about UTF-8 encoding!
* Using a hash map and vectors, create a text interface to allow a user to add
  employee names to a department in a company. For example, “Add Sally to
  Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all
  people in a department or all people in the company by department, sorted
  alphabetically.
-->

* 整数のリストが与えられ、ベクタを使ってmedian(ソートされた時に真ん中に来る値)、
  mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)を返してください。
* 文字列をピッグ・ラテン(`訳注`: 英語の言葉遊びの一つ)に変換してください。各単語の最初の子音は、
  単語の終端に移り、"ay"が足されます。従って、"first"は"irst-fay"になります。ただし、
  母音で始まる単語には、お尻に"hay"が付け足されます("apple"は"apple-hay"になります)。
  UTF-8エンコードに関する詳細を心に留めておいてください！
* ハッシュマップとベクタを使用して、ユーザに会社の部署に雇用者の名前を追加させられるテキストインターフェイスを作ってください。
  例えば、"Add Sally to Engineering"(開発部門にサリーを追加)や"Add Amir to Sales"(販売部門にアミールを追加)などです。
  それからユーザに、ある部署にいる人間の一覧や部署ごとにアルファベット順で並べ替えられた会社の全人間の一覧を扱わせてあげてください。

<!--
The standard library API documentation describes methods that vectors, strings,
and hash maps have that will be helpful for these exercises!
-->

標準ライブラリのAPIドキュメントには、この練習問題に有用な、ベクタ、文字列、ハッシュマップのメソッドが解説されています。

<!--
We’re getting into more complex programs in which operations can fail, so, it’s
a perfect time to discuss error handling. We’ll do that next!
-->

処理が失敗することもあるような、より複雑なプログラムに入り込んできています。
ということは、エラーの処理法について議論するのにぴったりということです。次はそれをします！

<!--
[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[access]: #accessing-values-in-a-hash-map
-->

[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#ライフタイムで参照を検証する
[access]: #ハッシュマップの値にアクセスする
