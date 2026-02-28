type Molecule: HashMap<Element, u32>;

struct Compound {
    molecule: Molecule,
    charge: i32,
}

struct Reaction {
    reactants: HashMap<Molecule, u32>,
    products: HashMap<Molecule, u32>,
}
