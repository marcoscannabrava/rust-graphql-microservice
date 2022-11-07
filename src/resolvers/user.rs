use mysql::{from_row, params, prelude::*, Error as DBError, Row};

use crate::schemas::{
    root::Context,
    user::{User, UserInput},
};

impl User {
    pub(crate) fn from_row(row: Row) -> Self {
        let (id, name, email) = from_row(row);
        User { id, name, email }
    }

    // --------
    // Queries
    // --------

    pub fn get_by_id(id: &String, context: &Context) -> Option<User> {
        let mut conn = context.db_pool.get().unwrap();
        let user: Result<Option<Row>, DBError> =
            conn.exec_first("SELECT * FROM user WHERE id=:id", params! {"id" => id});
        match user {
            Ok(user) => {
                let (id, name, email) = from_row(user.unwrap());
                Some(User { id, name, email })
            }
            Err(_err) => None,
        }
    }

    pub fn get_all(context: &Context) -> Vec<User> {
        let mut conn = context.db_pool.get().unwrap();
        conn.exec("SELECT * FROM user", ())
            .unwrap()
            .into_iter()
            .map(User::from_row)
            .collect()
    }

    #[allow(dead_code)]
    pub fn filter(field: &String, value: &String, context: &Context) -> Vec<User> {
        let mut conn = context.db_pool.get().unwrap();
        conn.exec(
            "SELECT * FROM user WHERE :field=:value",
            params! {"field" => field, "value" => value},
        )
        .unwrap()
        .into_iter()
        .map(User::from_row)
        .collect()
    }

    // --------
    // Commands
    // --------

    pub fn insert(context: &Context, user: UserInput) -> Result<User, DBError> {
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
            Err(err) => Err(err),
        }
    }
}
