<!--
## Storing Keys Associated with Values in Hash Maps
-->

## キーとそれに紐づいた値をハッシュマップに格納する

<!--
The last of our common collections is the *hash map*. The type `HashMap<K, V>`
stores a mapping of keys of type `K` to values of type `V`. It does this via a
*hashing function*, which determines how it places these keys and values into
memory. Many programming languages support this kind of data structure, but
often use a different name, such as hash, map, object, hash table, or
associative array, just to name a few.
-->

一般的なコレクションのトリを飾るのは、*ハッシュマップ*です。型`HashMap<K, V>`は、
`K`型のキーと`V`型の値の対応関係を保持します。これを*ハッシュ関数*を介して行います。
ハッシュ関数は、キーと値のメモリ配置方法を決めるものです。多くのプログラミング言語でもこの種のデータ構造はサポートされていますが、
しばしば名前が違います。hash、map、object、ハッシュテーブル、連想配列など、枚挙に<ruby>暇<rp>(</rp><rt>いとま</rt><rp>)</rp></ruby>はありません。

<!--
Hash maps are useful when you want to look up data not by an index, as
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

この節でハッシュマップの基礎的な API を見ていきますが、より多くのグッズが標準ライブラリにより、
`HashMap<K, V>`上に定義された関数に隠されています。いつものように、
もっと情報が欲しければ、標準ライブラリのドキュメントをチェックしてください。

<!--
### Creating a New Hash Map
-->

### 新規ハッシュマップを生成する

<!--
You can create an empty hash map with `new` and add elements with `insert`. In
Listing 8-20, we’re keeping track of the scores of two teams whose names are
Blue and Yellow. The Blue team starts with 10 points, and the Yellow team
starts with 50.
-->

空のハッシュマップを`new`で作り、要素を`insert`で追加することができます。リスト 8-20 では、
名前がブルーとイエローの 2 チームのスコアを追いかけています。ブルーチームは 10 点から、イエローチームは 50 点から始まります。

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

<!--
<span class="caption">Listing 8-20: Creating a new hash map and inserting some
keys and values</span>
-->

<span class="caption">リスト 8-20: ハッシュマップを生成してキーと値を挿入する</span>

<!--
Note that we need to first `use` the `HashMap` from the collections portion of
the standard library. Of our three common collections, this one is the least
often used, so it’s not included in the features brought into scope
automatically in the prelude. Hash maps also have less support from the
standard library; there's no built-in macro to construct them, for example.
-->

最初に標準ライブラリのコレクション部分から`HashMap`を`use`する必要があることに注意してください。
今までの 3 つの一般的なコレクションの内、これが最も使用頻度が低いので、初期化処理で自動的にスコープに導入される機能には含まれていません。
また、標準ライブラリからのサポートもハッシュマップは少ないです; 例えば、生成するための組み込みマクロがありません。

<!--
Just like vectors, hash maps store their data on the heap. This `HashMap` has
keys of type `String` and values of type `i32`. Like vectors, hash maps are
homogeneous: all of the keys must have the same type, and all of the values
must have the same type.
-->

ベクタと全く同様に、ハッシュマップはデータをヒープに保持します。この`HashMap`はキーが`String`型、
値は`i32`型です。ベクタのように、ハッシュマップは均質です：キーは全て同じ型でなければならず、
値も全て同じ型でなければなりません。

<!--
Another way of constructing a hash map is by using the `collect` method on a
vector of tuples, where each tuple consists of a key and its value. The
`collect` method gathers data into a number of collection types, including
`HashMap`. For example, if we had the team names and initial scores in two
separate vectors, we can use the `zip` method to create a vector of tuples
where “Blue” is paired with 10, and so forth. Then we can use the `collect`
method to turn that vector of tuples into a hash map, as shown in Listing 8-21.
-->

ハッシュマップを生成する別の方法は、タプルのベクタに対して`collect`メソッドを使用するものです。
ここで、各タプルは、キーと値から構成されています。`collect`メソッドはいろんなコレクション型にデータをまとめ上げ、
そこには`HashMap`も含まれています。例として、チーム名と初期スコアが別々のベクタに含まれていたら、
`zip`メソッドを使ってタプルのベクタを作り上げることができ、そこでは「ブルー」は 10 とペアになるなどします。
リスト 8-21 に示したように、それから`collect`メソッドを使って、そのタプルのベクタをハッシュマップに変換することができるわけです。

```rust
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

<!--
<span class="caption">Listing 8-21: Creating a hash map from a list of teams
and a list of scores</span>
-->

<span class="caption">リスト 8-21: チームのリストとスコアのリストからハッシュマップを作る</span>

<!--
The type annotation `HashMap<_, _>` is needed here because it’s possible to
`collect` into many different data structures and Rust doesn’t know which you
want unless you specify. For the type parameters for the key and value types,
however, we use underscores, and Rust can infer the types that the hash map
contains based on the types of the data in the vectors.
-->

ここでは、`HashMap<_, _>`という型注釈が必要になります。なぜなら、いろんなデータ構造に`まとめ上げる`ことができ、
コンパイラは指定しない限り、どれを所望なのかわからないからです。ところが、キーと値の型引数については、
アンダースコアを使用しており、コンパイラはベクタのデータ型に基づいてハッシュマップが含む型を推論することができるのです。

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
`String`のような所有権のある値なら、値はムーブされ、リスト 8-22 でデモされているように、
ハッシュマップはそれらの値の所有者になるでしょう。

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
// field_name と field_value はこの時点で無効になる。試しに使ってみて
// どんなコンパイルエラーが出るか確認してみて！
```

<!--
<span class="caption">Listing 8-22: Showing that keys and values are owned by
the hash map once they’re inserted</span>
-->

<span class="caption">リスト 8-22: 一旦挿入されたら、キーと値はハッシュマップに所有されることを示す</span>

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
the “Validating References with Lifetimes” section in Chapter 10.
-->

値への参照をハッシュマップに挿入したら、値はハッシュマップにムーブされません。参照が指している値は、
最低でもハッシュマップが有効な間は、有効でなければなりません。これらの問題について詳細には、
第 10 章の「ライフタイムで参照を有効化する」節で語ります。

<!--
### Accessing Values in a Hash Map
-->

### ハッシュマップの値にアクセスする

<!--
We can get a value out of the hash map by providing its key to the `get`
method, as shown in Listing 8-23.
-->

リスト 8-23 に示したように、キーを`get`メソッドに提供することで、ハッシュマップから値を取り出すことができます。

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

<!--
<span class="caption">Listing 8-23: Accessing the score for the Blue team
stored in the hash map</span>
-->

<span class="caption">リスト 8-23: ハッシュマップに保持されたブルーチームのスコアにアクセスする</span>

<!--
Here, `score` will have the value that’s associated with the Blue team, and the
result will be `Some(&10)`. The result is wrapped in `Some` because `get`
returns an `Option<&V>`; if there’s no value for that key in the hash map,
`get` will return `None`. The program will need to handle the `Option` in one
of the ways that we covered in Chapter 6.
-->

ここで、`score`はブルーチームに紐づけられた値になり、結果は`Some(&10)`となるでしょう。
結果は`Some`に包まれます。というのも、`get`は`Option<&V>`を返すからです; キーに対応する値がハッシュマップになかったら、
`get`は`None`を返すでしょう。プログラムは、この`Option`を第 6 章で講義した方法のどれかで扱う必要があるでしょう。

<!--
We can iterate over each key/value pair in a hash map in a similar manner as we
do with vectors, using a `for` loop:
-->

ベクタのように、`for`ループでハッシュマップのキーと値のペアを走査することができます：

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

<!--
This code will print each pair in an arbitrary order:
-->

このコードは、各ペアを任意の順番で出力します：

```text
Yellow: 50
Blue: 10
```

<!--
### Updating a Hash Map
-->

### ハッシュマップを更新する

<!--
Although the number of keys and values is growable, each key can only have one
value associated with it at a time. When we want to change the data in a hash
map, we have to decide how to handle the case when a key already has a value
assigned. We could replace the old value with the new value, completely
disregarding the old value. We could keep the old value and ignore the new
value, only adding the new value if the key *doesn’t* already have a value. Or
we could combine the old value and the new value. Let’s look at how to do each
of these!
-->

キーと値の数は伸長可能なものの、各キーには 1 回に 1 つの値しか紐づけることができません。
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
Even though the code in Listing 8-24 calls `insert` twice, the hash map will
only contain one key/value pair because we’re inserting the value for the Blue
team’s key both times.
-->

キーと値をハッシュマップに挿入し、同じキーを異なる値で挿入したら、そのキーに紐づけられている値は置換されます。
リスト 8-24 のコードは、`insert`を二度呼んでいるものの、ハッシュマップには一つのキーと値の組しか含まれません。
なぜなら、ブルーチームキーに対する値を 2 回とも挿入しているからです。

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```

<!--
<span class="caption">Listing 8-24: Replacing a value stored with a particular
key</span>
-->

<span class="caption">リスト 8-24: 特定のキーで保持された値を置き換える</span>

<!--
This code will print `{"Blue": 25}`. The original value of `10` has been
overwritten.
-->

このコードは、`{"Blue": 25}`と出力するでしょう。`10`という元の値は上書きされたのです。

<!--
#### Only Inserting a Value If the Key Has No Value
-->

#### キーに値がなかった時のみ値を挿入する

<!--
It’s common to check whether a particular key has a value and if it doesn’t,
insert a value for it. Hash maps have a special API for this called `entry`
that takes the key we want to check as a parameter. The return value of the
`entry` method is an enum called `Entry` that represents a value that might or
might not exist. Let’s say we want to check whether the key for the Yellow team
has a value associated with it. If it doesn’t, we want to insert the value 50,
and the same for the Blue team. Using the `entry` API, the code looks like
Listing 8-25.
-->

特定のキーに値があるか確認することは一般的であり、存在しない時に値を挿入することも一般的です。
ハッシュマップには、これを行う`entry`と呼ばれる特別な API があり、これは、引数としてチェックしたいキーを取ります。
この`entry`メソッドの戻り値は、`Entry`と呼ばれる enum であり、これは存在したりしなかったりする可能性のある値を表します。
イエローチームに対するキーに値が紐づけられているか否か確認したくなったとしましょう。存在しなかったら、
50 という値を挿入したく、ブルーチームに対しても同様です。`entry`API を使用すれば、コードはリスト 8-25 のようになります。

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```

<!--
<span class="caption">Listing 8-25: Using the `entry` method to only insert if
the key does not already have a value</span>
-->

<span class="caption">リスト 8-25: `entry`メソッドを使ってキーに値がない場合だけ挿入する</span>

<!--
The `or_insert` method on `Entry` is defined to return a mutable reference to
the value for the corresponding `Entry` key if that key exists, and if not,
inserts the parameter as the new value for this key and returns a mutable
reference to the new value. This technique is much cleaner than writing the
logic ourselves, and in addition, plays more nicely with the borrow checker.
-->

`Entry`上の`or_insert`メソッドは、対応する`Entry`キーが存在した時にそのキーに対する値への可変参照を返すために定義されており、
もしなかったら、引数をこのキーの新しい値として挿入し、新しい値への可変参照を返します。このテクニックの方が、
そのロジックを自分で書くよりもはるかに綺麗な上に、borrow checker とも親和性が高くなります。

<!--
Running the code in Listing 8-25 will print `{"Yellow": 50, "Blue": 10}`. The
first call to `entry` will insert the key for the Yellow team with the value
50 because the Yellow team doesn’t have a value already. The second call to
`entry` will not change the hash map because the Blue team already has the
value 10.
-->

リスト 8-25 のコードを実行すると、`{"Yellow": 50, "Blue": 10}`と出力するでしょう。
最初の`entry`呼び出しは、まだイエローチームに対する値がないので、値 50 でイエローチームのキーを挿入します。
`entry`の 2 回目の呼び出しはハッシュマップを変更しません。なぜなら、ブルーチームには既に 10 という値があるからです。

<!--
#### Updating a Value Based on the Old Value
-->

#### 古い値に基づいて値を更新する

<!--
Another common use case for hash maps is to look up a key’s value and then
update it based on the old value. For instance, Listing 8-26 shows code that
counts how many times each word appears in some text. We use a hash map with
the words as keys and increment the value to keep track of how many times we’ve
seen that word. If it’s the first time we’ve seen a word, we’ll first insert
the value 0:
-->

ハッシュマップの別の一般的なユースケースは、キーの値を探し、古い値に基づいてそれを更新することです。
例えば、リスト 8-26 は、各単語があるテキストに何回出現するかを数え上げるコードを示しています。
キーに単語を入れたハッシュマップを使用し、その単語を何回見かけたか追いかけるために値を増やします。
ある単語を見かけたのが最初だったら、まず 0 という値を挿入します：

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

<!--
<span class="caption">Listing 8-26: Counting occurrences of words using a hash
map that stores words and counts</span>
-->

<span class="caption">リスト 8-26: 単語とカウントを保持するハッシュマップを使って単語の出現数をカウントする</span>

<!--
This code will print `{"world": 2, "hello": 1, "wonderful": 1}`. The
`or_insert` method actually returns a mutable reference (`&mut V`) to the value
for this key. Here we store that mutable reference in the `count` variable, so
in order to assign to that value, we must first dereference `count` using the
asterisk (`*`). The mutable reference goes out of scope at the end of the `for`
loop, so all of these changes are safe and allowed by the borrowing rules.
-->

このコードは、`{"world": 2, "hello": 1, "wonderful": 1}`と出力するでしょう。
`or_insert`関数は実際、このキーに対する値への可変参照 (`&mut V`) を返すのです。
ここでその可変参照を`count`変数に保持しているので、その値に代入するには、
まずアスタリスク (`*`) で`count`を参照外ししなければならないのです。この可変参照は、
`for`ループの終端でスコープを抜けるので、これらの変更は全て安全であり、借用規則により許可されるのです。

<!--
### Hashing Function
-->

### ハッシュ関数

<!--
By default, `HashMap` uses a cryptographically secure hashing function that can
provide resistance to Denial of Service (DoS) attacks. This is not the fastest
hashing algorithm available, but the trade-off for better security that comes
with the drop in performance is worth it. If you profile your code and find
that the default hash function is too slow for your purposes, you can switch to
another function by specifying a different *hasher*. A hasher is a type that
implements the `BuildHasher` trait. We’ll talk about traits and how to
implement them in Chapter 10. You don’t necessarily have to implement your own
hasher from scratch; [crates.io](https://crates.io) has libraries shared by
other Rust users that provide hashers implementing many common hashing
algorithms.
-->

標準では、`HashMap`はサービス拒否 (DoS) アタックに対して抵抗を示す暗号学的に安全なハッシュ関数を使用します。
これは、利用可能な最速のハッシュアルゴリズムではありませんが、パフォーマンスの欠落と引き換えに安全性を得るというトレードオフは、
価値があります。自分のコードをプロファイリングして、自分の目的では標準のハッシュ関数は遅すぎると判明したら、
異なる*hasher*を指定することで別の関数に切り替えることができます。hasher とは、
`BuildHasher`トレイトを実装する型のことです。トレイトについてとその実装方法については、第 10 章で語ります。
必ずしも独自の hasher を 1 から作り上げる必要はありません; [crates.io](https://crates.io)には、
他の Rust ユーザによって共有された多くの一般的なハッシュアルゴリズムを実装した hasher を提供するライブラリがあります。

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
多くの機能を提供してくれるでしょう。今なら解決可能なはずの練習問題を用意しました：

<!--
* Given a list of integers, use a vector and return the mean (the average
value), median (when sorted, the value in the middle position), and mode (the
value that occurs most often; a hash map will be helpful here) of the list.
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

* 整数のリストが与えられ、ベクタを使って mean(平均値)、median(ソートされた時に真ん中に来る値)、
  mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう) を返してください。
* 文字列をピッグ・ラテン (`訳注`: 英語の言葉遊びの一つ) に変換してください。各単語の最初の子音は、
  単語の終端に移り、"ay"が足されます。従って、"first"は"irst-fay"になります。ただし、
  母音で始まる単語には、お尻に"hay"が付け足されます ("apple"は"apple-hay"になります)。
  UTF-8 エンコードに関する詳細を心に留めておいてください！
* ハッシュマップとベクタを使用して、ユーザに会社の部署に雇用者の名前を追加させられるテキストインターフェイスを作ってください。
  例えば、"Add Sally to Engineering"(開発部門にサリーを追加) や"Add Amir to Sales"(販売部門にアミールを追加) などです。
  それからユーザに、ある部署にいる人間の一覧や部署ごとにアルファベット順で並べ替えられた会社の全人間の一覧を扱わせてあげてください。

<!--
The standard library API documentation describes methods that vectors, strings,
and hash maps have that will be helpful for these exercises!
-->

標準ライブラリの API ドキュメントには、この練習問題に有用な、ベクタ、文字列、ハッシュマップのメソッドが解説されています。

<!--
We’re getting into more complex programs in which operations can fail; so, it’s
a perfect time to discuss error handling. We'll do that next!
-->

処理が失敗することもあるような、より複雑なプログラムに入り込んできています; ということは、
エラーの処理法について議論するのにぴったりということです。次はそれをします！
