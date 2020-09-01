// ANCHOR: here
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    // （略）
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}

fn function2() -> io::Result<()> {
    // --snip--
    // （略）
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}
// ANCHOR_END: here
