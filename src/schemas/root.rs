use juniper::{
    graphql_object, graphql_value, EmptySubscription, FieldError, FieldResult, RootNode,
};
use mysql::{params, prelude::*, Error as DBError, Row};

use super::{
    product::{Product, ProductInput},
    user::{User, UserInput},
};
use crate::{db::Pool};

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
    }

    #[graphql(description = "Get Single user reference by user ID")]
    fn user(context: &Context, id: String) -> FieldResult<User> {
        let user = User::get_by_id(&id, context);
        match user {
            Some(user) => return Ok(user),
            None => return Err(FieldError::new(
                "User Not Found",
                graphql_value!({ "not_found": "user not found" }),
            ))
        }
    }

    #[graphql(description = "List of all products")]
    fn products(context: &Context) -> FieldResult<Vec<Product>> {
        Ok(Product::get_all(context))
    }

    #[graphql(description = "Get Single user reference by user ID")]
    fn product(context: &Context, id: String) -> FieldResult<Product> {
        let product = Product::get_by_id(&id, context);
        match product {
            Some(product) => return Ok(product),
            None => return Err(FieldError::new(
                "Product Not Found",
                graphql_value!({ "not_found": "product not found" }),
            ))
        }
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    fn create_user(context: &Context, user: UserInput) -> FieldResult<User> {
        let mut conn = context.db_pool.get().unwrap();
        let new_id = uuid::Uuid::new_v4().simple().to_string();

        let insert: Result<Option<Row>, DBError> = conn.exec_first(
            "INSERT INTO user(id, name, email) VALUES(:id, :name, :email)",
            params! {
                "id" => &new_id,
                "name" => &user.name,
                "email" => &user.email,
            },
        );

        match insert {
            Ok(_opt_row) => Ok(User {
                id: new_id,
                name: user.name,
                email: user.email,
            }),
            Err(err) => {
                let msg = match err {
                    DBError::MySqlError(err) => err.message,
                    _ => "internal error".to_owned(),
                };
                Err(FieldError::new(
                    "Failed to create new user",
                    graphql_value!({ "internal_error": msg }),
                ))
            }
        }
    }

    fn create_product(context: &Context, product: ProductInput) -> FieldResult<Product> {
        let mut conn = context.db_pool.get().unwrap();
        let new_id = uuid::Uuid::new_v4().simple().to_string();

        let insert: Result<Option<Row>, DBError> = conn.exec_first(
            "INSERT INTO product(id, user_id, name, price) VALUES(:id, :user_id, :name, :price)",
            params! {
                "id" => &new_id,
                "user_id" => &product.user_id,
                "name" => &product.name,
                "price" => &product.price.to_owned(),
            },
        );

        match insert {
            Ok(_opt_row) => Ok(Product {
                id: new_id,
                user_id: product.user_id,
                name: product.name,
                price: product.price,
            }),
            Err(err) => {
                let msg = match err {
                    DBError::MySqlError(err) => err.message,
                    _ => "internal error".to_owned(),
                };
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
