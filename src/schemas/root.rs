use juniper::{
    graphql_object, graphql_value, EmptySubscription, FieldError, FieldResult, RootNode,
};

use super::{
    product::{Product, ProductInput},
    user::{User, UserInput},
};
use crate::db::Pool;

pub struct Context {
    pub db_pool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all users")]
    fn users(context: &Context) -> FieldResult<Vec<User>> {
        Ok(User::get_all(context))
        // TODO: add optional params (Enum?) to call User::filter and filter by any column
    }

    #[graphql(description = "Get Single user reference by user ID")]
    fn user(context: &Context, id: String) -> FieldResult<User> {
        let user = User::get_by_id(&id, context);
        match user {
            Some(user) => return Ok(user),
            None => {
                return Err(FieldError::new(
                    "User Not Found",
                    graphql_value!({ "not_found": "user not found" }),
                ))
            }
        }
    }

    #[graphql(description = "List of all products")]
    fn products(context: &Context) -> FieldResult<Vec<Product>> {
        Ok(Product::get_all(context))
    }

    #[graphql(description = "Get Single product reference by user ID")]
    fn product(context: &Context, id: String) -> FieldResult<Product> {
        match Product::get_by_id(&id, context) {
            Some(product) => return Ok(product),
            None => {
                return Err(FieldError::new(
                    "Product Not Found",
                    graphql_value!({ "not_found": "product not found" }),
                ))
            }
        }
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    // Creates User
    fn create_user(context: &Context, user: UserInput) -> FieldResult<User> {
        match User::insert(context, user) {
            Ok(user) => Ok(user),
            Err(err) => {
                let msg = err.to_string();
                Err(FieldError::new(
                    "Failed to create new user",
                    graphql_value!({ "internal_error": msg }),
                ))
            }
        }
    }

    // Creates Product
    fn create_product(context: &Context, product: ProductInput) -> FieldResult<Product> {
        match Product::insert(context, product) {
            Ok(product) => Ok(product),
            Err(err) => {
                let msg = err.to_string();
                Err(FieldError::new(
                    "Failed to create new product",
                    graphql_value!({ "internal_error": msg }),
                ))
            }
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
