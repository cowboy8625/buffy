#[test]
fn return_visibile() {
    use super::*;
    let mut test = Buffy::new("你好 hello");
    test.insert(0, "123");
    let result = test.get(1..5).map(|v| v.iter().map(Clone::clone).collect::<String>());
    assert_eq!(result, Some("23你好".to_string()));
}

#[test]
fn return_one() {
    use super::*;
    let mut test = Buffy::new("你好 hello");
    test.insert(0, "123");
    let result = test.get(3);
    assert_eq!(result, Some(&"你".to_string()));
}

#[test]
fn space_test() {
    use super::*;
    let mut test = Buffy::new("你好 hello");
    test.insert(0, "123");
    let result = test.get(5);
    assert_eq!(result, Some(&" ".to_string()));
}

#[test]
fn get_words() {
    use super::*;
    let mut test = Buffy::new("你好. How about you?");
    test.insert(0, "I know nothing about this words ");
    let left = test.words();
    let right =
        vec!["I", "know", "nothing", "about", "this", "words", "你", "好", "How", "about", "you"]
        .iter().map(ToString::to_string).collect::<Vec<String>>();
    assert_eq!(left, right);
}
