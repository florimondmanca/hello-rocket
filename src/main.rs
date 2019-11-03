/*
This is a "crate file".
See: https://doc.rust-lang.org/rust-by-example/crates.html
*/

/*
This line allows the use of the specified experimental features.
(Rocket relies on some currently unstable features of Rust, which
is why we need to use the nightly Rust build to use Rocket.)
*/
#![feature(proc_macro_hygiene, decl_macro)]

/*
Link the library "rocket" to this crate.
Public items from the "rocket" library can be accessed via `rocket::...`,
as for `rocket::get()` and `rocket::ignite()` below.
See: https://doc.rust-lang.org/rust-by-example/crates/link.html
*/
extern crate rocket;
use rocket_contrib::serve::StaticFiles;

/*
The `#[...]` syntax defines an attribute of the `index()` function.
See: https://doc.rust-lang.org/reference/attributes.html

For example, importing/exporting macros of a library is done by adding
the `#[macro_use]` and `#[macro_export]` attributes to a macro.
See: https://doc.rust-lang.org/1.7.0/book/macros.html#scoping-and-macro-importexport

`'static` is the lifetime of the returned `str` object.
See: https://doc.rust-lang.org/rust-by-example/scope/lifetime.html
The `static` lifetime is the longest one - the `str` will live until the end of the program.
(More specifically, it will be baked into the program code, and reused.)
See: https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html

`&` indicates that the function returns a reference to the string (a.k.a. borrowing),
instead of a value.
See: https://doc.rust-lang.org/rust-by-example/scope/borrow.html

It seems that `&'static str` is an idiom often only used for string literals.
There is another type of string, `String`, which allows modification.
See: https://www.quora.com/Why-does-Rust-have-two-different-string-types-static-str-and-String
*/
#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/*
The `!` in `routes!` indicates that it is a macro.
Macros are a metaprogramming feature of Rust.
Like functions, they enable reusing logic, but unlike functions,
they result in the expansion of the underlying macro's code.
They are often used to achieve variadic arguments (e.g. println!).
See: https://doc.rust-lang.org/1.7.0/book/macros.html

`.mount()` and `.launch()` are method calls.
Methods are defined on implementations of a struct
Struct
See: https://doc.rust-lang.org/1.7.0/book/method-syntax.html
*/
fn main() {
    rocket::ignite()
        .mount("/", rocket::routes![index])
        .mount("/public", StaticFiles::from("public"))
        .launch();
}
