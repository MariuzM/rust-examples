#[derive(Debug)] // allow to print 'Data' object
struct Data {
    bool: bool,
    float: f32,
    int: i32,
}

// impl std::fmt::Display for Data {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(
//             f,
//             "(value bool: {}, value float: {}, value int: {})",
//             self.bool, self.float, self.int
//         )
//     }
// }

pub fn structure() {
    let mut new_structure = Data {
        bool: false,
        float: 1.2,
        int: 2,
    };

    new_structure.bool = true;
    new_structure.float = 2.2;
    new_structure.int = 5;

    println!("{:?}", new_structure);
}
