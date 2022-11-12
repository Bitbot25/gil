use gil::Builder;

fn main() {
    let mut builder = Builder::default();
    let v0 = builder.salloc(4);
    let v1 = builder.salloc(4);
    let v3 = builder.add(v0, v1);
    dbg!(builder);
}
