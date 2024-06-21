type Tag = String;

pub struct Person {
    email: String,
    tags: Vec<Tag>,
}

pub struct Team {
    name: String,
    members: Vec<Person>,
    tags: Vec<Tag>,
}

pub struct AdTeam {
    email: String,
    tags: Vec<Tag>,
}

pub struct Role {
    name: String,
    description: String,
    required_tag: Tag,
}