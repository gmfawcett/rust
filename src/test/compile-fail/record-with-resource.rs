// error-pattern: copying a noncopyable value

resource my_resource(x: int) {
    log(error, x);
}

fn main() {
    {
        let a = {x: 0, y: my_resource(20)};
        let b = {x: 2 with a};
        log(error, (a, b));
    }
}
