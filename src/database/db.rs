use tokio_postgres::{Client, Error, NoTls, Row};

pub struct Database {
    client: Client,
}

impl Database {
    pub async fn connect(connection_string: &str) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;
        
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Self { client })
    }

    pub async fn execute(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<u64, Error> {
        self.client.execute(query, params).await
    }

    pub async fn query(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Vec<Row>, Error> {
        self.client.query(query, params).await
    }

    pub async fn query_one(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Row, Error> {
        self.client.query_one(query, params).await
    }

    pub async fn query_opt(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Option<Row>, Error> {
        self.client.query_opt(query, params).await
    }

    pub async fn batch_execute(&self, query: &str) -> Result<(), Error> {
        self.client.batch_execute(query).await
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub async fn insert(&self, table: &str, columns: &[&str], values: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<u64, Error> {
        let placeholders: Vec<String> = (1..=values.len()).map(|i| format!("${}", i)).collect();
        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table,
            columns.join(", "),
            placeholders.join(", ")
        );
        self.execute(&query, values).await
    }

    pub async fn insert_returning(&self, table: &str, columns: &[&str], values: &[&(dyn tokio_postgres::types::ToSql + Sync)], returning: &str) -> Result<Row, Error> {
        let placeholders: Vec<String> = (1..=values.len()).map(|i| format!("${}", i)).collect();
        let query = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
            table,
            columns.join(", "),
            placeholders.join(", "),
            returning
        );
        self.query_one(&query, values).await
    }

    pub async fn update(&self, table: &str, set_columns: &[&str], values: &[&(dyn tokio_postgres::types::ToSql + Sync)], where_clause: &str, where_values: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<u64, Error> {
        let set_parts: Vec<String> = set_columns.iter().enumerate().map(|(i, col)| format!("{} = ${}", col, i + 1)).collect();
        let where_start = values.len() + 1;
        let all_values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = values.iter().chain(where_values.iter()).copied().collect();
        
        let query = format!(
            "UPDATE {} SET {} WHERE {}",
            table,
            set_parts.join(", "),
            where_clause.replace("$1", &format!("${}", where_start))
        );
        self.execute(&query, &all_values).await
    }

    pub async fn delete(&self, table: &str, where_clause: &str, where_values: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<u64, Error> {
        let query = format!("DELETE FROM {} WHERE {}", table, where_clause);
        self.execute(&query, where_values).await
    }

    pub async fn select(&self, table: &str, columns: &[&str], where_clause: Option<&str>, where_values: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Vec<Row>, Error> {
        let query = match where_clause {
            Some(clause) => format!("SELECT {} FROM {} WHERE {}", columns.join(", "), table, clause),
            None => format!("SELECT {} FROM {}", columns.join(", "), table),
        };
        self.query(&query, where_values).await
    }

    pub async fn select_one(&self, table: &str, columns: &[&str], where_clause: &str, where_values: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Row, Error> {
        let query = format!("SELECT {} FROM {} WHERE {}", columns.join(", "), table, where_clause);
        self.query_one(&query, where_values).await
    }

    pub async fn exists(&self, table: &str, where_clause: &str, where_values: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<bool, Error> {
        let query = format!("SELECT EXISTS(SELECT 1 FROM {} WHERE {})", table, where_clause);
        let row = self.query_one(&query, where_values).await?;
        Ok(row.get(0))
    }

    pub async fn count(&self, table: &str, where_clause: Option<&str>, where_values: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<i64, Error> {
        let query = match where_clause {
            Some(clause) => format!("SELECT COUNT(*) FROM {} WHERE {}", table, clause),
            None => format!("SELECT COUNT(*) FROM {}", table),
        };
        let row = self.query_one(&query, where_values).await?;
        Ok(row.get(0))
    }
}

#[macro_export]
macro_rules! params {
    ($($param:expr),* $(,)?) => {
        &[$(&$param as &(dyn tokio_postgres::types::ToSql + Sync)),*]
        as &[&(dyn tokio_postgres::types::ToSql + Sync)]
    };
}
