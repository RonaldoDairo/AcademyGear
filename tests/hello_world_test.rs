/*{use gtest::{Log, Program, System};

#[test]
fn hello_test() {
    // Inicializar el sistema
    let sys = System::new();
    sys.init_logger();

    // Crear un programa de prueba
    let program = Program::current(&sys);

    // Enviar un mensaje de inicialización
    program.send(2, String::from("INIT MESSAGE"));

    // Verificar que el programa fue inicializado correctamente
    let mut res = program.send(2, String::from("Hello"));
    // assert!(res.log().is_empty());
    assert!(!res.main_failed());

    // Enviar un mensaje adicional y verificar la respuesta
    res = program.send(2, String::from("Hello"));
    let expected_log = Log::builder().dest(2).payload(String::from("Hello"));
    assert!(res.contains(&expected_log));
}}*/
use gtest::{Log, Program, System};
use hello_world::InputMessages;

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send_bytes(2, String::from("Hello"));
    assert!(!res.main_failed());

    // test `SendHelloTo`
    let hello_recipient: u64 = 4;
    let res = program.send(
        2,
        InputMessages::SendHelloTo(hello_recipient.into()),
    );
    let expected_log = Log::builder()
        .dest(hello_recipient)
        .payload(String::from("Hello"));
    assert!(res.contains(&expected_log))
}