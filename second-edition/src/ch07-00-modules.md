<!-- # Using Modules to Reuse and Organize Code -->

# モジュールを使用してコードを体系化し、再利用する

<!-- When you start writing programs in Rust, your code might live solely in the -->
<!-- `main` function. As your code grows, you’ll eventually move functionality into -->
<!-- other functions for reuse and better organization. By splitting your code into -->
<!-- smaller chunks, each chunk is easier to understand on its own. But what happens -->
<!-- if you have too many functions? Rust has a module system that enables the reuse -->
<!-- of code in an organized fashion. -->

Rustでのプログラミングをし始めた頃は、コードは全て`main`関数内に収まったかもしれません。コードが肥大化するにつれ、
最終的に機能を別の関数に移して再利用性とまとまりを高めるでしょう。コードを細切りにすることで、
個々のコード片はそれだけで理解しやすくなります。しかし、あまりにも多くの関数があったらどうなるでしょうか？
Rustにはコードの再利用を体系化された形で行うことのできるモジュールシステムが組み込まれています。

<!-- In the same way that you extract lines of code into a function, you can extract -->
<!-- functions (and other code, like structs and enums) into different modules. A -->
<!-- *module* is a namespace that contains definitions of functions or types, and -->
<!-- you can choose whether those definitions are visible outside their module -->
<!-- (public) or not (private). Here’s an overview of how modules work: -->

コードを関数に抽出するのと同様に、関数(や他のコード、構造体やenumなど)を異なるモジュールに抽出することができます。
*モジュール*とは、関数や型定義を含む名前空間のことで、それらの定義がモジュール外からも見えるようにするか(public)否か(private)は、
選択することができます。以下が、モジュールの動作法の概要です:

<!-- * The `mod` keyword declares a new module. Code within the module appears -->
<!--   either immediately following this declaration within curly braces or in -->
<!--   another file. -->
<!-- * By default, functions, types, constants, and modules are private. The `pub` -->
<!--   keyword makes an item public and therefore visible outside its namespace. -->
<!-- * The `use` keyword brings modules, or the definitions inside modules, into -->
<!--   scope so it’s easier to refer to them. -->

* `mod`キーワードで新規モジュールを宣言します。モジュール内のコードは、この宣言の直後の波かっこ内か、
別のファイルに存在します。
* 標準では、関数、型、定数、モジュールはプライベートです。`pub`キーワードで要素は公開され、
名前空間の外からも見えるようになります。
* `use`キーワードでモジュールやモジュール内の定義をスコープに入れることができるので、
参照するのが楽になります。

<!-- We’ll look at each of these parts to see how they fit into the whole. -->

この部品の各々を見て、それらが全体にどうはまり込むかを理解します。
