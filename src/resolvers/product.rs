use mysql::{from_row, params, prelude::*, Error as DBError, Row};

use crate::schemas::{product::{Product, ProductInput}, root::Context};

impl Product {
    pub(crate) fn from_row(row: Row) -> Self {
        let (id, user_id, name, price) = from_row(row);

        Self {
            id,
            user_id,
            name,
            price,
        }
    }

    pub fn get_by_id(id: &String, context: &Context) -> Option<Product> {
        let mut conn = context.db_pool.get().unwrap();
        let product: Result<Option<Row>, DBError> =
            conn.exec_first("SELECT * FROM product WHERE id=:id", params! {"id" => id});
        match product {
            Ok(product) => {
                let (id, user_id, name, price) = from_row(product.unwrap());
                Some(Product {
                    id,
                    user_id,
                    name,
                    price,
                })
            }
            Err(_err) => None,
        }
    }

    pub fn get_by_user_id(user_id: &String, context: &Context) -> Vec<Product> {
        let mut conn = context.db_pool.get().unwrap();
    
        conn.exec(
            "SELECT * FROM product WHERE user_id = :user_id",
            params! { "user_id" => user_id },
        )
        .unwrap()
        .into_iter()
        .map(Product::from_row)
        .collect()
    }
    
    pub fn get_all(context: &Context) -> Vec<Product> {
        let mut conn = context.db_pool.get().unwrap();
    
        conn.exec("SELECT * FROM product", ())
            .unwrap()
            .into_iter()
            .map(Product::from_row)
            .collect()
    }

    pub fn insert(context: &Context, product: ProductInput) -> Result<Product, DBError> {
        let mut conn = context.db_pool.get().unwrap();
        let new_id = uuid::Uuid::new_v4().simple().to_string();

        let insert: Result<Option<Row>, DBError> = conn.exec_first(
            "INSERT INTO product(id, user_id, name, price) VALUES(:id, :user_id, :name, :price)",
            params! {
                "id" => &new_id,
                "user_id" => &product.user_id,
                "name" => &product.name,
                "price" => &product.price,
            },
        );

        match insert {
            Ok(_opt_row) => Ok(Product {
                id: new_id,
                user_id: product.user_id,
                name: product.name,
                price: product.price,
            }),
            Err(err) => Err(err)
        }
    }
}


