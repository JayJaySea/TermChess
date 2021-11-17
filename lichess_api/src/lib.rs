use std::collections::HashMap;

pub fn test() {
    let resp = reqwest::blocking::get("https://httpbin.org/ip").unwrap()
        .json::<HashMap<String, String>>().unwrap();
    println!("{:#?}", resp);
}

#[cfg(test)]
mod test {
}
