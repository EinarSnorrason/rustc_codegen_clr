use std::marker::PhantomData;

use crate::{cil_node::CILNode, cil_root::CILRoot};

#[derive(Debug)]
enum CILIterElemUnsafe<'a> {
    Node(*mut CILNode, PhantomData<&'a ()>),
    Root(*mut CILRoot, PhantomData<&'a ()>),
}
pub enum CILIterElemMut<'a> {
    Node(&'a mut CILNode),
    Root(&'a mut CILRoot),
}
impl<'a> From<CILIterElemUnsafe<'a>> for CILIterElemMut<'a> {
    fn from(value: CILIterElemUnsafe<'a>) -> Self {
        match value {
            CILIterElemUnsafe::Node(ptr, _) => CILIterElemMut::Node(unsafe { &mut *ptr }),
            CILIterElemUnsafe::Root(ptr, _) => CILIterElemMut::Root(unsafe { &mut *ptr }),
        }
    }
}

pub struct CILIterMut<'a> {
    elems: Vec<(usize, CILIterElemUnsafe<'a>)>,
}

impl<'a> Iterator for CILIterMut<'a> {
    type Item = CILIterElemMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (idx, elem) = self.elems.iter_mut().last()?;
            if *idx == 0 {
                *idx += 1;
                return Some(match elem {
                    CILIterElemUnsafe::Node(node, _) => {
                        CILIterElemUnsafe::Node(*node, PhantomData).into()
                    }
                    CILIterElemUnsafe::Root(root, _) => {
                        CILIterElemUnsafe::Root(*root, PhantomData).into()
                    }
                });
            }
            match elem {
                CILIterElemUnsafe::Node(node_ptr, _) => match unsafe { &mut **node_ptr } {
                    CILNode::CreateDelegate { obj, site: _ } => {
                        if idx == &1 {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(obj), PhantomData),
                            ));
                            continue;
                        } else {
                            self.elems.pop();
                            continue;
                        }
                    }
                    CILNode::Add(a, b)
                    | CILNode::And(a, b)
                    | CILNode::Div(a, b)
                    | CILNode::DivUn(a, b)
                    | CILNode::Rem(a, b)
                    | CILNode::RemUn(a, b)
                    | CILNode::Mul(a, b)
                    | CILNode::Eq(a, b)
                    | CILNode::LtUn(a, b)
                    | CILNode::Lt(a, b)
                    | CILNode::GtUn(a, b)
                    | CILNode::Gt(a, b)
                    | CILNode::Or(a, b)
                    | CILNode::Sub(a, b)
                    | CILNode::Shl(a, b)
                    | CILNode::Shr(a, b)
                    | CILNode::ShrUn(a, b)
                    | CILNode::XOr(a, b)
                    | CILNode::LDElelemRef { arr: a, idx: b } => match *idx {
                        1 => {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(&mut **a), PhantomData),
                            ));
                            continue;
                        }
                        2 => {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(&mut **b), PhantomData),
                            ));
                            continue;
                        }
                        _ => {
                            self.elems.pop();
                            continue;
                        }
                    },
                    CILNode::BlackBox(a)
                    | CILNode::ZeroExtendToISize(a)
                    | CILNode::ConvU64(a)
                    | CILNode::ConvI64(a)
                    | CILNode::ConvF64(a)
                    | CILNode::ConvF64Un(a)
                    | CILNode::ConvU32(a)
                    | CILNode::ConvI32(a)
                    | CILNode::ConvF32(a)
                    | CILNode::ConvISize(a)
                    | CILNode::MRefToRawPtr(a)
                    | CILNode::ConvU16(a)
                    | CILNode::ConvI16(a)
                    | CILNode::ConvU8(a)
                    | CILNode::ConvI8(a)
                    | CILNode::ZeroExtendToUSize(a)
                    | CILNode::LDFieldAdress { addr: a, field: _ }
                    | CILNode::LDField { addr: a, field: _ }
                    | CILNode::TransmutePtr { val: a, .. }
                    | CILNode::LDIndI8 { ptr: a }
                    | CILNode::LDIndU8 { ptr: a }
                    | CILNode::LDIndI16 { ptr: a }
                    | CILNode::LDIndU16 { ptr: a }
                    | CILNode::LDIndI32 { ptr: a }
                    | CILNode::LDIndU32 { ptr: a }
                    | CILNode::LDIndI64 { ptr: a }
                    | CILNode::LDIndU64 { ptr: a }
                    | CILNode::LDIndBool { ptr: a }
                    | CILNode::LDIndF32 { ptr: a }
                    | CILNode::LDIndF64 { ptr: a }
                    | CILNode::LdObj { ptr: a, obj: _ }
                    | CILNode::LDIndPtr {
                        ptr: a,
                        loaded_ptr: _,
                    }
                    | CILNode::LDIndISize { ptr: a }
                    | CILNode::LDIndUSize { ptr: a }
                    | CILNode::Not(a)
                    | CILNode::Neg(a)
                    | CILNode::LDLen { arr: a } => {
                        if *idx == 1 {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(&mut **a), PhantomData),
                            ));
                            continue;
                        } else {
                            self.elems.pop();
                            continue;
                        }
                    }
                    CILNode::LDLoc(_)
                    | CILNode::LDLocA(_)
                    | CILNode::LDArg(_)
                    | CILNode::LDArgA(_)
                    | CILNode::SizeOf(_)
                    | CILNode::LdcI32(_)
                    | CILNode::LdcF32(_)
                    | CILNode::LdcI64(_)
                    | CILNode::LdcF64(_)
                    | CILNode::LdcU32(_)
                    | CILNode::LdcU64(_)
                    | CILNode::LdStr(_)
                    | CILNode::LdFalse
                    | CILNode::LdTrue
                    | CILNode::LDStaticField(_)
                    | CILNode::LDFtn(_)
                    | CILNode::LDTypeToken(_)
                    | CILNode::LoadAddresOfTMPLocal
                    | CILNode::LoadTMPLocal
                    | CILNode::LocAllocAligned { tpe: _, align: _ }
                    | CILNode::LoadGlobalAllocPtr { alloc_id: _ }
                    | CILNode::PointerToConstValue(_) => {
                        self.elems.pop();
                        continue;
                    }
                    CILNode::Call { site: _, args }
                    | CILNode::CallVirt { args, site: _ }
                    | CILNode::NewObj { site: _, args } => {
                        if *idx - 1 < args.len() {
                            let arg = &mut args[*idx - 1];
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(arg), PhantomData),
                            ));
                            continue;
                        } else {
                            self.elems.pop();
                            continue;
                        }
                    }
                    CILNode::SubTrees(trees, node) => {
                        if *idx - 1 < trees.len() {
                            let arg = &mut trees[*idx - 1];
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Root(std::ptr::from_mut(arg), PhantomData),
                            ));
                            continue;
                        }
                        if *idx - 1 < trees.len() + 1 {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(
                                    std::ptr::from_mut(node.as_mut()),
                                    PhantomData,
                                ),
                            ));
                            continue;
                        } else {
                            self.elems.pop();
                            continue;
                        }
                    }
                    CILNode::TemporaryLocal(pack) => {
                        let (_, roots, node) = pack.as_mut();
                        if *idx - 1 < roots.len() {
                            let arg = &mut roots[*idx - 1];
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Root(std::ptr::from_mut(arg), PhantomData),
                            ));
                            continue;
                        }
                        if *idx - 1 < roots.len() + 1 {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(node), PhantomData),
                            ));
                            continue;
                        } else {
                            self.elems.pop();
                            continue;
                        }
                    }
                    CILNode::CallI(fn_sig_and_args) => {
                        if *idx - 1 < fn_sig_and_args.2.len() {
                            let arg = &mut fn_sig_and_args.2[*idx - 1];
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(arg), PhantomData),
                            ));
                            continue;
                        } else if *idx - 1 < fn_sig_and_args.2.len() + 1 {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(
                                    std::ptr::from_mut(&mut fn_sig_and_args.1),
                                    PhantomData,
                                ),
                            ));
                        } else {
                            self.elems.pop();
                            continue;
                        }
                    }
                    CILNode::GetStackTop => {
                        self.elems.pop();
                        continue;
                    }
                    CILNode::InspectValue { val, inspect } => {
                        if *idx - 1 < inspect.len() {
                            let arg = &mut inspect[*idx - 1];
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Root(std::ptr::from_mut(arg), PhantomData),
                            ));
                            continue;
                        }
                        if *idx - 1 < inspect.len() + 1 {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(
                                    std::ptr::from_mut(val.as_mut()),
                                    PhantomData,
                                ),
                            ));
                            continue;
                        } else {
                            self.elems.pop();
                            continue;
                        }
                    } // /_ => todo!("Can't iter node:{node:?}", node = unsafe { &**node_ptr }),
                },
                CILIterElemUnsafe::Root(root_ptr, _) => match unsafe { &mut **root_ptr } {
                    CILRoot::SetTMPLocal { value: tree }
                    | CILRoot::SetStaticField {
                        value: tree,
                        descr: _,
                    }
                    | CILRoot::STLoc { tree, local: _ }
                    | CILRoot::STArg { tree, arg: _ }
                    | CILRoot::Ret { tree }
                    | CILRoot::Pop { tree }
                    | CILRoot::BTrue { cond: tree, .. }
                    | CILRoot::BFalse { cond: tree, .. }
                    | CILRoot::Throw(tree) => {
                        if *idx == 1 {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(
                                    std::ptr::from_mut(&mut *tree),
                                    PhantomData,
                                ),
                            ));
                            continue;
                        } else {
                            self.elems.pop();
                            continue;
                        }
                    }
                    CILRoot::BEq { a, b, .. }
                    | CILRoot::BNe { a, b, .. }
                    | CILRoot::BLt { a, b, .. }
                    | CILRoot::BLtUn { a, b, .. }
                    | CILRoot::BGt { a, b, .. }
                    | CILRoot::BGtUn { a, b, .. }
                    | CILRoot::BLe { a, b, .. }
                    | CILRoot::BGe { a, b, .. } => {
                        if *idx == 1 {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(&mut *a), PhantomData),
                            ));
                            continue;
                        } else if *idx == 2 {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(&mut *b), PhantomData),
                            ));
                            continue;
                        } else {
                            self.elems.pop();
                            continue;
                        }
                    }
                    CILRoot::SetField {
                        addr: a, value: b, ..
                    }
                    | CILRoot::STIndI8(a, b)
                    | CILRoot::STIndI16(a, b)
                    | CILRoot::STIndI32(a, b)
                    | CILRoot::STIndI64(a, b)
                    | CILRoot::STIndISize(a, b)
                    | CILRoot::STIndF32(a, b)
                    | CILRoot::STIndF64(a, b)
                    | CILRoot::STObj {
                        tpe: _,
                        addr_calc: a,
                        value_calc: b,
                    } => match *idx {
                        1 => {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(&mut *a), PhantomData),
                            ));
                            continue;
                        }
                        2 => {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(&mut *b), PhantomData),
                            ));
                            continue;
                        }
                        _ => {
                            self.elems.pop();
                            continue;
                        }
                    },
                    CILRoot::CpBlk {
                        dst: a,
                        src: b,
                        len: c,
                    }
                    | CILRoot::InitBlk {
                        dst: a,
                        val: b,
                        count: c,
                    } => match *idx {
                        1 => {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(&mut *a), PhantomData),
                            ));
                            continue;
                        }
                        2 => {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(&mut *b), PhantomData),
                            ));
                            continue;
                        }
                        3 => {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(&mut *c), PhantomData),
                            ));
                            continue;
                        }
                        _ => {
                            self.elems.pop();
                            continue;
                        }
                    },
                    CILRoot::SourceFileInfo(_)
                    | CILRoot::GoTo { .. }
                    | CILRoot::JumpingPad { .. }
                    | CILRoot::VoidRet
                    | CILRoot::Nop
                    | CILRoot::ReThrow
                    | CILRoot::Break => {
                        self.elems.pop();
                        continue;
                    }
                    CILRoot::Call { site: _, args } | CILRoot::CallVirt { site: _, args } => {
                        if *idx - 1 < args.len() {
                            let arg = &mut args[*idx - 1];
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(arg), PhantomData),
                            ));
                            continue;
                        } else {
                            self.elems.pop();
                            continue;
                        }
                    }
                    CILRoot::CallI {
                        sig: _,
                        args,
                        fn_ptr,
                    } => {
                        if *idx - 1 < args.len() {
                            let arg = &mut args[*idx - 1];
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(arg), PhantomData),
                            ));
                            continue;
                        } else if *idx - 1 < args.len() + 1 {
                            *idx += 1;
                            self.elems.push((
                                0,
                                CILIterElemUnsafe::Node(std::ptr::from_mut(fn_ptr), PhantomData),
                            ));
                        } else {
                            self.elems.pop();
                            continue;
                        }
                    } /*_ => todo!(
                          "Unhandled iter elem {root_ptr:?}",
                          root_ptr = unsafe { &**root_ptr }
                      ),*/
                },
            }
        }
    }
}

impl<'a> CILIterMut<'a> {
    pub fn new_node(node: &'a mut CILNode) -> Self {
        Self {
            elems: vec![(
                0,
                CILIterElemUnsafe::Node(std::ptr::from_mut(node), PhantomData),
            )],
        }
    }

    pub fn new_root(root: &'a mut CILRoot) -> Self {
        Self {
            elems: vec![(
                0,
                CILIterElemUnsafe::Root(std::ptr::from_mut(root), PhantomData),
            )],
        }
    }
}

impl<'a> IntoIterator for &'a mut CILNode {
    type Item = CILIterElemMut<'a>;

    type IntoIter = CILIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CILIterMut::new_node(self)
    }
}

impl<'a> IntoIterator for &'a mut CILRoot {
    type Item = CILIterElemMut<'a>;

    type IntoIter = CILIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CILIterMut::new_root(self)
    }
}
#[test]
fn iter() {
    use crate::{call_site::CallSite, FnSig, Type};
    let mut node = CILNode::Add(
        Box::new(CILNode::Mul(
            Box::new(CILNode::LDLoc(0)),
            Box::new(CILNode::SizeOf(Box::new(Type::U8))),
        )),
        Box::new(CILNode::LDLoc(1)),
    );
    let mut iter = (&mut node).into_iter();
    assert!(matches!(
        iter.next(),
        Some(CILIterElemMut::Node(CILNode::Add(_, _)))
    ));
    assert!(matches!(
        iter.next(),
        Some(CILIterElemMut::Node(CILNode::Mul(_, _)))
    ));
    assert!(matches!(
        iter.next(),
        Some(CILIterElemMut::Node(CILNode::LDLoc(_)))
    ));
    assert!(matches!(
        iter.next(),
        Some(CILIterElemMut::Node(CILNode::SizeOf(_)))
    ));
    assert!(matches!(
        iter.next(),
        Some(CILIterElemMut::Node(CILNode::LDLoc(1)))
    ));
    assert!(matches!(iter.next(), None));
    let mut root = CILRoot::Call {
        site: CallSite::new(
            None,
            "bob".into(),
            FnSig::new(&[Type::I32, Type::F32], Type::Void),
            true,
        ),
        args: [CILNode::LdcI32(-77), CILNode::LdcF32(3.14159)].into(),
    };
    let mut iter = (&mut root).into_iter();
    assert!(matches!(
        iter.next(),
        Some(CILIterElemMut::Root(CILRoot::Call { .. }))
    ));
    assert!(matches!(
        iter.next(),
        Some(CILIterElemMut::Node(CILNode::LdcI32(-77)))
    ));
    assert!(matches!(
        iter.next(),
        Some(CILIterElemMut::Node(CILNode::LdcF32(3.14159)))
    ));
    assert!(matches!(iter.next(), None));
}
