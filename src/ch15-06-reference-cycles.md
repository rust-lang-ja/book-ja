<!--
## Reference Cycles Can Leak Memory
-->

## 循環参照は、メモリをリークすることもある

<!--
Rust’s memory safety guarantees make it difficult, but not impossible, to
accidentally create memory that is never cleaned up (known as a *memory leak*).
Preventing memory leaks entirely is not one of Rust’s guarantees in the same
way that disallowing data races at compile time is, meaning memory leaks are
memory safe in Rust. We can see that Rust allows memory leaks by using `Rc<T>`
and `RefCell<T>`: it’s possible to create references where items refer to each
other in a cycle. This creates memory leaks because the reference count of each
item in the cycle will never reach 0, and the values will never be dropped.
-->

Rust のメモリ安全保証により誤って絶対に片付けられることのないメモリ (*メモリリーク*として知られています) を生成してしまいにくくなりますが、
不可能にはなりません。コンパイル時にデータ競合を防ぐのと同じようにメモリリークを完全に回避することは、
Rust の保証の一つではなく、メモリリークは Rust においてはメモリ安全であることを意味します。
Rust では、`Rc<T>`と`RefCell<T>`を使用してメモリリークを許可するとわかります：
要素がお互いに循環して参照する参照を生成することも可能ということです。循環の各要素の参照カウントが絶対に 0 にならないので、
これはメモリリークを起こし、値は絶対にドロップされません。

<!--
### Creating a Reference Cycle
-->

### 循環参照させる

<!--
Let’s look at how a reference cycle might happen and how to prevent it,
starting with the definition of the `List` enum and a `tail` method in Listing
15-25:
-->

リスト 15-25 の`List` enum の定義と`tail`メソッドから始めて、どう循環参照が起こる可能性があるのかとその回避策を見ましょう：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

<!-- Hidden fn main is here to disable the automatic wrapping in fn main that
doc tests do; the `use List` fails if this listing is put within a main -->

```rust
# fn main() {}
use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}
```

<!--
<span class="caption">Listing 15-25: A cons list definition that holds a
`RefCell<T>` so we can modify what a `Cons` variant is referring to</span>
-->

<span class="caption">リスト 15-25: `Cons`列挙子が参照しているものを変更できるように`RefCell<T>`を抱えているコンスリストの定義</span>

<!--
We’re using another variation of the `List` definition in Listing 15-5. The
second element in the `Cons` variant is now `RefCell<Rc<List>>`, meaning that
instead of having the ability to modify the `i32` value as we did in Listing
15-24, we want to modify which `List` value a `Cons` variant is pointing to.
We're also adding a `tail` method to make it convenient for us to access the
second item if we have a `Cons` variant.
-->

リスト 15-5 の`List`定義の別バリエーションを使用しています。`Cons`列挙子の 2 番目の要素はこれで`RefCell<Rc<List>>`になり、
リスト 15-24 のように`i32`値を変更する能力があるのではなく、`Cons`列挙子が指している`List`値の先を変えたいということです。
また、`tail`メソッドを追加して`Cons`列挙子があるときに 2 番目の要素にアクセスするのが便利になるようにしています。

<!--
In Listing 15-26, we’re adding a `main` function that uses the definitions in
Listing 15-25. This code creates a list in `a` and a list in `b` that points to
the list in `a`. Then it modifies the list in `a` to point to `b`, creating a
reference cycle. There are `println!` statements along the way to show what the
reference counts are at various points in this process.
-->

リスト 15-26 でリスト 15-25 の定義を使用する`main`関数を追加しています。このコードは、`a`にリストを、
`b`に`a`のリストを指すリストを作成します。それから`a`のリストを変更して`b`を指し、循環参照させます。
その流れの中に過程のいろんな場所での参照カウントを示す`println!`文が存在しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use List::{Cons, Nil};
# use std::rc::Rc;
# use std::cell::RefCell;
# #[derive(Debug)]
# enum List {
#     Cons(i32, RefCell<Rc<List>>),
#     Nil,
# }
#
# impl List {
#     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
#         match *self {
#             Cons(_, ref item) => Some(item),
#             Nil => None,
#         }
#     }
# }
#
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // a の最初の参照カウント = {}
    println!("a initial rc count = {}", Rc::strong_count(&a));
    // a の次の要素は = {:?}
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // b 作成後の a の参照カウント = {}
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // b の最初の参照カウント = {}
    println!("b initial rc count = {}", Rc::strong_count(&b));
    // b の次の要素 = {:?}
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // a を変更後の b の参照カウント = {}
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // a を変更後の a の参照カウント = {}
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // 次の行のコメントを外して循環していると確認してください; スタックオーバーフローします
    // println!("a next item = {:?}", a.tail());        // a の次の要素 = {:?}
}
```

<!--
<span class="caption">Listing 15-26: Creating a reference cycle of two `List`
values pointing to each other</span>
-->

<span class="caption">リスト 15-26: 2 つの`List`値がお互いを指して循環参照する</span>

<!--
in the variable `a` or `b`がかかる先が不明瞭だが、コード例を見る限り、この訳が合っているようだ
-->

<!--
We create an `Rc<List>` instance holding a `List` value in the variable `a`
with an initial list of `5, Nil`. We then create an `Rc<List>` instance
holding another `List` value in the variable `b` that contains the value 10 and
points to the list in `a`.
-->

最初のリストが`5, Nil`の`List`値を保持する`Rc<List>`インスタンスを変数`a`に生成します。
そして、値 10 と`a`のリストを指す別の`List`値を保持する`Rc<List>`インスタンスを変数`b`に生成します。

<!--
We modify `a` so it points to `b` instead of `Nil`, creating a cycle. We
do that by using the `tail` method to get a reference to the
`RefCell<Rc<List>>` in `a`, which we put in the variable `link`. Then we use
the `borrow_mut` method on the `RefCell<Rc<List>>` to change the value inside
from an `Rc<List>` that holds a `Nil` value to the `Rc<List>` in `b`.
-->

`a`が`Nil`ではなく`b`を指すように変更して、循環させます。`tail`メソッドを使用して、
`a`の`RefCell<Rc<List>>`への参照を得ることで循環させて、この参照は変数`link`に配置します。
それから`RefCell<Rc<List>>`の`borrow_mut`メソッドを使用して中の値を`Nil`値を持つ`Rc<List>`から、
`b`の`Rc<List>`に変更します。

<!--
When we run this code, keeping the last `println!` commented out for the
moment, we’ll get this output:
-->

最後の`println!`を今だけコメントアウトしたまま、このコードを実行すると、こんな出力が得られます：

```text
a initial rc count = 1
a next item = Some(RefCell { value: Nil })
a rc count after b creation = 2
b initial rc count = 1
b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
b rc count after changing a = 2
a rc count after changing a = 2
```

<!--
The reference count of the `Rc<List>` instances in both `a` and `b` are 2
after we change the list in `a` to point to `b`. At the end of `main`, Rust
will try to drop `b` first, which will decrease the count in each of the
`Rc<List>` instances in `a` and `b` by 1.
-->

`a`のリストを`b`を指すように変更した後の`a`と`b`の`Rc<List>`インスタンスの参照カウントは 2 です。
`main`の終端で、コンパイラはまず`b`をドロップしようとし、`a`と`b`の各`Rc<List>`インスタンスのカウントを 1 減らします。

<!--
However, because `a` is still referencing the `Rc<List>` that was in `b`, that
`Rc<List>` has a count of 1 rather than 0, so the memory the `Rc<List>` has on
the heap won’t be dropped. The memory will just sit there with a count of 1,
forever. To visualize this reference cycle, we’ve created a diagram in Figure
15-4:
-->

しかしながら、それでも`a`は`b`にあった`Rc<List>`を参照しているので、その`Rc<List>`のカウントは 0 ではなく 1 になり、
その`Rc<List>`がヒープに確保していたメモリはドロップされません。メモリはただ、カウント 1 のままそこに永遠に居座るのです。
この循環参照を可視化するために、図 15-4 に図式を作成しました：

<!--
<img alt="Reference cycle of lists" src="img/trpl15-04.svg" class="center" />
-->

<img alt="リストの循環参照" src="img/trpl15-04.svg" class="center" />

<!--
<span class="caption">Figure 15-4: A reference cycle of lists `a` and `b`
pointing to each other</span>
-->

<span class="caption">図 15-4: お互いを指すリスト`a`と`b`の循環参照</span>

<!--
If you uncomment the last `println!` and run the program, Rust will try to
print this cycle with `a` pointing to `b` pointing to `a` and so forth until it
overflows the stack.
-->

最後の`println!`のコメントを外してプログラムを実行したら、`a`が`b`を指して、`b`が`a`を指してと、
スタックがオーバーフローするまでコンパイラはこの循環を出力しようとするでしょう。

<!--
In this case, right after we create the reference cycle, the program ends. The
consequences of this cycle aren’t very dire. However, if a more complex program
allocated lots of memory in a cycle and held onto it for a long time, the
program would use more memory than it needed and might overwhelm the system,
causing it to run out of available memory.
-->

この場合、循環参照を作る直後にプログラムは終了します。この循環の結果は、それほど悲壮なものではありません。しかしながら、
より複雑なプログラムが多くのメモリを循環で確保し長い間その状態を保ったら、プログラムは必要以上のメモリを使用し、
使用可能なメモリを枯渇させてシステムを参らせてしまう可能性があります。

<!--
Creating reference cycles is not easily done, but it’s not impossible either.
If you have `RefCell<T>` values that contain `Rc<T>` values or similar nested
combinations of types with interior mutability and reference counting, you must
ensure that you don’t create cycles; you can’t rely on Rust to catch them.
Creating a reference cycle would be a logic bug in your program that you should
use automated tests, code reviews, and other software development practices to
minimize.
-->

循環参照は簡単にできることではありませんが、不可能というわけでもありません。
`Rc<T>`値を含む`RefCell<T>`値があるなどの内部可変性と参照カウントのある型がネストして組み合わさっていたら、
循環していないことを保証しなければなりません; コンパイラがそれを捕捉することを信頼できないのです。
循環参照をするのは、自動テストやコードレビューなどの他のソフトウェア開発手段を使用して最小化すべきプログラム上のロジックバグでしょう。

<!--
Another solution for avoiding reference cycles is reorganizing your data
structures so that some references express ownership and some references don’t.
As a result, you can have cycles made up of some ownership relationships and
some non-ownership relationships, and only the ownership relationships affect
whether or not a value can be dropped. In Listing 15-25, we always want `Cons`
variants to own their list, so reorganizing the data structure isn’t possible.
Let’s look at an example using graphs made up of parent nodes and child nodes
to see when non-ownership relationships are an appropriate way to prevent
reference cycles.
-->

循環参照を回避する別の解決策は、ある参照は所有権を表現して他の参照はしないというようにデータ構造を再構成することです。
結果として、所有権のある関係と所有権のない関係からなる循環ができ、所有権のある関係だけが、値がドロップされうるかどうかに影響します。
リスト 15-25 では、常に`Cons`列挙子にリストを所有してほしいので、データ構造を再構成することはできません。
親ノードと子ノードからなるグラフを使った例に目を向けて、どんな時に所有権のない関係が循環参照を回避するのに適切な方法になるか確認しましょう。

<!--
### Preventing Reference Cycles: Turning an `Rc<T>` into a `Weak<T>`
-->

### 循環参照を回避する：`Rc<T>`を`Weak<T>`に変換する

<!--
So far, we’ve demonstrated that calling `Rc::clone` increases the
`strong_count` of an `Rc<T>` instance, and an `Rc<T>` instance is only cleaned
up if its `strong_count` is 0. You can also create a *weak reference* to the
value within an `Rc<T>` instance by calling `Rc::downgrade` and passing a
reference to the `Rc<T>`. When you call `Rc::downgrade`, you get a smart
pointer of type `Weak<T>`. Instead of increasing the `strong_count` in the
`Rc<T>` instance by 1, calling `Rc::downgrade` increases the `weak_count` by 1.
The `Rc<T>` type uses `weak_count` to keep track of how many `Weak<T>`
references exist, similar to `strong_count`. The difference is the `weak_count`
doesn’t need to be 0 for the `Rc<T>` instance to be cleaned up.
-->

ここまで、`Rc::clone`を呼び出すと`Rc<T>`インスタンスの`strong_count`が増えることと、
`strong_count`が 0 になった時に`Rc<T>`インスタンスは片付けられることをデモしてきました。
`Rc::downgrade`を呼び出し、`Rc<T>`への参照を渡すことで、`Rc<T>`インスタンス内部の値への*弱い参照*(weak reference) を作ることもできます。
`Rc::downgrade`を呼び出すと、型`Weak<T>`のスマートポインタが得られます。
`Rc<T>`インスタンスの`strong_count`を 1 増やす代わりに、`Rc::downgrade`を呼び出すと、`weak_count`が 1 増えます。
`strong_count`同様、`Rc<T>`型は`weak_count`を使用して、幾つの`Weak<T>`参照が存在しているかを追跡します。
違いは、`Rc<T>`が片付けられるのに、`weak_count`が 0 である必要はないということです。

<!--
Strong references are how you can share ownership of an `Rc<T>` instance. Weak
references don’t express an ownership relationship. They won’t cause a
reference cycle because any cycle involving some weak references will be broken
once the strong reference count of values involved is 0.
-->

強い参照は、`Rc<T>`インスタンスの所有権を共有する方法です。弱い参照は、所有権関係を表現しません。
ひとたび、関係する値の強い参照カウントが 0 になれば、弱い参照が関わる循環はなんでも破壊されるので、
循環参照にはなりません。

<!--
Because the value that `Weak<T>` references might have been dropped, to do
anything with the value that a `Weak<T>` is pointing to, you must make sure the
value still exists. Do this by calling the `upgrade` method on a `Weak<T>`
instance, which will return an `Option<Rc<T>>`. You’ll get a result of `Some`
if the `Rc<T>` value has not been dropped yet and a result of `None` if the
`Rc<T>` value has been dropped. Because `upgrade` returns an `Option<T>`, Rust
will ensure that the `Some` case and the `None` case are handled, and there
won't be an invalid pointer.
-->

`Weak<T>`が参照する値はドロップされてしまっている可能性があるので、`Weak<T>`が指す値に何かをするには、
値がまだ存在することを確認しなければなりません。`Weak<T>`の`upgrade`メソッドを呼び出すことでこれをしてください。
このメソッドは`Option<Rc<T>>`を返します。`Rc<T>`値がまだドロップされていなければ、`Some`の結果が、
`Rc<T>`値がドロップ済みなら、`None`の結果が得られます。`upgrade`が`Option<T>`を返すので、
コンパイラは、`Some`ケースと`None`ケースが扱われていることを確かめてくれ、無効なポインタは存在しません。

<!--
As an example, rather than using a list whose items know only about the next
item, we’ll create a tree whose items know about their children items *and*
their parent items.
-->

例として、要素が次の要素を知っているだけのリストを使うのではなく、要素が子要素*と*親要素を知っている木を作りましょう。

<!--
#### Creating a Tree Data Structure: a `Node` with Child Nodes
-->

#### 木データ構造を作る：子ノードのある`Node`

<!--
To start, we’ll build a tree with nodes that know about their child nodes.
We’ll create a struct named `Node` that holds its own `i32` value as well as
references to its children `Node` values:
-->

手始めに子ノードを知っているノードのある木を構成します。独自の`i32`値と子供の`Node`値への参照を抱える`Node`という構造体を作ります：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
```

<!--
We want a `Node` to own its children, and we want to share that ownership with
variables so we can access each `Node` in the tree directly. To do this, we
define the `Vec<T>` items to be values of type `Rc<Node>`. We also want to
modify which nodes are children of another node, so we have a `RefCell<T>` in
`children` around the `Vec<Rc<Node>>`.
-->

`Node`に子供を所有してほしく、木の各`Node`に直接アクセスできるよう、その所有権を変数と共有したいです。
こうするために、`Vec<T>`要素を型`Rc<Node>`の値になるよう定義しています。どのノードが他のノードの子供になるかも変更したいので、
`Vec<Rc<Node>>`の周りの`children`を`RefCell<T>`にしています。

<!--
Next, we’ll use our struct definition and create one `Node` instance named
`leaf` with the value 3 and no children, and another instance named `branch`
with the value 5 and `leaf` as one of its children, as shown in Listing 15-27:
-->

次にこの構造体定義を使って値 3 と子供なしの`leaf`という 1 つの`Node`インスタンスと、
値 5 と`leaf`を子要素の一つとして持つ`branch`という別のインスタンスを作成します。
リスト 15-27 のようにですね：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::rc::Rc;
# use std::cell::RefCell;
#
# #[derive(Debug)]
# struct Node {
#     value: i32,
#    children: RefCell<Vec<Rc<Node>>>,
# }
#
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
```

<!--
<span class="caption">Listing 15-27: Creating a `leaf` node with no children
and a `branch` node with `leaf` as one of its children</span>
-->

<span class="caption">リスト 15-27: 子供なしの`leaf`ノードと`leaf`を子要素に持つ`branch`ノードを作る</span>

<!--
We clone the `Rc<Node>` in `leaf` and store that in `branch`, meaning the
`Node` in `leaf` now has two owners: `leaf` and `branch`. We can get from
`branch` to `leaf` through `branch.children`, but there’s no way to get from
`leaf` to `branch`. The reason is that `leaf` has no reference to `branch` and
doesn’t know they’re related. We want `leaf` to know that `branch` is its
parent. We’ll do that next.
-->

`leaf`の`Rc<Node>`をクローンし、`branch`に格納しているので、`leaf`の`Node`は`leaf`と`branch`という 2 つの所有者を持つことになります。
`branch.children`を通して`branch`から`leaf`へ辿ることはできるものの、`leaf`から`branch`へ辿る方法はありません。
理由は、`leaf`には`branch`への参照がなく、関係していることを知らないからです。`leaf`に`branch`が親であることを知ってほしいです。
次はそれを行います。

<!--
#### Adding a Reference from a Child to Its Parent
-->

#### 子供から親に参照を追加する

<!--
To make the child node aware of its parent, we need to add a `parent` field to
our `Node` struct definition. The trouble is in deciding what the type of
`parent` should be. We know it can’t contain an `Rc<T>`, because that would
create a reference cycle with `leaf.parent` pointing to `branch` and
`branch.children` pointing to `leaf`, which would cause their `strong_count`
values to never be 0.
-->

子供に親の存在を気付かせるために、`Node`構造体定義に`parent`フィールドを追加する必要があります。
`parent`の型を決める際に困ったことになります。`Rc<T>`を含むことができないのはわかります。
そうしたら、`leaf.parent`が`branch`を指し、`branch.children`が`leaf`を指して循環参照になり、
`strong_count`値が絶対に 0 にならなくなってしまうからです。

<!--
Thinking about the relationships another way, a parent node should own its
children: if a parent node is dropped, its child nodes should be dropped as
well. However, a child should not own its parent: if we drop a child node, the
parent should still exist. This is a case for weak references!
-->

この関係を別の方法で捉えると、親ノードは子供を所有すべきです：親ノードがドロップされたら、
子ノードもドロップされるべきなのです。ですが、子供は親を所有するべきではありません：
子ノードをドロップしても、親はまだ存在するべきです。弱い参照を使う場面ですね！

<!--
So instead of `Rc<T>`, we’ll make the type of `parent` use `Weak<T>`,
specifically a `RefCell<Weak<Node>>`. Now our `Node` struct definition looks
like this:
-->

従って、`Rc<T>`の代わりに`parent`の型を`Weak<T>`を使ったもの、具体的には`RefCell<Weak<Node>>`にします。
さあ、`Node`構造体定義はこんな見た目になりました：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

<!--
A node will be able to refer to its parent node but doesn’t own its parent.
In Listing 15-28, we update `main` to use this new definition so the `leaf`
node will have a way to refer to its parent, `branch`:
-->

ノードは親ノードを参照できるものの、所有はしないでしょう。リスト 15-28 で、
`leaf`ノードが親の`branch`を参照できるよう、この新しい定義を使用するように`main`を更新します：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::rc::{Rc, Weak};
# use std::cell::RefCell;
#
# #[derive(Debug)]
# struct Node {
#     value: i32,
#     parent: RefCell<Weak<Node>>,
#     children: RefCell<Vec<Rc<Node>>>,
# }
#
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // leaf の親 = {:?}
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

<!--
<span class="caption">Listing 15-28: A `leaf` node with a weak reference to its
parent node `branch`</span>
-->

<span class="caption">リスト 15-28: 親ノードの`branch`への弱い参照がある`leaf`ノード</span>

<!--
Creating the `leaf` node looks similar to how creating the `leaf` node looked
in Listing 15-27 with the exception of the `parent` field: `leaf` starts out
without a parent, so we create a new, empty `Weak<Node>` reference instance.
-->

`leaf`ノードを作成することは、`parent`フィールドの例外を除いてリスト 15-27 での`leaf`ノードの作成法の見た目に似ています：
`leaf`は親なしで始まるので、新しく空の`Weak<Node>`参照インスタンスを作ります。

<!--
At this point, when we try to get a reference to the parent of `leaf` by using
the `upgrade` method, we get a `None` value. We see this in the output from the
first `println!` statement:
-->

この時点で`upgrade`メソッドを使用して`leaf`の親への参照を得ようとすると、`None`値になります。
このことは、最初の`println!`文の出力でわかります：

```text
leaf parent = None
```

<!--
When we create the `branch` node, it will also have a new `Weak<Node>`
reference in the `parent` field, because `branch` doesn’t have a parent node.
We still have `leaf` as one of the children of `branch`. Once we have the
`Node` instance in `branch`, we can modify `leaf` to give it a `Weak<Node>`
reference to its parent. We use the `borrow_mut` method on the
`RefCell<Weak<Node>>` in the `parent` field of `leaf`, and then we use the
`Rc::downgrade` function to create a `Weak<Node>` reference to `branch` from
the `Rc<Node>` in `branch.`
-->

`branch`ノードを作る際、`branch`には親ノードがないので、こちらも`parent`フィールドには新しい`Weak<Node>`参照が入ります。
それでも、`leaf`は`branch`の子供になっています。一旦`branch`に`Node`インスタンスができたら、
`leaf`を変更して親への`Weak<Node>`参照を与えることができます。`leaf`の`parent`フィールドには、
`RefCell<Weak<Node>>`の`borrow_mut`メソッドを使用して、それから`Rc::downgrade`関数を使用して、
`branch`の`Rc<Node>`から`branch`への`Weak<Node>`参照を作ります。

<!--
When we print the parent of `leaf` again, this time we’ll get a `Some` variant
holding `branch`: now `leaf` can access its parent! When we print `leaf`, we
also avoid the cycle that eventually ended in a stack overflow like we had in
Listing 15-26; the `Weak<Node>` references are printed as `(Weak)`:
-->

再度`leaf`の親を出力すると、今度は`branch`を保持する`Some`列挙子が得られます：これで`leaf`が親にアクセスできるようになったのです！
`leaf`を出力すると、リスト 15-26 で起こっていたような最終的にスタックオーバーフローに行き着く循環を避けることもできます;
`Weak<Node>`参照は、`(Weak)`と出力されます：

```text
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

<!--
The lack of infinite output indicates that this code didn’t create a reference
cycle. We can also tell this by looking at the values we get from calling
`Rc::strong_count` and `Rc::weak_count`.
-->

無限の出力が欠けているということは、このコードは循環参照しないことを示唆します。
このことは、`Rc::strong_count`と`Rc::weak_count`を呼び出すことで得られる値を見てもわかります。

<!--
#### Visualizing Changes to `strong_count` and `weak_count`
-->

#### `strong_count`と`weak_count`への変更を可視化する

<!--
Let’s look at how the `strong_count` and `weak_count` values of the `Rc<Node>`
instances change by creating a new inner scope and moving the creation of
`branch` into that scope. By doing so, we can see what happens when `branch` is
created and then dropped when it goes out of scope. The modifications are shown
in Listing 15-29:
-->

新しい内部スコープを作り、`branch`の作成をそのスコープに移動することで、
`Rc<Node>`インスタンスの`strong_count`と`weak_count`値がどう変化するかを眺めましょう。
そうすることで、`branch`が作成され、それからスコープを抜けてドロップされる時に起こることが確認できます。
変更は、リスト 15-29 に示してあります：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::rc::{Rc, Weak};
# use std::cell::RefCell;
#
# #[derive(Debug)]
# struct Node {
#     value: i32,
#     parent: RefCell<Weak<Node>>,
#     children: RefCell<Vec<Rc<Node>>>,
# }
#
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        // leaf の strong_count = {}, weak_count = {}
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            // branch の strong_count = {}, weak_count = {}
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

<!--
<span class="caption">Listing 15-29: Creating `branch` in an inner scope and
examining strong and weak reference counts</span>
-->

<span class="caption">リスト 15-29: 内側のスコープで`branch`を作成し、強弱参照カウントを調査する</span>

<!--
4 行目後半、カッコ内、for は接続詞の用法かと思ったが、文ではなかった。for S to V のように訳した
通常 for の後は、名詞が来るため、そう書いているだけだろうか
-->

<!--
After `leaf` is created, its `Rc<Node>` has a strong count of 1 and a weak
count of 0. In the inner scope, we create `branch` and associate it with
`leaf`, at which point when we print the counts, the `Rc<Node>` in `branch`
will have a strong count of 1 and a weak count of 1 (for `leaf.parent` pointing
to `branch` with a `Weak<Node>`). When we print the counts in `leaf`, we’ll see
it will have a strong count of 2, because `branch` now has a clone of the
`Rc<Node>` of `leaf` stored in `branch.children`, but will still have a weak
count of 0.
-->

`leaf`作成後、その`Rc<Node>`の強カウントは 1、弱カウントは 0 になります。内側のスコープで`branch`を作成し、
`leaf`に紐付け、この時点でカウントを出力すると、`branch`の`Rc<Node>`の強カウントは 1、
弱カウントも 1 になります (`leaf.parent`が`Weak<Node>`で`branch`を指しているため)。
`leaf`のカウントを出力すると、強カウントが 2 になっていることがわかります。`branch`が今は、
`branch.children`に格納された`leaf`の`Rc<Node>`のクローンを持っているからですが、
それでも弱カウントは 0 でしょう。

<!--
When the inner scope ends, `branch` goes out of scope and the strong count of
the `Rc<Node>` decreases to 0, so its `Node` is dropped. The weak count of 1
from `leaf.parent` has no bearing on whether or not `Node` is dropped, so we
don’t get any memory leaks!
-->

内側のスコープが終わると、`branch`はスコープを抜け、`Rc<Node>`の強カウントは 0 に減るので、
この`Node`はドロップされます。`leaf.parent`からの弱カウント 1 は、`Node`がドロップされるか否かには関係ないので、
メモリリークはしないのです！

<!--
If we try to access the parent of `leaf` after the end of the scope, we’ll get
`None` again. At the end of the program, the `Rc<Node>` in `leaf` has a strong
count of 1 and a weak count of 0, because the variable `leaf` is now the only
reference to the `Rc<Node>` again.
-->

このスコープの終端以後に`leaf`の親にアクセスしようとしたら、再び`None`が得られます。
プログラムの終端で`leaf`の`Rc<Node>`の強カウントは 1、弱カウントは 0 です。
変数`leaf`が今では`Rc<Node>`への唯一の参照に再度なったからです。

<!--
All of the logic that manages the counts and value dropping is built into
`Rc<T>` and `Weak<T>` and their implementations of the `Drop` trait. By
specifying that the relationship from a child to its parent should be a
`Weak<T>` reference in the definition of `Node`, you’re able to have parent
nodes point to child nodes and vice versa without creating a reference cycle
and memory leaks.
-->

カウントや値のドロップを管理するロジックは全て、`Rc<T>`や`Weak<T>`とその`Drop`トレイトの実装に組み込まれています。
`Node`の定義で子供から親への関係は`Weak<T>`参照になるべきと指定することで、
循環参照やメモリリークを引き起こさずに親ノードに子ノードを参照させたり、その逆を行うことができます。

<!--
## Summary
-->

## まとめ

<!--
This chapter covered how to use smart pointers to make different guarantees and
trade-offs than those Rust makes by default with regular references. The
`Box<T>` type has a known size and points to data allocated on the heap. The
`Rc<T>` type keeps track of the number of references to data on the heap so
that data can have multiple owners. The `RefCell<T>` type with its interior
mutability gives us a type that we can use when we need an immutable type but
need to change an inner value of that type; it also enforces the borrowing
rules at runtime instead of at compile time.
-->

この章は、スマートポインタを使用して Rust が既定で普通の参照に対して行うのと異なる保証や代償を行う方法を講義しました。
`Box<T>`型は、既知のサイズで、ヒープに確保されたデータを指します。`Rc<T>`型は、ヒープのデータへの参照の数を追跡するので、
データは複数の所有者を保有できます。内部可変性のある`RefCell<T>`型は、不変型が必要だけれども、
その型の中の値を変更する必要がある時に使用できる型を与えてくれます; また、コンパイル時ではなく実行時に借用規則を強制します。

<!--
Also discussed were the `Deref` and `Drop` traits, which enable a lot of the
functionality of smart pointers. We explored reference cycles that can cause
memory leaks and how to prevent them using `Weak<T>`.
-->

`Deref`と`Drop`トレイトについても議論しましたね。これらは、スマートポインタの多くの機能を可能にしてくれます。
メモリリークを引き起こす循環参照と`Weak<T>`でそれを回避する方法も探究しました。

<!--
If this chapter has piqued your interest and you want to implement your own
smart pointers, check out [“The Rustonomicon”][nomicon] for more useful
information.
-->

この章で興味をそそられ、独自のスマートポインタを実装したくなったら、もっと役に立つ情報を求めて、
[“The Rustonomicon”][nomicon]をチェックしてください。

> 訳注：日本語版の The Rustonomicon は[こちら][nomicon-ja]です。

[nomicon]: https://doc.rust-lang.org/stable/nomicon/
[nomicon-ja]: https://doc.rust-jp.rs/rust-nomicon-ja/index.html

<!--
Next, we’ll talk about concurrency in Rust. You’ll even learn about a few new
smart pointers.
-->

次は、Rust での並行性について語ります。もういくつか新しいスマートポインタについてさえも学ぶでしょう。
