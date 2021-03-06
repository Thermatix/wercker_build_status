

use curl::easy::Easy;

mod urls;

pub fn set_up(token: &String) -> Easy {

    use curl::easy::List;

    let mut easy = Easy::new();
    let mut list = List::new();

    // add authorisation header to curl client
    list.append(&format!("Authorization: Bearer {}",token)).unwrap();
    easy.http_headers(list).unwrap();
    easy
}


pub fn get_runs(curl: &mut Easy, pipline_id: &String, author: &String)  -> String {
    let url = urls::get::runs(&pipline_id, &author);
    get(curl,url )
}

fn get(curl: &mut Easy, url: String) -> String {
    use std::str;

    let mut data = Vec::new();
    
    curl.url(url.as_str()).unwrap();
    // add mechanism to transfer data recived data into a vec
    {
        let mut transfer = curl.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    // convert byte result into utf_8 string
    str::from_utf8(&data).unwrap().to_string()
}
