use std::fs::File;
// use std::io::ErrorKind;

fn main() {
    // Matching on Different Errors
    // let f = File::open("hello.txt").unwrap_or_else(|error|{
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     }else {
    //         panic!("Problem creating the file: {:?}",error);
    //     }
    // });

    // Shortcuts for Panice on Error: unwrap and expect
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");
}
