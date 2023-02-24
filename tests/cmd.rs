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

#[test]
fn test_non_ascii() {
    let msg = "Привет, мир!!11адинадин";
    let mut cmd = Command::cargo_bin("axolotlsay").unwrap();
    let response = cmd.arg(msg).assert().get_output().stdout.clone();
    let stdout = String::from_utf8(response).unwrap();
    let correct = r#"         +-------------------------+
         | Привет, мир!!11адинадин |
         +-------------------------+
        /
≽(◕ ᴗ ◕)≼
"#;

    assert_eq!(stdout.as_str(), correct);
}
#[test]
fn test_eeeeeee() {
    let msg = "éééééééééééé";
    let mut cmd = Command::cargo_bin("axolotlsay").unwrap();
    let response = cmd.arg(msg).assert().get_output().stdout.clone();
    let stdout = String::from_utf8(response).unwrap();
    let correct = r#"         +--------------+
         | éééééééééééé |
         +--------------+
        /
≽(◕ ᴗ ◕)≼
"#;
    assert_eq!(stdout.as_str(), correct);
}

#[test]
fn test_nihongo() {
    let msg = "やめてください";
    let mut cmd = Command::cargo_bin("axolotlsay").unwrap();
    let response = cmd.arg(msg).assert().get_output().stdout.clone();
    let stdout = String::from_utf8(response).unwrap();
    let correct = r#"         +----------------+
         | やめてください |
         +----------------+
        /
≽(◕ ᴗ ◕)≼
"#;
    assert_eq!(stdout.as_str(), correct);
}

#[test]
fn test_rtl() {
    let msg = "لله أكبر ";
    let mut cmd = Command::cargo_bin("axolotlsay").unwrap();
    let response = cmd.arg(msg).assert().get_output().stdout.clone();
    let stdout = String::from_utf8(response).unwrap();
    let correct = r#"         +-----------+
         | لله أكبر  |
         +-----------+
        /
≽(◕ ᴗ ◕)≼
"#;
    assert_eq!(stdout.as_str(), correct);
}
