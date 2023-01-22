use crate::random_data;
// mod random_data;

#[derive(Debug)] // allow to print 'Data' object
struct Data {
    bool: bool,
    float: f32,
    int: i32,
    more_data: random_data::RandomData,
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
    let mut strc_1 = Data {
        bool: false,
        float: 1.2,
        int: 2,
        more_data: random_data::RandomData::new(),
    };

    print!("Test => {:?}", random_data::RandomData::new().val1);

    strc_1.bool = true;
    strc_1.float = 2.2;
    strc_1.int = 5;
    strc_1.more_data.val1 = 1.4;
    strc_1.more_data.val2 = 4;

    // println!("strc_1: ===> {:?}", strc_1);
    // println!("strc_1.more_data.val2: ===> {:?}", strc_1.more_data.val2);

    // let strc_2 = Data { int: 7, ..strc_1 };
    // println!("strc_2.int: ===> {:?}", strc_2.int);
}
