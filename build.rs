fn main() {
    cc::Build::new()
        .files(["CSTL/lib/type.c", "CSTL/lib/vector.c", "CSTL/lib/xstring.c", "CSTL/lib/list.c"])
        .std("c11")
        .compile("CSTL");
}
