use std::error::Error;
use std::fs;//处理和文件相关的任务
use std::env;

pub struct Config{
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool,
}
impl Config{
    pub fn new (args:&[String])->Result<Config, &'static str>{
        if args.len()<3{
            return Err("not enough arguments");
        }
        let query=args[1].clone();
        let filename=args[2].clone();
        let case_sensitive=env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename,case_sensitive })
    }
}
/**
 * 读取文件
 */
pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents=fs::read_to_string(config.filename)?;
    let results=if config.case_sensitive{
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };
    for line in results{
        println!("{}",line);
    }
    Ok(())
}
/**
 * 大小写敏感搜索
 */
pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut results=Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}
/**
 * 大小写不敏感搜索
 */
pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut results=Vec::new();
    let query=query.to_lowercase();
    for line in contents.lines(){
        if line.to_lowercase(). contains(&query){
            results.push(line);
        }
    }
    results
}
// TDD测试开发
#[cfg(test)]
mod tests{
    use super::*;
    /**
     * 大小写敏感
     */
    #[test]
    fn case_sensitive(){
        let query="duct";
        let contents="\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }
        /**
         * 大小写不敏感匹配
         */
        #[test]
        fn case_insensitive(){
            let query="rUst";
            let contents="\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
            assert_eq!(
                vec!["Rust:","Trust me."],
                search_case_insensitive(query,contents));
        }

}