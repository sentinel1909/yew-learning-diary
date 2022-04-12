use uuid::Uuid;

// struct to represent a diary entry
pub struct Entry<'a> {
    pub id: Uuid,
    pub date: &'a str,
    pub title: &'a str,
    pub content: &'a str,
}

// impl block which contains a new() method to create individual entries, stored in a vector
impl Entry<'_> {
    pub fn new() -> Vec<Entry<'static>> {
        let data = vec![
            Entry {
                id: Uuid::new_v4(),
                date: "2022-03-26",
                title: "It Lives!",
                content: "Barebones site up and running. It's super easy to build a release version with Trunk.  Just type trunk build --release, which creates a /dist folder. Drag all the files in the /dist folder to your web host and you're done."
            },
            Entry {
                id: Uuid::new_v4(),
                date: "2022-04-03",
                title: "Displaying Content with Iteration",
                content: "Diary content is stored in a vector of structs and displayed by iterating over the elements of the vector."
            },
            Entry {
                id: Uuid::new_v4(),
                date: "2022-04-11",
                title: "Breaking up the Code",
                content: "I've refactored the code a bit to take advantage of external modules. I've found this difficult to understand, but have settled on  a structure that I think I get now. Added a link to the GitHub repo for this site. Please feel free to study, critique, bash, use, whatever.",
            },
            Entry {
                id: Uuid::new_v4(),
                date: "2022-04-12",
                title: "Functional Components and use_state",
                content: "Broke the header and footer out into separate components, using Yew's notion of functional components. Added the uuid crate to generate a unique ID for each diary entry. Added the chrono crate and figured out Yew's equivalent of use_state to add the year to the copyright notice in the footer. Added some CSS to start formatting the site."
            }
        ];
        data
    }
}
