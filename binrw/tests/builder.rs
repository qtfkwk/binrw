use binrw::BinrwNamedArgs;

#[test]
fn test() {
    #[derive(PartialEq, Debug)]
    struct NotClone;

    #[derive(BinrwNamedArgs)]
    struct Test<T: Clone> {
        blah: u32,
        not_copy: String,
        not_clone: NotClone,
        generic: T,
    }

    let x = Test::<String>::builder()
        .blah(3)
        .not_copy("a string here".into())
        .not_clone(NotClone)
        .generic("generic string :o".into())
        .finalize();

    assert_eq!(x.blah, 3);
    assert_eq!(x.not_copy, "a string here");
    assert_eq!(x.not_clone, NotClone);
    assert_eq!(x.generic, "generic string :o");
}
