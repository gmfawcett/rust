fn main() {
    let f: fn() = fn () {
        #debug("This is a bare function")
    };
    f();
}