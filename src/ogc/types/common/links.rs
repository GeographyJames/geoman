use crate::ogc::types::common::Link;

pub type Links = Vec<Link>;

pub trait Linked {
    fn insert_or_update(&mut self, other: &[Link]);
}

impl Linked for Links {
    fn insert_or_update(&mut self, others: &[Link]) {
        for link in others {
            self.iter_mut()
                .find(|l| l.rel == link.rel)
                .map(|l| link.href.clone_into(&mut l.href))
                .unwrap_or_else(|| self.push(link.to_owned()));
        }
    }
}
