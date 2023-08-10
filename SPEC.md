# Specification of the `view!` macro

## General example

Here is how the macro generally looks like:

```rust
view![
    div(
        id = "some-id",
        class = "some-class1 some-class2",
        class[foo] = move || is_foo.get(),
        class["foo bar"] = move || is_foo_bar.get(),
        custom_prop = "some value",
        div(
            "Press the button: ",
            button(on[click] = move |_| is_clicked.set(true), ["I'm a button"])
        )
    ),
    div(
        "Check the checkbox: ",
        input(
            r#type = "checkbox",
            "type" = "checkbox",
            on[click] = move |_| is_checked.set(true),
            prop[checked] = move |_| is_checked.get(),
        )
    ),
    "some-web-component"("some-custom-attribute" = true),
    Await(future = || fetch_monkeys(3), |data| view![p(
        *data,
        " little monkeys, jumping on the bed"
    )]),
    Await(
        AwaitProps {
            blocking: false,
            future: || fetch_monkeys(3),
        },
        |data| view![p(*data, " little monkeys, jumping on the bed")],
    ),
    Await(
        false,
        || fetch_monkeys(3),
        |data| view![p(*data, " little monkeys, jumping on the bed")],
    ),
]
```
