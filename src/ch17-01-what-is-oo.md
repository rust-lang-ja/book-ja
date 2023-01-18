<!--
## Characteristics of Object-Oriented Languages
-->

## オブジェクト指向言語の特徴

<!--
There is no consensus in the programming community about what features a
language must have to be considered object oriented. Rust is influenced by many
programming paradigms, including OOP; for example, we explored the features
that came from functional programming in Chapter 13. Arguably, OOP languages
share certain common characteristics, namely objects, encapsulation, and
inheritance. Let’s look at what each of those characteristics mean and whether
Rust supports them.
-->

言語がオブジェクト指向と考えられるのになければならない機能について、プログラミングコミュニティ内での総意はありません。
Rust は OOP を含めた多くのプログラミングパラダイムに影響を受けています; 例えば、
第 13 章で関数型プログラミングに由来する機能を探究しました。議論はあるかもしれませんが、
OOP 言語は特定の一般的な特徴を共有しています。具体的には、オブジェクトやカプセル化、
継承などです。それらの個々の特徴が意味するものと Rust がサポートしているかを見ましょう。

<!--
### Objects Contain Data and Behavior
-->

### オブジェクトは、データと振る舞いを含む

<!--
The book *Design Patterns: Elements of Reusable Object-Oriented Software* by
Enoch Gamma, Richard Helm, Ralph Johnson, and John Vlissides (Addison-Wasley
Professional, 1994) colloquially referred to as *The Gang of Four book*, is a
catlog of object-oriented design patterns. It defines OOP this way:
-->

エーリヒ・ガンマ (Enoch Gamma)、リチャード・ヘルム (Richard Helm)、ラルフ・ジョンソン (Ralph Johnson)、
ジョン・ブリシディース (John Vlissides)(アディソン・ワズリー・プロ) により、
1994 年に書かれた*デザインパターン：再利用可能なオブジェクト指向ソフトウェアの要素*という本は、
俗に*4 人のギャングの本*(`訳注`: the Gang of Four book; GoF とよく略される) と呼ばれ、オブジェクト指向デザインパターンのカタログです。
そこでは、OOP は以下のように定義されています：

<!--
> Object-oriented programs are made up of objects. An *object* packages both
> data and the procedures that operate on that data. The procedures are
> typically called *methods* or *operations*.
-->

> オブジェクト指向プログラムは、オブジェクトで構成される。オブジェクトは、
> データとそのデータを処理するプロシージャを梱包している。このプロシージャは、
> 典型的に*メソッド*または*オペレーション*と呼ばれる。

<!--
Using this definition, Rust is object oriented: structs and enums have data,
and `impl` blocks provide methods on structs and enums. Even though structs and
enums with methods aren’t *called* objects, they provide the same
functionality, according to the Gang of Four’s definition of objects.
-->

この定義を使用すれば、Rust はオブジェクト指向です：構造体と enum にはデータがありますし、
`impl`ブロックが構造体と enum にメソッドを提供します。メソッドのある構造体と enum は、
オブジェクトとは呼ばれないものの、GoF のオブジェクト定義によると、同じ機能を提供します。

<!--
### Encapsulation that Hides Implementation Details
-->

### カプセル化は、実装詳細を隠蔽する

<!--
Another aspect commonly associated with OOP is the idea of *encapsulation*,
which means that the implementation details of an object aren’t accessible to
code using that object. Therefore, the only way to interact with an object is
through its public API; code using the object shouldn’t be able to reach into
the object’s internals and change data or behavior directly. This enables the
programmer to change and refactor an object’s internals without needing to
change the code that uses the object.
-->

OOP とよく紐づけられる別の側面は、カプセル化の思想です。これは、オブジェクトの実装詳細は、
そのオブジェクトを使用するコードにはアクセスできないことを意味します。故に、
オブジェクトと相互作用する唯一の手段は、その公開 API を通してです; オブジェクトを使用するコードは、
オブジェクトの内部に到達して、データや振る舞いを直接変更できるべきではありません。
このために、プログラマはオブジェクトの内部をオブジェクトを使用するコードを変更する必要なく、
変更しリファクタリングできます。

<!--
We discussed how to control encapsulation in Chapter 7: we can use the `pub`
keyword to decide which modules, types, functions, and methods in our code
should be public, and by default everything else is private. For example, we
can define a struct `AveragedCollection` that has a field containing a vector
of `i32` values. The struct can also have a field that contains the average of
the values in the vector, meaning the average doesn’t have to be computed
on demand whenever anyone needs it. In other words, `AveragedCollection` will
cache the calculated average for us. Listing 17-1 has the definition of the
`AveragedCollection` struct:
-->

カプセル化を制御する方法は、第 7 章で議論しました：`pub`キーワードを使用して、
自分のコードのどのモジュールや型、関数、メソッドを公開するか決められ、
既定ではそれ以外のものは全て非公開になります。例えば、
`i32`値のベクタを含むフィールドのある`AveragedCollection`という構造体を定義できます。
この構造体はさらに、ベクタの値の平均を含むフィールドを持てます。つまり、平均は誰かが必要とする度に、
オンデマンドで計算する必要はないということです。言い換えれば、`AveragedCollection`は、
計算した平均をキャッシュしてくれるわけです。リスト 17-1 には、`AveragedCollection`構造体の定義があります：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
```

<!--
<span class="caption">Listing 17-1: An `AveragedCollection` struct that
maintains a list of integers and the average of the items in the
collection</span>
-->

<span class="caption">リスト 17-1: 整数のリストとコレクションの要素の平均を管理する`AveragedCollection`構造体</span>

<!--
The struct is marked `pub` so that other code can use it, but the fields within
the struct remain private. This is important in this case because we want to
ensure that whenever a value is added or removed from the list, the average is
also updated. We do this by implementing `add`, `remove`, and `average` methods
on the struct, as shown in Listing 17-2:
-->

構造体は、他のコードが使用できるように`pub`で印づけされていますが、構造体のフィールドは非公開のままです。
値が追加されたりリストから削除される度に、平均も更新されることを保証したいので、今回の場合重要です。
`add`や`remove`、`average`メソッドを構造体に実装することでこれをします。リスト 17-2 のようにですね：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
# pub struct AveragedCollection {
#     list: Vec<i32>,
#     average: f64,
# }
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

<!--
<span class="caption">Listing 17-2: Implementations of the public methods
`add`, `remove`, and `average` on `AveragedCollection`</span>
-->

<span class="caption">リスト 17-2: `AveragedCollection`の`add`、`remove`、`average`公開メソッドの実装</span>

<!--
The public methods `add`, `remove`, and `average` are the only ways to modify
an instance of `AveragedCollection`. When an item is added to `list` using the
`add` method or removed using the `remove` method, the implementations of each
call the private `update_average` method that handles updating the `average`
field as well.
-->

`add`、`remove`、`average`の公開メソッドが`AveragedCollection`のインスタンスを変更する唯一の方法になります。
要素が`add`メソッドを使用して`list`に追加されたり、`remove`メソッドを使用して削除されたりすると、
各メソッドの実装が`average`フィールドの更新を扱う非公開の`update_average`メソッドも呼び出します。

<!--
We leave the `list` and `average` fields private so there is no way for
external code to add or remove items to the `list` field directly; otherwise,
the `average` field might become out of sync when the `list` changes. The
`average` method returns the value in the `average` field, allowing external
code to read the `average` but not modify it.
-->

`list`と`average`フィールドを非公開のままにしているので、外部コードが要素を`list`フィールドに直接追加したり削除したりする方法はありません;
そうでなければ、`average`フィールドは、`list`が変更された時に同期されなくなる可能性があります。
`average`メソッドは`average`フィールドの値を返し、外部コードに`average`を読ませるものの、
変更は許可しません。

<!--
Because we’ve encapsulated the implementation details of the struct
`AveragedCollection`, we can easily change aspects, such as the data structure,
in the future. For instance, we could use a `HashSet<i32>` instead of a
`Vec<i32>` for the `list` field. As long as the signatures of the `add`
`remove`, and `average` public methods stay the same, code using
`AveragedCollection` wouldn’t need to change. If we made `list` public instead,
this wouldn’t necessarily be the case: `HashSet<i32>` and `Vec<i32>` have
different methods for adding and removing items, so the external code would
likely have to change if it were modifying `list` directly.
-->

構造体`AveragedCollection`の実装詳細をカプセル化したので、データ構造などの側面を将来容易に変更することができます。
例を挙げれば、`list`フィールドに`Vec<i32>`ではなく`HashSet<i32>`を使うこともできます。
`add`、`remove`、`average`といった公開メソッドのシグニチャが同じである限り、`AveragedCollection`を使用するコードは変更する必要がないでしょう。
代わりに`list`を公開にしたら、必ずしもこうはならないでしょう：`HashSet<i32>`と`Vec<i32>`は、
要素の追加と削除に異なるメソッドを持っているので、外部コードが直接`list`を変更しているなら、
外部コードも変更しなければならない可能性が高いでしょう。

<!--
If encapsulation is a required aspect for a language to be considered object
oriented, then Rust meets that requirement. The option to use `pub` or not for
different parts of code enables encapsulation of implementation details.
-->

カプセル化が、言語がオブジェクト指向と考えられるのに必要な側面ならば、Rust はその条件を満たしています。
コードの異なる部分で`pub`を使用するかしないかという選択肢のおかげで、実装詳細をカプセル化することが可能になります。

<!--
### Inheritance as a Type System and as Code Sharing
-->

### 型システム、およびコード共有としての継承

<!--
*Inheritance* is a mechanism whereby an object can inherit from another
object’s definition, thus gaining the parent object’s data and behavior without
you having to define them again.
-->

*継承*は、それによってオブジェクトが他のオブジェクトの定義から受け継ぐことができる機構であり、
それ故に、再定義する必要なく、親オブジェクトのデータと振る舞いを得ます。

<!--
If a language must have inheritance to be an object-oriented language, then
Rust is not one. There is no way to define a struct that inherits the parent
struct’s fields and method implementations. However, if you’re used to having
inheritance in your programming toolbox, you can use other solutions in Rust,
depending on your reason for reaching for inheritance in the first place.
-->

言語がオブジェクト指向言語であるために継承がなければならないのならば、Rust は違います。
親構造体のフィールドとメソッドの実装を受け継ぐ構造体を定義する方法はありません。しかしながら、
継承がプログラミング道具箱にあることに慣れていれば、そもそも継承に手を伸ばす理由によって、
Rust で他の解決策を使用することができます。

<!--
You choose inheritance for two main reasons. One is for reuse of code: you can
implement particular behavior for one type, and inheritance enables you to
reuse that implementation for a different type. You can share Rust code using
default trait method implementations instead, which you saw in Listing 10-14
when we added a default implementation of the `summarize` method on the
`Summary` trait. Any type implementing the `Summary` trait would have the
`summarize` method available on it without any further code. This is similar to
a parent class having an implementation of a method and an inheriting child
class also having the implementation of the method. We can also override the
default implementation of the `summarize` method when we implement the
`Summary` trait, which is similar to a child class overriding the
implementation of a method inherited from a parent class.
-->

継承を選択する理由は主に 2 つあります。1 つ目は、コードの再利用です：ある型に特定の振る舞いを実装し、
継承により、その実装を他の型にも再利用できるわけです。デフォルトのトレイトメソッド実装を代わりに使用して、
Rust コードを共有でき、これは、リスト 10-14 で`Summary`トレイトに`summarize`メソッドのデフォルト実装を追加した時に見かけました。
`Summary`トレイトを実装する型は全て、追加のコードなく`summarize`メソッドが使用できます。
これは、親クラスにメソッドの実装があり、継承した子クラスにもそのメソッドの実装があることと似ています。
また、`Summary`トレイトを実装する時に、`summarize`メソッドのデフォルト実装を上書きすることもでき、
これは、親クラスから継承したメソッドの実装を子クラスが上書きすることに似ています。

<!--
The other reason to use inheritance relates to the type system: to enable a
child type to be used in the same places as the parent type. This is also
called *polymorphism*, which means that you can substitute multiple objects for
each other at runtime if they share certain characteristics.
-->

継承を使用するもう 1 つの理由は、型システムに関連しています：親の型と同じ箇所で子供の型を使用できるようにです。
これは、*多相性*(polymorphism) とも呼ばれ、複数のオブジェクトが特定の特徴を共有しているなら、
実行時にお互いに代用できることを意味します。

<!--
> ### Polymorphism
>
> To many people, polymorphism is synonymous with inheritance. But it’s
> actually a more general concept that refers to code that can work with data
> of multiple types. For inheritance, those types are generally subclasses.
>
> Rust instead uses generics to abstract over different possible types and
> trait bounds to impose constraints on what those types must provide. This is
> sometimes called *bounded parametric polymorphism*.
-->

> ### 多相性
>
> 多くの人にとって、多相性は、継承の同義語です。ですが、実際には複数の型のデータを取り扱えるコードを指すより一般的な概念です。
> 継承について言えば、それらの型は一般的にはサブクラスです。
>
> Rust は代わりにジェネリクスを使用して様々な可能性のある型を抽象化し、トレイト境界を使用してそれらの型が提供するものに制約を課します。
> これは時に、*パラメータ境界多相性*(bounded parametric polymorphism) と呼ばれます。

<!--
Inheritance has recently fallen out of favor as a programming design solution
in many programming languages because it’s often at risk of sharing more code
than necessary. Subclasses shouldn’t always share all characteristics of their
parent class but will do so with inheritance. This can make a program’s design
less flexible. It also introduces the possibility of calling methods on
subclasses that don’t make sense or that cause errors because the methods don’t
apply to the subclass. Some languages will also only allow a subclass
to inherit from one class, further restricting the flexibility of a program’s
design.
-->

継承は、近年、多くのプログラミング言語において、プログラムの設計解決策としては軽んじられています。
というのも、しばしば必要以上にコードを共有してしまう危険性があるからです。サブクラスは、
必ずしも親クラスの特徴を全て共有するべきではないのに、継承ではそうなってしまうのです。
これにより、プログラムの設計の柔軟性を失わせることもあります。また、道理に合わなかったり、メソッドがサブクラスには適用されないために、
エラーを発生させるようなサブクラスのメソッドの呼び出しを引き起こす可能性が出てくるのです。
さらに、サブクラスに 1 つのクラスからだけ継承させる言語もあり、さらにプログラムの設計の柔軟性が制限されます。

<!--
For these reasons, Rust takes a different approach, using trait objects instead
of inheritance. Let’s look at how trait objects enable polymorphism in Rust.
-->

これらの理由により、継承ではなくトレイトオブジェクトを使用して Rust は異なるアプローチを取っています。
Rust において、トレイトオブジェクトがどう多相性を可能にするかを見ましょう。
