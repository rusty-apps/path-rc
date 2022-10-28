use pathrc::PathRC;

fn main() -> std::io::Result<()> {
    let dir = pathrc::get_path().unwrap();
    let mut path_rc = PathRC::new(dir);
    path_rc.search_files();
    path_rc.consolidate_aliases()
}
