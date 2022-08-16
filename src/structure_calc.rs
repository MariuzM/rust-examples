pub fn structure_calc() {
    struct Structure {
        number_one: u32,
        number_two: u32,
    }

    impl Structure {
        fn area(&self) -> u32 {
            self.number_one * self.number_two
        }
    }

    let rect: Structure = Structure {
        number_one: 55,
        number_two: 44,
    };

    println!("rect: {}", rect.area());
}
