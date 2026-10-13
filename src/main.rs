// Medial scheduler for learn
//main.rs
mod agenda;
mod medico;
mod reserva;

use agenda::Agenda;
use medico::Medico;
use reserva::EstadoReserva;
use reserva::Reserva;

fn main() {
    let medico1 = Medico {
        id: 1,
        nombre: String::from("Carlos Perez"),
        especialidad: String::from("Medico General"),
        tiempo_consulta: 30,
        disponibilidad: true,
    };

    let reserva1 = Reserva {
        id: 1,
        nombre_paciente: String::from("Daniel Alvarez"),
        id_medico: medico1.id,
        fecha: String::from("05/10/2026"),
        hora: 1500,
        estado: EstadoReserva::Pendiente,
    };

    let reserva2 = Reserva {
        id: 2,
        nombre_paciente: String::from("Jose Perez"),
        id_medico: medico1.id,
        fecha: String::from("08/10/2026"),
        hora: 1400,
        estado: EstadoReserva::Confirmada,
    };

    let reserva3 = Reserva {
        id: 3,
        nombre_paciente: String::from("Juan Delgado"),
        id_medico: medico1.id,
        fecha: String::from("15/11/2026"),
        hora: 1730,
        estado: EstadoReserva::Pendiente,
    };

    let mut reservas = vec![reserva1, reserva2, reserva3];

    let mut contador = 0;

    for i in &reservas {
        println!(
            "Reserva de hora \n ID: {:?} Nombre paciente: {:?} Estado reserva: {:?},",
            i.id, i.nombre_paciente, i.estado
        );
        if i.estado_pendiente() {
            contador += 1;
        }
    }
    println!("Las reservas pendientes son: {contador}");

    let busqueda = buscar_reserva(&reservas, 2);
    match busqueda {
        Some(reserva) => println!("{:?}", reserva),
        _ => println!("No se encontro una reserva con esa ID"),
    }

    if let Some(reserva) = buscar_reserva_mut(&mut reservas, 3) {
        match reserva.confirmar() {
            Ok(()) => println!("La reserva se ha confirmado"),
            Err(e) => println!("{e}"),
        }
        println!("Reserva ID {}: {:?}", reserva.id, reserva.estado);
    }

    if let Some(reserva) = buscar_reserva_mut(&mut reservas, 3) {
        match reserva.cancelar() {
            Ok(()) => println!("Reserva cancelada correctamente"),
            Err(e) => println!("{e}"),
        }
    } else {
        println!("No se encontro la reserva");
    }

    let reserva4 = Reserva {
        id: 4,
        nombre_paciente: String::from("Jorge Delgado"),
        id_medico: medico1.id,
        fecha: String::from("15/11/2026"),
        hora: 1030,
        estado: EstadoReserva::Pendiente,
    };
    reservas.push(reserva4);
    let mut agenda = Agenda::new();
    for reserva in reservas {
        agenda.agregar_reserva(reserva).expect("Error");
    }
    //
}

// -- FUNCIONES --

fn buscar_reserva(reservas: &[Reserva], id: i32) -> Option<&Reserva> {
    reservas.iter().find(|reserva| reserva.id == id)
}

fn buscar_reserva_mut(reservas: &mut [Reserva], id: i32) -> Option<&mut Reserva> {
    reservas.iter_mut().find(|reserva| reserva.id == id)
}

// -- TESTS --
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prueba_reserva() {
        let reservas = crear_reserva(2);
        let reserva = buscar_reserva(&reservas, 2);
        assert!(reserva.is_some());
        assert_eq!(reserva.unwrap().id, 2);
    }

    #[test]
    fn reserva_no_existente() {
        let reservas = crear_reserva(3);
        assert!(buscar_reserva(&reservas, 4).is_none());
    }

    #[test]
    fn test_buscar_reserva_mut() {
        let mut reservas = crear_reserva(3);
        let reserva = buscar_reserva_mut(&mut reservas, 2).expect("No se encontro esta reserva");
        reserva.confirmar().unwrap();
        assert_eq!(reserva.estado, EstadoReserva::Confirmada);
        let reserva = buscar_reserva(&reservas, 2);
        match reserva {
            Some(r) => assert_eq!(r.estado, EstadoReserva::Confirmada),
            _ => panic!("Error no se encontro esta reserva"),
        }
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
