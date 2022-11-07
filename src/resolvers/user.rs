use mysql::{from_row, params, prelude::*, Error as DBError, Row};

use crate::schemas::{root::Context, user::User};

impl User {
    pub(crate) fn from_row(row: Row) -> Self {
        let (id, name, email) = from_row(row);
        User { id, name, email }
    }
    
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
}

