use std::process::Command;

#[test]
fn test_hello_world() {

    let foo = Command::new("./target/debug/built-to-rust")
                      .arg("not-used")
                      .output()
                      .unwrap();

    let actual = String::from_utf8_lossy(&foo.stdout);

    assert_eq!(actual, "Hello, World!\n");
}