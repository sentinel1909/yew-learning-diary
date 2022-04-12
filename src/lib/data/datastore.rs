// struct to represent a diary entry
pub struct Entry<'a> {
    pub id: i32,
    pub date: &'a str,
    pub title: &'a str,
    pub content: &'a str,
}

pub fn get_data() -> Vec<Entry<'static>> {
    let data = vec![
        Entry {
            id: 1,
            date: "2022-03-26",
            title: "It Lives!",
            content: "Barebones site up and running."
        },
        Entry {
            id: 2,
            date: "2022-04-03",
            title: "Multiple Content Entries!",
            content: "Diary content is stored in a data vector and displayed by iterating over the content."
        },
        Entry {
            id: 3,
            date: "2022-04-11",
            title: "Refactor to use External Modules",
            content: "I've refactored the code a bit to take advantage of external modules. I've found this difficult to understand, but have settled on  a structure that I think I get now. Added a link to the GitHub repo for this site. Please feel free to study, critique, bash, use, whatever.",
        }
    ];
    data
}