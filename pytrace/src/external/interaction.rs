use crate::internal;
use pyo3::prelude::*;
use std::sync::Arc;

pub trait ToInternal {
    fn to_internal(&self) -> internal::Primitive;
}

#[pyclass]
#[derive(Clone)]
pub struct Construct {
    pub contents: InterTree,
}

#[pyclass]
#[derive(Clone)]
pub struct Primitive {
    pub obj: Arc<dyn ToInternal>,
}

impl Primitive {
    pub fn extract(self) -> internal::Interaction {
        self.obj.to_internal().wrap()
    }

    pub fn wrap(self) -> Construct {
        InterTree::Item(self).wrap()
    }
}

#[derive(Copy, Clone)]
pub enum Interaction {
    Inter,
    Diff,
    Union,
}

#[derive(Clone)]
pub enum InterTree {
    Item(Primitive),
    Node(Interaction, Box<InterTree>, Box<InterTree>),
}

impl InterTree {
    pub fn wrap(self) -> Construct {
        Construct { contents: self }
    }

    pub fn inter(self, other: Self) -> Self {
        Self::Node(Interaction::Inter, Box::new(self), Box::new(other))
    }

    pub fn diff(self, other: Self) -> Self {
        Self::Node(Interaction::Diff, Box::new(self), Box::new(other))
    }

    pub fn union(self, other: Self) -> Self {
        Self::Node(Interaction::Union, Box::new(self), Box::new(other))
    }

    // See explanations below
    pub fn canonical(&self) -> Vec<internal::Interaction> {
        match self {
            Self::Item(p) => vec![p.clone().extract()],
            Self::Node(inter, a, b) => {
                let a_can = a.canonical();
                let b_can = b.canonical();
                match inter {
                    Interaction::Union => vec_union(&a_can, &b_can),
                    Interaction::Inter => {
                        // (A \ B) & (C \ D) = (A & C) \ (B | D)
                        let mut res = Vec::new();
                        for x in &a_can {
                            for y in &b_can {
                                let internal::Interaction(x_in, x_out) = x;
                                let internal::Interaction(y_in, y_out) = y;
                                res.push(internal::Interaction(
                                    vec_union(x_in, y_in),
                                    vec_union(x_out, y_out),
                                ));
                            }
                        }
                        res
                    }
                    Interaction::Diff => {
                        // (A \ B) \ (C \ D) = (A \ (B | C)) | ((A & D) \ B)
                        let mut res = a_can;
                        for y in b_can {
                            let acc = res;
                            res = Vec::new();
                            for x in acc {
                                let internal::Interaction(x_in, x_out) = &x;
                                let internal::Interaction(y_in, y_out) = &y;
                                for z in y_in {
                                    res.push(internal::Interaction(
                                        x_in.to_vec(),
                                        vec_union(&x_out, &[z.clone()]),
                                    ));
                                }
                                if !y_out.is_empty() {
                                    res.push(internal::Interaction(
                                        vec_union(&x_in, &y_out),
                                        x_out.to_vec(),
                                    ));
                                }
                            }
                        }
                        res
                    }
                }
            }
        }
    }
}

fn vec_union<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut res = Vec::new();
    for x in a {
        res.push(x.clone());
    }
    for x in b {
        res.push(x.clone());
    }
    res
}

#[pymethods]
impl Construct {
    pub fn inter(&self, other: &Construct) -> Self {
        Self {
            contents: self.contents.clone().inter(other.contents.clone()),
        }
    }

    pub fn union(&self, other: &Construct) -> Self {
        Self {
            contents: self.contents.clone().union(other.contents.clone()),
        }
    }

    pub fn diff(&self, other: &Construct) -> Self {
        Self {
            contents: self.contents.clone().diff(other.contents.clone()),
        }
    }
}


// About the InterTree::canonical() function
//
// The internal algorithm only manages complex objects expressed as :
// Union_i ( Inter_j (A_i,j) \ Union_k (B_i,k) )
// This corresponds to a collection (i.e. union) of [Vec, Vec],
// where the first (resp. second) Vec represents all objects inside
// (resp. outside) of which we need to be.
// The user of the Python library need not be aware of this restriction,
// as the interface allows arbitrary set operations.
// This is done using a tree (an InterTree) of operations, which is then
// converted into a canonical representation before being pushed to the
// scene.
//
// The process of translating InterTree -> Vec<[Vec, Vec]> is done recursively
// by InterTree::canonical()
// It relies on the following :
// (&, |, \ represent intersection, union, difference)
// * a leaf is already under canonical representation :
//        A -> { [A, ()] }
// * an union is easy to canonicalize
//        X | Y -> { X, Y }
// * A\B & C\D = A&C \ B|D
//        [A, B] & [C, D] -> [(A.., C..), (B.., D..)]
// * & is distributive on |
//        { [A, B], [A', B'] } & { [C, D], [C', D'] }
//     -> {
//             [(A.., C..), (B.., D..)],
//             [(A'.., C..), (B'.., D..)],
//             [(A.., C'..), (B.., D'..)],
//             [(A'.., C'..), (B'.., D'..)],
//        }
// And finally:
// * { X, Y } \ { Z, W } = { X\Z\W, Y\Z\W }
// * [A, B] \ [C, ()] = [A, (B.., C..)]
// * [A, B] \ [C, D] = { [A, (B.., C..)], [(A.., D..), B] }
// These three rules are applied by iteratively removing all elements
// of b_can at each step. The result becomes the basis of the next step.
