import Surreal from "surrealdb";

// Define the database configuration interface
interface DbConfig {
  url: string;
  namespace: string;
  database: string;
  username: string,
  password: string,
}

// Define the default database configuration
const DEFAULT_CONFIG: DbConfig = {
  url: "ws://127.0.0.1:8011/rpc",
  namespace: "rs",
  database: "artifact",
  username: "lei0lei",
  password: "12345678",
};

// Define the function to get the database instance
export async function getDb(config: DbConfig = DEFAULT_CONFIG): Promise<Surreal> {
  const db = new Surreal();

  try {
    await db.connect(config.url,{
      auth: {
        username: config.username,
        password: config.password,
      },
    }
    );
    await db.use({ namespace: config.namespace, database: config.database });
    return db;
  } catch (err) {
    console.error("Failed to connect to SurrealDB:", err instanceof Error ? err.message : String(err));
    await db.close();
    throw err;
  }
}