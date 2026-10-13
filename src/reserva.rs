//reserva.rs
#[derive(PartialEq, Eq, Debug)]
pub struct Reserva {
    pub id: i32,
    pub nombre_paciente: String,
    pub id_medico: i32,
    pub fecha: String,
    pub hora: i32,
    pub estado: EstadoReserva,
}

#[derive(PartialEq, Eq, Debug)]
pub enum EstadoReserva {
    Pendiente,
    Confirmada,
    Cancelada,
}

impl Reserva {
    pub fn estado_pendiente(&self) -> bool {
        matches!(self.estado, EstadoReserva::Pendiente)
    }

    pub fn confirmar(&mut self) -> Result<(), String> {
        match self.estado {
            EstadoReserva::Pendiente => {
                self.estado = EstadoReserva::Confirmada;
                Ok(())
            }
            EstadoReserva::Cancelada => Err(String::from("La reserva ya esta Cancelada")),
            EstadoReserva::Confirmada => Err(String::from("La reserva ya esta Confirmada")),
        }
    }

    pub fn cancelar(&mut self) -> Result<(), String> {
        if matches!(
            self.estado,
            EstadoReserva::Pendiente | EstadoReserva::Confirmada
        ) {
            self.estado = EstadoReserva::Cancelada;
            return Ok(());
        }
        Err(String::from("La reserva ya esta cancelada"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crear_reserva(estado: EstadoReserva) -> Reserva {
        let test_reserva = Reserva {
            id: 1,
            nombre_paciente: String::from("paciente prueba"),
            id_medico: 1,
            fecha: String::from("23/10/26"),
            hora: 0900,
            estado,
        };
        test_reserva
    }

    #[test]
    fn confirmar() {
        let mut test_reserva = crear_reserva(EstadoReserva::Pendiente);
        let result = test_reserva.confirmar();
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn cancelar() {
        let mut test_reserva = crear_reserva(EstadoReserva::Cancelada);
        assert!(test_reserva.cancelar().is_err());
        assert_eq!(test_reserva.estado, EstadoReserva::Cancelada);
    }

    #[test]
    fn doble_cancelacion() {
        let mut test_reserva = crear_reserva(EstadoReserva::Pendiente);
        let result = test_reserva.cancelar();
        assert_eq!(result, Ok(()));
        assert!(test_reserva.cancelar().is_err());
    }
}
