fn leak() {
    let data = vec![0; 1024];
    std::mem::forget(data);
}

fn main() {
    leak();
}

#[cfg(test)]
mod test {
    use super::leak;

    #[test]
    fn test_leak() {
        leak();
    }
}