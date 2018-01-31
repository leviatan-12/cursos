use std::string::FromUtf8Error;

fn bytecode_to_string_with_match(str: Vec<u8>) -> Result<String,FromUtf8Error> {
    match String::from_utf8(str) {
        Ok(str) => Ok(str.to_uppercase()),
        Err(err) => Err(err)
    }
}

fn bytecode_to_string(str: Vec<u8>) -> Result<String, FromUtf8Error>{
    String::from_utf8(str).map(|s| s.to_uppercase())
}

fn main() {
    let bytestring_con_error = vec!(130, 131, 132, 133);
    let bytestring_bueno = vec!(80, 82, 84, 85, 86);

    let s1_error = bytecode_to_string_with_match(bytestring_con_error.clone());
    let s1_bueno = bytecode_to_string_with_match(bytestring_bueno.clone());

    let s2_error = bytecode_to_string(bytestring_con_error.clone());
    let s2_bueno = bytecode_to_string(bytestring_bueno.clone());

    println!("Read the string {:?}", s1_error);
    println!("Read the string {:?}", s1_bueno);

    println!("Read the string {:?}", s2_error);
    println!("Read the string {:?}", s2_bueno);
}
