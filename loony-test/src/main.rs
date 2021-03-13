type FnArg = Box<dyn Fn() -> String>;

struct User {
    describe: FnArg,
}

fn main() {
    let user = User{
        describe: Box::new(|| {
            return "Sankar".to_string()
        }),
    };

    let d = (user.describe)();
}