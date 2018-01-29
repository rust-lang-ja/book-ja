<!-- # Error Handling -->

# エラー処理

<!-- Rust’s commitment to reliability extends to error handling. Errors are a fact -->
<!-- of life in software, so Rust has a number of features for handling situations -->
<!-- in which something goes wrong. In many cases, Rust requires you to acknowledge -->
<!-- the possibility of an error occurring and take some action before your code -->
<!-- will compile. This requirement makes your program more robust by ensuring that -->
<!-- you’ll discover errors and handle them appropriately before you’ve deployed -->
<!-- your code to production! -->

Rustの信頼性への傾倒は、エラー処理にも及びます。ソフトウェアにおいて、エラーは生きている証しです。
従って、Rustには何かがおかしくなる場面を扱う機能がたくさんあります。多くの場面で、
コンパイラは、エラー発生の可能性を知り、コードのコンパイルが通るまでに何かしら対応を行うことを要求してきます。
この要求により、エラーを発見し、コードを実用に供する前に適切に対処していることを確認することでプログラムを頑健なものにしてくれるのです！

<!-- Rust groups errors into two major categories: *recoverable* and *unrecoverable* -->
<!-- errors. Recoverable errors are situations in which it’s reasonable to report -->
<!-- the problem to the user and retry the operation, like a file not found error. -->
<!-- Unrecoverable errors are always symptoms of bugs, like trying to access a -->
<!-- location beyond the end of an array. -->

Rustでは、エラーは大きく二つに分類されます: *回復可能*と*回復不能*なエラーです。回復可能なエラーとは、
問題をユーザに報告し、処理を再試行することが合理的になる場面のことです。例えば、ファイルが見つからないような場面ですね。
回復不能なエラーは、常にバグの兆候です。例えば、配列の境界を超えた箇所にアクセスしようとすることなどです。

<!-- Most languages don’t distinguish between these two kinds of errors and handle -->
<!-- both in the same way using mechanisms like exceptions. Rust doesn’t have -->
<!-- exceptions. Instead, it has the value `Result<T, E>` for recoverable errors and -->
<!-- the `panic!` macro that stops execution when it encounters unrecoverable -->
<!-- errors. This chapter covers calling `panic!` first and then talks about -->
<!-- returning `Result<T, E>` values. Additionally, we’ll explore considerations to -->
<!-- take into account when deciding whether to try to recover from an error or to -->
<!-- stop execution. -->

多くの言語では、この2種のエラーを区別することはなく、例外などの機構を使用して同様に扱います。
Rustには例外が存在しません。代わりに、回復可能なエラーには`Result<T, E>`値を、回復不能なエラーに遭遇したら、
`panic!`マクロで実行を中止します。この章では、まず`panic!`の呼び出しを解説し、
それから`Result<T, E>`を戻り値にする話をします。加えて、エラーからの回復を試みるか、
実行を中止するか決定する際に考慮すべき事項についても、掘り下げましょう。
