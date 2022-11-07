use juniper::GraphQLInputObject;

use crate::{schemas::{root::Context, user::User}};

/// Product
#[derive(Default, Debug)]
pub struct Product {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub price: f64,
}

#[juniper::graphql_object(Context = Context)]
impl Product {
    fn id(&self) -> &str {
        &self.id
    }
    fn user_id(&self) -> &str {
        &self.user_id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn price(&self) -> f64 {
        self.price
    }

    fn user(&self, context: &Context) -> Option<User> {
        User::get_by_id(&self.user_id, context)
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Product Input")]
pub struct ProductInput {
    pub user_id: String,
    pub name: String,
    pub price: f64,
}
