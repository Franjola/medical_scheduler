// modulo de agenda
use crate::reserva::EstadoReserva;
use crate::reserva::Reserva;

#[derive(Debug)]
pub struct Agenda {
    pub agenda: Vec<Reserva>,
}

impl Agenda {
    pub fn new() -> Self {
        Self { agenda: Vec::new() }
    }

    pub fn agregar_reserva(&mut self, reserva: Reserva) -> Result<(), String> {
        if self.agenda.iter().any(|reserva0| reserva0.id == reserva.id) {
            return Err(String::from("Error, ya existe una reserva con esta ID."));
        }
        let disponibilidad =
            self.horario_disponible(reserva.id_medico, reserva.hora, &reserva.fecha);
        if disponibilidad {
            self.agenda.push(reserva);
            return Ok(());
        }
        Err(String::from("Error no se pudo agendar la hora solicitada"))
    }

    fn horario_disponible(&self, medico: i32, hora: i32, fecha: &str) -> bool {
        for reserva in self.agenda.iter() {
            if reserva.id_medico == medico {
                if reserva.hora == hora && reserva.fecha == fecha {
                    if reserva.estado == EstadoReserva::Cancelada {
                        continue;
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn contar_pendientes(&self) -> usize {
        self.agenda
            .iter()
            .filter(|reserva| reserva.estado == EstadoReserva::Pendiente)
            .count()
    }

    pub fn pacientes_pendientes(&self) -> Vec<&str> {
        self.agenda
            .iter()
            .filter(|reserva| reserva.estado == EstadoReserva::Pendiente)
            .map(|reserva| reserva.nombre_paciente.as_str())
            .collect()
    }

    pub fn eliminar_canceladas(&mut self) {
        self.agenda
            .retain(|reserva| reserva.estado != EstadoReserva::Cancelada);
    }

    pub fn reprogramar_reserva(
        &mut self,
        id: i32,
        fecha: String,
        hora: i32,
        medico: i32,
    ) -> Result<(), String> {
        let consulta = self.horario_disponible(medico, hora, &fecha); // aqui tambien valida que la reserva no este ya cancalada
        if !self.medico_disponible(medico, hora, &fecha) {
            return Err(String::from("El medico no esta disponible"));
        }
        let reserva = match self.buscar_reserva_mut(id) {
            Some(reserva) => reserva,
            None => return Err(String::from("El horario no esta disponible")),
        };

        if consulta {
            reserva.hora = hora;
            reserva.fecha = fecha;
            reserva.estado = EstadoReserva::Pendiente;
            return Ok(());
        }
        Err(String::from("Horario no disponible"))
    }

    fn buscar_reserva_mut(&mut self, id: i32) -> Option<&mut Reserva> {
        self.agenda.iter_mut().find(|reserva| reserva.id == id)
    }

    fn medico_disponible(&self, medico: i32, hora: i32, fecha: &str) -> bool {
        todo!("Terminar funciona para ver si medico esta disponible")
    }
}

//
// -- TEST --

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn nuevas_funcionalidades() {
        let mut agenda = Agenda::new();
        agenda
            .agregar_reserva(crear_reserva(
                1,
                String::from("Eduardo Gomez"),
                1300,
                EstadoReserva::Pendiente,
            ))
            .expect("error");
        agenda
            .agregar_reserva(crear_reserva(
                2,
                String::from("Leduardo Gomez"),
                1400,
                EstadoReserva::Pendiente,
            ))
            .expect("error");
        agenda
            .agregar_reserva(crear_reserva(
                3,
                String::from("Peduardo Gomez"),
                1330,
                EstadoReserva::Cancelada,
            ))
            .expect("error");

        agenda.eliminar_canceladas();
        assert_eq!(agenda.agenda.len(), 2);
        assert!(
            agenda
                .agenda
                .iter()
                .all(|reserva| reserva.estado != EstadoReserva::Cancelada)
        );
        agenda.eliminar_canceladas();
        assert_eq!(agenda.agenda.len(), 2);
    }

    fn crear_reserva(id: i32, nombre: String, hora: i32, estado: EstadoReserva) -> Reserva {
        Reserva {
            id,
            nombre_paciente: nombre,
            id_medico: 1,
            fecha: String::from("15/11/2026"),
            hora,
            estado,
        }
    }
}
/*    #[test]
    fn prueba_horario() {
        let reserva1 = Reserva {
            id: 1,
            nombre_paciente: String::from("Daniel Alvarez"),
            id_medico: 1,
            fecha: String::from("05/10/2026"),
            hora: 1500,
            estado: EstadoReserva::Cancelada,
        };
        let reserva2 = Reserva {
            id: 2,
            nombre_paciente: String::from("Janiel Alvarez"),
            id_medico: 1,
            fecha: String::from("05/10/2026"),
            hora: 1500,
            estado: EstadoReserva::Pendiente,
        };

        let reservas = vec![reserva1, reserva2];
        assert!(!horario_disponible(&reservas, 1, 1500, "05/10/2026"));
    }

    #[test]
    fn reserva_exitosa() {
        let mut lista = Vec::new();
        let reserva = Reserva {
            id: 1,
            nombre_paciente: String::from("Daniel Alvarez"),
            id_medico: 1,
            fecha: String::from("05/10/2026"),
            hora: 1500,
            estado: EstadoReserva::Pendiente,
        };
        assert!(reserva_hora(&mut lista, reserva).is_ok());
        assert_eq!(lista.len(), 1);
    }

    #[test]
    fn hora_ocupada() {
        let mut reservas = crear_reserva(1);
        let reserva = Reserva {
            id: 2,
            nombre_paciente: String::from("Daniel Alvarez"),
            id_medico: 1,
            fecha: String::from("05/10/2026"),
            hora: 1500,
            estado: EstadoReserva::Pendiente,
        };
        //assert!(reserva_hora(&mut reservas, reserva).is_err());
    }

    #[test]
    fn reserva_id_duplicada() {
        let mut reservas = crear_reserva(1);
        let reserva = Reserva {
            id: 1,
            nombre_paciente: String::from("Daniel Alvarez"),
            id_medico: 1,
            fecha: String::from("05/10/2026"),
            hora: 1600,
            estado: EstadoReserva::Pendiente,
        };
        let result = reserva_hora(&mut reservas, reserva);
        assert!(result.is_err());
        assert_eq!(
            result,
            Err(String::from("Error, ya existe una reserva con esta ID."))
        );
        assert_eq!(reservas.len(), 1);
        assert_eq!(reservas[0].hora, 1500);
    }

    fn crear_reserva(num: i32) -> Vec<Reserva> {
        let mut reservas = Vec::new();
        for id in 1..=num {
            let reserva = Reserva {
                id,
                nombre_paciente: String::from("Daniel Alvarez"),
                id_medico: 1,
                fecha: String::from("05/10/2026"),
                hora: 1500,
                estado: EstadoReserva::Pendiente,
            };
            reservas.push(reserva);
        }
        reservas
    }
}
*/
