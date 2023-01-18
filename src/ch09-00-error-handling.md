<!--
# Error Handling
-->

# エラー処理

<!--
Rust’s commitment to reliability extends to error handling. Errors are a fact
of life in software, so Rust has a number of features for handling situations
in which something goes wrong. In many cases, Rust requires you to acknowledge
the possibility of an error and take some action before your code will compile.
This requirement makes your program more robust by ensuring that you'll
discover errors and handle them appropriately before you’ve deployed your code
to production!
-->

Rust の信頼性への傾倒は、エラー処理にも及びます。ソフトウェアにおいて、エラーは生きている証しです。
従って、Rust には何かがおかしくなる場面を扱う機能がたくさんあります。多くの場面で、
コンパイラは、プログラマにエラーの可能性を知り、コードのコンパイルが通るまでに何かしら対応を行うことを要求してきます。
この要求により、エラーを発見し、コードを実用に供する前に適切に対処していることを確認することでプログラムを頑健なものにしてくれるのです！

<!--
Rust groups errors into two major categories: *recoverable* and *unrecoverable*
errors. For a recoverable errors, such as a file not found error, it’s
reasonable to report the problem to the user and retry the operation.
Unrecoverable errors are always symptoms of bugs, like trying to access a
location beyond the end of an array.
-->

Rust では、エラーは大きく二つに分類されます：*回復可能*と*回復不能*なエラーです。
ファイルが見つからないなどの回復可能なエラーには、問題をユーザに報告し、処理を再試行することが合理的になります。
回復不能なエラーは、常にバグの兆候です。例えば、配列の境界を超えた箇所にアクセスしようとすることなどです。

<!--
Most languages don’t distinguish between these two kinds of errors and handle
both in the same way, using mechanisms like exceptions. Rust doesn’t have
exceptions. Instead, it has the value `Result<T, E>` for recoverable errors and
the `panic!` macro that stops execution when the program encounters an
unrecoverable error. This chapter covers calling `panic!` first and then talks
about returning `Result<T, E>` values. Additionally, we’ll explore
considerrations when deciding whether to try to recover from an error or to
stop execution.
-->

多くの言語では、この 2 種のエラーを区別することはなく、例外などの機構を使用して同様に扱います。
Rust には例外が存在しません。代わりに、回復可能なエラーには`Result<T, E>`値があり、
プログラムが回復不能なエラーに遭遇した時には、実行を中止する`panic!`マクロがあります。
この章では、まず`panic!`の呼び出しを講義し、それから`Result<T, E>`を戻り値にする話をします。
加えて、エラーからの回復を試みるか、実行を中止するか決定する際に考慮すべき事項についても、探究しましょう。
