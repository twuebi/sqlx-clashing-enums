docker run -d -e POSTGRES_USER="postgres"  -e POSTGRES_PASSWORD="password"     -p 5432:5432       postgres:13.5
sleep 0.5
cargo run -p migrate
cargo run -p a
cargo run -p b
