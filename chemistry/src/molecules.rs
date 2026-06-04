use crate::element::Element;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Molecule(pub HashMap<Element, u32>);

impl Molecule {
    pub fn new() -> Self {
        Molecule(HashMap::new())
    }
}

impl FromIterator<(Element, u32)> for Molecule {
    fn from_iter<I: IntoIterator<Item = (Element, u32)>>(iter: I) -> Self {
        let mut map = HashMap::new();

        for (elem, count) in iter {
            *map.entry(elem).or_insert(0) += count;
        }

        Molecule(map)
    }
}

pub struct Compound {
    molecule: Molecule,
    charge: i32,
}

pub struct Reaction {
    reactants: HashMap<Molecule, u32>,
    products: HashMap<Molecule, u32>,
}

#[cfg(test)]
mod test {
    use super::*;

    fn sodium_chlorine() {
        // Given
        let chlorine: Molecule = [(Element::Cl, 2)].into_iter().collect();
        let sodium: Molecule = [(Element::Na, 1)].into_iter().collect();
        assert_ne!(chlorine, sodium);
    }
}
