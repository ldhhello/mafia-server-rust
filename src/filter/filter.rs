pub struct Filter {
    word_list: Vec<String>
}

impl Filter {
    fn from(word_str: &str) -> Filter {
        let vec = word_str.split("\n")
            .map(|x| x.to_string())
            .filter(|x| x.len() > 0)
            .collect();

        return Filter {
            word_list: vec
        }
    }
    pub fn new() -> Filter {
        let word_str = include_str!("dirty_words.txt");

        return Self::from(word_str);
    }
    pub fn filter(&self, mut str: String) -> String {
        for word in &self.word_list {
            let mut now_pos = 0;
            loop {
                let Some(pos) = str[now_pos..].find(word.as_str()) else { break };
                str.replace_range(now_pos+pos..now_pos+pos+word.len(), "*".repeat(word.len()).as_str());
                now_pos += pos;
            }
        }

        return str;
    }
}

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let filter = Filter::from("hello\nworld\nabcd\n");

    assert_eq!(filter.word_list.len(), 3);
    assert_eq!(filter.word_list[0], "hello");
    assert_eq!(filter.word_list[1], "world");
    assert_eq!(filter.word_list[2], "abcd");

    let filter = Filter::from("hello\nworld\nabcd");

    assert_eq!(filter.word_list.len(), 3);
    assert_eq!(filter.word_list[0], "hello");
    assert_eq!(filter.word_list[1], "world");
    assert_eq!(filter.word_list[2], "abcd");

    assert_eq!(filter.filter("hello!!".into()), "*****!!");
    assert_eq!(filter.filter("hello!!hello".into()), "*****!!*****");
    assert_eq!(filter.filter("hell! hello hello world!".into()), "hell! ***** ***** *****!");
    assert_eq!(filter.filter("abcabcdabcdhello!".into()), "abc*************!");
    assert_eq!(filter.filter("".into()), "");
    assert_eq!(filter.filter("hell!!".into()), "hell!!");

    Ok(())
}