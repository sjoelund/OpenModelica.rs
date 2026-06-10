// Auto-generated from MetaModelica source
/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF AGPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GNU AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium)
 * Public License (OSMC-PL) are obtained from OSMC, either from the above
 * address, from the URLs:
 * http://www.openmodelica.org or
 * https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica,
 * and in the OpenModelica distribution.
 *
 * GNU AGPL version 3 is obtained from:
 * https://www.gnu.org/licenses/licenses.html#GPL
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::Util;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Rational {
    /// numerator
    pub n: i32,
    /// denominator
    pub d: i32,
}

impl metamodelica::gc::MMTrace for Rational {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.n, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.d, __mmv)?;
        Ok(())
    }
}
impl Default for Rational {
    fn default() -> Self {
        Self {
            n: Default::default(),
            d: Default::default(),
        }
    }
}

pub type RATIONAL = Rational;

pub static ZERO: std::sync::LazyLock<Arc<Rational>> = std::sync::LazyLock::new(|| { Arc::new(Rational { n: 0, d: 1 }) });

pub static ONE: std::sync::LazyLock<Arc<Rational>> = std::sync::LazyLock::new(|| { Arc::new(Rational { n: 1, d: 1 }) });

pub fn isEqual(mut r1: Arc<Rational>, mut r2: Arc<Rational>) -> bool {
    let mut b: bool = r1.n.clone() == r2.n.clone() && r1.d.clone() == r2.d.clone();
    b
}

pub(crate) fn compare(mut r1: Arc<Rational>, mut r2: Arc<Rational>) -> i32 {
    let mut i: i32;
    let mut gn: i32 = Util::gcd(r1.n.clone(), r2.n.clone());
    let mut gd: i32 = Util::gcd(r1.d.clone(), r2.d.clone());
    i = Util::intCompare(intDiv(r1.n.clone(), gn.clone()) * intDiv(r2.d.clone(), gd.clone()), intDiv(r2.n.clone(), gn.clone()) * intDiv(r1.d.clone(), gd.clone()));
    i
}

pub fn toString(mut r: Arc<Rational>) -> ArcStr {
    let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(r.n.clone())); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(r.d.clone())); ArcStr::from(__mm_s) };
    r#str
}

pub(crate) fn normalize(mut r: Arc<Rational>) -> Arc<Rational> {
    let mut r: Arc<Rational> = r;
    if r.n.clone() == 0 {
        assign_field!(r.d = 1);
    } else if r.d.clone() < 0 {
        r = Arc::new(Rational { n: -(r.n.clone()), d: -(r.d.clone()) });
    }
    r
}

pub fn add(mut r1: Arc<Rational>, mut r2: Arc<Rational>) -> Arc<Rational> {
    let mut r: Arc<Rational>;
    let mut g: i32 = Util::gcd(r1.d.clone(), r2.d.clone());
    r = reduce(r1.n.clone() * intDiv(r2.d.clone(), g.clone()) + intDiv(r1.d.clone(), g.clone()) * r2.n.clone(), intDiv(r1.d.clone(), g.clone()) * r2.d.clone());
    r
}

pub(crate) fn neg(mut r: Arc<Rational>) -> Arc<Rational> {
    let mut s: Arc<Rational> = Arc::new(Rational { n: -(r.n.clone()), d: r.d.clone() });
    s
}

pub(crate) fn sub(mut r1: Arc<Rational>, mut r2: Arc<Rational>) -> Arc<Rational> {
    let mut r: Arc<Rational>;
    let mut g: i32 = Util::gcd(r1.d.clone(), r2.d.clone());
    r = reduce(r1.n.clone() * intDiv(r2.d.clone(), g.clone()) - intDiv(r1.d.clone(), g.clone()) * r2.n.clone(), intDiv(r1.d.clone(), g.clone()) * r2.d.clone());
    r
}

pub fn mul(mut r1: Arc<Rational>, mut r2: Arc<Rational>) -> Arc<Rational> {
    let mut r: Arc<Rational>;
    let mut g1: i32 = Util::gcd(r1.n.clone(), r2.d.clone());
    let mut g2: i32 = Util::gcd(r2.n.clone(), r1.d.clone());
    r = reduce(intDiv(r1.n.clone(), g1.clone()) * intDiv(r2.n.clone(), g2.clone()), intDiv(r1.d.clone(), g2.clone()) * intDiv(r2.d.clone(), g1.clone()));
    r
}

pub(crate) fn inv(mut r: Arc<Rational>) -> Arc<Rational> {
    let mut s: Arc<Rational> = Arc::new(Rational { n: Util::intSign(r.n.clone()) * r.d.clone(), d: intAbs(r.n.clone()) });
    s
}

pub(crate) fn div(mut r1: Arc<Rational>, mut r2: Arc<Rational>) -> Arc<Rational> {
    let mut r: Arc<Rational>;
    let mut g1: i32 = Util::gcd(r1.n.clone(), r2.n.clone());
    let mut g2: i32 = Util::gcd(r2.d.clone(), r1.d.clone());
    r = reduce(intDiv(r1.n.clone(), g1.clone()) * intDiv(r2.d.clone(), g2.clone()), intDiv(r1.d.clone(), g2.clone()) * intDiv(r2.n.clone(), g1.clone()));
    r
}

fn reduce(mut i1: i32, mut i2: i32) -> Arc<Rational> {
    let mut r: Arc<Rational>;
    let mut d: i32 = Util::gcd(i1.clone(), i2.clone());
    r = normalize(Arc::new(Rational { n: intDiv(i1.clone(), d.clone()), d: intDiv(i2.clone(), d.clone()) }));
    r
}


