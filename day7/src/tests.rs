use super::*;

use super::parsers::*;

fn test_data() -> &'static str {
    include_str!("../example.txt")
}

fn test_cd(i: &str) -> (&str, &str) {
    let (i, cmd) = parse_cmd_line(i).unwrap();
    let Command::Cd { name } = cmd else { panic!() };
    (i, name)
}

fn test_ls(i: &str) -> (&str, &str) {
    let (i, cmd) = parse_cmd_line(i).unwrap();
    let Command::Ls { contents } = cmd else { panic!()};
    (i, contents)
}

fn test_dir_parse(i: &str) -> (&str, &str) {
    let (i, entry) = parse_ls_entry(i).unwrap();
    let LsEntry::Directory(dir) = entry else { panic!()};
    (i, dir)
}
fn test_file_parse(i: &str) -> (&str, String, usize) {
    let (i, entry) = parse_ls_entry(i).unwrap();
    let LsEntry::File(File { name, size }) = entry else { panic!() };
    (i, name, size)
}

#[test]
fn test_example_parse() {
    let i = test_data();

    let (i, dir_root) = test_cd(i);
    assert_eq!(dir_root, "/");
    let (i, contents_root) = test_ls(i);
    let (remain, dir_a) = test_dir_parse(contents_root);
    assert_eq!(dir_a, "a");
    let (remain, file_b_name, file_b_size) = test_file_parse(remain);
    assert_eq!(file_b_name, "b.txt");
    assert_eq!(file_b_size, 14_848_514);
    let (remain, file_c_name, file_c_size) = test_file_parse(remain);
    assert_eq!(file_c_name, "c.dat");
    assert_eq!(file_c_size, 8_504_156);
    let (remain, dir_d) = test_dir_parse(remain);
    assert_eq!(dir_d, "d");
    let (i, cd_a) = test_cd(i);
    let (i, contents_a) = test_ls(i);
    let (i, cd_e) = test_cd(i);
    let (i, contents_e) = test_ls(i);
    let (i, cd_parent1) = test_cd(i);
    let (i, cd_parent2) = test_cd(i);
    let (i, cd_d) = test_cd(i);
    let (i, contents_d) = test_ls(i);

    assert_eq!(
        contents_root,
        "dir a
14848514 b.txt
8504156 c.dat
dir d"
    );
    assert_eq!(cd_a, "a");
    assert_eq!(
        contents_a,
        "dir e
29116 f
2557 g
62596 h.lst"
    );
    assert_eq!(cd_e, "e");
    assert_eq!(contents_e, "584 i");
    assert_eq!(cd_parent1, "..");
    assert_eq!(cd_parent2, "..");
    assert_eq!(cd_d, "d");
    assert_eq!(
        contents_d,
        "4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"
    );
}
