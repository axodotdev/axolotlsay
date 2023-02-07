use assert_cmd::Command;

#[test]
fn it_works() {
    let msg = "hello world!";
    let axo = "≽(◕ ᴗ ◕)≼";
    let mut cmd = Command::cargo_bin("axolotlsay").unwrap();
    let stdout = String::from_utf8(cmd.arg(msg).assert().get_output().stdout.clone()).unwrap();

    assert!(stdout.contains(axo));
    assert!(stdout.contains(msg));
}
