use uuid::Uuid;

pub struct Entry {
    pub id: Uuid,
    pub date: String,
    pub title: String,
    pub content: String,
}

pub struct Entries(Vec<Entry>);

impl Entries {
    pub fn new() -> Entries {
        Entries(Vec::new())
    }

    pub fn add(&mut self, elem: Entry) {
        self.0.push(elem)
    }
}

impl IntoIterator for Entries {
    type Item = Entry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub fn build_entries() -> Entries {
    let mut built_entries = Entries::new();
    let entry1 = Entry {
        id: Uuid::new_v4(),
        date: "2022-03-26".to_owned(),
        title: "It lives!".to_owned(),
        content: "Barebones site up and running. It's super easy to build a release version with Trunk.  Just type trunk build --release, which creates a /dist folder. Drag all the files in the /dist folder to your web host and you're done.".to_owned(),
    };
    let entry2 = Entry {
        id: Uuid::new_v4(),
        date: "2022-04-03".to_owned(),
        title: "Breaking up the Code".to_owned(),
        content: "I've refactored the code a bit to take advantage of external modules. I've found this difficult to understand, but have settled on  a structure that I think I get now. Added a link to the GitHub repo for this site. Please feel free to study, critique, bash, use, whatever.".to_owned(),
    };
    let entry3 = Entry {
        id: Uuid::new_v4(),
        date: "2022-04-12".to_owned(),
        title: "Functional Components and use_state".to_owned(),
        content: "Broke the header and footer out into separate components, using Yew's notion of functional components. Added the uuid crate to generate a unique ID for each diary entry (displayed while I play, but will eventually hide this). Added the chrono crate and figured out Yew's equivalent of use_state to add the year to the copyright notice in the footer. Added some CSS to start formatting the site.".to_owned(),
    };
    built_entries.add(entry1);
    built_entries.add(entry2);
    built_entries.add(entry3);
    built_entries
}







