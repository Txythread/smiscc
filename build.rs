fn main() {
	cc::Build::new()
        .file("src/util/signpost.c")
        .compile("signpost_shim");
}
