pub trait Expression {
    type Input;
    type Output;

    fn dynamic_call(&self, input: Self::Input) -> Self::Output;

    fn visit<Visitor: ExpressionVisitor>(&self, visitor: Visitor) -> Visitor::Result;
}

pub trait ExpressionVisitor {
    type Result;

    fn visit_argument<Input>(self, expr: &ArgumentExpression<Input>) -> Self::Result;
    fn visit_property_access<SubExpr: Expression, PropType>(self, expr: &PropertyAccessExpression<SubExpr, PropType>) -> Self::Result;
}

#[derive(Debug, Clone, Default)]
pub struct ArgumentExpression<Input> {
    _marker: core::marker::PhantomData<Input>,
}

impl<Input> ArgumentExpression<Input> {
    pub const fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }
}

impl<Input> Expression for ArgumentExpression<Input> {
    type Input = Input;
    type Output = Input;

    #[inline]
    fn dynamic_call(&self, input: Self::Input) -> Self::Output {
        input
    }

    #[inline]
    fn visit<Visitor: ExpressionVisitor>(&self, visitor: Visitor) -> <Visitor as ExpressionVisitor>::Result {
        visitor.visit_argument(self)
    }
}

#[derive(Clone)]
pub struct PropertyAccessExpression<SubExpr: Expression, PropType> {
    pub sub_expr: SubExpr,
    pub property_name: &'static str,
    pub native_fn: fn(SubExpr::Output) -> PropType,
}

impl<SubExpr: Expression, PropType> PropertyAccessExpression<SubExpr, PropType> {
    #[inline]
    pub fn new(
        sub_expr: SubExpr,
        property_name: &'static str,
        native_fn: fn(SubExpr::Output) -> PropType,
    ) -> Self {
        Self {
            sub_expr,
            property_name,
            native_fn,
        }
    }

    #[inline]
    pub fn prepare(
        property_name: &'static str,
        native_fn: fn(SubExpr::Output) -> PropType,
    ) -> impl Fn(SubExpr) -> Self {
        move |sub_expr| {
            Self::new(
                sub_expr,
                property_name,
                native_fn,
            )
        }
    }
}

impl<SubExpr: Expression, PropType> Expression for PropertyAccessExpression<SubExpr, PropType> {
    type Input = SubExpr::Input;
    type Output = PropType;

    #[inline]
    fn dynamic_call(&self, input: Self::Input) -> Self::Output {
        let sub_expr_output = self.sub_expr.dynamic_call(input);
        (self.native_fn)(sub_expr_output)
    }

    #[inline]
    fn visit<Visitor: ExpressionVisitor>(&self, visitor: Visitor) -> <Visitor as ExpressionVisitor>::Result {
        visitor.visit_property_access(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn property_access_works() {
        #[derive(Debug)]
        struct User {
            id: u64,
            name: String,
            is_deleted: bool,
        }

        fn test_expr<'a, T: core::fmt::Debug + Eq>(
            prop_name: &str,
            prop_expr: impl Expression<Input = &'a User, Output = T>,
            getter: fn(&'a User) -> T,
            users_set: &'a [User],
        ) {
            for user in users_set {
                let expr_result = prop_expr.dynamic_call(user);
                let native_result = getter(user);

                assert_eq!(expr_result, native_result,
                           "Testing {prop_name:?} property of {user:?}. Expected: {expected:?}, actual: {actual:?}",
                           prop_name = prop_name,
                           user = user,
                           expected = native_result,
                           actual = expr_result,
                );
            }
        }

        let users = [
            User { id: 1, name: "The first user".into(), is_deleted: false },
            User { id: 2, name: "The second user".into(), is_deleted: true },
            User { id: 3, name: "The third user".into(), is_deleted: true },
            User { id: 4, name: "The fourth user".into(), is_deleted: false },
        ];

        test_expr(
            "id",
            PropertyAccessExpression::new(ArgumentExpression::new(), "id", |u: &User| u.id),
            |u| u.id,
            &users,
        );
        test_expr(
            "name",
            PropertyAccessExpression::new(ArgumentExpression::new(), "name", |u: &User| u.name.as_str()),
            |u| u.name.as_str(),
            &users,
        );
        test_expr(
            "is_deleted",
            PropertyAccessExpression::new(ArgumentExpression::new(), "is_deleted", |u: &User| u.is_deleted),
            |u| u.is_deleted,
            &users,
        );
    }

    #[test]
    fn nested_property_access_works() {
        #[derive(Debug)]
        struct User {
            id: u64,
            info: UserInfo,
            is_deleted: bool,
        }

        #[derive(Debug)]
        struct UserInfo {
            name: String,
            email: String,
        }

        let user_email_expr = PropertyAccessExpression::new(
            PropertyAccessExpression::new(
                ArgumentExpression::<&User>::new(),
                "info",
                |u| &u.info,
            ),
            "email",
            |i| &i.email,
        );

        let users = [
            User { id: 1, info: UserInfo { name: "The first user".into(), email: "first@mail.com".into() }, is_deleted: false },
            User { id: 2, info: UserInfo { name: "The second user".into(), email: "second@mail.com".into() }, is_deleted: true },
            User { id: 3, info: UserInfo { name: "The third user".into(), email: "third@mail.com".into() }, is_deleted: true },
            User { id: 4, info: UserInfo { name: "The fourth user".into(), email: "fourth@mail.com".into() }, is_deleted: false },
        ];

        for user in &users {
            let expr_result = user_email_expr.dynamic_call(user);
            let native_result = &user.info.email;

            assert_eq!(expr_result, native_result,
                       "Testing \"info.email\" property of {user:?}. Expected: {expected:?}, actual: {actual:?}",
                       user = user,
                       expected = native_result,
                       actual = expr_result,
            );
        }
    }

    #[test]
    fn prop_path_visitor_test() {
        #[derive(Debug)]
        struct User {
            id: u64,
            info: UserInfo,
            is_deleted: bool,
        }

        #[derive(Debug)]
        struct UserInfo {
            name: String,
            email: String,
        }

        struct PropertyPathExtractor;

        impl ExpressionVisitor for PropertyPathExtractor {
            type Result = Vec<&'static str>;

            fn visit_argument<Input>(self, _expr: &ArgumentExpression<Input>) -> Self::Result {
                vec![]
            }

            fn visit_property_access<SubExpr: Expression, PropType>(self, expr: &PropertyAccessExpression<SubExpr, PropType>) -> Self::Result {
                let mut path = expr.sub_expr.visit(PropertyPathExtractor);
                path.push(expr.property_name);
                path
            }
        }

        let user_email_expr = PropertyAccessExpression::new(
            PropertyAccessExpression::new(
                ArgumentExpression::<&User>::new(),
                "info",
                |u| &u.info,
            ),
            "email",
            |i| &i.email,
        );

        assert_eq!(
            user_email_expr.visit(PropertyPathExtractor),
            ["info", "email"]
        )
    }
}
