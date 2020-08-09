# LINQ to entities in Rust. Design paper.

## First implementation step

First step of implementing LINQ in Rust is function-like procedural macro which returns a parsed expression.

For example. This code:
```rust
fn not_deleted(query: impl Quaryable<Item = User>) -> impl Quaryable<Item = User> {
    let not_deleted_expr = expr!(|u| !u.info.is_deleted);
    query.filter(not_deleted_expr)
}
```
should be transformed into something like this:
```rust
// Generated somewhere aroung User type
#[derive(...)]
struct UserReflectionProperties {
    // ...
    info: ReflectionProperty<User, UserInfo>,
}

impl Reflection for User {
    type Properties = UserReflectionProperties;
}

#[derive(...)]
struct UserInfoReflectionProperties {
    // ...
    is_deleted: ReflectionProperty<UserInfo, bool>,
}

impl Reflection for UserInfo {
    type Properties = UserInfoReflectionProperties;
}

// Inplace code
fn not_deleted(query: impl Quaryable<Item = User>) -> impl Quaryable<Item = User> {
    let not_deleted_expr = Expression::unary(
        UnaryOp::Not,
        Expression::prop(
            Expression::prop(
                Expression::argument(),
                |x| x.info
            ),
            |x| x.is_deleted
        )
    );
    query.filter(not_deleted_expr)
}
```
