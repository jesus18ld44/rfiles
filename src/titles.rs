use std::fs::OpenOptions;
use std::path::Path;
use std::io;
use std::io::prelude::*;

pub fn get_titles<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let titles = std::fs::read_dir(path)?;
    let mut titles_vec = Vec::new();

    for title in titles {
        // titles_vec.push(title?.path().display().to_string().strip_prefix("D:/").unwrap().to_string());

        // titles_vec.push(title?
        //     .file_name()
        //     .to_str()
        //     .map(|x| Ok(x))
        //     .unwrap());

        // match title?.file_name().to_str() {
        //     None => panic!(""), 
        //     Some(t) => titles_vec.push(t.to_string().strip_prefix("D:\\"))
        // };

        title?.file_name().to_str()
            .filter(|t| &t[t.len()-3..] != "srt")
            .map(|t| titles_vec.push(t.to_string()));

    }

    Ok(titles_vec)
}

pub fn write_titles_to_file<P: AsRef<Path>>(file_name: P, titles: Vec<String>) -> io::Result<()>{

    // let mut file = File::create(file_name)?;
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open(file_name)
        .expect("unable to open");


    for title in titles {
        file.write(title.as_bytes())?;
        file.write(b"\n")?;
    }    

    Ok(())
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_vector_titles() {
        let path = Path::new("H:/Peliculas/");

        let movies = get_titles(path).unwrap();

        for movie in movies {
            println!("{}", movie);
        }
    }

    #[test]
    fn write_file() {
        let path = Path::new("H:/Peliculas");
        let movies = get_titles(path).unwrap();
        write_titles_to_file("D:\\nuevo.txt", movies).unwrap();
    }
}