use fbin::FBin;

fn main() {
    let mut f = FBin::open("test");
    f.write(b"hahahaha");
    f.seek(2).unwrap();
    f.write(b"hiiii");
}