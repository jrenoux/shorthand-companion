
mod teeline_dict {
    use std::collections::HashMap;
    use std::{fs, io};

    type Longhand = String;
    type Shorthand = String; // the path to the img file
    type TeelineDict = HashMap<Longhand, Shorthand>;

    pub fn load(location: String) -> Result<TeelineDict, io::Error> {
        let mut dict : TeelineDict = TeelineDict::new();
        // open the path location
        let paths = fs::read_dir(location)?;

        // get all the files in this location
        for path_result in paths {
            let path = path_result?;
            // get the word
            match path.path().file_stem() {
                None => {
                    //TODO what do we do then?
                }
                // add the file in the dictonary. The name stem is the longhand word
                // and the whole path represents where the teeline picture is stored
                Some(name_stem) => {
                    // we add it to the dict
                    let lh:Longhand = name_stem.to_str().unwrap().to_string().clone();
                    let sh:Shorthand = path.path().to_str().unwrap().to_string().clone();
                    dict.insert(lh, sh);
                }
            }
        }
        Ok(dict)
    }

    pub fn add(lh: Longhand, sh: Shorthand) {

    }

    pub fn remove(lh: Longhand) {

    }

    pub fn edit(lh: Longhand, sh: Shorthand) {

    }
}



#[cfg(test)]
mod test {
    use super::teeline_dict;
    #[test]
    fn test_load() {
        let dict = teeline_dict::load("test/resources/load".to_string()).unwrap();
        assert_eq!(3, dict.len());
        // test each option
        let sh_told = "test/resources/load/told.png".to_string();
        let sh_much = "test/resources/load/much.png";
        let sh_lab = "test/resources/load/lab.png";
        assert_eq!(*dict.get("told").unwrap(), sh_told);
        assert_eq!(*dict.get("much").unwrap(), sh_much);
        assert_eq!(*dict.get("lab").unwrap(), sh_lab)
    }
}