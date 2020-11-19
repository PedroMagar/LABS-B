### LABS-B ###
Laboratory System - Backend

Backend escrito em Rust através do framework Actix.

### Compilando ###
Para compilar o código é necessário ter o rust instalado e fazer algumas pré configurações:
 + Configurar BD - Atualmente em MySQL, basta executar a última versão da query mysql (maior número) da pasta db/scripts
 + Configurar váriáveis de ambiente - O projeto depende da váriavel de ambiente DATABASE_URL configurada corretamente
 + Instalar o Diesel
 + Compilar o código

# BD
Para executar o backend, configure uma variavel de sistema de nome "DATABASE_URL" com a string de conexão.
Ex: DATABASE_URL=mysql://usuário:senha@localhost/labs?sslmode=disable

# Diesel
Também é necessário instalar o diesel com o banco de  dados necessário.
 + Ubuntu - comando: sudo apt install libpq-dev libmysqlclient-dev
 + Windows - não operacional 

Para Instalar o diesel execute o seguinte código no terminal (Ubuntu/Windows): cargo install diesel_cli --no-default-features --features postgres,mysql

### Execução ###


### Próximos Passos ###
# Login
Atualmente apesar de ter uma comunicação com o banco de dados funcional, ainda se encontra em desenvolvimento a parte de Login.

# Incompatibilidade com o Windows
Problemas com a integração da  API do MySQL no Windows com o diesel, caso não consiga corrigir planeja-se alterar o banco de dados do MySQL para o PostgreSQL.
