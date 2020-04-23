#[derive(Serialize, Deserialize)]
pub struct Amostra {
    pub id: Option<i32>,
    pub Nome: String,
    pub Matriz: String,
    pub Dopante: String,
    pub Autor: String,
    pub Local: String
}