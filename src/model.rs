pub mod teeline_dict {
    use std::collections::{BTreeMap, HashMap};
    use std::{fs, io};
    use std::path::Path;

    pub type Longhand = String;
    pub type Shorthand = String; // the path to the img file
    pub type TeelineDict = BTreeMap<Longhand, Shorthand>;
    pub fn load(location: &String) -> Result<TeelineDict, io::Error> {
        let mut dict = BTreeMap::new();
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
                // add the file in the dictionary. The name stem is the longhand word
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
}



#[cfg(test)]
mod test {
    use std::path::Path;
    use crate::model::teeline_dict;
    #[test]
    fn test_load() {
        let tl_dict = teeline_dict::load(&"test/resources/load".to_string()).unwrap();
        assert_eq!(3, tl_dict.len());
        // test each option
        let sh_told = Path::new("test/resources/load/told.png");
        let sh_much = Path::new("test/resources/load/much.png");
        let sh_lab = Path::new("test/resources/load/lab.png");

        assert_eq!(Path::new(tl_dict.get("told").unwrap()), sh_told);
        assert_eq!(Path::new(tl_dict.get("much").unwrap()), sh_much);
        assert_eq!(Path::new(tl_dict.get("lab").unwrap()), sh_lab)
    }
}