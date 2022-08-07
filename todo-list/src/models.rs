use sqlx::{MySqlExecutor};

#[derive(Debug)]
pub struct Todo {
    name: String,
    is_checked: bool,
}

impl Todo {
    pub fn new(name: String, is_checked: bool) -> Self {
        Todo {
            name,
            is_checked
        }
    }

    pub async fn all<'a, D: MySqlExecutor<'a>>(pool: D) -> Result<Vec<Todo>, sqlx::Error> {
        let db_todos = sqlx::query!("SELECT * FROM todos").fetch_all(pool).await?;

        let mut todos: Vec<Todo> = Vec::new();

        for todo in db_todos {
            let todo = Todo::new(todo.name, match todo.is_checked {
                Some(value) =>  if value != 0 { true } else { false}
                None => false,
            });

            todos.push(todo);
        };

        Ok(todos)
    }

    pub async fn save<'a, D: MySqlExecutor<'a>>(&self, pool: D) -> Result<(), sqlx::Error>
    {
        sqlx::query("INSERT INTO todos(name, is_checked) values(?, ?)")
            .bind(self.name.clone())
            .bind(self.is_checked)
            .execute(pool).await?;

        Ok(())
    }
}