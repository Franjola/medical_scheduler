// medico.rs
#[derive(Debug)]
pub struct Medico {
    pub id: i32,
    pub nombre: String,
    pub especialidad: String,
    pub tiempo_consulta: i32,
    pub disponibilidad: bool,
}
