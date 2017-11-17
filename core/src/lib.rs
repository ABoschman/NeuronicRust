#[allow(dead_code)]

fn hello_world() -> bool {
    true
}


#[cfg(test)]
mod tests {

    use hello_world;

    #[test]
    fn it_works() {
        assert!(hello_world());
    }
}
