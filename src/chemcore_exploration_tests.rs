use chemcore::daylight::read_smiles;
use chemcore::molecule::{Atom, Element, Molecule};
use gamma::graph::Graph;
use gamma::traversal::{DepthFirst, Step};

#[test]
fn test_converts_methane() {
    let str = "C";

    let molecule = read_smiles(str, None).unwrap();
    let traversal = DepthFirst::new(&molecule, 0).expect("traversal error");

    // No bonds - nothing to traverse
    assert_eq!(traversal.collect::<Vec<_>>(), vec![]);

    // Graph trait
    assert_eq!(molecule.degree(0), Ok(0));
    assert_eq!(molecule.size(), 0);
    assert_eq!(molecule.order(), 1);

    // Molecule trait
    assert_eq!(
        molecule.atom(0),
        Ok(&Atom {
            isotope: None,
            element: Some(Element::C),
            hydrogens: 4,
            electrons: 0,
            parity: None,
        })
    );
    assert_eq!(molecule.charge(0), Ok(0.));
}

#[test]
fn test_converts_ethane() {
    let str = "CC";

    let molecule = read_smiles(str, None).unwrap();
    let traversal = DepthFirst::new(&molecule, 0).expect("traversal error");

    // No bonds - nothing to traverse
    assert_eq!(traversal.collect::<Vec<_>>(), vec![Step::new(0, 1, false),]);

    // Graph trait
    assert_eq!(molecule.degree(1), Ok(1));
    assert_eq!(molecule.size(), 1);
    assert_eq!(molecule.order(), 2);

    // Molecule trait
    assert_eq!(
        molecule.atom(0),
        Ok(&Atom {
            isotope: None,
            element: Some(Element::C),
            hydrogens: 3,
            electrons: 0,
            parity: None,
        })
    );
    assert_eq!(
        molecule.atom(1),
        Ok(&Atom {
            isotope: None,
            element: Some(Element::C),
            hydrogens: 3,
            electrons: 0,
            parity: None,
        })
    );
    assert_eq!(molecule.charge(0), Ok(0.));
    assert_eq!(molecule.bond_order(0, 1), Ok(1.));
}

#[test]
fn test_converts_propane() {
    let str = "CCC";

    let molecule = read_smiles(str, None).unwrap();
    let traversal = DepthFirst::new(&molecule, 0).expect("traversal error");

    // No bonds - nothing to traverse
    assert_eq!(
        traversal.collect::<Vec<_>>(),
        vec![Step::new(0, 1, false), Step::new(1, 2, false)]
    );

    // Graph trait
    assert_eq!(molecule.degree(2), Ok(1));
    assert_eq!(molecule.size(), 2);
    assert_eq!(molecule.order(), 3);

    // Molecule trait
    assert_eq!(
        molecule.atom(0),
        Ok(&Atom {
            isotope: None,
            element: Some(Element::C),
            hydrogens: 3,
            electrons: 0,
            parity: None,
        })
    );
    assert_eq!(
        molecule.atom(1),
        Ok(&Atom {
            isotope: None,
            element: Some(Element::C),
            hydrogens: 2,
            electrons: 0,
            parity: None,
        })
    );
    assert_eq!(
        molecule.atom(2),
        Ok(&Atom {
            isotope: None,
            element: Some(Element::C),
            hydrogens: 3,
            electrons: 0,
            parity: None,
        })
    );
    assert_eq!(molecule.charge(0), Ok(0.));
    assert_eq!(molecule.bond_order(0, 1), Ok(1.));
    assert_eq!(molecule.bond_order(1, 2), Ok(1.));
}
