use juniper::{graphql_object, GraphQLInputObject};

use crate::{schemas::{product::Product, root::Context}};

/// User
#[derive(Default, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User Input")]
pub struct UserInput {
    pub name: String,
    pub email: String,
}

#[graphql_object(Context = Context)]
impl User {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn email(&self) -> &str {
        &self.email
    }

    fn products(&self, context: &Context) -> Vec<Product> {
        Product::get_by_user_id(&self.id, context)
    }
}
