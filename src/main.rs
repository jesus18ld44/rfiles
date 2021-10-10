use movies::titles;
use std::path::Path;


pub fn main() {
    let path = Path::new("H:/Peliculas/");

    let movies = titles::get_titles(path).unwrap();
    
    for movie in movies {
        println!("{}", movie);
    }

}
